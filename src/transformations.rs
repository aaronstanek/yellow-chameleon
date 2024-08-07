use std::path::Path;

use std::process::{Command, Stdio};

use crate::commands::{
    git_add_all, git_commit, git_diff, git_push, ls, mkdir, mkdir_all, mv, rm, GitDiffResult,
};
use crate::get_json_configuration::SourceConfiguration;

pub(crate) fn apply_ignore_list(
    source_path: &Option<String>,
    ignore_list: Vec<String>,
) -> Result<(), String> {
    let read_dir = match source_path {
        None => String::from("source"),
        Some(s) => format!("source/{s}"),
    };
    for filename in ignore_list {
        let full_path = format!("{read_dir}/{filename}");
        match rm(&full_path) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    return Ok(());
}

pub(crate) fn apply_dest_path(
    source_path: &Option<String>,
    dest_path: &Option<String>,
) -> Result<(), String> {
    let temp_dir_path = match dest_path {
        None => return Ok(()),
        Some(relative_path) => format!("temp/{relative_path}"),
    };
    match mkdir_all(&temp_dir_path) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    let true_source_path = match source_path {
        None => String::from("source"),
        Some(path) => format!("source/{path}"),
    };
    let files_to_move_1 = match ls(&true_source_path) {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    for filename in files_to_move_1 {
        match mv(
            format!("{true_source_path}/{filename}").as_str(),
            &temp_dir_path,
        ) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    let files_to_move_2 = match ls("temp") {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    for filename in files_to_move_2 {
        match mv(format!("temp/{filename}").as_str(), &true_source_path) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    Ok(())
}

pub(crate) fn apply_lock_list(
    source_path: &Option<String>,
    lock_list: &Vec<String>,
) -> Result<(), String> {
    for lock_item in lock_list {
        let mut lock_item_parts: Vec<&str> = lock_item.split("/").collect();
        lock_item_parts.pop();
        let mut dir_tree = match source_path {
            None => String::from("source"),
            Some(path) => format!("source/{path}"),
        };
        for lock_item_part in lock_item_parts {
            dir_tree.push('/');
            dir_tree.push_str(lock_item_part);
            if Path::new(&dir_tree).is_symlink() || Path::new(&dir_tree).is_file() {
                match rm(&dir_tree) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
            if !(Path::new(&dir_tree).is_dir()) {
                match mkdir(&dir_tree) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
        }
        let read_from = format!("destination/{lock_item}");
        if !(Path::new(&read_from).exists()) {
            continue;
        }
        match mv(&read_from, &dir_tree) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    Ok(())
}

pub(crate) fn git_upload(
    source_path: &Option<String>,
    dest_repo_url: &str,
    dest_pat: &Option<String>,
) -> Result<(), String> {
    let cwd = match source_path {
        None => String::from("source"),
        Some(path) => format!("source/{path}"),
    };
    match git_add_all(&cwd) {
        Err(e) => return Err(e),
        Ok(_) => {}
    };
    match git_diff(&cwd) {
        Err(e) => return Err(e),
        Ok(diff) => match diff {
            GitDiffResult::NoChanges => return Ok(()),
            GitDiffResult::Changes => {}
        },
    };
    match git_commit(&cwd) {
        Err(e) => return Err(e),
        Ok(_) => {}
    };
    match git_push(&cwd, dest_repo_url, dest_pat) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    return Ok(());
}
