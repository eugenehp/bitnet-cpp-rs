# bitnet-cpp-sys

Raw bindings to bitnet.cpp with cuda and metal support.

See [bitnet-cpp](https://crates.io/crates/bitnet-cpp) for a safe API.


## Development

```shell
cargo clean && BUILD_DEBUG=true CMAKE_VERBOSE=true cargo build
```