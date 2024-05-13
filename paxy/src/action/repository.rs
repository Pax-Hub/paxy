//! Handles repository-related actions.

pub fn handle_repository_action(repository_subcommand: RepositorySubcommand) -> Result<(), Error> {
    use crate::app::ui::console_template::cli::*;

    match repository_subcommand {
        RepositorySubcommand::List(repository_list_arguments) => {
            list::handle_repository_list_action(repository_list_arguments)
                .context(RepositoryListSnafu {})?
        }
        RepositorySubcommand::Search(repository_search_arguments) => {
            search::handle_repository_search_action(repository_search_arguments)
                .context(RepositorySearchSnafu {})?
        }
        RepositorySubcommand::Install(repository_install_arguments) => {
            install::handle_repository_install_action(repository_install_arguments)
                .context(RepositoryInstallSnafu {})?
        }
        RepositorySubcommand::Update(repository_update_arguments) => {
            update::handle_repository_update_action(repository_update_arguments)
                .context(RepositoryUpdateSnafu {})?
        }
        RepositorySubcommand::Uninstall(repository_uninstall_arguments) => {
            uninstall::handle_repository_uninstall_action(repository_uninstall_arguments)
                .context(RepositoryUninstallSnafu {})?
        }
        RepositorySubcommand::Downgrade(repository_downgrade_arguments) => {
            downgrade::handle_repository_downgrade_action(repository_downgrade_arguments)
                .context(RepositoryDowngradeSnafu {})?
        }
    }

    Ok(())
}

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
    if let Some(path) = path {
        if !path.is_dir() {
            ::std::fs::create_dir_all(path.clone()).expect("Inufficient permissions");
        }
    } else {
        let mut file = home!();
        file.push(".paxy");
        if !file.is_dir() {
            ::std::fs::create_dir_all(file).expect("Inufficient permissions");
        }
    }
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Could not list:\n  {source}"))]
    RepositoryList { source: list::Error },

    #[non_exhaustive]
    #[snafu(display("Could not search:\n  {source}"))]
    RepositorySearch { source: search::Error },

    #[non_exhaustive]
    #[snafu(display("Could not install:\n  {source}"))]
    RepositoryInstall { source: install::Error },

    #[non_exhaustive]
    #[snafu(display("Could not update:\n  {source}"))]
    RepositoryUpdate { source: update::Error },

    #[non_exhaustive]
    #[snafu(display("Could not uninstall:\n  {source}"))]
    RepositoryUninstall { source: uninstall::Error },

    #[non_exhaustive]
    #[snafu(display("Could not downgrade:\n  {source}"))]
    RepositoryDowngrade { source: downgrade::Error },
}

// endregion: ERRORS

// region: IMPORTS

use std::path::PathBuf;

use snafu::{ResultExt, Snafu};

use crate::app::ui::console_template::cli::RepositorySubcommand;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

pub mod downgrade;
pub mod install;
pub mod list;
pub mod search;
pub mod uninstall;
pub mod update;

// endregion: EXTERNAL-SUBMODULES

// region: RE-EXPORTS

pub(crate) use home;

// endregion: RE-EXPORTS
