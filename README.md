# BitNet.cpp for Rust

Almost safe wrapper for [BitNet-cpp](https://github.com/microsoft/BitNet) based on [llama-cpp](https://github.com/ggerganov/llama.cpp)

## Development

Add bitnet submodule:

```shell
cd bitnet-cpp-sys
git submodule add --name bitnet --depth 10 -- https://github.com/microsoft/BitNet.git bitnet 
```

Pull recursive updates:

```shell
cd bitnet-cpp-sys/bitnet
git submodule update --init --recursive
```

Configure using python:

```shell
cd bitnet-cpp-sys/bitnet
pip install -r requirements.txt
python setup_env.py --hf-repo HF1BitLLM/Llama3-8B-1.58-100B-tokens -q i2_s # only needs gen_code() 
```

## Roadmap

- [ ] minimize crate size
- [ ] rewrite sampler [PR 9294](https://github.com/ggerganov/llama.cpp/pull/9294)
- [ ] look into Metal implementation of MatMul kernerls for BitNet
- [ ] add more examples
- [ ] replace all remaining python from BitNet repo
- [ ] add better division between `arm64` and `x86_64`

## Thanks

Heavily inspired by [llama-cpp-rs](https://github.com/utilityai/llama-cpp-rs)

## License

[MIT](/LICENSE)

## Copyright

© 2024, Eugene Hauptmann