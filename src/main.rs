use anyhow::Result;
use clap::Parser;
use monitoring::monitoring;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
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
