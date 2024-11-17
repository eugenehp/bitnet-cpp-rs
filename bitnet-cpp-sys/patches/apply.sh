#!/bin/sh

PATCHES="$( cd "$( dirname "$0" )" && pwd )"
DIR="$PATCHES/.."

cd "$DIR/bitnet"
git reset --hard # reset state of the git submodule
git clean -f -d # cleans untracked files
git apply --whitespace=fix "$PATCHES/bitnet.patch"

cd "$DIR/bitnet/3rdparty/llama.cpp"
git reset --hard # reset state of the git submodule
git clean -f -d # cleans untracked files
git apply "$PATCHES/llama.cpp.patch"