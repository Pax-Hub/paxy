use csv::Writer;

use crate::home;

// TODO verify if repo is real
fn add_repo(repo: &str) {
    let mut file = home!();
    file.push(".paxy");
    file.push("repos.csv");
    let mut repos = Writer::from_path(file.as_path()).unwrap();
    repos.write_field(repo).unwrap();
    
}