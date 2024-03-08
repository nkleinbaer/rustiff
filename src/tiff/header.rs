use crate::utils::ByteOrder;
use std::io::{self, Error, ErrorKind};

pub struct TiffHeader {
    pub byte_order: ByteOrder,
    pub ifd_offset: u32,
}

pub fn get_byte_order(bytes: &[u8]) -> io::Result<ByteOrder> {
    if bytes.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough bytes to determine endianness",
        ));
    }

    match bytes[0..2] {
        [0x49, 0x49] => Ok(ByteOrder::LittleEndian),
        [0x4D, 0x4D] => Ok(ByteOrder::BigEndian),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid endianness bytes",
        )),
    }
}

pub fn check_magic_bytes(bytes: &[u8], byte_order: &ByteOrder) -> io::Result<bool> {
    if bytes.len() < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough bytes to check magic_bytes",
        ));
    }

    match bytes[2..4] {
        [0x2a, 0x00] if matches!(byte_order, ByteOrder::LittleEndian) => Ok(true),
        [0x00, 0x2a] if matches!(byte_order, ByteOrder::BigEndian) => Ok(true),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Invalid endianness bytes",
        )),
    }
}

pub fn get_ifd_offset(bytes: &[u8], byte_order: &ByteOrder) -> io::Result<u32> {
    if bytes.len() < 8 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough bytes to get first IFD offset",
        ));
    }

    let ifd_offset: [u8; 4] = bytes[4..8].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => Ok(u32::from_le_bytes(ifd_offset)),
        ByteOrder::BigEndian => Ok(u32::from_be_bytes(ifd_offset)),
    }
}

pub fn parse_tiff_header(bytes: &[u8]) -> io::Result<TiffHeader> {
    if bytes.len() < 8 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough bytes in tiff header",
        ));
    }

    let byte_order = get_byte_order(&bytes)?;

    check_magic_bytes(&bytes, &byte_order)?;

    let ifd_offset = get_ifd_offset(&bytes, &byte_order)?;

    Ok(TiffHeader {
        byte_order: byte_order,
        ifd_offset: ifd_offset,
    })
}
