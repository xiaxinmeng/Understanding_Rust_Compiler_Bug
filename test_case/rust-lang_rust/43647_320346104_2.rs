sh
sudo env UNIFIED_CGROUP_HIERARCHY=no \
    rkt run --insecure-options=image docker://alpine:3.5 \
        --volume=bla,kind=host,source=/tmp/bla/target/x86_64-unknown-linux-musl/debug/ \
        --mount volume=bla,target=/bla/ \
        --exec=/bin/sh -- -c 'ldd -- /bla/bla; stat /bla/bla; /bla/bla; stat /lib64/ld-linux-x86-64.so.2'
