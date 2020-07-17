use server_common::FileParser;
use server_common::config::TcpServerConfig;
use std::path::Path;
use std::io::Result;
use std::borrow::Borrow;
use crate::server::LoginServer;
use std::net::SocketAddr;

pub const FLYFF_CLIENT_LOGIN_PORT: u16 = 28000;

pub struct LoginConfig {
    realm_id: String,
    bind_address: Option<String>,
    queue_address: String,
}

impl TcpServerConfig for LoginConfig {
    fn bind_address(&self) -> String {
        self.bind_address.as_ref().unwrap_or(&String::from("localhost")).clone()
    }

    fn port(&self) -> u16 {
        FLYFF_CLIENT_LOGIN_PORT
    }
}

impl LoginConfig {
    pub(crate) fn new() -> LoginConfig {
        LoginConfig {
            realm_id: String::new(),
            bind_address: None,
            queue_address: String::new()
        }
    }

    fn with_realm_id (mut self, id: String) -> LoginConfig {
        self.realm_id = id;
        return self
    }

    fn with_ip_address(mut self, addr: String) -> LoginConfig {
        self.bind_address = Some(addr);
        return self;
    }

    fn with_queue_address(mut self, addr: String) -> LoginConfig {
        self.queue_address = addr;
        return self;
    }
}

pub struct LoginConfigParser;

impl LoginConfigParser {
    pub fn new() -> LoginConfigParser {
        LoginConfigParser{}
    }
}

impl FileParser<LoginConfig> for LoginConfigParser {
    fn parse(self, _: &Path) -> Result<LoginConfig> {
        Ok(LoginConfig::new()
               .with_realm_id("1".to_string())
               .with_ip_address("127.0.0.1".to_string())
               .with_queue_address("TBD".to_string())
        )
    }
}