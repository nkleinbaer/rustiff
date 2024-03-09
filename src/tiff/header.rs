use crate::tiff::error::TiffErrorKind;

use std::fmt;

#[derive(Clone, Copy)]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl fmt::Display for ByteOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ByteOrder::LittleEndian => write!(f, "Little Endian"),
            ByteOrder::BigEndian => write!(f, "Big Endian"),
        }
    }
}

pub struct TiffHeader {
    pub byte_order: ByteOrder,
    pub ifd_offset: u32,
}

fn get_byte_order(header_bytes: &[u8; 8]) -> Result<ByteOrder, TiffErrorKind> {
    match header_bytes[0..2] {
        [0x49, 0x49] => Ok(ByteOrder::LittleEndian),
        [0x4D, 0x4D] => Ok(ByteOrder::BigEndian),
        _ => Err(TiffErrorKind::InvalidByteOrder),
    }
}

fn check_magic_bytes(header_bytes: &[u8; 8], byte_order: ByteOrder) -> Result<bool, TiffErrorKind> {
    match header_bytes[2..4] {
        [0x2a, 0x00] if matches!(byte_order, ByteOrder::LittleEndian) => Ok(true),
        [0x00, 0x2a] if matches!(byte_order, ByteOrder::BigEndian) => Ok(true),
        _ => Err(TiffErrorKind::InvalidMagicBytes),
    }
}

fn get_ifd_offset(header_bytes: &[u8; 8], byte_order: ByteOrder) -> u32 {
    let ifd_offset: [u8; 4] = header_bytes[4..8].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => u32::from_le_bytes(ifd_offset),
        ByteOrder::BigEndian => u32::from_be_bytes(ifd_offset),
    }
}

pub fn parse_tiff_header(bytes: &[u8]) -> Result<TiffHeader, TiffErrorKind> {
    let header_bytes: &[u8; 8] = match bytes[0..8].try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err(TiffErrorKind::InvalidHeader),
    };

    let byte_order = get_byte_order(header_bytes)?;

    check_magic_bytes(header_bytes, byte_order)?;

    let ifd_offset = get_ifd_offset(header_bytes, byte_order);

    Ok(TiffHeader {
        byte_order,
        ifd_offset,
    })
}
