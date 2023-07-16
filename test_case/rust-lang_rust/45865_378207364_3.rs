
$ rustc hello.rs -L $(pwd) -l static=hello -C lto -C linker=clang-4.0 -C link-args="-fuse-ld=gold -flto" -O
