use clap::{Parser, Subcommand};
mod port_discovery;
mod utils;
mod root_page;
mod server;
mod qrcode_gen;
use crate::port_discovery::discover_port;
use crate::server::start_server;


#[derive(Parser)]
#[command(name = "anydrop")]
#[command(version = "1.0")]
#[command(about = "A fast local file sharing utility in Rust")]
struct Cli {
    #[arg(short, long, default_value_t = 0)]
    port: u16,
    #[arg(short, long)]
    verbose: bool,

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
    let port = discover_port(cli.port, cli.verbose).await?;

    match cli.command {
        Commands::Send { file } => {
            start_server(server::ServerType::Send(file),port.1, port.0, port.2, cli.verbose).await?;
        },
        Commands::Recieve { path } => {
            start_server(server::ServerType::Recieve(path),port.1, port.0, port.2, cli.verbose).await?;
        }
    }
    
    


    


    Ok(())
}
