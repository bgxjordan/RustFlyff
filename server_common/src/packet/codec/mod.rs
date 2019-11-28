use crate::packet::FlyffClientPacket;
use std::io::Cursor;

mod decoder;
mod encoder;

pub struct FlyffPacketCodec {
    next_index: usize,

    matched_start_header: bool,
    matched_unknown_i32: bool,

    command_payload_len: usize,
    command_payload_bytes_read: usize,
    command_payload: Vec<u8>,

    next_packet: FlyffClientPacket,
}

impl FlyffPacketCodec {
    pub fn new() -> FlyffPacketCodec {
        FlyffPacketCodec {
            next_index: 0,
            matched_start_header: false,
            matched_unknown_i32: false,
            command_payload_len: 0,
            command_payload_bytes_read: 0,
            command_payload: Vec::<u8>::new(),

            next_packet: FlyffClientPacket::new(),
        }
    }

    fn reset(&mut self) {
        self.next_index = 0;
        self.matched_start_header = false;
        self.matched_unknown_i32 = false;
        self.command_payload_len = 0;
        self.command_payload_bytes_read = 0;
        self.command_payload = Vec::<u8>::new();

        self.next_packet = FlyffClientPacket::new();
    }

    fn request_more_data<T, Err>(&mut self, cursor: &Cursor<Vec<u8>>) ->  Result<Option<T>, Err> {
        self.next_index += cursor.position() as usize;
        Ok(None)
    }
}