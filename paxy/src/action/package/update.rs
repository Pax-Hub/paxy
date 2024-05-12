pub fn run_update(
    package_update_arguments: ui::cli_template::PackageUpdateArguments,
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
