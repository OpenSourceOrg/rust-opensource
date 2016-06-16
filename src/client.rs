use std::io::{self, Read};

use hyper::{self, Client};
use serde_json;
use url::{self, Url};

use super::license::License;

const BASE_URL: &'static str = "https://api.opensource.org";

#[derive(Debug, Deserialize)]
pub struct ClientError {
    detail: ErrorDetail,
}

#[derive(Debug, Deserialize)]
enum ErrorDetail {
    ParseError,
    ConnectionError,
    ReadError,
    JsonError,
}

impl From<url::ParseError> for ClientError {
    fn from(_: url::ParseError) -> ClientError {
        ClientError { detail: ErrorDetail::ParseError }
    }
}

impl From<hyper::Error> for ClientError {
    fn from(_: hyper::Error) -> ClientError {
        ClientError { detail: ErrorDetail::ConnectionError }
    }
}

impl From<io::Error> for ClientError {
    fn from(_: io::Error) -> ClientError {
        ClientError { detail: ErrorDetail::ReadError }
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(_: serde_json::Error) -> ClientError {
        ClientError { detail: ErrorDetail::JsonError }
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
    Ok(body)
}

fn get_license(id: &str) -> Result<License, ClientError> {
    let path = format!("license/{}", id);
    match api_call(&path) {
        Ok(data) => {
            let l: License = try!(serde_json::from_str(&data));
            Ok(l)
        }
        Err(e) => Err(e),
    }
}

fn get_licenses(path: &str) -> Result<Vec<License>, ClientError> {
    match api_call(&path) {
        Ok(data) => {
            let l: Vec<License> = try!(serde_json::from_str(&data));
            Ok(l)
        }
        Err(e) => Err(e),
    }
}

pub fn get(id: &str) -> Result<License, ClientError> {
    get_license(id)
}

pub fn all() -> Result<Vec<License>, ClientError> {
    get_licenses("licenses/")
}

pub fn tagged(keyword: &str) -> Result<Vec<License>, ClientError> {
    let path = format!("licenses/{}", keyword);
    get_licenses(&path)
}
