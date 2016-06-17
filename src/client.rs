// Copyright © 2016 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::error;
use std::fmt;
use std::io::{self, Read};

use hyper::{self, Client};
use serde_json;
use url::{self, Url};

use super::license::License;

const BASE_URL: &'static str = "https://api.opensource.org";

/// A client error. You can use the Error trait to interact with it.
#[derive(Debug)]
pub struct ClientError {
    detail: ErrorDetail,
}

#[derive(Debug)]
enum ErrorDetail {
    ParseError(url::ParseError),
    ConnectionError(hyper::Error),
    ReadError(io::Error),
    JsonError(serde_json::Error),
    APIError(RequestError),
}

#[derive(Debug, Deserialize)]
struct RequestError {
    pub errors: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct Message {
    pub message: String
}

impl From<url::ParseError> for ClientError {
    fn from(err: url::ParseError) -> ClientError {
        ClientError { detail: ErrorDetail::ParseError(err) }
    }
}

impl From<hyper::Error> for ClientError {
    fn from(err: hyper::Error) -> ClientError {
        ClientError { detail: ErrorDetail::ConnectionError(err) }
    }
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> ClientError {
        ClientError { detail: ErrorDetail::ReadError(err) }
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> ClientError {
        ClientError { detail: ErrorDetail::JsonError(err) }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.detail {
            ErrorDetail::ParseError(ref err) => err.fmt(f),
            ErrorDetail::ConnectionError(ref err) => err.fmt(f),
            ErrorDetail::ReadError(ref err) => err.fmt(f),
            ErrorDetail::JsonError(ref err) => err.fmt(f),
            ErrorDetail::APIError(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for ClientError {
    fn description(&self) -> &str {
        match self.detail {
            ErrorDetail::ParseError(ref err) => err.description(),
            ErrorDetail::ConnectionError(ref err) => err.description(),
            ErrorDetail::ReadError(ref err) => err.description(),
            ErrorDetail::JsonError(ref err) => err.description(),
            ErrorDetail::APIError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.detail {
            ErrorDetail::ParseError(ref err) => Some(err as &error::Error),
            ErrorDetail::ConnectionError(ref err) => Some(err as &error::Error),
            ErrorDetail::ReadError(ref err) => Some(err as &error::Error),
            ErrorDetail::JsonError(ref err) => Some(err as &error::Error),
            ErrorDetail::APIError(ref err) => Some(err as &error::Error),
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.errors)
    }
}

impl error::Error for RequestError {
    fn description(&self) -> &str {
        "An API error occurred."
    }

    fn cause(&self) -> Option<&error::Error> {
        Some(self as &error::Error)
    }
}

fn url_join(input: &str) -> Result<Url, url::ParseError> {
    try!(Url::parse(BASE_URL)).join(input)
}

fn api_call(path: &str) -> Result<String, ClientError> {
    let url = try!(url_join(path));
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut body = String::new();
    try!(response.read_to_string(&mut body));
    if response.status == hyper::Ok {
        Ok(body)
    } else {
        let err: RequestError = try!(serde_json::from_str(&body));
        Err(ClientError { detail: ErrorDetail::APIError(err) })
    }
}

macro_rules! license {
    ($path:expr, $license:ty) => (
    match api_call($path) {
        Ok(data) => {
            let l: $license = try!(serde_json::from_str(&data));
            Ok(l)
        }
        Err(e) => Err(e),
    }
    )
}

/// Return a License for the given `id`.
pub fn get(id: &str) -> Result<License, ClientError> {
    let path = format!("license/{}", id);
    license!(&path, License)
}

/// Return a Vec of all known Licenses.
pub fn all() -> Result<Vec<License>, ClientError> {
    license!("licenses/", Vec<License>)
}

/// Return a Vec of all Licenses contain `keyword`.
pub fn tagged(keyword: &str) -> Result<Vec<License>, ClientError> {
    let path = format!("licenses/{}", keyword);
    license!(&path, Vec<License>)
}
