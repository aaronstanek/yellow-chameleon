use std::env::var;

use crate::sanitize_path::sanitize;

pub(crate) struct EnvironmentConfiguration {
    pub source_path: String,
    pub dest_repo_url: String,
    pub dest_pat: String,

    pub git_name: String,
    pub git_email: String,
}

fn get_required_var(name: &str, name_on_error: &str) -> Result<String, String> {
    match var(name) {
        Err(_) => Err(format!("Expected input {name_on_error} to be defined")),
        Ok(value) => {
            if value.len() == 0 {
                Err(format!("Expected input {name_on_error} to be defined"))
            } else {
                Ok(value)
            }
        }
    }
}

fn get_optional_var(name: &str) -> Option<String> {
    match var(name) {
        Err(_) => None,
        Ok(value) => {
            if value.len() == 0 {
                None
            } else {
                Some(value)
            }
        }
    }
}

pub(crate) fn get_environment_configuration() -> Result<EnvironmentConfiguration, String> {
    let source_path = match get_optional_var("CAM_SOURCE_PATH") {
        None => String::from("source"),
        Some(s) => match sanitize(s) {
            None => String::from("source"),
            Some(sanitized) => format!("source/{sanitized}"),
        },
    };

    let dest_repo = match get_required_var("CAM_DEST_REPO", "destination-repository") {
        Err(e) => return Err(e),
        Ok(s) => s,
    };

    let dest_pat_secret = match get_required_var("CAM_DEST_PAT_SECRET", "destination-pat") {
        Err(e) => return Err(e),
        Ok(s) => s,
    };

    let dest_pat_user = match get_required_var("CAM_DEST_PAT_USER", "destination-pat-username") {
        Err(e) => return Err(e),
        Ok(s) => s,
    };

    let dest_repo_url =
        format!("https://{dest_pat_user}:{dest_pat_secret}@github.com/{dest_repo}.git");

    let git_name = match get_required_var("CAM_GIT_NAME", "git-name") {
        Err(e) => return Err(e),
        Ok(s) => s,
    };

    let git_email = match get_required_var("CAM_GIT_EMAIL", "git-email") {
        Err(e) => return Err(e),
        Ok(s) => s,
    };

    Ok(EnvironmentConfiguration {
        source_path: source_path,
        dest_repo_url: dest_repo_url,
        dest_pat: dest_pat_secret,
        git_name: git_name,
        git_email: git_email,
    })
}
