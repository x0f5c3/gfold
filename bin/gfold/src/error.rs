use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("invalid color mode provided (exec \"--help\" for options): {0}")]
    InvalidColorMode(String),
    #[error("invalid display mode provided (exec \"--help\" for options): {0}")]
    InvalidDisplayMode(String),
    #[error("could not convert path (Path) to &str: {0}")]
    PathToStrConversionFailure(PathBuf),
    #[error("could not find home directory")]
    HomeDirNotFound,
}
