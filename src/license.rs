// Copyright © 2016 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#[derive(Debug, Deserialize)]
pub struct License {
    pub id: String,
    pub identifiers: Vec<Identifier>,
    pub keywords: Vec<String>,
    pub links: Vec<Link>,
    pub name: String,
    pub other_names: Vec<OtherName>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub superseded_by: Option<String>,
    pub text: Vec<Text>,
}

#[derive(Debug, Deserialize)]
pub struct Identifier {
    pub identifier: String,
    pub scheme: String,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct OtherName {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Text {
    pub media_type: String,
    pub title: String,
    pub url: String,
}
