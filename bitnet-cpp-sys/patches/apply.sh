#!/bin/sh

PATCHES="$( cd "$( dirname "$0" )" && pwd )"
DIR="$PATCHES/.."

cd "$DIR/bitnet"
git reset --hard origin/HEAD # reset state of the git submodule
git clean -f -d # cleans untracked files
git apply --whitespace=fix "$PATCHES/bitnet.patch"
git commit -am "temporary"

cd "$DIR/bitnet/3rdparty/llama.cpp"
git reset --hard origin/HEAD # reset state of the git submodule
git clean -f -d # cleans untracked files
git apply "$PATCHES/llama.cpp.patch"
git commit -am "temporary"