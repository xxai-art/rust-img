#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cargo build

TO="https://f004.backblazeb2.com/file/xxai-jxl/" \
  RUST_BACKTRACE=short \
  exec ${1:-./target/debug/img}
