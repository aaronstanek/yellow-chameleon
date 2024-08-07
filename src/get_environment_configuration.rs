use std::env::var;

use crate::sanitize_path::sanitize;

pub(crate) struct PatCredentials {
    pub secret: String,
    pub user: String,
}
pub(crate) struct EnvironmentConfiguration {
    pub source_path: Option<String>,
    pub dest_repo: String,

    pub source_pat: Option<PatCredentials>,
    pub dest_pat: Option<PatCredentials>,

    pub git_name: String,
    pub git_email: String,
}

fn get_required_var(name: &str, name_on_error: &str) -> Result<String, String> {
    match var(name) {
        Err(_e) => Err(format!("Expected input {name_on_error} to be defined")),
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
        Err(_e) => None,
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
        None => None,
        Some(s) => sanitize(s),
    };

    let dest_repo = match get_required_var("CAM_DEST_REPO", "destination-repository") {
        Err(e) => return Err(e),
        Ok(s) => s,
    };

    let source_pat_secret = get_optional_var("CAM_SOURCE_PAT_SECRET");
    let source_pat_user = get_optional_var("CAM_SOURCE_PAT_USER");
    let source_pat = match source_pat_secret {
        None => None,
        Some(secret) => match source_pat_user {
            None => {
                return Err(String::from(
                    "Expected source-pat-username to be defined because source-pat is defined",
                ));
            }
            Some(user) => Some(PatCredentials {
                secret: secret,
                user: user,
            }),
        },
    };

    let dest_pat_secret = get_optional_var("CAM_DEST_PAT_SECRET");
    let dest_pat_user = get_optional_var("CAM_DEST_PAT_USER");
    let dest_pat = match dest_pat_secret {
        None => None,
        Some(secret) => match dest_pat_user {
            None => {
                return Err(String::from("Expected destination-pat-username to be defined because destination-pat is defined"));
            }
            Some(user) => Some(PatCredentials {
                secret: secret,
                user: user,
            }),
        },
    };

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
        dest_repo: dest_repo,
        source_pat: source_pat,
        dest_pat: dest_pat,
        git_name: git_name,
        git_email: git_email,
    })
}