# bitnet-cpp-sys

Raw bindings to bitnet.cpp.

See [bitnet-cpp](https://crates.io/crates/bitnet-cpp) for a safe API.


## Development

Atomic git patches are incorporated into the `build.rs`, but if you want to dive deeper, see the patch generation below.

```shell
cargo clean && cargo build
```

## How to manually apply generated patches

Apply patches from the python generation:

```shell
./patches/apply.sh
```

## Updating submodules

Generate python changes:

```shell
cd bitnet
pip install -r requirements.txt
python setup_env.py --hf-repo HF1BitLLM/Llama3-8B-1.58-100B-tokens -q i2_s
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
# this won't build without cleaning first, though normal cargo build works as is.
cargo clean && BUILD_DEBUG=true CMAKE_VERBOSE=true cargo build
```

## Publishing

Check which files will be included in the published crate:

```shell
cargo package --list --allow-dirty
```

Do a dry run:

```shell
cargo publish --allow-dirty --dry-run 
```

Or you can simply remove `bitnet/3rdparty/llama.cpp/common/build-info.cpp` and run:

```shell
cargo publish --dry-run
```

## License

[MIT](/LICENSE)

## Copyright

© 2024, Eugene Hauptmann