use crate::tiff::error::TiffErrorKind;
use std::fmt;
use std::ops::Range;

use super::error::TiffError;
use super::ifd::MINIMUM_IFD_LENGTH;

const HEADER_LEN: usize = 8;
const BYTE_ORDER: Range<usize> = 0..2;
const MAGIC_BYTES: Range<usize> = 2..4;
const IFD_OFFSET: Range<usize> = 4..8;

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

fn get_byte_order(header_bytes: &[u8; 8]) -> Result<ByteOrder, TiffError> {
    match header_bytes[BYTE_ORDER] {
        [0x49, 0x49] => Ok(ByteOrder::LittleEndian),
        [0x4D, 0x4D] => Ok(ByteOrder::BigEndian),
        _ => Err(TiffError::new(TiffErrorKind::InvalidByteOrder)),
    }
}

fn check_magic_bytes(header_bytes: &[u8; 8], byte_order: ByteOrder) -> Result<bool, TiffError> {
    match header_bytes[MAGIC_BYTES] {
        [0x2a, 0x00] if matches!(byte_order, ByteOrder::LittleEndian) => Ok(true),
        [0x00, 0x2a] if matches!(byte_order, ByteOrder::BigEndian) => Ok(true),
        _ => Err(TiffError::new(TiffErrorKind::InvalidMagicBytes)),
    }
}

fn get_ifd_offset(header_bytes: &[u8; 8], byte_order: ByteOrder) -> u32 {
    let ifd_offset: [u8; 4] = header_bytes[IFD_OFFSET].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => u32::from_le_bytes(ifd_offset),
        ByteOrder::BigEndian => u32::from_be_bytes(ifd_offset),
    }
}

pub fn parse_tiff_header(bytes: &[u8]) -> Result<TiffHeader, TiffError> {
    let nbytes = bytes.len();
    if nbytes < HEADER_LEN {
        return Err(TiffError::new(TiffErrorKind::InvalidHeader));
    }
    let header_bytes: &[u8; 8] = bytes[0..8].try_into().unwrap();

    let byte_order = get_byte_order(header_bytes)?;

    check_magic_bytes(header_bytes, byte_order)?;

    let ifd_offset = get_ifd_offset(header_bytes, byte_order);

    if ifd_offset as usize >= nbytes - MINIMUM_IFD_LENGTH {
        return Err(TiffError::new(TiffErrorKind::InvalidHeader));
    }

    Ok(TiffHeader {
        byte_order,
        ifd_offset,
    })
}
