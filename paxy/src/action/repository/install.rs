#[allow(unused)]
pub fn handle_repository_install_action(
    repository_install_arguments: RepositoryInstallArguments,
) -> Result<(), Error> {
    use crate::app::ui::console_template::cli::*;

    todo!();

    // Ok(())
}

#[allow(unused)]
fn add_repo(repo: &str, name: &str) {
    let mut file = repository::home!();
    file.push(".paxy");
    repository::ensure_path(None);
    file.push("repos.bson");
    let mut doc = if !file.is_file() {
        warn!("file not found. Creating");
        let doc = doc! {"paxy-official": "https://github.com/Pax-Hub/paxy-pkg-repository.git"};
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
    repository::ensure_path(Some(&file));
    Repository::clone(repo, file).unwrap();
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn plugin(manifest: PathBuf) -> PathBuf {
    todo!()
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// endregion: ERRORS

// region: IMPORTS

use std::{
    fs::{write, File},
    path::PathBuf,
};

use bson::{doc, Document};
use git2::Repository;
use log::{info, warn};
#[allow(unused)]
use snafu::{ResultExt, Snafu};

use crate::action::repository;
use crate::app::ui::console_template::cli::RepositoryInstallArguments;

// endregion: IMPORTS

// region: TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_repo_norm_test() {
        add_repo("https://github.com/Pax-Hub/paxy-pkg-repository.git", "paxy");
        let mut file = home!();
        file.push(".paxy");
        file.push("repos.bson");
        let doc = Document::from_reader(File::open(file.clone()).unwrap()).unwrap();
        assert_eq!(
            doc,
            doc! {"paxy-official": "https://github.com/Pax-Hub/paxy-pkg-repository.git", "paxy": "https://github.com/Pax-Hub/paxy-pkg-repository.git"}
        );
    }
}

// endregion: TESTS
