#[derive(Debug, Snafu)]
#[non_exhaustive]
pub enum Error {
    #[non_exhaustive]
    #[snafu(display(""))]
    Dummy {},
}

// region: IMPORTS

use std::path::PathBuf;

use snafu::Snafu;

// endregion: IMPORTS
#[allow(dead_code)]
#[allow(unused_variables)]
fn plugin(manifest: PathBuf) -> PathBuf {
    todo!()
}
