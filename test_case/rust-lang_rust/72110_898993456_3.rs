
$ cargo build --release
$ du target/release/hello
824	target/release/hello
$ strip target/release/hello
$ du target/release/hello
544	target/release/hello
