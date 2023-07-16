
$ rustc --version
rustc 1.24.0-nightly (687d3d15b 2018-01-02)
$ cargo build --release
$ du -h ./target/release/mdbook
12M	./target/release/mdbook
$ strip ./target/release/mdbook 
$ du -h ./target/release/mdbook
6.3M	./target/release/mdbook
