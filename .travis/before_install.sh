#!/bin/bash

set -ev

if [[ "$TRAVIS_OS_NAME" == "linux" ]]; then
  gettext --version
  wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
  tar xzf master.tar.gz
  cd kcov-master
  mkdir build
  cd build
  cmake ..
  sudo make install
  cd ../..
  rm -rf kcov-master master.tar.gz
  wget https://github.com/emcrisostomo/fswatch/releases/download/1.9.3/fswatch-1.9.3.tar.gz
  tar xzf fswatch-1.9.3.tar.gz
  cd fswatch-1.9.3
  ./configure --prefix=${HOME}/fswatch/1.9.3
  make install
  cd ..
  rm -rf fswatch-1.9.3 fswatch-1.9.3.tar.gz
  wget https://github.com/emcrisostomo/fswatch/archive/release/1.10.0.tar.gz
  tar xzf 1.10.0.tar.gz
  cd fswatch-release-1.10.0
  # desperate times...
  sed -Eie "s/AM_GNU_GETTEXT_VERSION\\(\\[0.19.4\\]\\)/AM_GNU_GETTEXT_VERSION([0.18.3])/g" configure.ac
  ./autogen.sh
  ./configure --prefix=${HOME}/fswatch/1.10.0
  make install
  cd ..
  rm -rf 1.10.0.tar.gz fswatch-release-1.10.0
fi
if [[ "$TRAVIS_OS_NAME" == "osx" ]]; then
  brew update
  brew install fswatch
fi
