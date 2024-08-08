use std::fs::read_dir;
use std::process::{Command, Stdio};

use chrono::offset::Utc;

pub(crate) fn ls(directory: &str) -> Result<Vec<String>, String> {
    match read_dir(directory) {
        Err(_) => Err(String::from(
            "Internal Error: failed to list entires in a directory",
        )),
        Ok(entries_raw) => {
            let mut entries_refined: Vec<String> = Vec::new();
            for entry in entries_raw {
                match entry {
                    Err(_) => {
                        return Err(String::from(
                            "Internal Error: unable to read entry in directory",
                        ))
                    }
                    Ok(e) => match e.file_name().into_string() {
                        Err(_) => {
                            return Err(String::from(
                                "Internal Error: file name is not valid Unicode",
                            ))
                        }
                        Ok(s) => entries_refined.push(s),
                    },
                }
            }
            Ok(entries_refined)
        }
    }
}

pub(crate) fn mv(original_path: &str, move_to_dir: &str) -> Result<(), String> {
    match Command::new("mv")
        .arg(original_path)
        .arg(move_to_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Err(_) => Err(String::from("Internal Error: failed to call mv")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("mv returned nonzero exit code"))
            }
        }
    }
}

pub(crate) fn rm(path: &str) -> Result<(), String> {
    match Command::new("rm")
        .arg("-rf")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Err(_) => Err(String::from("Internal Error: failed to call rm")),
        Ok(_) => Ok(()),
    }
}

pub(crate) fn mkdir(path: &str) -> Result<(), String> {
    match Command::new("mkdir")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Err(_) => Err(String::from("Internal Error: failed to call mkdir")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("mkdir returned nonzero exit code"))
            }
        }
    }
}

pub(crate) fn mkdir_all(path: &str) -> Result<(), String> {
    match Command::new("mkdir")
        .arg("-p")
        .arg(path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        Err(_) => Err(String::from("Internal Error: failed to call mkdir")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("mkdir returned nonzero exit code"))
            }
        }
    }
}

fn git_config_impl(key: &str, value: &str) -> Result<(), String> {
    match Command::new("git")
        .arg("config")
        .arg("--global")
        .arg(key)
        .arg(value)
        .stdout(Stdio::null())
        .status()
    {
        Err(_) => Err(String::from("Internal Error: failed to call git config")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("git config returned nonzero exit code"))
            }
        }
    }
}

pub(crate) fn git_config(name: &str, email: &str) -> Result<(), String> {
    match git_config_impl("user.name", name) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    match git_config_impl("user.email", email) {
        Err(e) => return Err(e),
        Ok(_) => {}
    }
    return Ok(());
}

pub(crate) fn git_clone(repo_url: &str, pat: &Option<String>) -> Result<(), String> {
    let mut base_command = Command::new("git");
    let mut command_with_args = base_command
        .arg("clone")
        .arg("--filter=tree:0")
        .arg(repo_url)
        .arg("destination")
        .stdout(Stdio::null());
    match pat {
        None => {}
        Some(secret) => {
            command_with_args = command_with_args.env("GH_TOKEN", secret);
        }
    };
    match command_with_args.status() {
        Err(_) => Err(String::from("Internal Error: unable to call git clone")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("git clone returned a nonzero exit code"))
            }
        }
    }
}

pub(crate) fn git_add_all(cwd: &str) -> Result<(), String> {
    match Command::new("git")
        .arg("add")
        .arg("-A")
        .current_dir(cwd)
        .stdout(Stdio::null())
        .status()
    {
        Err(_) => return Err(String::from("Internal Error: unable to call git add")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("git add returned a nonzero exit code"))
            }
        }
    }
}

pub(crate) enum GitDiffResult {
    NoChanges,
    Changes,
}

pub(crate) fn git_diff(cwd: &str) -> Result<GitDiffResult, String> {
    match Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--name-only")
        .current_dir(cwd)
        .stdout(Stdio::null())
        .output()
    {
        Err(_) => return Err(String::from("Internal Error: unable to call git diff")),
        Ok(output) => {
            if !(output.status.success()) {
                return Err(String::from("git diff returned a nonzero exit code"));
            }
            // If git diff returns any sequence of characters that contains a character
            // that is not an ASCII control code or ASCII whitespace
            // then we can assume that the current repository state is
            // different than the most recent commit.
            //
            // If git diff returns a sequence not containing such a character,
            // then we can assume that the current repository state is,
            // the same as the most recent commit.
            //
            // In a UTF-8 byte sequence, we can search for characters that are not
            // ASCII control codes or ASCII whitespace by checking if any byte
            // in the sequence has a value greater than 32.
            for byte in output.stdout {
                if byte > 32 {
                    return Ok(GitDiffResult::Changes);
                }
            }
            return Ok(GitDiffResult::NoChanges);
        }
    }
}

pub(crate) fn git_commit(cwd: &str) -> Result<(), String> {
    let commit_message = Utc::now()
        .format("Sync at %Y-%m-%d %H:%M:%S UTC")
        .to_string();
    match Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .current_dir(cwd)
        .stdout(Stdio::null())
        .status()
    {
        Err(_) => return Err(String::from("Internal Error: unable to call git commit")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("git commit returned a nonzero exit code"))
            }
        }
    }
}

pub(crate) fn git_push(cwd: &str, repo_url: &str, pat: &Option<String>) -> Result<(), String> {
    let mut base_command = Command::new("git");
    let mut command_with_args = base_command.arg("push").arg(repo_url);
    match pat {
        None => {}
        Some(secret) => {
            command_with_args = command_with_args.env("GH_TOKEN", secret);
        }
    };
    command_with_args = command_with_args.current_dir(cwd).stdout(Stdio::null());
    match command_with_args.status() {
        Err(_) => Err(String::from("Internal Error: unable to call git push")),
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(String::from("git push returned a nonzero exit code"))
            }
        }
    }
}
