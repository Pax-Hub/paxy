pub fn run_uninstall(
    package_uninstall_arguments: ui::cli_template::PackageUninstallArguments,
) -> Result<(), Error> {
    todo!()
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use snafu::Snafu;

use crate::app::ui;

// endregion: IMPORTS
