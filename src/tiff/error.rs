#[derive(Debug)]
pub enum TiffErrorKind {
    InvalidHeader,
    InvalidByteOrder,
    InvalidMagicBytes,
}
