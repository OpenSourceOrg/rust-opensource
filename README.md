<a href="https://opensource.org/licenses"><img align="right" width="150" height="200" src="https://opensource.org/files/OSIApproved.png"></a>
# rust-opensource #

`rust-opensource` is an API Wrapper that allows you to query the Open Source
License API with Rust.

## Install ##

The crate is called `opensource` and you can depend on it via cargo:

```ini
[dependencies]
opensource = "0.2.0"
```

Documentation can be found at
[OpenSourceOrg.github.io/rust-opensource](https://OpenSourceOrg.github.io/rust-opensource).

# Examples #

```rust
extern crate opensource;

use opensource::client;

fn main() {
    let license = client::get("BSD-3").unwrap();
    println!("{}", license.name);
}
```

A better way is to use match:

```rust
extern crate opensource;

use opensource::client;

fn main() {
    let license = client::get("this-license-does-not-exist");
    match license {
        Ok(license) => println!("{}", license.name),
        Err(err) => println!("{}", err),
    }
}
```
