#!/bin/sh

DIR="$( cd "$( dirname "$0" )" && pwd )"

cd "$DIR/../bitnet-cpp-sys/bitnet"
git diff > "$DIR/bitnet.patch"
diff /dev/null include/bitnet-lut-kernels.h > "$DIR/bitnet-lut-kernels.patch"
diff /dev/null include/kernel_config.ini > "$DIR/bitnet-lut-kernels.patch"

cd "$DIR/../bitnet-cpp-sys/bitnet/3rdparty/llama.cpp"
git diff > "$DIR/llama.cpp.patch"