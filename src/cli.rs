use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArguments {
    #[arg(short, long)]
    pub game_path: PathBuf,
}
