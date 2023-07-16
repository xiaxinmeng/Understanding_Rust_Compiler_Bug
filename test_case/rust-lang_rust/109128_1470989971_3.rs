console
export DYLD_LIBRARY_PATH="$(rustc +dev --print sysroot)/lib"
cp .//build/aarch64-apple-darwin/stage1-std/aarch64-apple-darwin/release/deps/libstd-8c92ab8e515128e4.dylib /Users/yukang/rust/build/aarch64-apple-darwin/stage1/lib
