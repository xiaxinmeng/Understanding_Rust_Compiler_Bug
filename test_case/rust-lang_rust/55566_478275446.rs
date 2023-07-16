
$ wget -nv -P /tmp https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-musl.tar.gz
2019-03-30 12:30:11 URL:https://static.rust-lang.org/dist/rust-nightly-x86_64-unknown-linux-musl.tar.gz [207388273/207388273] -> "/tmp/rust-nightly-x86_64-unknown-linux-musl.tar.gz" [1]
$ tar xf /tmp/rust-nightly-x86_64-unknown-linux-musl.tar.gz -C /tmp
$ /tmp/rust-nightly-x86_64-unknown-linux-musl/install.sh --prefix=/tmp/rust-nightly
install: creating uninstall script at /tmp/rust-nightly/lib/rustlib/uninstall.sh
install: installing component 'rustc'
install: installing component 'cargo'
install: installing component 'rustfmt-preview'
install: installing component 'llvm-tools-preview'
install: installing component 'rust-analysis-x86_64-unknown-linux-musl'
install: installing component 'rust-std-x86_64-unknown-linux-musl'

    Rust is ready to roll.

$ PATH=/tmp/rust-nightly/bin:$PATH
$ cargo -vV
cargo 1.35.0-nightly (0e35bd8af 2019-03-13)
release: 1.35.0
commit-hash: 0e35bd8af0ec72d3225c4819b330b94628f0e9d0
commit-date: 2019-03-13
$ rustc -vV
rustc 1.35.0-nightly (e782d790f 2019-03-29)
binary: rustc
commit-hash: e782d790f1b63d82af39248bebe027f92d891bcc
commit-date: 2019-03-29
host: x86_64-unknown-linux-musl
release: 1.35.0-nightly
LLVM version: 8.0
$ cargo new --bin linkage-bug
     Created binary (application) `linkage-bug` package
$ cd linkage-bug
$ echo 'curl = "0.4.20"' >> Cargo.toml
$ cat > src/main.rs << 'EOF'
> extern crate curl;
>
> use std::io::{stdout, Write};
>
> use curl::easy::Easy;
>
> // Print a web page onto stdout
> fn main() {
>     let mut easy = Easy::new();
>     easy.url("https://www.rust-lang.org/").unwrap();
>     easy.write_function(|data| {
>         stdout().write_all(data).unwrap();
>         Ok(data.len())
>     }).unwrap();
>     easy.perform().unwrap();
>
>     println!("{}", easy.response_code().unwrap());
> }
> EOF
$ cargo run
    Updating crates.io index
   Compiling semver-parser v0.7.0
   Compiling pkg-config v0.3.14
   Compiling libc v0.2.51
   Compiling cc v1.0.32
   Compiling cfg-if v0.1.7
   Compiling openssl-probe v0.1.2
   Compiling semver v0.9.0
   Compiling libz-sys v1.0.25
   Compiling curl-sys v0.4.17
   Compiling rustc_version v0.2.3
   Compiling openssl-sys v0.9.43
   Compiling socket2 v0.3.8
   Compiling curl v0.4.20
   Compiling linkage-bug v0.1.0 (/tmp/foo/linkage-bug)
    Finished dev [unoptimized + debuginfo] target(s) in 10.66s
     Running `target/debug/linkage-bug`
error: could not execute process `target/debug/linkage-bug` (never executed)

Caused by:
  No such file or directory (os error 2)
$ file target/debug/linkage-bug
target/debug/linkage-bug: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld64.so.1, with debug_info, not stripped
