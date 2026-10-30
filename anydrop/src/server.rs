use std::net::{IpAddr};
use std::env;
use std::path::{PathBuf};
use crate::{qrcode_gen::qr_from_str, root_page::{send_page, recieve_page}};
use crate::utils::verbose_print;
use axum::extract::State;
use tokio::{
    net::TcpListener,
    fs::File,
    io::AsyncWriteExt
};
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
    Recieve(Option<String>)
}

#[derive(Clone)]
pub struct AppState {
    pub dir: PathBuf,
}


pub async fn start_server(
    server: ServerType,
    port: u16,
    local_ip: IpAddr,
    listener: TcpListener,
    verbose: bool,
) -> Result<(), String> {

    let url = format!("http://{}:{}",local_ip, port);

    let qr_code = qr_from_str(&url);

    println!("{}\nURL IS: {}",qr_code,url);

    let app: Router = get_app(server)?;

    verbose_print("[+] Serving Html Page on HTTP socket".to_string(), verbose);

    axum::serve(listener, app).await
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn get_app(server: ServerType ) -> Result<Router, String> {

    let router:Router = match server {
        ServerType::Send(file_str) => {    
            let file_path = PathBuf::from(&file_str);

            if !file_path.exists() {
                return Err(format!("[-] File Path '{}' doesn't exist", file_str))
            }
            if file_path.is_dir() {
                return Err(format!("[-] File Path '{}' doesn't exist", file_str))
            }

            let state = AppState {
                dir: file_path
            };

            let app: Router = Router::new()
                .route("/", get(send_page))
                .route("/download", get(stream_file))
                .with_state(state);

            app
        },
        ServerType::Recieve(desination_str) => {
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

            let state = AppState {
                dir: destination_path 
            };

            let app:Router =  Router::new()
                .route("/", get(recieve_page))
                .route("/upload", post(handle_upload))
                .with_state(state);


            app

        }
    };

    Ok(router)


}


async fn stream_file(State(state): State<AppState>) -> Result<impl IntoResponse, String> {
    let file_path = state.dir; 
    
    let file: (File, &str) = match File::open(&file_path).await {
        Ok(f) => (f, file_path.file_name().and_then(|n| n.to_str()).unwrap_or("Unkown Name")),
        Err(e) => return Err(format!("[-] Error opening file: {}", e))
    };

    let stream = ReaderStream::new(file.0);

    let body = Body::from_stream(stream);

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/octet-stream".to_string()),
            (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", file.1)),
        ],
        body,
    )
    .into_response())
}

async fn handle_upload(State(state): State<AppState>, mut multipart: Multipart, ) -> impl IntoResponse {
    let mut save_path = state.dir;

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(_) => return Html("[-] Failed to parse multipart stream".to_string()),
    } {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
                match field.file_name().map(|n| n.to_string()){
                    Some(s) =>{
                        save_path.push(&s);
                        if save_path.exists() {
                            let mut i: usize = 0;
                            let file_name: (&str, String) = s.split_once(".")
                            .and_then(|t| Some((t.0, format!(".{}", t.1)))).unwrap_or((&s, "".to_string()));
                            save_path.pop();
                            save_path.push(format!("{}_{}{}",file_name.0,i.to_string(),file_name.1));

                            while save_path.exists() {
                                i+=1;
                                save_path.pop();
                                save_path.push(format!("{}_{}{}",file_name.0,i.to_string(),file_name.1));
                            }
                        }                        
                    },
                    None => {
                        let mut file_name = ("anydrop_file", 0);
                        save_path.push(format!("{}{}",file_name.0, file_name.1));
                        let mut i: usize = 1;
                        while save_path.exists() {
                            save_path.pop();
                            file_name.1 = i;
                            save_path.push(format!("{}{}",file_name.0, file_name.1));
                            i+=1;
                        }
                    }
                }

            let mut file = match File::create(&save_path).await {
                Ok(f) => f,
                Err(e) => return Html(format!("[-] Failed to create file on disk: {}", e)),
            };
            let mut field = field;
            while let Some(chunk_result) = field.chunk().await.transpose() {
                let chunk = match chunk_result {
                    Ok(c) => c,
                    Err(e) => return Html(format!("[-] Error reading chunk: {}", e)),
                };

                if let Err(e) = file.write_all(&chunk).await {
                    return Html(format!("[-] Failed to write to disk: {}", e));
                }
            }

            break;
        }
    }

    Html("Upload complete!".to_string())
}