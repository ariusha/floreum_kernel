#!/bin/sh
xorriso -as mkisofs \
    -b limine/limine-bios-cd.bin \
    -no-emul-boot \
    -boot-load-size 4 \
    -boot-info-table \
    --efi-boot limine/limine-uefi-cd.bin \
    -efi-boot-part --efi-boot-image \
    --protective-msdos-label \
    ./drive \
    -o "floreum.iso"
./limine/limine bios-install floreum.iso