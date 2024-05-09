#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Could not list:\n  {source}"))]
    PackageError { source: package::Error },

    #[non_exhaustive]
    #[snafu(display("Could not search:\n  {source}"))]
    RepositoryError { source: repository::Error },
}

#[macro_export]
macro_rules! home {
    () => {
        match home::home_dir() {
            Some(path) => path,
            None => panic!("Impossible to get your home dir!"),
        }
    };
}

#[inline]
pub fn ensure_path(path: Option<&PathBuf>) {
    if path.is_none() {
        let mut file = home!();
        file.push(".paxy");
        if !file.is_dir() {
            ::std::fs::create_dir_all(file).expect("Inufficient permissions");
        }
    } else {
        if !path
            .unwrap()
            .is_dir()
        {
            ::std::fs::create_dir_all(
                path.unwrap()
                    .clone(),
            )
            .expect("Inufficient permissions");
        }
    }
}

#[inline]
pub(crate) fn ensure_file<F: Fn(File)>(file: &PathBuf, f: F) {
    if !file.is_file() {
        f(File::create(file).unwrap())
    }
}

// region: IMPORTS

use std::{fs::File, path::{Path, PathBuf}};

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod package;
pub mod repository;

// endregion: MODULES

// region: RE-EXPORTS

// endregion: RE-EXPORTS
