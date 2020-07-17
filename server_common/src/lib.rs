pub mod config;
pub mod packet;

use async_trait::async_trait;
use std::io::Result;
use std::path::Path;
//use tokio::prelude::Future;
//use tokio::prelude::stream::*;
//use tokio::net::tcp::Incoming;

//pub type FlyffServerFuture = MapErr<Incoming, fn(io::Error)>;
//pub type FlyffClientProcessFuture = Box<dyn Future<Item = (), Error = ()> + Send>;

pub trait FileParser<TargetType> {
    fn parse(self, filename: &Path) -> Result<TargetType>;
}

#[async_trait]
pub trait ServerOperations<ConfigType> {
    fn configure(&mut self, config: &ConfigType);
    async fn start(&mut self) -> Result<()>;
}