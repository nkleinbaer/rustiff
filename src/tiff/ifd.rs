use crate::tiff::error::TiffErrorKind;
use crate::tiff::header::ByteOrder;

use super::error::TiffError;

pub const MINIMUM_IFD_LENGTH: usize = 126;
pub const MINIMUM_IFD_FIELDS: usize = 10;
pub const IFD_FIELD_LENGTH: usize = 12;

pub struct ImageFileDirectory {
    pub n_fields: u16,
    pub next_ifd_offset: u32,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub tag: u16,
    pub ftype: FieldType,
    pub count: u32,
    pub value: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Clone, Copy, Debug)]
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

impl FieldType {
    // Mapping integer codes to enum members
    fn from_u16(code: u16) -> Option<FieldType> {
        match code {
            1 => Some(FieldType::Byte),
            2 => Some(FieldType::ASCII),
            3 => Some(FieldType::Short),
            4 => Some(FieldType::Long),
            5 => Some(FieldType::Rational),
            6 => Some(FieldType::SByte),
            7 => Some(FieldType::Undefined),
            8 => Some(FieldType::SShort),
            9 => Some(FieldType::SLong),
            10 => Some(FieldType::SRational),
            11 => Some(FieldType::Float),
            12 => Some(FieldType::Double),
            _ => None,
        }
    }

    // Getting the byte length of data types
    fn type_length(&self) -> usize {
        match self {
            FieldType::Byte | FieldType::SByte | FieldType::ASCII => 1,
            FieldType::Short | FieldType::SShort => 2,
            FieldType::Long | FieldType::SLong | FieldType::Float => 4,
            FieldType::Rational | FieldType::SRational | FieldType::Double => 8,
            FieldType::Undefined => 1, // Undefined can be considered as 1 byte but may vary
        }
    }
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

fn get_fields(bytes: &[u8], byte_order: ByteOrder, n_fields: u16) -> Result<Vec<Field>, TiffError> {
    let mut fields = Vec::new();

    let mut pointer: usize = 2;
    for i in 0..n_fields {
        let tag_bytes: [u8; 2] = bytes[pointer..pointer + 2].try_into().unwrap();
        let ftype_bytes: [u8; 2] = bytes[pointer + 2..pointer + 4].try_into().unwrap();
        let count_bytes: [u8; 4] = bytes[pointer + 4..pointer + 8].try_into().unwrap();
        let value_or_offset_bytes: [u8; 4] = bytes[pointer + 8..pointer + 12].try_into().unwrap();

        let tag = u16::from_le_bytes(tag_bytes);
        let ftype = match FieldType::from_u16(u16::from_le_bytes(ftype_bytes)) {
            Some(field_type) => field_type,
            None => return Err(TiffError::new(TiffErrorKind::InvalidIFD)),
        };
        let count = u32::from_le_bytes(count_bytes);
        let value_or_offset = u32::from_le_bytes(value_or_offset_bytes);

        let field = Field {
            tag,
            ftype,
            count,
            value: None,
            offset: None,
        };

        if (count as usize) * ftype.type_length() <= 4 {
            fields.push(Field {
                value: Some(value_or_offset),
                ..field
            });
        } else {
            fields.push(Field {
                offset: Some(value_or_offset),
                ..field
            });
        };

        pointer += 12;
    }
    Ok(fields)
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

    let fields = get_fields(bytes, byte_order, n_fields)?;

    Ok(ImageFileDirectory {
        n_fields,
        next_ifd_offset,
        fields,
    })
}
