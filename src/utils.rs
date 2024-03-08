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
