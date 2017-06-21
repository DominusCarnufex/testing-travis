#!/bin/sh

# Compiles a test harness and then runs it in QEMU.

# Compile test harness
rustc -O -o tests.elf --test src/lib.rs \
    --target $TARGET -C linker="$GCC_VARIANT-gcc"

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

# Launch QEMU
qemu-system-arm -kernel ci/$ARCH/$TARGET/zImage -initrd initrd \
    -nographic -no-reboot $QEMU_ARGS | tee out.log

# Check if tests went right
grep "0 failed;" out.log
