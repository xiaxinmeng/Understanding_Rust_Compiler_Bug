plain
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 14.18µs

 finished in 8.878 seconds
test: basic_install
$ sh /checkout/src/tools/rust-installer/gen-installer.sh --image-dir=/checkout/src/tools/rust-installer/test/image1 --work-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rust-installer/workdir --output-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rust-installer/outdir
error: failed to load manifest for workspace member `/checkout/library/std`
Caused by:
  failed to parse manifest at `/checkout/library/std/Cargo.toml`

Caused by:
Caused by:
  the cargo feature `public-dependency` requires a nightly version of Cargo, but this is the `beta` channel
  See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
  See https://doc.rust-lang.org/beta/cargo/reference/unstable.html#public-dependency for more information about using this feature.
TEST FAILED!

Build completed unsuccessfully in 0:26:36
