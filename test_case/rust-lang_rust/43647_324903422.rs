
# echo 'fn main() {}' > x.rs
# ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc --target x86_64-unknown-linux-musl x.rs
# file x
x: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=45654470ff0026421c064536a65c073dbfceb840, not stripped
# ldd x
        not a dynamic executable
