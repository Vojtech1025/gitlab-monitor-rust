use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{config::GitLabConfig, models::GitLabRelease};

#[derive(Debug)]
pub struct AppState {
    pub config: GitLabConfig,
    pub releases: Arc<Mutex<Vec<GitLabRelease>>>,
    pub previous_releases: Arc<Mutex<Vec<GitLabRelease>>>,
    pub client: reqwest::Client,
    pub has_new_releases: Arc<Mutex<bool>>,
}
