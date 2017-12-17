extern crate opensource;

use opensource::client;

fn main() {
    let license = client::get("BSD-3").unwrap();
    println!("{}", license.name);
}
