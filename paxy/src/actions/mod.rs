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

// region: IMPORTS

use snafu::Snafu;
use std::path::PathBuf;

// endregion: IMPORTS

// region: MODULES

pub mod package;
pub mod repository;

// endregion: MODULES

// region: RE-EXPORTS

// endregion: RE-EXPORTS
