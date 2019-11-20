use server_common::FileParser;
use server_common::config::TcpServerConfig;
use std::path::Path;
use std::io;

pub struct LoginConfig {
    realm_id: String,
    ip_address: String,
    port: u16,
    queue_address: String,
}

impl TcpServerConfig for LoginConfig {
    fn ip_address(&self) -> String {
        self.ip_address.clone()
    }

    fn port(&self) -> u16 {
        self.port.clone()
    }
}

impl LoginConfig {
    pub(crate) fn new() -> LoginConfig {
        LoginConfig {
            realm_id: String::new(),
            ip_address: String::new(),
            port: 23000,
            queue_address: String::new()
        }
    }

    fn with_realm_id (mut self, id: String) -> LoginConfig {
        self.realm_id = id;
        return self
    }

    fn with_ip_address(mut self, addr: String) -> LoginConfig {
        self.ip_address = addr;
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
    fn parse(self, _: &Path) -> Result<LoginConfig, io::Error> {
        Ok(LoginConfig::new()
               .with_realm_id("1".to_string())
               .with_ip_address("127.0.0.1".to_string())
               .with_queue_address("TBD".to_string())
        )
    }
}