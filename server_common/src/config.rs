pub trait TcpServerConfig {
    fn ip_address(&self) -> String;
    fn port(&self) -> u16;
}