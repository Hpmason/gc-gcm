#[cfg(not(feature = "no_std"))]
use std::io;

/// An error resulting from parsing or opening a GcmFile
#[derive(Debug)]
pub enum GcmError {
    ParseError(binrw::Error),

    #[cfg(not(feature = "no_std"))]
    IoError(std::io::Error),
}

#[cfg(not(feature = "no_std"))]
impl From<io::Error> for GcmError {
    fn from(err: io::Error) -> Self {
        GcmError::IoError(err)
    }
}

impl From<binrw::Error> for GcmError {
    fn from(err: binrw::Error) -> Self {
        GcmError::ParseError(err)
    }
}
