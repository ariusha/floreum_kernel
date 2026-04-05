qemu-system-x86_64 \
    -machine q35,kernel-irqchip=split \
    -cpu +x2apic \
    --no-reboot \
    -m 3G \
    -smp 4 \
    -serial stdio \
    -drive if=pflash,format=raw,unit=0,readonly=on,file=$HOME/ovmf/OVMF.fd \
    -drive if=pflash,format=raw,unit=1,file=$HOME/ovmf/OVMF_VARS.fd \
    -cdrom floreum.iso