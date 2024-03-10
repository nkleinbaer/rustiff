#[derive(Clone, Copy, Debug)]
pub enum TiffErrorKind {
    InvalidHeader,
    InvalidByteOrder,
    InvalidMagicBytes,
    InvalidIFD,
}

#[derive(Debug)]
pub struct TiffError {
    kind: TiffErrorKind,
}

impl TiffError {
    pub fn new(kind: TiffErrorKind) -> TiffError {
        TiffError { kind }
    }
    fn kind(&self) -> TiffErrorKind {
        self.kind
    }
}
