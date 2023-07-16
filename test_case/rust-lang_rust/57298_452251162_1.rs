
   Compiling clippy_lints v0.0.212 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints)
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy' with args: '--edition=2018 --crate-name clippy_lints /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=bb00f581a588ec59 -C extra-filename=-bb00f581a588ec59 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps --extern cargo_metadata=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libcargo_metadata-507650c169410b97.rlib --extern if_chain=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libif_chain-70f92e7b45d67c40.rlib --extern itertools=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libitertools-e0e54779e0fe999d.rlib --extern lazy_static=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/liblazy_static-d8407dbd198e2382.rlib --extern matches=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libmatches-a26e32e0d355a8c6.rlib --extern pulldown_cmark=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libpulldown_cmark-8750f6e476612de3.rlib --extern quine_mc_cluskey=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libquine_mc_cluskey-f8d7f569c9699a06.rlib --extern regex_syntax=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libregex_syntax-0874f0ec2e4a1b15.rlib --extern semver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libsemver-548d87d186ca3880.rlib --extern serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde-f7eea94ee0e33ecb.rlib --extern serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-3c0db970ced9d85e.so --extern smallvec=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libsmallvec-9d8252e171534702.rlib --extern toml=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libtoml-311012f61c2d596c.rlib --extern unicode_normalization=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libunicode_normalization-44f9750f35c35a9e.rlib --extern url=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/liburl-390e734d0e3de798.rlib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/build/backtrace-sys-1a8a5e1d01182edf/out'
error[E0308]: mismatched types
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/consts.rs:307:51
    |
307 |                 let ret = miri_to_const(self.tcx, result);
    |                                                   ^^^^^^
    |                                                   |
    |                                                   expected reference, found struct `rustc::ty::Const`
    |                                                   help: consider borrowing here: `&result`
    |
    = note: expected type `&rustc::ty::Const<'_>`
               found type `rustc::ty::Const<'_>`

error[E0308]: mismatched types
  --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/enum_clike.rs:73:99
   |
73 |                     if let Some(Constant::Int(val)) = constant.and_then(|c| miri_to_const(cx.tcx, c)) {
   |                                                                -                                  ^ expected reference, found struct `rustc::ty::Const`
   |                                                                |
   |                                                                help: consider using `as_ref` instead: `as_ref().`
   |
   = note: expected type `&rustc::ty::Const<'_>`
              found type `rustc::ty::Const<'_>`

error[E0599]: no method named `def_id` found for type `std::option::Option<rustc::ty::Binder<rustc::ty::ExistentialTraitRef<'_>>>` in the current scope
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/len_zero.rs:307:46
    |
307 |             .associated_items(tt.principal().def_id())
    |                                              ^^^^^^

error: aborting due to 3 previous errors

Some errors occurred: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: Could not compile `clippy_lints`.

To learn more, run the command again with --verbose.
