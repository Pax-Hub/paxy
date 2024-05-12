pub fn run_update(
    repository_update_arguments: ui::cli_template::RepositoryUpdateArguments,
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
