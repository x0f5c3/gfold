//! [gfold](https://github.com/nickgerace/gfold) is a CLI-driven application that helps you keep
//! track of multiple Git repositories. The source code uses private modules rather than leveraging
//! a library via `lib.rs`.

#![warn(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use env_logger::Builder;
use std::env;
use tracing::debug;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;

use crate::cli::CliHarness;
pub(crate) use error::CliError;

mod cli;
mod config;
mod display;
mod error;

/// Initializes the logger based on the debug flag and `RUST_LOG` environment variable and uses
/// the [`CliHarness`] to generate a [`Config`](config::Config). Then, this calls
/// [`CliHarness::run()`].
fn main() -> anyhow::Result<()> {
    match env::var("RUST_LOG") {
        Err(_) => tracing_subscriber::fmt()
            .pretty()
            .with_max_level(Level::ERROR)
            .init(),
        Ok(filt) => tracing_subscriber::fmt()
            .pretty()
            .with_env_filter(filt)
            .init(),
    }
    debug!("initialized logger");

    let cli_harness = CliHarness::new();
    cli_harness.run()?;
    Ok(())
}
