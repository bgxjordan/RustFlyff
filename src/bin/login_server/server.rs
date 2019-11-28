use server_common::*;
use crate::config::LoginConfig;

use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;
use tokio::codec::{Decoder};
use server_common::config::TcpServerConfig;
use std::net::SocketAddr;
use server_common::packet::codec::FlyffPacketCodec;

pub struct LoginServer;

impl LoginServer {
    pub(crate) fn new() -> LoginServer {
        LoginServer{}
    }
}
impl Server<LoginConfig> for LoginServer {

    fn start(&self, config: &LoginConfig) -> FlyffServerFuture {
        let server_listen_addr = format!("{}:{}", config.ip_address(), config.port()).parse::<SocketAddr>()
            .unwrap_or_else(|ex| panic!("Unable to parse server bind address! {}", ex));

        let tcp_server = TcpListener::bind(&server_listen_addr)
            .unwrap_or_else( |ex| panic!("Failed to bind server address {}! {}", &server_listen_addr, ex));

        println!("Login server listening @ {}", &server_listen_addr);

        tcp_server
            .incoming()
            .map_err(|client_err| println!("Client connection failure: {:?}", client_err))
    }
}

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