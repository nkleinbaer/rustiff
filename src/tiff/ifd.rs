use std::{
    io::{self, Error, ErrorKind},
    u32,
};

use crate::utils::ByteOrder;

pub struct IFD {
    start_offset: u32,
    pub n_fields: u16,
    pub next_ifd_offset: u32,
}

pub struct Field {
    tag: u16,
    ftype: FieldType,
    count: u32,
    offset: u32,
}

pub enum FieldType {
    Byte,
    ASCII,
    Short,
    Long,
    Rational,
    SByte,
    Undefined,
    SShort,
    SLong,
    SRational,
    Float,
    Double,
}

fn get_n_fields(bytes: &[u8], offset: u32, byte_order: ByteOrder) -> io::Result<u16> {
    let min_length = offset + 2;
    if bytes.len() < min_length as usize {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough bytes to get number of fields",
        ));
    }

    let i: usize = offset.try_into().unwrap();

    let n_fields: [u8; 2] = bytes[i..i + 2].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => Ok(u16::from_le_bytes(n_fields)),
        ByteOrder::BigEndian => Ok(u16::from_be_bytes(n_fields)),
    }
}

fn get_next_ifd_offset(
    bytes: &[u8],
    offset: u32,
    n_fields: u16,
    byte_order: ByteOrder,
) -> io::Result<u32> {
    let internal_offset: u32 = (n_fields * 12).into();
    let min_length = offset + internal_offset + 4;

    if bytes.len() < min_length as usize {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Not enough bytes to get next IFD offset",
        ));
    }

    let i: usize = (offset + internal_offset).try_into().unwrap();

    let next_ifd_offset: [u8; 4] = bytes[i..i + 4].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => Ok(u32::from_le_bytes(next_ifd_offset)),
        ByteOrder::BigEndian => Ok(u32::from_be_bytes(next_ifd_offset)),
    }
}

pub fn parse_ifd(bytes: &[u8], offset: u32, byte_order: ByteOrder) -> io::Result<IFD> {
    let n_fields = get_n_fields(&bytes, offset, byte_order)?;
    let next_ifd_offset = get_next_ifd_offset(&bytes, offset, n_fields, byte_order)?;

    Ok(IFD {
        start_offset: offset,
        n_fields,
        next_ifd_offset,
    })
}
