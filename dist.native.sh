#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

export NATIVE=1

if [ -z "${HOME}" ]; then
  export HOME=/root
fi

# ./sh/jpegxl-rs.sh
if ! [ -x "$(command -v cargo)" ]; then
  cargo_env=$HOME/.cargo/env
  if [ -f "$cargo_env" ]; then
    source $cargo_env
  fi
fi
if ! [ -x "$(command -v cargo)" ]; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain nightly
  source $cargo_env
fi

source ./sh/cflag.sh

cargo build $RUST_FEATURES --release --target $RUST_TARGET

name=$(grep "^name" Cargo.toml | sed 's/name = //g' | awk -F\" '{print $2}')

pre=/opt/bin/$name

if [ -f "$pre" ]; then
  rm -rf /tmp/$name
  sudo mv $pre /tmp
fi

sudo mkdir -p /opt/bin
sudo mv target/$RUST_TARGET/release/$name /opt/bin

case $(uname -s) in
Linux*)
  sudo systemctl restart $name || sudo ./service.sh
  sleep 5

  if ! sudo systemctl is-active --quiet img.service; then
    sudo journalctl -u $name -n 10 --no-pager --no-hostname
    echo -e "\n\n ❗服务启动失败\n\n"
    sudo rm -rf /tmp/$name.failed
    sudo mv /opt/bin/$name /tmp/$name.failed
    sudo mv /tmp/$name /opt/bin/$name
    sudo systemctl restart $name && sleep 5 || true
  fi

  sudo systemctl status $name --no-pager
  sudo journalctl -u $name -n 10 --no-pager --no-hostname
  ;;
*)
  ./supervisor.sh
  ;;
esac
