use core::fmt;

pub type Result<T> = std::result::Result<T, HuffmanError<'static>>;

#[derive(Debug)]
pub enum HuffmanError<'a> {
    TreeError(&'a str),
    ByteStringConversionError(&'a str),
}

impl fmt::Display for HuffmanError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HuffmanError::ByteStringConversionError(e) => {
                write!(f, "Binary String Conversion Error: {}", e)
            }
            HuffmanError::TreeError(e) => write!(f, "Tree Error: {}", e),
        }
    }
}
