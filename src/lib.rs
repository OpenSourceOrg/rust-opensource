// Copyright © 2016 Daniele Tricoli <eriol@mornie.org>.
// All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

//! # opensource #
//!
//! `opensource` is an API Wrapper that allows you to query the Open Source
//! License API with Rust.
//!
//! ## Example ##
//!
//! ```no_run
//! extern crate opensource;
//!
//! use opensource::client;
//!
//! fn main() {
//!     let license = client::get("BSD-3").unwrap();
//!     println!("{}", license.name);
//! }
//! ```
//!
//! A better way is to use match:
//!
//! ```no_run
//! extern crate opensource;
//!
//! use opensource::client;
//!
//! fn main() {
//!     let license = client::get("this-license-does-not-exist");
//!     match license {
//!         Ok(license) => println!("{}", license.name),
//!         Err(err) => println!("{}", err),
//!     }
//! }
//! ```

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod client;
pub mod license;
