use std::fs::{create_dir, create_dir_all, read, read_to_string, write, File};

use bson::{doc, Document};
use git2::Repository;
use log::{info, warn};

use crate::{ensure_path, home};

fn add_repo(repo: &str, name: &str) {
    let mut file = home!();
    file.push(".paxy");
    ensure_path!();
    file.push("repos.bson");
    let mut doc = if !file.is_file() {
        warn!("file not found. Creating");
        let doc = doc! {"paxy-official": "https://github.com/Pax-Hub/paxy-pkg-repository.git"};
        let mut buf = vec![];
        doc.to_writer(&mut buf).unwrap();
        write(file.clone(), buf).unwrap();
        doc
    } else {
        info!("Reading from pre-exisiting file");
        Document::from_reader(
            File::open(file.clone()).unwrap()
        )
        .unwrap()
    };
    doc.insert(name, repo);
    let mut buf = vec![];
    doc.to_writer(&mut buf).unwrap();
    write(file.clone(), buf).unwrap();
    file.pop();
    file.push("repos");
    file.push(name);
    ensure_path!(file);
    Repository::clone(repo, file).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_repo_norm_test() {
        add_repo("https://github.com/Pax-Hub/paxy-pkg-repository.git", "paxy");
        let mut file = home!();
        file.push(".paxy");
        file.push("repos.bson");
        let doc = Document::from_reader(
            File::open(file.clone()).unwrap()
        )
        .unwrap();
        assert_eq!(doc, doc! {"paxy-official": "https://github.com/Pax-Hub/paxy-pkg-repository.git", "paxy": "https://github.com/Pax-Hub/paxy-pkg-repository.git"});
    }
}