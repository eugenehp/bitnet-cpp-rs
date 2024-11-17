#!/bin/sh

DIR="$( cd "$( dirname "$0" )" && pwd )"

cd "$DIR/../bitnet-cpp-sys/bitnet"
git reset --hard # reset state of the git submodule

touch include/kernel_config.ini
patch include/kernel_config.ini "$DIR/kernel_config.patch"

touch include/bitnet-lut-kernels.h
patch include/bitnet-lut-kernels.h "$DIR/bitnet-lut-kernels.patch"

git apply "$DIR/bitnet.patch"
# git apply "$DIR/bitnet-lut-kernels.patch"
# git apply "$DIR/kernel_config.patch"

# cd "$DIR/../bitnet-cpp-sys/bitnet/3rdparty/llama.cpp"
# git apply "$DIR/llama.cpp.patch"