
$ qemu-system-arm \
    -M virt \
    -append "root=/dev/vda rw" \
    -hda armhf.qcow2 \
    -kernel vmlinuz-armhf \
    -m 1024 \
    -monitor telnet:127.0.0.1:1234,server,nowait \
    -no-reboot \
    -nographic \
    -net nic,model=virtio \
    -net user,hostfwd=tcp::12345-:12345 \
    -serial stdio
