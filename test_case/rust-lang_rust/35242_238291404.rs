
./configure --target=x86_64-unknown-linux-uclibc --enable-rustbuild
make
echo 'fn main() {}' > foo.rs
./build/x86_64-unknown-linux-gnu/stage2/bin/rustc foo.rs --target x86_64-unknown-linux-uclibc
