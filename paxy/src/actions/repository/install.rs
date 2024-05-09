#[allow(unused)]
fn add_repo(repo: &str, name: &str) {
    let mut file = super::home!();
    file.push(".paxy");
    ensure_path(None);
    file.push("repos.bson");
    let mut doc = if !file.is_file() {
        warn!("file not found. Creating");
        let doc = doc! {"paxy-pkgs": "https://github.com/Pax-Hub/paxy-pkg-repository.git"};
        let mut buf = vec![];
        doc.to_writer(&mut buf)
            .unwrap();
        write(file.clone(), buf).unwrap();
        doc
    } else {
        info!("Reading from pre-exisiting file");
        Document::from_reader(File::open(file.clone()).unwrap()).unwrap()
    };
    doc.insert(name, repo);
    let mut buf = vec![];
    doc.to_writer(&mut buf)
        .unwrap();
    write(file.clone(), buf).unwrap();
    file.pop();
    file.push("repos");
    file.push(name);
    ensure_path(Some(&file));
    if Repository::clone(repo, file.clone()).is_err() {
        remove_dir_all(file.clone()).unwrap();
        Repository::clone(repo, file);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn plugin(manifest: PathBuf) -> PathBuf {
    todo!()
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use std::{
    fs::{remove_dir_all, write, File},
    path::PathBuf,
};

use bson::{doc, Document};
use git2::Repository;
use log::{info, warn};
use snafu::Snafu;

use crate::actions::ensure_path;

// endregion: IMPORTS

// region: TESTS

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn repo_add_norm() {
        let mut repo_file = home!();
        repo_file.push(".paxy");
        repo_file.push("repos.bson");
        if repo_file.is_file() {
            fs::remove_file(&repo_file).unwrap();
        }
        add_repo("https://github.com/Pax-Hub/paxy-pkg-repository.git", "paxy");
        let doc = Document::from_reader(File::open(repo_file.clone()).unwrap()).unwrap();
        assert_eq!(
            doc,
            doc! {"paxy-pkgs": "https://github.com/Pax-Hub/paxy-pkg-repository.git", "paxy": "https://github.com/Pax-Hub/paxy-pkg-repository.git"}
        );
    }
}

// endregion: TESTS
