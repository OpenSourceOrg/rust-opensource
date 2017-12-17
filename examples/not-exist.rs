extern crate opensource;

use opensource::client;

fn main() {
    let license = client::get("this-license-does-not-exist");
    match license {
        Ok(license) => println!("{}", license.name),
        Err(err) => println!("{}", err),
    }
}
