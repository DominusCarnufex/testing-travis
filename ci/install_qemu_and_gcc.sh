#!/bin/sh

# Installs a recent QEMU and a cross compiler for the given arch.

if [ -z $LIBC_VARIANT ]; then LIBC_VARIANT=$ARCH; fi

sudo apt-get install -y --no-install-recommends curl xz-utils \
  gcc-$GCC_VARIANT libc6-dev-$LIBC_VARIANT-cross
curl http://download.qemu.org/qemu-2.9.0.tar.xz | tar xJf -
cd qemu-2.9.0
./configure --target-list=$ARCH-softmmu --disable-docs
make -s -j4
sudo make -s install
cd ..
