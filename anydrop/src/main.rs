use clap::{Parser, Subcommand};
mod port_discovery;
mod utils;
mod root_page;
mod server;
use crate::port_discovery::discover_port;
use crate::server::start_server;
mod tcp_stream;


#[derive(Parser)]
#[command(name = "anydrop")]
#[command(version = "1.0")]
#[command(about = "A fast local file sharing utility in Rust")]
pub struct Cli {
    #[arg(short, long, default_value_t = 0, global = true)]
    pub port: u16,
    #[arg(short, long,  global = true)]
    pub verbose: bool,
    #[arg(short,long,  global = true)]
    pub safe: bool,
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Send {
        file: String,
    },
    Recive {
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
        Commands::Recive { path } => {
            start_server(server::ServerType::Recive(path.clone()),port.1, port.0, port.2, &cli).await?;
        }
    }
    Ok(())
}
