# rust-opensource #

`rust-opensource` is an API Wrapper that allows you to query the Open Source
License API with Rust.

## Install ##

The crate is called `opensource` and you can depend on it via cargo:

```ini
[dependencies]
opensource = "0.1.0"
```

**NOTE:** It's currently using `serde` so you must use `nigtly` for now.

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
