#!/bin/sh

DIR="$( cd "$( dirname "$0" )" && pwd )"

cd "$DIR/../bitnet-cpp-sys/bitnet/3rdparty/llama.cpp"
git diff > "$DIR/llama.cpp.patch"
# because next section will treat this as a dirty "submodule"
git reset --hard # reset state of the git submodule
git clean -f -d # cleans untracked files

cd "$DIR/../bitnet-cpp-sys/bitnet"
git add -N * # needed because git is not tracking created files
git diff > "$DIR/bitnet.patch"
# diff /dev/null include/bitnet-lut-kernels.h > "$DIR/bitnet-lut-kernels.patch"
# diff /dev/null include/kernel_config.ini > "$DIR/kernel_config.patch"

