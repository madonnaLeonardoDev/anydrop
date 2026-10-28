use clap::Parser;
mod port_discovery;
mod utils;
mod root_page;
mod server;
mod qrcode_gen;
use crate::port_discovery::discover_port;
use crate::server::start_server;

#[derive(Parser, Debug)]
#[command(
    name = "anydrop",
    author = "Author",
    version = "0.1.0",
    about = "Lightning-fast local network file sharing for any browser"
)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    port: u16,
    #[arg(short, long)]
    verbose: bool
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();
    
    let port = discover_port(args.port, args.verbose).await?;
    start_server(port.1, port.0, port.2, args.verbose).await?;


    Ok(())
}
