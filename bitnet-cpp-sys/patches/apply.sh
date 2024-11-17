#!/bin/sh

DIR="$( cd "$( dirname "$0" )" && pwd )"

cd "$DIR/bitnet"
git reset --hard # reset state of the git submodule
git clean -f -d # cleans untracked files
git apply --whitespace=fix "$DIR/bitnet.patch"

# touch include/kernel_config.ini
# patch include/kernel_config.ini "$DIR/kernel_config.patch"

# touch include/bitnet-lut-kernels.h
# patch include/bitnet-lut-kernels.h "$DIR/bitnet-lut-kernels.patch"


cd "$DIR/bitnet/3rdparty/llama.cpp"
git reset --hard # reset state of the git submodule
git clean -f -d # cleans untracked files
git apply "$DIR/llama.cpp.patch"