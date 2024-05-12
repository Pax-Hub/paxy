pub fn run_list(
    package_list_arguments: ui::cli_template::PackageListArguments,
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
