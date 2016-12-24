#!/bin/bash

set -ev

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
  sudo ldconfig ~/fswatch/1.9.3/lib
  for file in target/debug/1.9.3/fswatch-*; do
    mkdir -p "target/cov/$(basename $file)"
    kcov --verify --exclude-pattern=/.cargo,/usr "target/cov/$(basename $file)" "$file"
  done
  sudo ldconfig ~/fswatch/1.10.0/lib
  for file in target/debug/1.10.0/fswatch-*; do
    mkdir -p "target/cov/$(basename $file)"
    kcov --verify --exclude-pattern=/.cargo,/usr "target/cov/$(basename $file)" "$file"
  done
  bash <(curl -s https://codecov.io/bash) &&
  echo "Uploaded code coverage";
fi
