use tokio::codec::Encoder;
use tokio::io;
use bytes::BytesMut;
use crate::packet::FlyffServerPacket;
use crate::packet::codec::FlyffPacketCodec;

impl Encoder for FlyffPacketCodec {
    type Item = FlyffServerPacket;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        unimplemented!()
    }
}