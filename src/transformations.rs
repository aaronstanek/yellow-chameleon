use std::path::Path;

use crate::commands::{
    git_add_all, git_commit, git_diff, git_push, ls, mkdir, mkdir_all, mv, rm, GitDiffResult,
};

pub(crate) fn apply_ignore_list(
    source_path: &String,
    ignore_list: Vec<String>,
) -> Result<(), String> {
    for filename in ignore_list {
        let full_path = format!("{source_path}/{filename}");
        match rm(&full_path) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    return Ok(());
}

pub(crate) fn apply_dest_path(
    source_path: &String,
    dest_path: &Option<String>,
) -> Result<(), String> {
    let temp_dir_inner_path = match dest_path {
        None => return Ok(()),
        Some(relative_path) => format!("temp/{relative_path}"),
    };
    match mkdir_all(&temp_dir_inner_path) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    let files_to_move_1 = match ls(&source_path) {
        Err(e) => return Err(e),
        Ok(v) => v,
    };
    for filename in files_to_move_1 {
        match mv(
            format!("{source_path}/{filename}").as_str(),
            &temp_dir_inner_path,
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
        match mv(format!("temp/{filename}").as_str(), &source_path) {
            Err(e) => return Err(e),
            Ok(_) => {}
        }
    }
    Ok(())
}

pub(crate) fn apply_lock_list(source_path: &String, lock_list: &Vec<String>) -> Result<(), String> {
    for lock_item in lock_list {
        let mut lock_item_parts: Vec<&str> = lock_item.split("/").collect();
        lock_item_parts.pop();
        let mut dir_tree = source_path.clone();
        for lock_item_part in lock_item_parts {
            dir_tree.push('/');
            dir_tree.push_str(lock_item_part);
            let dir_tree_path = Path::new(&dir_tree);
            if dir_tree_path.is_symlink() || !(dir_tree_path.is_dir()) {
                match rm(&dir_tree) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
                match mkdir(&dir_tree) {
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
        }
        let write_to = format!("{source_path}/{lock_item}");
        match rm(&write_to) {
            Err(e) => return Err(e),
            Ok(_) => {}
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
    source_path: &String,
    dest_repo_url: &str,
    dest_pat: &str,
) -> Result<GitDiffResult, String> {
    match git_add_all(&source_path) {
        Err(e) => return Err(e),
        Ok(_) => {}
    };
    match git_diff(&source_path) {
        Err(e) => return Err(e),
        Ok(diff) => match diff {
            GitDiffResult::NoChanges => return Ok(GitDiffResult::NoChanges),
            GitDiffResult::Changes => {}
        },
    };
    match git_commit(&source_path) {
        Err(e) => return Err(e),
        Ok(_) => {}
    };
    match git_push(&source_path, dest_repo_url, dest_pat) {
        Err(e) => return Err(e),
        Ok(_) => {}
    };
    return Ok(GitDiffResult::Changes);
}
