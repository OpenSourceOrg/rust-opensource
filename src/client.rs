// Copyright © 2016 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::io::{self, Read};

use hyper::{self, Client};
use serde_json;
use url::{self, Url};

use super::license::License;

const BASE_URL: &'static str = "https://api.opensource.org";

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
    APIError { errors: Vec<Message> }
}

#[derive(Debug, Deserialize)]
struct APIError {
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
        let err: APIError = try!(serde_json::from_str(&body));
        Err(ClientError { detail: ErrorDetail::APIError{ errors: err.errors } })
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

pub fn get(id: &str) -> Result<License, ClientError> {
    let path = format!("license/{}", id);
    license!(&path, License)
}

pub fn all() -> Result<Vec<License>, ClientError> {
    license!("licenses/", Vec<License>)
}

pub fn tagged(keyword: &str) -> Result<Vec<License>, ClientError> {
    let path = format!("licenses/{}", keyword);
    license!(&path, Vec<License>)
}
