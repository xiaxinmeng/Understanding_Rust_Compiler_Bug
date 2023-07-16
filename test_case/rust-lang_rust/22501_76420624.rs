 sh
rm -rf test-pretty-dump; mkdir test-pretty-dump
# compile stage1 rustc first, with no flags, to avoid errors as stage0 doesn't have them
make x86_64-unknown-linux-gnu/stage1/bin/rustc
make check-stage1 RUSTFLAGS="-Z unstable-options -Z pretty-keep-going -Z pretty-dump-dir=test-pretty-dump --xpretty=typed,unsuffixed_literals"
