
rust/src/ci/docker/disabled $ grep -r ^COPY * | head -n3
aarch64-gnu/Dockerfile:COPY disabled/aarch64-gnu/config /build/.config
aarch64-gnu/Dockerfile:COPY scripts/qemu-bare-bones-rcS rootfs/etc/init.d/rcS
aarch64-gnu/Dockerfile:COPY scripts/qemu-bare-bones-addentropy.c /tmp/addentropy.c
