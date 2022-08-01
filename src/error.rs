use std::error;
use std::io;
use std::fmt;
use std::string;

#[derive(Debug)]
pub enum UpdaterError {
    Base,
    IO(io::Error),
    Http(reqwest::Error),
    StringErr(string::FromUtf8Error)
}

impl fmt::Display for UpdaterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            UpdaterError::Base => write!(f, "An error happened"),
            UpdaterError::IO(e) => write!(f, "IOError: {}", e),
            UpdaterError::Http(e) => write!(f, "HTTPError: {}", e),
            UpdaterError::StringErr(e) => write!(f, "StringError: {}", e),
        }
    }
}

impl error::Error for UpdaterError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            UpdaterError::Base => None,
            UpdaterError::IO(ref e) => Some(e),
            UpdaterError::Http(ref e) => Some(e),
            UpdaterError::StringErr(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for UpdaterError {
    fn from(err: io::Error) -> UpdaterError {
        UpdaterError::IO(err)
    }
}

impl From<reqwest::Error> for UpdaterError {
    fn from(err: reqwest::Error) -> UpdaterError {
        UpdaterError::Http(err)
    }
}

impl From<string::FromUtf8Error> for UpdaterError {
    fn from(err: string::FromUtf8Error) -> UpdaterError {
        UpdaterError::StringErr(err)
    }
}
