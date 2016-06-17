// Copyright © 2016 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod client;
pub mod license;
