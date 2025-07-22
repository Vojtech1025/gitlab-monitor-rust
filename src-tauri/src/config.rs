#[derive(Debug, Clone)]
pub struct GitLabConfig {
    pub api_token: String,
    pub base_url: String,
    pub projects: Vec<String>,
}

// Load configuration from environment variables (and optional .env)
// This was previously inline in lib.rs; moved here for clarity.
pub fn load_config() -> Result<GitLabConfig, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let api_token = std::env::var("GITLAB_API_TOKEN")
        .map_err(|_| "GITLAB_API_TOKEN environment variable not set")?;

    let base_url =
        std::env::var("GITLAB_BASE_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());

    let projects_str = std::env::var("GITLAB_PROJECTS")
        .map_err(|_| "GITLAB_PROJECTS environment variable not set")?;

    let projects: Vec<String> = projects_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if projects.is_empty() {
        return Err("No projects specified in GITLAB_PROJECTS".into());
    }

    Ok(GitLabConfig {
        api_token,
        base_url,
        projects,
    })
}
