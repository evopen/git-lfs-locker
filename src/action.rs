use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn pick_repo() -> Result<Option<PathBuf>, String> {
    match nfd2::open_pick_folder(None).unwrap() {
        nfd2::Response::Okay(p) => {
            if std::fs::read_dir(&p)
                .ok()
                .unwrap()
                .into_iter()
                .map(|p| p.unwrap().path())
                .find(|p| p.file_name().unwrap().eq(".git"))
                .is_none()
            {
                return Err(String::from("Not a Git repo"));
            }

            Ok(Some(p))
        }
        nfd2::Response::OkayMultiple(_) => Ok(None),
        nfd2::Response::Cancel => Ok(None),
    }
}
fn get_lfs_files(path: &Path) -> Vec<String> {
    let output = std::process::Command::new("git")
        .arg("lfs")
        .arg("ls-files")
        .arg("-n")
        .current_dir(&path)
        .output()
        .expect("failed to run git lfs ls-files");
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|s| String::from(s))
        .collect()
}

fn check_git_repo_valid(path: &Path) {}
