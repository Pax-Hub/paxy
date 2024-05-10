//region: IMPORTS
use bson::Document;
use crate::data::config::{Config,load_conf};
use std::{
    path::PathBuf,
    io::Write,
};
use snafu::{Snafu,ensure,OptionExt,ResultExt};
//endregion: IMPORTS

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to serialize repo data into byte stream {source}"))]
    FailedToSerialize {source: bson::ser::Error},
    
    #[snafu(display("failed to write to repo data file {source}"))]
    FailedToWriteData {source: std::io::Error},

    #[snafu(display("failed to delete directory {source}"))]
    FailedToDeleteDirectory {source: std::io::Error},
    
    #[snafu(display("repo not found"))]
    FailedToFindRepo {},
}

#[allow(dead_code)]
fn delete_repo(repo_name: &str) -> Result<(),Error> {
    let mut config = load_conf();
    let mut readable_data = config.repositories;
    
    readable_data.get(repo_name).context(FailedToFindRepoSnafu{})?;
    println!("{:?}",readable_data);
    readable_data.remove(repo_name);
    let mut buf = vec![];
    let rbd_result = readable_data.to_writer(&mut buf);

    rbd_result.context(FailedToSerializeSnafu{})?;

    let mut repos_file_path: PathBuf = home!();
    repos_file_path.push(".paxy");
    repos_file_path.push("repos.bson");
    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).open(repos_file_path).unwrap();
    let ftw_result = file.write_all(&buf);

    ftw_result.context(FailedToWriteDataSnafu{})?;

    config.repositories = readable_data;

    let mut config_toml_path: PathBuf = home!();

    config_toml_path.push(".paxy");
    config_toml_path.push("config.toml");
    
    let mut config_toml_file = std::fs::OpenOptions::new().write(true).truncate(true).open(config_toml_path).unwrap();
    config_toml_file.write_all(
        toml::to_string(&config)
            .unwrap()
            .as_bytes(),
    )
        .expect("Permission error");

    let mut repos_directory_path =  home!();
    repos_directory_path.push(".paxy");
    repos_directory_path.push("repos");
    repos_directory_path.push(repo_name);
    let repo_folder_path = repos_directory_path.as_path();
    // ⚠⚠⚠SECURITY VULNERABILITY INCOMING⚠⚠⚠
    let delete_dir_result = std::fs::remove_dir_all(repo_folder_path);
    delete_dir_result.context(FailedToDeleteDirectorySnafu{})?;
    
    Ok(())
}

//region: TESTS
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn delete_repo_test() {
	Config::default();
	assert_eq!(delete_repo("paxy-pkgs").is_ok(),true);
   }
}
//endregion: TESTS
