# bitnet-cpp-sys

Raw bindings to bitnet.cpp with cuda and metal support.

See [bitnet-cpp](https://crates.io/crates/bitnet-cpp) for a safe API.


## Development

Apply patches from the python generation:

```shell
./patches/apply.sh
```

Create patches after you're done generating python changes:

```shell
./patches/create.sh
```

Clean git commits from submodules to start over or run commits:

```shell
./patches/clean.sh
```

```shell
cargo clean && BUILD_DEBUG=true CMAKE_VERBOSE=true cargo build
```