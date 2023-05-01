//! [gfold](https://github.com/nickgerace/gfold) is a CLI-driven application that helps you keep
//! track of multiple Git repositories. The source code uses private modules rather than leveraging
//! a library via `lib.rs`.

#![warn(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc)]

use env_logger::Builder;
use std::env;
use tracing_subscriber::filter::LevelFilter;
use tracing::debug;
use tracing::LevelFilter;

use crate::cli::CliHarness;

mod cli;
mod config;
mod display;

/// Initializes the logger based on the debug flag and `RUST_LOG` environment variable and uses
/// the [`CliHarness`] to generate a [`Config`](config::Config). Then, this calls
/// [`CliHarness::run()`].
fn main() -> anyhow::Result<()> {
    match env::var("RUST_LOG").is_err() {
        true => tracing_subscriber::fmt().pretty().,
        false => env_logger::init(),
    }
    debug!("initialized logger");

    let cli_harness = CliHarness::new();
    cli_harness.run()?;
    Ok(())
}
