socket for rust
=====================

[![Build Status](https://travis-ci.org/tickbh/psocket-rs.svg?branch=master)](https://travis-ci.org/tickbh/psocket-rs) [![Crates.io](https://img.shields.io/crates/v/psocket.svg)](https://crates.io/crates/psocket)

A Rust library for socket. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
psocket = "0.1"
```

and this to your crate root:

```rust
extern crate psocket;
```

How to use
```rust
extern crate share_memory;
extern crate time;
use share_memory::ShareMemory;
type ARRAY = [i32; 80];
fn main () {
    let size : usize = std::mem::size_of::<ARRAY>();
    let mut share = ShareMemory::new_create(String::from("."), size * 2, None).unwrap();
    if let Some(addr) = share.first_memory().ok().unwrap() {
        let mut data: &mut ARRAY = unsafe {
            std::mem::transmute(addr)
        };
        data[0] += 1;
        println!("{:?}", data[0]);
    }
    loop {
    }
}
```
now it will print 1, if you start multi process, if you change the data

```rust
// unix为name和path_name得到共享内存的索引
// windows由name得到共享内存的索引,将忽略path_name
pub fn new_create(name: String, size: usize, path_name: Option<String>) -> Result<ShareMemory>;
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
