pub trait TcpServerConfig {
    fn bind_address(&self) -> String;
    fn port(&self) -> u16;
}