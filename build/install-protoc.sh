#!/bin/sh
set -e
# check to see if protobuf folder is empty
if [ ! -d "$HOME/protobuf/lib" ]; then
  wget https://github.com/protocolbuffers/protobuf/releases/download/v3.8.0/protobuf-all-3.8.0.tar.gz
  tar -xzvf protobuf-all-3.8.0.tar.gz
  cd protobuf-3.8.0 && ./configure --prefix=$HOME/protobuf && make && make install
else
  echo "Using cached protobuf."
fi
