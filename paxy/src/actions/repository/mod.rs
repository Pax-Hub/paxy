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

    #[non_exhaustive]
    #[snafu(display("Could not remove:\n  {source}"))]
    CouldNotRemove { source: rm_repo::Error }
}

// region: IMPORTS

use std::path::PathBuf;

use snafu::Snafu;

// endregion: IMPORTS

// region: MODULES

pub mod downgrade;
pub mod install;
pub mod list;
pub mod search;
pub mod uninstall;
pub mod update;
pub mod rm_repo;

// endregion: MODULES

// region: RE-EXPORTS

#[allow(unused_imports)]
pub use downgrade::*;
pub(crate) use home;
#[allow(unused_imports)]
pub use install::*;
#[allow(unused_imports)]
pub use list::*;
#[allow(unused_imports)]
pub use search::*;
#[allow(unused_imports)]
pub use uninstall::*;
#[allow(unused_imports)]
pub use update::*;
#[allow(unused_imports)]
pub use rm_repo::*;

// endregion: RE-EXPORTS
