use std::net::{IpAddr};
use crate::qrcode_gen::qr_from_str;
use tokio::net::TcpListener;
use crate::root_page::get_index_page;
use crate::utils::verbose_print;
use axum::{routing::get, Router};

pub async fn start_server(
    port: u16,
    local_ip: IpAddr,
    listener: TcpListener,
    verbose: bool,
) -> Result<(), String> {

    let url = format!("http://{}:{}",local_ip, port);

    let qr_code = qr_from_str(&url);

    println!("{}\nURL IS: {}",qr_code,url);

    let app: Router = Router::new()
        .route("/", get(get_index_page));

    verbose_print("[+] Serving Html Page on HTTP socket".to_string(), verbose);

    axum::serve(listener, app).await
        .map_err(|e| e.to_string())?;

    Ok(())
}