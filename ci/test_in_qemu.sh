#!/bin/sh

# Compiles a test harness and then runs it in QEMU.

if [ -z $RUSTC_TARGET ]; then RUSTC_TARGET=$TARGET; fi

# Compile test harness
rustc -O -o tests.elf --test src/lib.rs \
    --target $RUSTC_TARGET -C linker="$GCC_VARIANT-gcc"

# Update initrd
unxz -c ci/$ARCH/initrd.xz > initrd
mkdir initramfs
cd initramfs
cpio -i < ../initrd
mkdir lib
cp /usr/$GCC_VARIANT/lib/* lib
mv ../tests.elf .
find . | cpio -o -H newc > ../initrd
cd ..

# Run test in QEMU
qemu-system-$ARCH -kernel ci/$ARCH/$TARGET/zImage -initrd initrd \
    -nographic -no-reboot -append 'console=ttyS0' $QEMU_ARGS | tee out.log

# Check if it worked
grep "0 failed;" out.log
