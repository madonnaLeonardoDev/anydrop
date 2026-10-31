use local_ip_address::local_ip;
use crate::utils::verbose_print;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use crate::Cli;

pub async fn discover_port(cli: &Cli) -> Result<(IpAddr, u16, TcpListener), String> {
    let mut port = cli.port;
    
    let local_ip = local_ip()
        .map_err(|e| e.to_string())?;

    let bind_addr = SocketAddr::from(([0,0,0,0], port));
    let listener = match TcpListener::bind(bind_addr).await {
        Ok(l) => {
            port = l.local_addr()
                .map_err(|e| e.to_string())?.port();
            verbose_print(format!("[+] Succefully bound to port: {}", port), cli.verbose);
            l
        },
        Err(_) => {
            verbose_print(format!("[-] Port {} is busy, fallback to ephemeral free port", port), cli.verbose);

            let fallback_addr = SocketAddr::from(([0,0,0,0], 0));

            let l =TcpListener::bind(&fallback_addr).await
                .map_err(|e| e.to_string())?;

            port = l.local_addr()
                .map_err(|e| e.to_string())?.port();

            verbose_print(format!("[+] Succefully bound to port: {}", port), cli.verbose);

            l
        }
    };

    Ok((local_ip, port ,listener))
}