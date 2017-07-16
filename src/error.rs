use std::io;

use hyper;
use serde_json;
use roadrunner;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Uri(hyper::error::UriError),
    Http(hyper::error::Error),
    Json(serde_json::Error),
    RoadRunner(roadrunner::Error),
    Status(hyper::StatusCode),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<hyper::error::UriError> for Error {
    fn from(err: hyper::error::UriError) -> Error {
        Error::Uri(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<roadrunner::Error> for Error {
    fn from(err: roadrunner::Error) -> Error {
        Error::RoadRunner(err)
    }
}
