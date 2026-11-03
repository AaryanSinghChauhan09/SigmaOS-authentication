//! SigmaOS-authentication - SigmaOS Component CLI
//!
//! Command-line interface for SigmaOS-authentication component.

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use authentication::{Component, ComponentStatus, Config, authentication};

#[derive(Parser)]
#[command(name = "authentication")]
#[command(about = "SigmaOS SigmaOS-authentication component", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the component
    Init {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Start the component
    Start,
    /// Stop the component
    Stop,
    /// Get component status
    Status,
    /// Run the component
    Run {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::default();
    let mut component = authentication::new(config);

    match cli.command {
        Commands::Init { debug } => {
            if debug {
                println!("Initializing SigmaOS-authentication in debug mode...");
            } else {
                println!("Initializing SigmaOS-authentication...");
            }
            component.initialize()?;
            println!("SigmaOS-authentication initialized successfully");
        }
        Commands::Start => {
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-authentication started successfully");
        }
        Commands::Stop => {
            component.stop()?;
            println!("SigmaOS-authentication stopped successfully");
        }
        Commands::Status => {
            let status = component.status();
            println!("SigmaOS-authentication status: {:?}", status);
        }
        Commands::Run { debug } => {
            if debug {
                println!("Running SigmaOS-authentication in debug mode...");
            } else {
                println!("Running SigmaOS-authentication...");
            }
            component.initialize()?;
            component.start()?;
            println!("SigmaOS-authentication is running. Press Ctrl+C to stop.");
            
            // Keep running
            tokio::runtime::Runtime::new()?.block_on(async {
                tokio::signal::ctrl_c().await.unwrap();
                info!("Received Ctrl+C, shutting down...");
            });
            
            component.stop()?;
            println!("SigmaOS-authentication stopped");
        }
    }

    Ok(())
}
