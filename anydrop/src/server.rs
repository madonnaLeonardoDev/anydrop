use crate::Cli;
use std::collections::HashMap;
use std::net::{IpAddr};
use std::env;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,Nonce,
};
use tokio::io::AsyncRead;
use walkdir::{WalkDir};
use std::path::{PathBuf};
use crate::{utils::print_url, root_page::{send_page, recive_page}};
use crate::utils::{AnydropPath, verbose_print};
use axum::extract::{State, DefaultBodyLimit, Query};
use serde::{Deserialize};
use tokio::{
    net::TcpListener,
    fs::{File},
    io::AsyncWriteExt
};
use crate::enrcypted_stream::{EncryptedStream, generate_key_utf8};
use tokio_util::io::{ReaderStream};
use axum::{
    Router,
    routing::{get, post},
    body::Body,
    http::{header, StatusCode},
    response::{IntoResponse},
    extract::Multipart,
    response::Html
};

#[derive(Clone)]
pub enum ServerType {
    Send(String),
    Recive(Option<String>)
}

#[derive(Clone)]
pub struct AppState {
    pub dir: Option<PathBuf>,
    pub dirs_map: Option<HashMap<String,AnydropPath>>,
    pub encryption: Option<[u8; 32]>
}

#[derive(Deserialize)]
pub struct DownloadQuery {
    pub name: String
}

pub async fn start_server(
    server: ServerType,
    port: u16,
    local_ip: IpAddr,
    listener: TcpListener,
    cli: &Cli,
) -> Result<(), String> {

    let app: (Router, String) = get_app(server, cli, local_ip, port)?;

    verbose_print("[+] Serving Html Page on HTTP socket".to_string(), cli.verbose);
    print!("{}", app.1);
    axum::serve(listener, app.0).await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

fn get_app(server: ServerType, cli: &Cli, local_ip: IpAddr, port: u16) -> Result<(Router, String), String> {
    let router:(Router, String) = match server {
        ServerType::Send(file_str) => { 
            let file_path = PathBuf::from(&file_str);
            let mut dirs: HashMap<String,AnydropPath> = HashMap::new();
            if !file_path.exists() {
                return Err(format!("[-] File Path '{}' doesn't exist", file_str))
            }
            if file_path.is_dir() {
                for entry in WalkDir::new(&file_path).follow_links(true) {
                    match entry {
                        Ok(d) => {
                            if d.path() != &file_path {
                                dirs.insert(d.file_name().to_str().unwrap().to_owned(),PathBuf::from(d.path()).into());
                            }
                                
                            },
                        Err(e) => return Err(format!("[-] Failed accessing element inside dir: {}",e))
                    }
                }

            }
            let mut key = None;
            if cli.safe {
                key = Some(generate_key_utf8());
            }

            let state = AppState {
                dir: None,
                dirs_map: Some(dirs),
                encryption: key
            };
            let url = print_url(&local_ip, &port, &state.encryption);
            let app: Router = Router::new()
                .route("/", get(send_page))
                .route("/download", get(handle_download))
                .with_state(state);

            (app, url)
        },
        ServerType::Recive(desination_str) => {
            let destination_path = match desination_str {
                Some(s) => PathBuf::from(s),
                None => {
                    env::current_dir()
                        .map_err(|e| e.to_string())?
                }
            };
            
            if !destination_path.is_dir(){
                return  Err("[-] Destination path must be a directory".to_string())
            }

            let mut key: Option<[u8; 32]> = None;
            if cli.safe {
                key = Some(generate_key_utf8());
            }

            let state = AppState {
                dir: Some(destination_path),
                dirs_map: None,
                encryption: key
            };
            let url = print_url(&local_ip, &port, &state.encryption);
            let app:Router =  Router::new()
                .route("/", get(recive_page))
                .route("/upload", post(handle_upload))
                .with_state(state)
                .layer(DefaultBodyLimit::disable());
            (app, url)

        }
    };

    Ok(router)


}


async fn handle_download(
    State(state): State<AppState>,
    Query(query): Query<DownloadQuery>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let name = query.name;
    let dirs_map = state.dirs_map.as_ref().unwrap();
    match dirs_map.get(&name) {
        Some(file_path) => {
            let file = file_path.open().await
                .map_err(|e| 
                {
                    eprintln!("[-] Could not open file: {}",e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Could not open file".to_string())
                })?;
                
    
    let stream_file: Box<dyn AsyncRead + Send + Unpin> = 
    if let Some(key) = state.encryption {
        Box::new(EncryptedStream::new(file, &key))
    } else {
        Box::new(file)
    };
    
    let stream = ReaderStream::new(stream_file);

    let body = Body::from_stream(stream);

        return Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/octet-stream".to_string()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", name))
        ],
        body,
    )
    .into_response())
        }
        None => return Err((StatusCode::NOT_FOUND,"No file matching query found".to_string()))
    };
}


