# postcode-rs
[![crates.io](https://img.shields.io/crates/d/postcode)](https://crates.io/crates/postcode)
[![crates.io](https://img.shields.io/crates/v/postcode)](https://crates.io/crates/postcode)
[![API](https://docs.rs/postcode/badge.svg)](https://docs.rs/postcode)

Postcode & Geolocation API for the UK. This is an API client for [postcodes.io](https://postcodes.io/).

```
postcode = "0.1.1"
```

# Example
```rust
use postcode::Postcode;

#[async_std::main]
async fn main() {
    let code = "SW1W0NY";
    
    let post = Postcode::from_code(code).await.unwrap();

    println!("{} ({}, {}) -> ({}, {})", code, post.region, post.country, post.latitude, post.longitude);
}
```
```
SW1W0NY (London, England) -> (51.495373, -0.147421)
```

# License
```
MIT License

Copyright (c) 2021 Grant Handy

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```