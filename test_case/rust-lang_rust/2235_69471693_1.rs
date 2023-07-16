
rustc --target i686-apple-darwin -C lto --emit ir foo.rust
emcc -v foo.ll -o test.html
