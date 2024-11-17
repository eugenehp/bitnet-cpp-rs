#!/bin/sh

PATCHES="$( cd "$( dirname "$0" )" && pwd )"
DIR="$PATCHES/.."

cd "$DIR/bitnet/3rdparty/llama.cpp"
git reset --hard origin/HEAD
git clean -f -d

cd "$DIR/bitnet"
git reset --hard
git clean -f -d

