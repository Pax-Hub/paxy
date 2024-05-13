//! Handles various package or repository actions that form the core
//! functionality of paxy, not the application related functionality - UI,
//! logging, config, OS, etc.

/// Receives console input and delegates actions
pub fn handle_action(console_input: CliTemplate) -> Result<(), Error> {
    use crate::app::ui::console_template::cli::*;

    if let Some(entity) = console_input.entity {
        match entity {
            EntitySubcommand::Package(package_subcommand) => {
                package::handle_package_action(package_subcommand).context(PackageSnafu)?;
            }
            EntitySubcommand::Repository(repository_subcommand) => {
                repository::handle_repository_action(repository_subcommand)
                    .context(RepositorySnafu)?;
            }
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
    #[snafu(display("Could not complete package action:\n  {source}"))]
    Package { source: package::Error },

    #[non_exhaustive]
    #[snafu(display("Could not complete repository action:\n  {source}"))]
    Repository { source: repository::Error },
}

// endregion: ERRORS

// region: IMPORTS

use snafu::{ResultExt, Snafu};

use crate::app::ui::console_template::cli::CliTemplate;

// endregion: IMPORTS

// region: EXTERNAL-SUBMODULES

pub mod package;
pub mod repository;

// region: EXTERNAL-SUBMODULES
