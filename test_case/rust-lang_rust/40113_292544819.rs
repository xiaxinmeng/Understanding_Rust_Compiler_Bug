sh
pkgver=1.16.0
_stage0dir="$srcdir/stage0"

# Don't set wrong LD_LIBRARY_PATH, we will rather set it manually when
# invoking make.
sed -i /LD_LIBRARY_PATH/d src/bootstrap/bootstrap.py

# Precompiled rustc and rust-std from VoidLinux.
cp -flr "$srcdir"/rustc-$pkgver-*/rustc/* \
        "$srcdir"/rust-std-$pkgver-*/rust-std-*/* \
        "$srcdir"/cargo-*-*/cargo/* \
        "$_stage0dir"/  

# Note: We use llvm 3.8.1 for now.
./configure \
        --build="$_ctarget" \
        --host="$_ctarget" \
        --prefix="/usr" \
        --release-channel="stable" \
        --enable-local-rust \
        --local-rust-root="$_stage0dir" \
        --llvm-root="/usr" \
        --musl-root="/usr" \
        --enable-rustbuild \
        --enable-vendor \
        --disable-docs \
        --disable-jemalloc \
        --disable-rpath

# Set LD_LIBRARY_PATH, so rustc in stage0 can find correct libs.
LD_LIBRARY_PATH="$_stage0dir/lib" \
        make RUST_BACKTRACE=1 RUST_CRT_STATIC="false" VERBOSE=1

LD_LIBRARY_PATH="$_stage0dir/lib" \
        make install DESTDIR="$pkgdir"
