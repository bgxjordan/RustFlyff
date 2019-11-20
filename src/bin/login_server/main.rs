mod config;
mod server;

use config::LoginConfigParser;
use server_common::{FileParser, Server};
use server_common::config::TcpServerConfig;

use std::net::SocketAddr;
use std::path::Path;
use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_filename = Path::new("loginserver.json");

    let config = LoginConfigParser::new()
        .parse(config_filename)
        .unwrap_or_else(|ex| panic!("Unable to process login config: {}! {}", config_filename.display(), ex));

    let tcp_server_loop = server::LoginServer::new()
        .start(&config).
        // any filter logic would happen prior to for_each
        for_each(|client_sock| {
            tokio::spawn(server::login_process(client_sock));
            Ok(())
        });

    tokio::run(tcp_server_loop);

    Ok(())
}
