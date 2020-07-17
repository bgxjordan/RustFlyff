mod client;
mod config;
mod server;

use config::LoginConfigParser;
use server::LoginServer;
use server_common::{FileParser, ServerOperations};

use std::path::Path;
use crate::config::{LoginConfig};
use std::ops::DerefMut;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut server = LoginServer::new();

    let config_filename = Path::new("loginserver.json");

    let config = LoginConfigParser::new()
        .parse(config_filename)
        .unwrap_or_else(|ex| panic!("Unable to process login config: {}! {}", config_filename.display(), ex));

    // monitor.add(server)

    server.configure(&config);
    server.start().await
}
