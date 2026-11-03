use anyhow::Result;
use clap::Parser;
use monitoring::monitoring;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = monitoring::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = monitoring::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-monitoring operational");
    Ok(())
}
