use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GitLabConfig {
    pub api_token: String,
    pub base_url: String,
    pub projects: Vec<String>,
}

// Get the directory where the executable is located
fn get_executable_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get executable directory")?;
    Ok(exe_dir.to_path_buf())
}

// Ensure .env file exists in the executable directory, provide helpful guidance if not
fn ensure_env_file_exists() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let exe_dir = get_executable_dir()?;
    let env_file_path = exe_dir.join(".env");
    let template_path = exe_dir.join("gitlab-config.example");
    
    // Check if .env exists
    if !env_file_path.exists() {
        if template_path.exists() {
            println!("Configuration needed: Please copy 'gitlab-config.example' to '.env' and edit it with your GitLab settings.");
            println!("Template found at: {}", template_path.display());
            println!("Create .env file at: {}", env_file_path.display());
        } else {
            println!("Configuration template not found. Please ensure 'gitlab-config.example' exists in the installation directory.");
        }
    }
    
    Ok(env_file_path)
}

// Load configuration from environment variables (and optional .env)
// This was previously inline in lib.rs; moved here for clarity.
pub fn load_config() -> Result<GitLabConfig, Box<dyn std::error::Error>> {
    // Try to load .env from executable directory first
    let env_file_path = ensure_env_file_exists()?;
    
    if env_file_path.exists() {
        // Load .env from the executable directory
        match dotenv::from_path(&env_file_path) {
            Ok(_) => println!("Loaded configuration from: {}", env_file_path.display()),
            Err(e) => eprintln!("Warning: Failed to load .env file: {}", e),
        }
    } else {
        // Fallback to default dotenv behavior (current directory)
        dotenv::dotenv().ok();
    }

    let api_token = std::env::var("GITLAB_API_TOKEN")
        .map_err(|_| {
            let exe_dir = get_executable_dir().unwrap_or_else(|_| PathBuf::from("."));
            let template_path = exe_dir.join("gitlab-config.example");
            
            if template_path.exists() {
                format!(
                    "GITLAB_API_TOKEN environment variable not set.\n\
                    \n\
                    SETUP REQUIRED:\n\
                    1. Copy 'gitlab-config.example' to '.env' in the installation directory\n\
                    2. Edit the .env file with your actual GitLab API token\n\
                    3. Replace <your-base-url> with your GitLab URL (or remove for gitlab.com)\n\
                    4. Replace <your-projects> with comma-separated project paths (e.g., group/project1,group/project2)\n\
                    5. Restart the application\n\
                    \n\
                    Files location:\n\
                    Template: {}\n\
                    Create: {}\n\
                    \n\
                    Example .env content:\n\
                    GITLAB_API_TOKEN=glpat-xxxxxxxxxxxxxxxxxxxx\n\
                    GITLAB_BASE_URL=https://gitlab.com\n\
                    GITLAB_PROJECTS=mygroup/project1,mygroup/project2",
                    template_path.display(),
                    env_file_path.display()
                )
            } else {
                "GITLAB_API_TOKEN environment variable not set. Please create a .env file with your GitLab configuration.".to_string()
            }
        })?;

    let base_url =
        std::env::var("GITLAB_BASE_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());

    let projects_str = std::env::var("GITLAB_PROJECTS")
        .map_err(|_| "GITLAB_PROJECTS environment variable not set. Please add comma-separated project paths to your .env file.")?;

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
