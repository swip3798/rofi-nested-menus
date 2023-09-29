use std::path::PathBuf;

use clap::Parser;
use lazy_static::lazy_static;

/// Simple CLI app to create nested rofi menus with json
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Json File
    pub path: PathBuf,

    /// Rofi theme
    #[arg(short, long)]
    pub theme: Option<PathBuf>,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
