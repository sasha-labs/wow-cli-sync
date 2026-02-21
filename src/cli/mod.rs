pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "wcs")]
#[command(version)]
#[command(about = "A minimal WoW addon updater with lockfile support.")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    silent: bool,
}

impl Cli {
    pub fn run(&self) {
        // let dir = config.addons_dir.as_deref().unwrap_or(".");
        let dir = "./test-data/";

        match &self.command {
            Commands::List => {
                let _ = commands::list::run(dir);
            }
            Commands::Install => todo!(),
            Commands::Update { dry_run } => todo!(),
            Commands::Export { file } => todo!(),
            Commands::Import { file } => todo!(),
            Commands::Search { query } => todo!(),
            Commands::Remove { name } => todo!(),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Install,
    Update {
        #[arg(long)]
        dry_run: bool,
    },
    List,
    Export {
        file: Option<String>,
    },
    Import {
        file: String,
    },
    Search {
        query: String,
    },
    Remove {
        name: String,
    },
}
