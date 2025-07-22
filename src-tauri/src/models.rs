use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabRelease {
    pub project_name: String,
    pub project_path: String,
    pub tag_name: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub web_url: String,
}
