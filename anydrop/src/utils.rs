use qrcode::{QrCode, render::unicode};
use std::{io::Error, net::IpAddr, path::PathBuf};
use tokio::fs::{File, metadata};

pub fn verbose_print(msg:String ,verbose: bool) {
    if verbose {
        println!("{}", msg);
    }
}

#[derive(Clone)]
pub struct AnydropPath {
    pub path: PathBuf
}
#[allow(unused)]
impl AnydropPath {
    pub async fn open(&self) -> Result<File, Error>{
        Ok(File::open(&self.path).await?)
    }
    
    pub async fn file_size(&self) -> Result<u64, Error> {
        match metadata(&self.path).await {
        Ok(m) => return Ok(m.len()),
        Err(e) => {
            return  Err(e);
        }
    };
    }
}

impl From<PathBuf> for AnydropPath {
    fn from(value: PathBuf) -> Self {
        Self { path: value }
    }
}

pub fn qr_from_str(str: &str) -> String {
    let code = QrCode::new(str).unwrap();

    let unicode_str = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();

    unicode_str
}


pub fn print_url(ip: &IpAddr, port: &u16, key: &Option<[u8;32]>) -> String {
    let url = if let Some(k) = key {
        let key_str = str::from_utf8(k).unwrap();
        format!("http://{}:{}#{}",ip, port, key_str)
    } else {
        format!("http://{}:{}",ip, port) 
    };

    let qr_code = qr_from_str(&url);

    format!("URL IS: {}\n{}",url, qr_code)
}
