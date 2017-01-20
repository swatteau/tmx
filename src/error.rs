use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    BadXml,
    BadAxis(String),
    BadIndex(String),
    BadOrientation(String),
    BadPropertyType(String),
    BadRenderOrder(String),
    BadDrawOrder(String),
    BadProbability(f32),
    UnknownAttribute(String),
    InvalidColor(String),
    InvalidNumber(String),
    InvalidPoint(String),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadXml => write!(f, "Invalid XML input"),
            Error::BadAxis(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `staggeraxis` attribute",
                       value)
            }
            Error::BadIndex(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `staggerindex` attribute",
                       value)
            }
            Error::BadOrientation(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `orientation` attribute",
                       value)
            }
            Error::BadPropertyType(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `type` attribute",
                       value)
            }
            Error::BadRenderOrder(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `renderorder` attribute",
                       value)
            }
            Error::BadDrawOrder(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `draworder` attribute",
                       value)
            }
            Error::BadProbability(ref value) => {
                write!(f,
                       "Illegal value `{}` for the `probability` attribute",
                       value)
            }
            Error::UnknownAttribute(ref attr) => write!(f, "Unknown attribute: `{}`", attr),
            Error::InvalidColor(ref color) => write!(f, "Invalid color: `{}`", color),
            Error::InvalidNumber(ref num) => write!(f, "Invalid number: `{}`", num),
            Error::InvalidPoint(ref point) => write!(f, "Invalid point: `{}`", point),
            Error::Io(ref err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadXml => "Invalid XML input",
            Error::BadAxis(..) => "Bad axis value",
            Error::BadIndex(..) => "Bad index value",
            Error::BadOrientation(..) => "Bad orientation value",
            Error::BadPropertyType(..) => "Bad property type value",
            Error::BadRenderOrder(..) => "Bad renderorder value",
            Error::BadDrawOrder(..) => "Bad draworder value",
            Error::BadProbability(..) => "Bad probability value",
            Error::UnknownAttribute(..) => "Unknown attribute",
            Error::InvalidColor(..) => "Invalid color",
            Error::InvalidNumber(..) => "Invalid number",
            Error::InvalidPoint(..) => "Invalid point",
            Error::Io(ref err) => err.description(),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
