# `mac_address`
`mac_address` provides a cross platform way to retrieve the MAC address of network hardware. See [the Wikipedia entry](https://en.wikipedia.org/wiki/MAC_address) for more information.

Currently does not support MacOS.

# Example

```rust
extern crate mac_address;

use mac_address::get_mac_address;

fn main() {
    match get_mac_address() {
        Ok(ma) => { 
            println!("MAC addr = {}", ma);
            println!("bytes = {:?}", ma.bytes());
        },
        Err(e) => println!("{:?}", e)
    }
}
```

# License
`mac_address` is licensed under both MIT and Apache 2.0