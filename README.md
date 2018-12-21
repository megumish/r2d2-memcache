# r2d2-memcache
[![Build Status](https://travis-ci.org/megumish/r2d2-memcache.svg?branch=master)](https://travis-ci.org/megumish/r2d2-memcache)
[![Latest Version](https://img.shields.io/crates/v/r2d2-memcache.svg)](https://crates.io/crates/r2d2-memcache)
[![Docs](https://docs.rs/r2d2-memcache/badge.svg)](https://docs.rs/r2d2-memcache/)

## Install

Add this to your `Cargo.toml`:

```toml
[dependencies]
r2d2-memcache = "0.2.0"
```

## Basic Usage

```rust
extern crate r2d2_memcache;

fn main() {
    let manager = r2d2_memcache::MemcacheConnectionManager::new("memcache://localhost:11211");
    let pool = r2d2_memcache::r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    conn.flush().unwrap();
    let value = "bar";
    conn.set("foo_get", value, 10).unwrap();
    let result: String = conn.get("foo_get").unwrap().unwrap();
    assert!(result == "bar");
}
```

## License
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
 * This software includes the work that is distributed in the Apache License 2.0
