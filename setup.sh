#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

direnv allow
./sh/libjxl.sh
./dist.native.sh
rm -rf jpegxl-rs
