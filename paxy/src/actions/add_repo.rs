use std::fs::read_to_string;

use bson::Document;

use crate::home;

fn add_repo(repo: String, name: String) {
    let mut file = home!();
    file.push(".paxy");
    file.push("repos.bson");
    let mut doc = Document::from_reader(read_to_string(file).unwrap().as_bytes()).unwrap();
    doc.insert(name, repo);
}