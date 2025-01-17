# BitNet.cpp for Rust

Almost safe wrapper for [BitNet-cpp](https://github.com/microsoft/BitNet) based on [llama-cpp](https://github.com/ggerganov/llama.cpp)

## Development

See [bitnet-cpp-sys](https://github.com/eugenehp/bitnet-cpp-rs/tree/main/bitnet-cpp-sys)

## Roadmap

- [x] minimize crate size
- [x] rewrite sampler [PR 9294](https://github.com/ggerganov/llama.cpp/pull/9294)
- [ ] look into Metal implementation of MatMul kernerls for BitNet
- [ ] add more examples
- [x] replace all remaining python from BitNet repo
- [ ] add better division between `arm64` and `x86_64`
- [x] move python code generation into `patch` files

## Platform support

| arm64 | x64 | OS | comments |
|---|---|---|---|
| ✅ | ❌ | MacOS | tested fully on Apple Silicon macs | 
| ⚠️ | ❌ | Linux | should test on ARM based linux | 
| ❓ | ❌ | Windows | should test on ARM based windows | 

## Thanks

Heavily inspired by [llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs)

## License

[MIT](/LICENSE)

## Copyright

© 2024-2025, Eugene Hauptmann
