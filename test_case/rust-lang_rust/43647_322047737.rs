`sh
mkdir /tmp/bla/ || rm -fr /tmp/bla/ &&
sudo env UNIFIED_CGROUP_HIERARCHY=no \
    rkt run --insecure-options=image 'docker://stephank/archlinux:makepkg' \
        --net=host \
        --dns=host \
        --user=0 \
        --working-dir='/bla/' \
        --volume=bla,kind=host,source=/tmp/bla/ \
        --mount volume=bla,target=/bla/ \
        --exec=/bin/sh -- -c '
        set -x
        yes | pacman -Sy rustup musl pax-utils &&
        rustup install nightly-x86_64-unknown-linux-gnu &&
        rustup default nightly-x86_64-unknown-linux-gnu &&
        rustup target add x86_64-unknown-linux-musl &&
        cargo init --bin --name bla --vcs none . &&
        env RUSTFLAGS="-C target-feature=+crt-static" \
            cargo build --target x86_64-unknown-linux-musl &&
        file target/x86_64-unknown-linux-musl/debug/bla
        scanelf -ain target/x86_64-unknown-linux-musl/debug/bla
        target/x86_64-unknown-linux-musl/debug/bla'
        
sudo env UNIFIED_CGROUP_HIERARCHY=no \
    rkt run --insecure-options=image 'docker://alpine' \
        --working-dir='/bla/' \
        --net=host \
        --dns=host \
        --volume=bla,kind=host,source=/tmp/bla/ \
        --mount volume=bla,target=/bla/ \
        --exec=/bin/sh -- -c '
        set -x
        apk add --update file
        target/x86_64-unknown-linux-musl/debug/bla
        ldd target/x86_64-unknown-linux-musl/debug/bla
        file target/x86_64-unknown-linux-musl/debug/bla'
