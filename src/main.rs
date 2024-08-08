mod commands;
mod get_environment_configuration;
mod get_json_configuration;
mod read_json_file;
mod sanitize_path;
mod transformations;

use std::path::Path;
use std::process::ExitCode;

use crate::commands::{git_clone, git_config, GitDiffResult};
use crate::get_environment_configuration::get_environment_configuration;
use crate::get_json_configuration::{get_destination_configuration, get_source_configuration};
use crate::transformations::{apply_dest_path, apply_ignore_list, apply_lock_list, git_upload};

fn main_impl() -> Result<GitDiffResult, String> {
    let environment_configuration = match get_environment_configuration() {
        Err(e) => return Err(e),
        Ok(c) => c,
    };

    if !(Path::new(&environment_configuration.source_path).is_dir()) {
        return Err(String::from("source path is not a directory"));
    }

    let source_configuration =
        match get_source_configuration(&environment_configuration.source_path) {
            Err(e) => return Err(e),
            Ok(c) => c,
        };

    match apply_ignore_list(
        &environment_configuration.source_path,
        source_configuration.ignore_list,
    ) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    match git_config(
        &environment_configuration.git_name,
        &environment_configuration.git_email,
    ) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    match git_clone(
        &environment_configuration.dest_repo_url,
        &environment_configuration.dest_pat,
    ) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    let destination_configuration = match get_destination_configuration() {
        Err(e) => return Err(e),
        Ok(c) => c,
    };

    match apply_dest_path(
        &environment_configuration.source_path,
        &destination_configuration.path,
    ) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    match apply_lock_list(
        &environment_configuration.source_path,
        &destination_configuration.lock_list,
    ) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    git_upload(
        &environment_configuration.source_path,
        &environment_configuration.dest_repo_url,
        &environment_configuration.dest_pat,
    )
}

fn main() -> ExitCode {
    match main_impl() {
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("Sync stopped due to an error");
            ExitCode::FAILURE
        }
        Ok(git_diff_result) => {
            println!(
                "{}",
                match git_diff_result {
                    GitDiffResult::NoChanges =>
                        "No changes detected in source. Destination repository was not updated.",
                    GitDiffResult::Changes => "Sync successful",
                }
            );
            ExitCode::SUCCESS
        }
    }
}
