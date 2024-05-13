//! Handles package related actions.

pub fn handle_package_action(package_subcommand: PackageSubcommand) -> Result<(), Error> {
    use crate::app::ui::console_template::cli::*;

    match package_subcommand {
        PackageSubcommand::List(package_list_arguments) => {
            list::handle_package_list_action(package_list_arguments).context(PackageListSnafu {})?
        }
        PackageSubcommand::Search(package_search_arguments) => {
            search::handle_package_search_action(package_search_arguments)
                .context(PackageSearchSnafu {})?
        }
        PackageSubcommand::Install(package_install_arguments) => {
            install::handle_package_install_action(package_install_arguments)
                .context(PackageInstallSnafu {})?
        }
        PackageSubcommand::Update(package_update_arguments) => {
            update::handle_package_update_action(package_update_arguments)
                .context(PackageUpdateSnafu {})?
        }
        PackageSubcommand::Uninstall(package_uninstall_arguments) => {
            uninstall::handle_package_uninstall_action(package_uninstall_arguments)
                .context(PackageUninstallSnafu {})?
        }
        PackageSubcommand::Downgrade(package_downgrade_arguments) => {
            downgrade::handle_package_downgrade_action(package_downgrade_arguments)
                .context(PackageDowngradeSnafu {})?
        }
    }

    Ok(())
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display("Could not list:\n  {source}"))]
    PackageList { source: list::Error },

    #[non_exhaustive]
    #[snafu(display("Could not search:\n  {source}"))]
    PackageSearch { source: search::Error },

    #[non_exhaustive]
    #[snafu(display("Could not install:\n  {source}"))]
    PackageInstall { source: install::Error },

    #[non_exhaustive]
    #[snafu(display("Could not update:\n  {source}"))]
    PackageUpdate { source: update::Error },

    #[non_exhaustive]
    #[snafu(display("Could not uninstall:\n  {source}"))]
    PackageUninstall { source: uninstall::Error },

    #[non_exhaustive]
    #[snafu(display("Could not downgrade:\n  {source}"))]
    PackageDowngrade { source: downgrade::Error },
}

// endregion: ERRORS

// region: IMPORTS

use snafu::{ResultExt, Snafu};

use crate::app::ui::console_template::cli::PackageSubcommand;

// endregion: IMPORTS

// region: MODULES

pub mod downgrade;
pub mod install;
pub mod list;
pub mod search;
pub mod uninstall;
pub mod update;

// endregion: MODULES
