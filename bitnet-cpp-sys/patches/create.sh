#!/bin/sh

PATCHES="$( cd "$( dirname "$0" )" && pwd )"
DIR="$PATCHES/.."

cd "$DIR/bitnet/3rdparty/llama.cpp"
git diff > "$PATCHES/llama.cpp.patch"
# because next section will treat this as a dirty "submodule"
git reset --hard # reset state of the git submodule
git clean -f -d # cleans untracked files

cd "$DIR/bitnet"
git add -N * # needed because git is not tracking created files
git diff > "$PATCHES/bitnet.patch"
# diff /dev/null include/bitnet-lut-kernels.h > "$DIR/bitnet-lut-kernels.patch"
# diff /dev/null include/kernel_config.ini > "$DIR/kernel_config.patch"

