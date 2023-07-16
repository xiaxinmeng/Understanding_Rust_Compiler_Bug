
// build just the rustdoc binary, useful for local experiments
make x86_64-unknown-linux-gnu/stage1/bin/rustdoc
// run the src/test/rustdoc tests
make check-stage1-rustdocck
