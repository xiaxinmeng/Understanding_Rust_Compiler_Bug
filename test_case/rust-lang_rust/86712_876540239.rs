shell
export RUSTFLAGS="-Ctarget-feature=+crt-static -Clinker=clang -Clink-arg=-fuse-ld=lld"
cargo bisect-rustc --start 2020-08-07 --end 2020-08-09 -- run
