use chrono::{DateTime, Utc};
use urlencoding;

use crate::{config::GitLabConfig, models::GitLabRelease, state::AppState};

// Fetch releases for a GitLab project (returns all releases)
pub async fn fetch_project_releases(
    client: &reqwest::Client,
    config: &GitLabConfig,
    project_path: &str,
) -> Result<Vec<GitLabRelease>, Box<dyn std::error::Error + Send + Sync>> {
    let encoded_project = urlencoding::encode(project_path);
    let url = format!(
        "{}/api/v4/projects/{}/releases",
        config.base_url, encoded_project
    );

    let response = client
        .get(&url)
        .header("PRIVATE-TOKEN", &config.api_token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch releases for {}: {}",
            project_path,
            response.status()
        )
        .into());
    }

    let releases_json: Vec<serde_json::Value> = response.json().await?;

    let mut releases = Vec::new();
    for r in releases_json {
        if let (Some(tag_name), Some(name), Some(created_at)) = (
            r.get("tag_name").and_then(|v| v.as_str()),
            r.get("name").and_then(|v| v.as_str()),
            r.get("created_at").and_then(|v| v.as_str()),
        ) {
            let created = DateTime::parse_from_rfc3339(created_at)?.with_timezone(&Utc);
            let released_at = r
                .get("released_at")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            releases.push(GitLabRelease {
                project_name: project_path
                    .split('/')
                    .next_back()
                    .unwrap_or(project_path)
                    .to_string(),
                project_path: project_path.to_string(),
                tag_name: tag_name.to_string(),
                name: name.to_string(),
                description: r
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                created_at: created,
                released_at,
                web_url: r
                    .get("_links")
                    .and_then(|l| l.get("self"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            });
        }
    }

    Ok(releases)
}

// Attribute extraction helpers
pub fn extract_tag_attribute(tag: &str) -> String {
    let lower = tag.to_lowercase();
    if let Some(idx) = lower.find("-v") {
        tag[..idx].to_string()
    } else {
        String::new()
    }
}

pub fn filter_latest_releases(releases: &[GitLabRelease]) -> Vec<GitLabRelease> {
    use std::collections::HashSet;
    let mut seen: HashSet<(String, String)> = HashSet::new();
    let mut filtered = Vec::new();

    for release in releases {
        let attr = extract_tag_attribute(&release.tag_name);
        let key = (release.project_path.clone(), attr);
        if seen.insert(key) {
            filtered.push(release.clone());
        }
    }

    filtered
}

pub fn detect_new_releases(
    current: &[GitLabRelease],
    previous: &[GitLabRelease],
) -> Vec<GitLabRelease> {
    let mut new_releases = Vec::new();

    for current_release in current {
        let is_new = !previous.iter().any(|prev| {
            prev.project_path == current_release.project_path
                && prev.tag_name == current_release.tag_name
        });

        if is_new {
            new_releases.push(current_release.clone());
        }
    }

    new_releases
}

// Fetch all releases for configured projects
pub async fn fetch_all_releases(
    state: &AppState,
) -> Result<Vec<GitLabRelease>, Box<dyn std::error::Error + Send + Sync>> {
    let mut releases = Vec::new();
    for project_path in &state.config.projects {
        match fetch_project_releases(&state.client, &state.config, project_path).await {
            Ok(mut project_releases) => {
                if project_releases.is_empty() {
                    println!("No releases found for project: {}", project_path);
                }
                releases.append(&mut project_releases);
            }
            Err(e) => {
                eprintln!("Error fetching releases for {}: {}", project_path, e);
            }
        }
    }

    // Sort releases by creation date (newest first)
    releases.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    let mut filtered_releases = filter_latest_releases(&releases);

    // Sort alphabetically by project name
    filtered_releases.sort_by(|a, b| a.project_name.cmp(&b.project_name));

    Ok(filtered_releases)
}
