use async_trait::async_trait;
use bytes::BytesMut;
use futures::StreamExt;
use std::io::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, Decoder};
use std::net::SocketAddr;

pub struct EchoClient {
    remote_host: SocketAddr,
}

#[async_trait]
pub trait AsyncClientConnection {
    async fn process(&mut self, stream: &mut TcpStream);
}


impl EchoClient{
    pub fn new(sock: &SocketAddr) -> EchoClient {
        println!("[EchoClient] socket: {}", sock);
        EchoClient{
            remote_host: *sock
        }
    }

    fn close(&mut self, error: Option<Error>) {
        match error {
            Some(err) => self.on_client_error(&err),
            None => self.on_client_closed()
        }
    }

    fn on_client_connected(&self) {
        println!("[EchoClient] Socket connected");
    }

    fn on_client_error(&self, err: &Error) {
        println!("[EchoClient] Socket closed with error: {:?}", err)
    }

    fn on_client_closed(&self) {
        println!("[EchoClient] Socket received FIN packet and closed connection");
    }

    fn on_bytes(&self, bytes: &BytesMut) {
        println!("[EchoClient] [Echo] bytes: {:?}", bytes);
    }
}

#[async_trait]
impl AsyncClientConnection for EchoClient {
    async fn process(&mut self, stream: &mut TcpStream) {
        let (stream_out, mut stream_in) = BytesCodec::new().framed(stream).split();

        self.on_client_connected();

        println!("[EchoClient] Listening for data....");

        // We loop while there are messages coming from the Stream `framed`.
        // The stream will return None once the client disconnects.
        while let Some(message) = stream_in.next().await {
            match message {
                Ok(bytes) => self.on_bytes(&bytes),
                Err(err) => self.close(Some(err)),
            }
        }
        self.close(None);
    }
}