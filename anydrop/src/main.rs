use clap::{Parser, Subcommand};
mod port_discovery;
mod utils;
mod root_page;
mod server;
use crate::port_discovery::discover_port;
use crate::server::start_server;
mod enrcypted_stream;


#[derive(Parser)]
#[command(name = "anydrop")]
#[command(version = "1.0")]
#[command(about = "A fast local file sharing utility in Rust")]
pub struct Cli {
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short,long)]
    pub safe: bool,
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Send {
        #[arg(short, long)]
        file: String,
    },
    Recieve {
        #[arg(short, long)]
        path: Option<String>,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    let port = discover_port(&cli).await?;

    match &cli.command {
        Commands::Send { file } => {
            start_server(server::ServerType::Send(file.clone()),port.1, port.0, port.2, &cli).await?;
        },
        Commands::Recieve { path } => {
            start_server(server::ServerType::Recieve(path.clone()),port.1, port.0, port.2, &cli).await?;
        }
    }
    Ok(())
}
