#[allow(unused)]
pub fn handle_package_install_action(
    package_install_arguments: PackageInstallArguments,
) -> Result<(), Error> {
    use crate::app::ui::console_template::cli::*;

    todo!();

    // Ok(())
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn plugin(manifest: PathBuf) -> PathBuf {
    todo!()
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

use std::path::PathBuf;

#[allow(unused)]
use snafu::{ResultExt, Snafu};

use crate::app::ui::console_template::cli::PackageInstallArguments;

// endregion: IMPORTS
