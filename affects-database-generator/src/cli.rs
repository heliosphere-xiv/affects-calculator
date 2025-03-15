use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CliArguments {
    #[arg(short, long)]
    pub game_path: PathBuf,
    #[arg(short, long)]
    pub bnpc_path: String,
    #[arg(short, long, default_value_t = false)]
    pub pretty: bool,
    #[arg(short, long)]
    pub output: PathBuf,
}
