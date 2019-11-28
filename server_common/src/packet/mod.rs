use to_vec::ToVec;
use std::io::{Cursor, Read};
use bytes::{BufMut, Buf};

pub mod codec;

const FLYFF_CLIENT_START_HEADER: u8 = 0x5e;

#[derive(Debug)]
pub struct FlyffClientPacket {
    unknown_i32: i32,
    command: FlyffPacketCommand,
    charserver_value: i32,
}

#[derive(Debug)]
pub struct FlyffServerPacket {
    command: FlyffPacketCommand
}

#[derive(Clone, Debug)]
pub struct FlyffPacketCommand {
    unknown_i32: i32,
    id: u32,
    data: Vec<u8>,
}

impl FlyffClientPacket {
    fn new() -> FlyffClientPacket {
        FlyffClientPacket {
            unknown_i32: 0,
            command: FlyffPacketCommand::new(),
            charserver_value: 0,
        }
    }

    fn from(packet: &FlyffClientPacket) -> FlyffClientPacket {
        FlyffClientPacket {
            unknown_i32: packet.unknown_i32,
            command: packet.command.clone(),
            charserver_value: packet.charserver_value,
        }
    }
}

impl ToVec<u8> for FlyffPacketCommand {
    fn to_vec(self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::<u8>::with_capacity(self.data.len() + 4));

        cursor.put_u32_le(self.id);
        cursor.put_slice(self.data.as_slice());

        cursor.into_inner()
    }
}

impl ToVec<u8> for FlyffServerPacket {
    fn to_vec(self) -> Vec<u8> {
        let cmd = self.command.to_vec();
        let mut cursor = Cursor::new(Vec::<u8>::with_capacity(cmd.len() + 5));

        cursor.put_u8(FLYFF_CLIENT_START_HEADER);
        cursor.put_u32_le(cmd.len() as u32);
        cursor.put_slice(cmd.as_slice());

        cursor.into_inner()
    }
}

impl FlyffPacketCommand {
    fn new() -> FlyffPacketCommand {
        FlyffPacketCommand {
            unknown_i32: 0,
            id: 0,
            data: Vec::<u8>::new(),
        }
    }

    fn from(command_buffer: &Vec<u8>) -> FlyffPacketCommand {
        let mut cmd = FlyffPacketCommand::new();
        let mut cursor = Cursor::new(command_buffer);

        cmd.unknown_i32 = cursor.get_i32_le();
        cmd.id = cursor.get_u32_le();
        cursor.read_to_end(&mut cmd.data);

        cmd
    }
}