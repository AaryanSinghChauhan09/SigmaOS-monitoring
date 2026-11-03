use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use monitoring::monitoring;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    let config = monitoring::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = monitoring::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-monitoring running");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
