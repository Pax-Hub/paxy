#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Could not list:\n  {source}"))]
    CouldNotList { source: list::Error },

    #[non_exhaustive]
    #[snafu(display("Could not search:\n  {source}"))]
    CouldNotSearch { source: search::Error },

    #[non_exhaustive]
    #[snafu(display("Could not install:\n  {source}"))]
    CouldNotInstall { source: install::Error },

    #[non_exhaustive]
    #[snafu(display("Could not update:\n  {source}"))]
    CouldNotUpdate { source: update::Error },

    #[non_exhaustive]
    #[snafu(display("Could not uninstall:\n  {source}"))]
    CouldNotUninstall { source: uninstall::Error },

    #[non_exhaustive]
    #[snafu(display("Could not downgrade:\n  {source}"))]
    CouldNotDowngrade { source: downgrade::Error },
}

// region: IMPORTS

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod install;
pub mod list;
pub mod search;
pub mod uninstall;
pub mod update;
pub mod downgrade;

// endregion: MODULES

// region: RE-EXPORTS

pub use install::*;
pub use list::*;
pub use search::*;
pub use uninstall::*;
pub use update::*;
pub use downgrade::*;

// endregion: RE-EXPORTS
