#[allow(unused)]
pub fn handle_package_list_action(
    package_list_arguments: PackageListArguments,
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

use crate::app::ui::console_template::cli::PackageListArguments;

// endregion: IMPORTS
