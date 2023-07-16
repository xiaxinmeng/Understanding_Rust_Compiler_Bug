
export RUST_LOG=cargo::ops::cargo_rustc::fingerprint
./x.py build src/libtest -v
./x.py test src/test/compile-fail -v
