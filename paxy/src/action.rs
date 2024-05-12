// region: ERRORS

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Could not complete package action:\n  {source}"))]
    PackageError { source: package::Error },

    #[non_exhaustive]
    #[snafu(display("Could not complete repository action:\n  {source}"))]
    RepositoryError { source: repository::Error },
}

// endregion: ERRORS

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

pub mod package;
pub mod repository;

// region: EXTERNAL-SUBMODULES

