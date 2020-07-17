use server_common::*;
use crate::client::{AsyncClientConnection, EchoClient};
use crate::config::{LoginConfig, FLYFF_CLIENT_LOGIN_PORT};

use server_common::config::TcpServerConfig;
//use server_common::packet::codec::FlyffPacketCodec;

use async_trait::async_trait;
use std::io::Result;
use std::net::{SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{BytesCodec, Decoder};
use futures::stream::StreamExt;
use std::ops::{Deref, DerefMut};
use std::error::Error;
use std::borrow::{Borrow, BorrowMut};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

pub struct LoginServer {
    client_listen_addr: SocketAddr,
}

impl LoginServer {
    pub(crate) fn new() -> LoginServer {
        LoginServer{
            client_listen_addr: format!("{}:{}", "127.0.0.1", FLYFF_CLIENT_LOGIN_PORT).parse::<SocketAddr>()
                .unwrap_or_else(|ex| panic!("Unable to parse default bind address! {}", ex)),
        }
    }
}

#[async_trait]
impl ServerOperations<LoginConfig> for LoginServer {
    fn configure(&mut self, config: &LoginConfig) {
        self.client_listen_addr = format!("{}:{}", config.bind_address(), config.port()).parse::<SocketAddr>()
            .unwrap_or_else(|ex| panic!("Unable to parse server bind address! {}", ex));
    }
    
    async fn start(&mut self) -> Result<()> {
        let mut flyff_client_listener = TcpListener::bind(&self.client_listen_addr).await
            .unwrap_or_else( |ex| panic!("Failed to bind server address {}! {}", &self.client_listen_addr, ex));

        println!("Login server listening @ {}", &self.client_listen_addr);

        loop {
            println!("[Server] Waiting for connection...");

            // todo: eventually we want this to be a poll w/ timeout
            let (mut stream, client_socket_details) = flyff_client_listener.accept().await?;

            println!("[Server] New client connected");

            tokio::spawn(async move {
                EchoClient::new(&client_socket_details)
                    .process(&mut stream).await;
            });

            // prune disconnected clients

            // push monitoring stats

            // shutdown logic stub - exit loop here
        }

        Ok(())

        // shutdown()

        /*
        tcp_server
            .incoming()
            .map_err(|client_err| println!("Client connection failure: {:?}", client_err))

         */
    }
}

/*
pub fn login_process(client_sock: TcpStream) -> FlyffClientProcessFuture {
    let client_ip = client_sock.peer_addr()
        .expect("Failed to get client IP");
    let (_sock_out, sock_in) = FlyffPacketCodec::new().framed(client_sock).split();

    Box::new(

        sock_in.for_each(move |client_packet| {
            println!("[{}] bytes = {:?}", client_ip.to_string(), client_packet);
            Ok(())
        })
            .and_then(move |()| {
                println!("[{}] Closed connection", client_ip.to_string());
                Ok(())
            })
            .or_else(move |err| {
                println!("[{}] Socket closed with error: {:?}", client_ip.to_string(), err);
                // We have to return the error to catch it in the next ``.then` call
                Err(err)
            })
            .then(move |result| {
                println!("[{}] Socket closed with result: {:?}", client_ip.to_string(), result);
                Ok(())
            })
    )
}

 */