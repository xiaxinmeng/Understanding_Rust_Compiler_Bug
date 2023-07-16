bash
#!/usr/bin/bash

mkdir rusty

export CARGO_HOME="$PWD/rusty" RUSTUP_HOME="$PWD/rusty"
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:$PWD/rusty/lib/pkgconfig
export CFLAGS+=" -static -static-libgcc" LDFLAGS+=" -static-libgcc"


curl https://sh.rustup.rs -sSo rustup.sh
./rustup.sh -v -y --no-modify-path --default-host=${MSYSTEM_CARCH:-x86_64}-pc-windows-gnu --default-toolchain=stable

export PATH=$PATH:$PWD/rusty/bin

# cargo version at this time: cargo 1.45.1 (f242df6ed 2020-07-22)

cargo install cargo-c

git clone --branch master --depth 1 https://github.com/xiph/rav1e.git || {
    git -C rav1e fetch
    git remote set-head -a origin
    git -C rav1e reset --hard origin/HEAD
}

(
    cd rav1e &&
        cargo capi install --release --prefix "$PWD/local" --jobs "$(nproc)" &&
        install -Dp -t "$CARGO_HOME/lib" "./local/lib/librav1e.a" && 
        install -Dp -t "$CARGO_HOME/lib/pkgconfig" "./local/lib/pkgconfig/rav1e.pc" && 
        install -Dp -t "$CARGO_HOME/include/rav1e" "./local/include/rav1e"/*.h
) || exit

git clone --branch master --depth 1 https://git.ffmpeg.org/ffmpeg.git || {
    git -C ffmpeg fetch
    git remote set-head -a origin
    git -C ffmpeg reset --hard origin/HEAD
}

find . -name rav1e.dll -o -name librav1e.dll.a -delete

(
    cd ffmpeg &&
        ./configure --enable-librav1e --disable-shared --enable-static --disable-ffprobe --pkg-config-flags=--static &&
        make -j "$(nproc)"
) || exit
