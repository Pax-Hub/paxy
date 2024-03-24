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

pub mod add_repo;
pub mod downgrade;
pub mod install;
pub mod list;
pub mod rm_repo;
pub mod search;
pub mod uninstall;
pub mod update;

// endregion: MODULES

// region: RE-EXPORTS

#[allow(unused_imports)]
pub use downgrade::*;
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

// endregion: RE-EXPORTS
#[macro_export]
macro_rules! home {
    () => {
        match home::home_dir() {
            Some(path) => path,
            None => panic!("Impossible to get your home dir!"),
        }
    };
}

#[macro_export]
macro_rules! ensure_path {
    () => {
        let mut file = home!();
        file.push(".paxy");
        if !file.is_dir() {
            ::std::fs::create_dir_all(file).expect("Inufficient permissions");
        }
    };
    ($path:ident) => {
        if !$path.is_dir() {
            ::std::fs::create_dir_all($path.clone()).expect("Inufficient permissions");
        }
    }
}