use hyper;
use hyper::status::StatusCode;

use serde_json;

use std;

#[derive(Debug)]
pub enum Error {
    Connection(hyper::Error),
    Unauthorized,
    UnexpectedResponse(StatusCode),
    FailedToReadResponse(std::io::Error),
    FailedToParseJSON(serde_json::Error)
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Connection(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::FailedToParseJSON(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::FailedToReadResponse(err)
    }
}

pub fn error_description(error: &Error) -> String {
    match *error {
        Error::Connection(ref err) => format!("There is a network issue: '{}'", err),
        Error::Unauthorized => "Failed to access Feedly API, it's either your access token is expired or invalid".to_string(),
        Error::UnexpectedResponse(code) => format!("Feedly API has returned unexpected status code {}", code),
        Error::FailedToReadResponse(ref err) => format!("Failed to read Feedly API response body: '{}'", err),
        Error::FailedToParseJSON(ref err) => format!("Failed to parse JSON response from Feedly API: '{}'", err)
    }
}
