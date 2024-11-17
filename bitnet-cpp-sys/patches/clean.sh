#!/bin/sh

DIR="$( cd "$( dirname "$0" )" && pwd )"

cd "$DIR/bitnet/3rdparty/llama.cpp"
git reset --hard
git clean -f -d

cd "$DIR/bitnet"
git reset --hard
git clean -f -d