async fn handle_upload(
    State(state): State<AppState>, 
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut save_path = state.dir.unwrap();

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "[-] Failed to parse multipart stream".to_string())),
    } {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            match field.file_name().map(|n| n.to_string()) {
                Some(s) => {
                    save_path.push(&s);
                    if save_path.exists() {
                        let mut i: usize = 0;
                        let file_name: (&str, String) = s.split_once(".")
                            .map(|t| (t.0, format!(".{}", t.1)))
                            .unwrap_or((&s, "".to_string()));
                        save_path.pop();
                        save_path.push(format!("{}_{}{}", file_name.0, i.to_string(), file_name.1));

                        while save_path.exists() {
                            i += 1;
                            save_path.pop();
                            save_path.push(format!("{}_{}{}", file_name.0, i.to_string(), file_name.1));
                        }
                    }                        
                },
                None => {
                    let mut file_name = ("anydrop_file", 0);
                    save_path.push(format!("{}{}", file_name.0, file_name.1));
                    let mut i: usize = 1;
                    while save_path.exists() {
                        save_path.pop();
                        file_name.1 = i;
                        save_path.push(format!("{}{}", file_name.0, file_name.1));
                        i += 1;
                    }
                }
            }

            let mut file = match File::create(&save_path).await {
                Ok(f) => f,
                Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Failed to create file on disk: {}", e))),
            };
            
            let mut field = field;

            if let Some(k) = state.encryption {
                let cipher = match Aes256Gcm::new_from_slice(&k) {
                    Ok(c) => c,
                    Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Error generating cipher: {}", e))),
                };

                let mut buffer: Vec<u8> = Vec::new();

                while let Some(chunk_result) = field.chunk().await.transpose() {
                    let chunk = match chunk_result {
                        Ok(c) => c,
                        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Error reading chunk: {}", e))),
                    };

                    buffer.extend_from_slice(&chunk);

                    while buffer.len() >= 4 { 
                        let mut len_bytes = [0u8; 4];
                        len_bytes.copy_from_slice(&buffer[0..4]);
                        let cipher_len = u32::from_be_bytes(len_bytes) as usize;
                        let total_packet_len = 4 + 12 + cipher_len;

                        if buffer.len() < total_packet_len {
                            break;
                        }

                        let nonce = match Nonce::try_from(&buffer[4..16]) {
                            Ok(n) => n,
                            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Error reading nonce: {}", e))),
                        };

                        let ciphertext = &buffer[16..total_packet_len];

                        let plaintext = match cipher.decrypt(&nonce, ciphertext) {
                            Ok(pt) => pt,
                            Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Decryption Failed: {}", e))),
                        };

                        if let Err(e) = file.write_all(&plaintext).await {
                            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Failed to write to disk: {}", e)));
                        }

                        buffer.drain(0..total_packet_len);
                    }
                }
            } else {
                while let Some(chunk_result) = field.chunk().await.transpose() {
                    let chunk = match chunk_result {
                        Ok(c) => c,
                        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Error reading chunk: {}", e))),
                    };

                    if let Err(e) = file.write_all(&chunk).await {
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("[-] Failed to write to disk: {}", e)));
                    }
                }
            }

            return Ok(Html("Upload complete!".to_string()));
        }
    }

    Ok(Html("No file uploaded".to_string()))
}