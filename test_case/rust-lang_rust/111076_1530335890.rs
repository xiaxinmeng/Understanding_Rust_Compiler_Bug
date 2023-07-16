plain
test: multiple_components
test: uninstall_from_installed_script
test: uninstall_from_installed_script_with_args_fails
test: combine_installers
$ sh /checkout/src/tools/rust-installer/combine-installers.sh --work-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rust-installer/workdir --output-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rust-installer/outdir --package-name=rust --input-tarballs=/checkout/obj/build/x86_64-unknown-linux-gnu/test/rust-installer/outdir/rustc.tar.gz,/checkout/obj/build/x86_64-unknown-linux-gnu/test/rust-installer/outdir/cargo.tar.gz
error: failed to load manifest for workspace member `/checkout/library/std`
Caused by:
  failed to parse manifest at `/checkout/library/std/Cargo.toml`

Caused by:
Caused by:
  the cargo feature `public-dependency` requires a nightly version of Cargo, but this is the `beta` channel
  See https://doc.rust-lang.org/book/appendix-07-nightly-rust.html for more information about Rust release channels.
  See https://doc.rust-lang.org/beta/cargo/reference/unstable.html#public-dependency for more information about using this feature.
TEST FAILED!

Build completed unsuccessfully in 0:30:47
