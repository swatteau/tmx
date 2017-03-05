[![License](https://img.shields.io/badge/license-Apache%202-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

## About `tmx`

`tmx` is a simple Rust crate for reading [Tiled](http://www.mapeditor.org/) XML files.

## Getting Started

```rust
extern crate tmx;

fn main() {
    match tmx::Map::open("some_file.tmx") {
        Ok(map) => println!("Got a map!"),
        Err(e) => println!("Got an error: {}", e)
    };
}
```

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
See the accompanying [LICENSE](LICENSE.txt) file.
