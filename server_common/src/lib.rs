pub mod config;

use std::io;
use std::path::Path;
use tokio::prelude::Future;
use tokio::prelude::stream::*;
use tokio::net::tcp::Incoming;
use tokio::net::TcpStream;

pub type FlyffServerFuture = MapErr<Incoming, fn(io::Error)>;
pub type FlyffClientProcessFuture = Box<dyn Future<Item = (), Error = ()> + Send>;

pub trait FileParser<TargetType> {
    fn parse(self, filename: &Path) -> Result<TargetType, io::Error>;
}

pub trait Server<ConfigType> {
    fn start(&self, config: &ConfigType) -> FlyffServerFuture;
}