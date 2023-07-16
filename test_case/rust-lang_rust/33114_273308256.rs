
qemu-system-ppc -drive file=disk.img,if=virtio,format=raw,index=0 -net nic,macaddr=52:54:00:fa:ce:11,model=virtio -net user -initrd initrd.img-3.16.0-4-powerpc -kernel vmlinux-3.16.0-4-powerpc --append "root=/dev/vda3"
