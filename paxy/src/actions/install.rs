#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use std::path::Path;

use snafu::Snafu;

// endregion: IMPORTS

fn plugin(manifest: Box<Path>) -> Box<Path> {
    todo!()
}
