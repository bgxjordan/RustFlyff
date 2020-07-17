use bytes::{BytesMut, Buf, BufMut};
// use tokio::codec::Decoder;
// use tokio::io::ErrorKind;
use std::io::{Cursor, Read};
use std::cmp::max;
//use crate::packet::{FlyffClientPacket, FLYFF_CLIENT_START_HEADER, FlyffPacketCommand};
//use crate::packet::codec::FlyffPacketCodec;

/*
impl Decoder for FlyffPacketCodec {
    type Item = FlyffClientPacket;
    type Error = tokio::io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {

        let mut cursor = Cursor::new(buf[self.next_index..].to_vec());

        if !self.matched_start_header {
            match cursor.has_remaining() {
                // Need more data
                false => return self.request_more_data::<Self::Item, Self::Error>(&cursor),

                true => match cursor.get_u8() {
                    FLYFF_CLIENT_START_HEADER => self.matched_start_header = true,

                    // Bail if the client marker is not found
                    client_header => return Err(Self::Error::new(ErrorKind::InvalidData,
                                                                 format!("Not a Flyff client packet! First byte should be 0x{:X} but was 0x{:X}", FLYFF_CLIENT_START_HEADER, client_header)))
                }
            }
        }

        if !self.matched_unknown_i32 {
            match cursor.remaining() < 4 {
                // Need more data
                true => return self.request_more_data::<Self::Item, Self::Error>(&cursor),

                // Read the unknown int
                false => {
                    self.matched_unknown_i32 = true;
                    self.next_packet.unknown_i32 = cursor.get_i32_le();
                }
            }
        }

        if self.command_payload_len == 0 {
            match cursor.remaining() < 4 {
                // Need more data
                true => return self.request_more_data::<Self::Item, Self::Error>(&cursor),

                // Get command length and resize buffer
                false => {
                    self.command_payload_len = cursor.get_u32_le() as usize;
                    self.command_payload = Vec::<u8>::with_capacity(self.command_payload_len);
                }
            }
        }

        if self.command_payload_bytes_read < self.command_payload_len {
            match cursor.has_remaining() {
                // Need more data
                false =>  return self.request_more_data::<Self::Item, Self::Error>(&cursor),

                // Read as many bytes of the command payload as are available
                true => self.command_payload_bytes_read += cursor.read(&mut self.command_payload[max(0, self.command_payload_bytes_read-1)..]).unwrap()
            }
        }

        // Need more data - update the buffer index
        if cursor.remaining() < 4 {
            return self.request_more_data::<Self::Item, Self::Error>(&cursor)
        }

        self.next_packet.charserver_value = cursor.get_i32_le();
        self.command_payload.put_i32_le(self.next_packet.charserver_value);

        self.next_packet.command = FlyffPacketCommand::from(&self.command_payload);

        let packet = FlyffClientPacket::from(&self.next_packet);
        self.reset();

        Ok(Some(packet))
    }
}

#[cfg(test)]
mod decoder_tests {
    use super::*;

    #[test]
    fn not_matched_start_header_requests_more_data() {
        let mut codec = FlyffPacketCodec::new();
        let mut bytes = BytesMut::new();

        assert!(codec.decode(&mut bytes).unwrap().is_none());
    }

    #[test]
    fn bad_client_start_header() {
        let mut codec = FlyffPacketCodec::new();
        let mut bytes = BytesMut::from(String::from("\x01"));

        assert!(codec.decode(&mut bytes).is_err());
    }

    #[test]
    fn valid_client_start_header() {
        let mut codec = FlyffPacketCodec::new();
        let mut bytes = BytesMut::from([FLYFF_CLIENT_START_HEADER].to_vec());

        assert!(codec.decode(&mut bytes).unwrap().is_none());
    }
}

 */