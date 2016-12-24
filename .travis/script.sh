#!/bin/bash

set -ev

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
  sudo ldconfig ${HOME}/fswatch/1.9.3/lib
  LIBRARY_PATH=~/fswatch/1.9.3/lib cargo test --verbose
  LIBRARY_PATH=~/fswatch/1.9.3/lib cargo test --verbose --features use_time
  mkdir target/debug/1.9.3
  mv target/debug/fswatch-* target/debug/1.9.3/
  sudo ldconfig ${HOME}/fswatch/1.10.0/lib
  LIBRARY_PATH=~/fswatch/1.10.0/lib cargo test --verbose --features fswatch_1_10_0
  LIBRARY_PATH=~/fswatch/1.10.0/lib cargo test --verbose --features "fswatch_1_10_0 use_time"
  mkdir target/debug/1.10.0
  mv target/debug/fswatch-* target/debug/1.10.0/
fi
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
  cargo test --verbose
  cargo test --verbose --features use_time
fi
