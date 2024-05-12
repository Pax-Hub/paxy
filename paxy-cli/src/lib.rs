//! Has the [`run_cli`] function

/// Calls the [`ui::run_common::<C>`] function supplying it with the commandline
///  interface template as a type. Any errors are thrown back to the calling
/// function. A debug message is then displayed conveying that the program is
/// being run in the CLI mode.
pub fn run_cli() -> Result<(), paxy::Error> {
    let (cli_input, _logging_worker_guards) = ui::run_common::<CliTemplate>()?;

    tracing::debug!(
        "Running in {} mode... {}",
        "CLI".blue(),
        console::Emoji("🔤", "")
    );

    if let Some(entity) = cli_input.entity {
        match entity {
            EntitySubcommand::Package(package_subcommand) => match package_subcommand {
                PackageSubcommand::List(package_list_arguments) => {
                    package::list::run_list(package_list_arguments)
                        .context(paxy::action::package::CouldNotListSnafu {})
                        .context(paxy::action::PackageSnafu)
                        .context(paxy::ActionSnafu)?
                }
                PackageSubcommand::Search(package_search_arguments) => {
                    package::search::run_search(package_search_arguments)
                        .context(paxy::action::package::CouldNotSearchSnafu {})
                        .context(paxy::action::PackageSnafu)
                        .context(paxy::ActionSnafu)?;
                }
                PackageSubcommand::Install(package_install_arguments) => {
                    package::install::run_install(package_install_arguments)
                        .context(paxy::action::package::CouldNotInstallSnafu {})
                        .context(paxy::action::PackageSnafu)
                        .context(paxy::ActionSnafu)?;
                }
                PackageSubcommand::Update(package_update_arguments) => {
                    package::update::run_update(package_update_arguments)
                        .context(paxy::action::package::CouldNotUpdateSnafu {})
                        .context(paxy::action::PackageSnafu)
                        .context(paxy::ActionSnafu)?;
                }
                PackageSubcommand::Uninstall(package_uninstall_arguments) => {
                    package::uninstall::run_uninstall(package_uninstall_arguments)
                        .context(paxy::action::package::CouldNotUninstallSnafu {})
                        .context(paxy::action::PackageSnafu)
                        .context(paxy::ActionSnafu)?;
                }
                PackageSubcommand::Downgrade(package_downgrade_arguments) => {
                    package::downgrade::run_downgrade(package_downgrade_arguments)
                        .context(paxy::action::package::CouldNotDowngradeSnafu {})
                        .context(paxy::action::PackageSnafu)
                        .context(paxy::ActionSnafu)?;
                }
            },
            EntitySubcommand::Repository(repository_subcommand) => match repository_subcommand {
                RepositorySubcommand::List(repository_list_arguments) => {
                    repository::list::run_list(repository_list_arguments)
                        .context(paxy::action::repository::CouldNotListSnafu {})
                        .context(paxy::action::RepositorySnafu)
                        .context(paxy::ActionSnafu)?;
                }
                RepositorySubcommand::Search(repository_search_arguments) => {
                    repository::search::run_search(repository_search_arguments)
                        .context(paxy::action::repository::CouldNotSearchSnafu {})
                        .context(paxy::action::RepositorySnafu)
                        .context(paxy::ActionSnafu)?;
                }
                RepositorySubcommand::Install(repository_install_arguments) => {
                    repository::install::run_install(repository_install_arguments)
                        .context(paxy::action::repository::CouldNotInstallSnafu {})
                        .context(paxy::action::RepositorySnafu)
                        .context(paxy::ActionSnafu)?;
                }
                RepositorySubcommand::Update(repository_update_arguments) => {
                    repository::update::run_update(repository_update_arguments)
                        .context(paxy::action::repository::CouldNotUpdateSnafu {})
                        .context(paxy::action::RepositorySnafu)
                        .context(paxy::ActionSnafu)?;
                }
                RepositorySubcommand::Uninstall(repository_uninstall_arguments) => {
                    repository::uninstall::run_uninstall(repository_uninstall_arguments)
                        .context(paxy::action::repository::CouldNotUninstallSnafu {})
                        .context(paxy::action::RepositorySnafu)
                        .context(paxy::ActionSnafu)?;
                }
                RepositorySubcommand::Downgrade(repository_downgrade_arguments) => {
                    repository::downgrade::run_downgrade(repository_downgrade_arguments)
                        .context(paxy::action::repository::CouldNotDowngradeSnafu {})
                        .context(paxy::action::RepositorySnafu)
                        .context(paxy::ActionSnafu)?;
                }
            },
        }
    }

    Ok(())
}

#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""), visibility(pub))]
    CliDummy {},
}

// region: IMPORTS

use owo_colors::OwoColorize;
use paxy::{
    action::{package, repository},
    app::ui,
};
use snafu::{ResultExt, Snafu};

// endregion: IMPORTS
