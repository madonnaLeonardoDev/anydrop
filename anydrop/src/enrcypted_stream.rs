use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,Nonce,
};
use rand::{Rng, rng};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, ReadBuf};

pub fn generate_key_utf8() -> [u8; 32]{
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

    let mut bytes = [0u8; 32];
    
    rng().fill_bytes(&mut bytes);
    for b in &mut bytes{
     *b = CHARSET[(*b as usize) & 63];
    };

    bytes
}

//ENCRYPTION OF STREAM

pub struct EncryptedStream<R> {
    inner: R,               
    cipher: Aes256Gcm,   
    read_buf: Vec<u8>,      
    out_buf: Vec<u8>,       
    buffer_offset: usize,    
    eof_reached: bool,      
}

impl<R: AsyncRead + Unpin> EncryptedStream<R> {
    pub fn new(inner: R, key_bytes: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(key_bytes).expect("Session key must be 32 bytes long");

        Self {
            inner,
            cipher,
            read_buf: vec![0u8; 128 * 1024], 
            out_buf: Vec::new(),
            buffer_offset: 0,
            eof_reached: false,
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for EncryptedStream<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        
        if self.buffer_offset < self.out_buf.len() {
            let remaining = &self.out_buf[self.buffer_offset..];
            let n = std::cmp::min(remaining.len(), buf.remaining());
            buf.put_slice(&remaining[..n]);
            self.buffer_offset += n;
            return Poll::Ready(Ok(()));
        }

        if self.eof_reached {
            return Poll::Ready(Ok(()));
        }

        let this = self.get_mut();
        let mut read_buf = ReadBuf::new(&mut this.read_buf);
        match Pin::new(&mut this.inner).poll_read(cx, &mut read_buf) {
            Poll::Ready(Ok(())) => {
                let filled_len = read_buf.filled().len();
                
                if filled_len == 0 {
                    this.eof_reached = true;
                    return Poll::Ready(Ok(()));
                }
                let chunk = &this.read_buf[..filled_len];

                let mut nonce_bytes = [0u8; 12];
                rng().fill_bytes(&mut nonce_bytes);
                let aes_nonce = Nonce::from(nonce_bytes);

                let ciphertext = match this.cipher.encrypt(&aes_nonce, chunk) {
                    Ok(ct) => ct,
                    Err(_) => {
                        return Poll::Ready(Err(io::Error::new(
                            io::ErrorKind::Other,
                            "On-the-fly chunk encryption failed",
                        )))
                    }
                };


                let cipher_len = ciphertext.len() as u32;
                let mut packet = Vec::with_capacity(4 + 12 + ciphertext.len());
                packet.extend_from_slice(&cipher_len.to_be_bytes());
                packet.extend_from_slice(&nonce_bytes);
                packet.extend_from_slice(&ciphertext);

                this.out_buf = packet;
                this.buffer_offset = 0;

                let n = std::cmp::min(this.out_buf.len(), buf.remaining());
                buf.put_slice(&this.out_buf[..n]);
                this.buffer_offset += n;

                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,           
        }
    }
}