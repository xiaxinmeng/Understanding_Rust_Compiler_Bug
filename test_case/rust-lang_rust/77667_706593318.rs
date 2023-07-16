
set -x

rm -r weird
cp -r build/x86_64-unknown-linux-gnu/stage1 weird
cp -r  ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-pc-windows-msvc weird/lib/rustlib
