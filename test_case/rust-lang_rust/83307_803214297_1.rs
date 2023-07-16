shell
$ cd json5format
$ RUSTC=$HOME/rust/build/x86_64-unknown-linux-gnu/stage1/bin/rustc \
    RUSTFLAGS="-Zinstrument-coverage" \
    RUSTDOCFLAGS="-Zinstrument-coverage -Zunstable-options --persist-doctests target/debug/doctestbins" \
    LLVM_PROFILE_FILE="json5format-%m.profraw" \
    cargo test 
