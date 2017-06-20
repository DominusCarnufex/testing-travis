#!/bin/sh

# Installs a recent QEMU for the given arch.

sudo apt-get install -y --no-install-recommends curl xz-utils
curl http://download.qemu.org/qemu-2.9.0.tar.xz | tar xJf -
cd qemu-2.9.0
./configure --target-list=$1-softmmu --disable-docs
time make -j4
sudo make install
