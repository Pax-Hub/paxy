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

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod package;
pub mod repository;

// endregion: MODULES

// region: RE-EXPORTS

// endregion: RE-EXPORTS
