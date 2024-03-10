use crate::tiff::error::TiffErrorKind;
use crate::tiff::header::ByteOrder;

use super::error::TiffError;

pub const MINIMUM_IFD_LENGTH: usize = 126;
pub const MINIMUM_IFD_FIELDS: usize = 10;
pub const IFD_FIELD_LENGTH: usize = 12;

pub struct ImageFileDirectory {
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

fn get_n_fields(bytes: &[u8], byte_order: ByteOrder) -> u16 {
    let n_fields: [u8; 2] = bytes[..2].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => u16::from_le_bytes(n_fields),
        ByteOrder::BigEndian => u16::from_be_bytes(n_fields),
    }
}

fn get_next_ifd_offset(bytes: &[u8], n_fields: u16, byte_order: ByteOrder) -> u32 {
    let offset: usize = (n_fields * 12).into();

    let next_ifd_offset: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();

    match byte_order {
        ByteOrder::LittleEndian => u32::from_le_bytes(next_ifd_offset),
        ByteOrder::BigEndian => u32::from_be_bytes(next_ifd_offset),
    }
}

pub fn parse_ifd(
    bytes: &[u8],
    offset: u32,
    byte_order: ByteOrder,
) -> Result<ImageFileDirectory, TiffError> {
    let n_bytes = bytes.len();

    if n_bytes < MINIMUM_IFD_LENGTH {
        return Err(TiffError::new(TiffErrorKind::InvalidIFD));
    }

    let n_fields = get_n_fields(&bytes, byte_order);

    let n_extra_fields = (n_fields as usize) - MINIMUM_IFD_FIELDS;

    if n_extra_fields < 0 || n_bytes < MINIMUM_IFD_LENGTH + (n_extra_fields * IFD_FIELD_LENGTH) {
        return Err(TiffError::new(TiffErrorKind::InvalidIFD));
    }

    let next_ifd_offset = get_next_ifd_offset(&bytes, n_fields, byte_order);

    Ok(ImageFileDirectory {
        n_fields,
        next_ifd_offset,
    })
}
