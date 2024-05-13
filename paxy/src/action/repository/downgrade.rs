#[allow(unused)]
pub fn handle_repository_downgrade_action(
    repository_downgrade_arguments: RepositoryDowngradeArguments,
) -> Result<(), Error> {
    use crate::app::ui::console_template::cli::*;

    todo!();

    // Ok(())
}

// region: ERRORS

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// endregion: ERRORS

// region: IMPORTS

#[allow(unused)]
use snafu::{ResultExt, Snafu};

use crate::app::ui::console_template::cli::RepositoryDowngradeArguments;

// endregion: IMPORTS
