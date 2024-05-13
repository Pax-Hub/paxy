#[allow(unused)]
pub fn handle_package_update_action(
    package_update_arguments: PackageUpdateArguments,
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

use crate::app::ui::console_template::cli::PackageUpdateArguments;

// endregion: IMPORTS
