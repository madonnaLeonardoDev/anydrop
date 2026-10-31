use aes_gcm::{
    AeadInOut, Aes256Gcm, Nonce,
};
use rand::{Rng, rng};
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};
use tokio_util::bytes::BufMut;

pub fn generate_key_utf8() -> [u8; 32]{
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let mut bytes = [0u8; 32];
    
    rng().fill_bytes(&mut bytes);
    for b in &mut bytes{
     *b = CHARSET[(*b as usize) & 63];
    };

    bytes
}

pub struct TcpStream<Stream,> {
    inner: Stream,
    buffer: Vec<u8>,
}

impl <Stream: AsyncRead + Unpin> TcpStream<Stream> {
    pub fn new(stream: Stream) -> Self {
        Self {
            inner: stream,

            buffer: Vec::new() 
        }
    }

    pub async fn decrypt_chunk(&mut self,cipher: &Aes256Gcm ,dest: &mut [u8]) -> Result<bool, String> {
        let mut bytes_read: usize = 0;
        //READ Ciphertext_len header
        let mut len_buf: [u8; 4]= [0u8; 4];
        while bytes_read < 4 {
            let n = self.inner.read(&mut len_buf[bytes_read..]).await.map_err(|e| e.to_string())?;
            if n == 0 {
                return Ok(false);
            }
            bytes_read += n;
        };
        let ciphertext_len = u32::from_le_bytes(len_buf) as usize;
        

        //READ Nonce
        bytes_read = 0;
        let mut nonce_bytes = [0u8; 12];
        while bytes_read < 12 /*NONCE BYTES LENGHT */ {
            let n = self.inner.read(&mut nonce_bytes[bytes_read..]).await.map_err(|e| e.to_string())?;
            if n == 0 {
                return Err("Error reading nonce bytes".to_string());
            }
            bytes_read += n;
        };

        let nonce = Nonce::from(nonce_bytes);

        self.buffer.resize(ciphertext_len, 0);

        //Read Ciphertext
        bytes_read = 0;
        while bytes_read < ciphertext_len {
            let n = self.inner.read(&mut self.buffer[bytes_read..]).await.map_err(|e| e.to_string())?;
            if n == 0 {
                return Err("Error reading ciphertext bytes".to_string());
            }
            bytes_read += n;
        }
        cipher.decrypt_in_place(&nonce, &[], &mut self.buffer)
                .map_err(|e| format!("[-] Decryption failed: {}",e))?;

        if dest.len() < ciphertext_len - 16 {
            return Err("[-] Destination slice hasnt got enough room".to_string())
        }

        dest[..ciphertext_len - 16].copy_from_slice(&self.buffer[..ciphertext_len - 16]);

        Ok(true)
    }

    pub async fn encrypt_chunk(&mut self, cipher: &Aes256Gcm, chunk_size: usize, dest: &mut [u8]) -> Result<bool, String> {

        self.buffer.resize(4 + 12 + chunk_size, 0u8);

        let bytes_read = self.inner.read(&mut self.buffer[..chunk_size]).await.map_err(|e| e.to_string())?;
        
        if bytes_read == 0 {
            return Ok(false)
        }

        self.buffer.truncate(bytes_read);



        let mut nonce_b = [0u8; 12];

        rng().fill_bytes(&mut nonce_b);

        let nonce = Nonce::from(nonce_b.clone());

        cipher.encrypt_in_place(&nonce, &[], &mut self.buffer)
            .map_err(|e| format!("[-] Encryption failed: {}",e))?;

        let len_b = (self.buffer.len() as u32).to_le_bytes();

        if dest.len() < self.buffer.len() + 4 + 12 {
            return Err("[-] Destination slice hasnt got enough room".to_string())
        }
        let total_len = 4 + 12 + self.buffer.len();

        dest[..4].copy_from_slice(&len_b);
        dest[4..16].copy_from_slice(&nonce_b);
        dest[16..total_len].copy_from_slice(&self.buffer);
        Ok(true)
    }

    pub async fn read_chunk(&mut self, buf: &mut Vec<u8>) -> Result<bool, String> {
        let mut bytes_read: usize = 0;
        let mut len_buf = [0u8; 4];
        while bytes_read < 4 {
            let n = self.inner.read(&mut len_buf[bytes_read..]).await.map_err(|e| e.to_string())?;

            if n == 0 {
                return  Ok(false)
            }

            bytes_read += n;
        }

        let len = u32::from_le_bytes(len_buf) as usize;
        bytes_read = 0;
        buf.resize(len, 0);
        while bytes_read < len{
            let n = self.inner.read(&mut buf[bytes_read..]).await.map_err(|e| e.to_string())?;

            if bytes_read == 0 {
                return  Err("Error reading chunk bytes".to_string());
            }

            bytes_read += n;
        }
        Ok(true)
    }
}