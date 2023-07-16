 sh
# Ubuntu 16.04
# SRC_DIR is the rust-lang/rust source

main() {
    ARCH=armel
    BUILD=x86_64-unknown-linux-gnu
    HOST=arm-unknown-linux-gnueabi
    PREFIX=arm-linux-gnueabi

   sudo apt-get install -y --no-install-recommends \
        g++-$PREFIX libc6-dev-$ARCH-cross

   $SRC_DIR/configure \
       --enable-ccache \
       --enable-ninja \
       --enable-rustbuild \
       --host=$HOST

   # work around rust-lang/rust#38037
   nice $SRC_DIR/x.py build --stage 1 src/llvm --host=$BUILD

   # STDOUT
   # Building LLVM for x86_64-unknown-linux-gnu

   nice $SRC_DIR/x.py build --stage 1 src/rustc --host=$HOST

   # STDOUT
   # Building stage0 std artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabi)
   # Building stage0 test artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabi)
   # Building LLVM for arm-unknown-linux-gnueabi
   # Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabi)
   # Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   # Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   # Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   # Building stage1 std artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabi)
   # Building stage1 test artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabi)
   # Building stage1 compiler artifacts (x86_64-unknown-linux-gnu -> arm-unknown-linux-gnueabi)
}

main
