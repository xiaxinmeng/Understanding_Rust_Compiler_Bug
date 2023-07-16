
$ cargo clean
$ git checkout 194a91c45d5cbeadeb16afd75ce451753b230b81
$ time cargo build --release
...
   Compiling clippy_lints v0.0.212 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints)
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy' with args: '--edition=2018 --crate-name clippy_lints /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=c6858112a7aa0015 -C extra-filename=-c6858112a7aa0015 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps --extern cargo_metadata=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libcargo_metadata-d9f69ee9ecf3f2bc.rlib --extern if_chain=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libif_chain-034c1fe5e35ac2aa.rlib --extern itertools=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libitertools-465a56980f87d580.rlib --extern lazy_static=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/liblazy_static-bd61bca7ded02344.rlib --extern matches=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libmatches-ed88046a7241fda0.rlib --extern pulldown_cmark=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libpulldown_cmark-a2098d7fea542802.rlib --extern quine_mc_cluskey=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libquine_mc_cluskey-27879b8929c1d298.rlib --extern regex_syntax=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libregex_syntax-7939a1c60445dea8.rlib --extern semver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libsemver-644f304805e46813.rlib --extern serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde-32cfc6411fa73157.rlib --extern serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-94528bc15b1433de.so --extern smallvec=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libsmallvec-7611e02ec03f1d18.rlib --extern toml=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libtoml-5220d440fc5fe620.rlib --extern unicode_normalization=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libunicode_normalization-5354167e2fc18963.rlib --extern url=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/liburl-9a42211ad038b251.rlib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/build/backtrace-sys-49c9d8a23ceb873e/out'
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

real	5m8.794s
user	15m45.628s
sys	0m11.081s

$ rustc -vV
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust' with args: '-vV'
rustc 1.33.0-dev (9d5481282 2019-01-08)
binary: rustc
commit-hash: 9d54812829e9d92dac35a4a0f358cdc5a2475371
commit-date: 2019-01-08
host: x86_64-unknown-linux-gnu
release: 1.33.0-dev
LLVM version: 8.0

$ cargo -vV
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/cargo/cargo//target/release//cargo' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust' with args: '-vV'
cargo 1.33.0-dev (a62e36b4 2019-01-06)
release: 1.33.0
commit-hash: a62e36b4417cbf872960eda156130da932fb765d
commit-date: 2019-01-06


$ time cargo test --release
   Compiling clippy_lints v0.0.212 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints)
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy' with args: '--edition=2018 --crate-name clippy_lints /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=c6858112a7aa0015 -C extra-filename=-c6858112a7aa0015 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps --extern cargo_metadata=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libcargo_metadata-d9f69ee9ecf3f2bc.rlib --extern if_chain=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libif_chain-034c1fe5e35ac2aa.rlib --extern itertools=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libitertools-465a56980f87d580.rlib --extern lazy_static=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/liblazy_static-bd61bca7ded02344.rlib --extern matches=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libmatches-ed88046a7241fda0.rlib --extern pulldown_cmark=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libpulldown_cmark-a2098d7fea542802.rlib --extern quine_mc_cluskey=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libquine_mc_cluskey-27879b8929c1d298.rlib --extern regex_syntax=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libregex_syntax-7939a1c60445dea8.rlib --extern semver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libsemver-644f304805e46813.rlib --extern serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde-32cfc6411fa73157.rlib --extern serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libserde_derive-94528bc15b1433de.so --extern smallvec=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libsmallvec-7611e02ec03f1d18.rlib --extern toml=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libtoml-5220d440fc5fe620.rlib --extern unicode_normalization=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/libunicode_normalization-5354167e2fc18963.rlib --extern url=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/deps/liburl-9a42211ad038b251.rlib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/release/build/backtrace-sys-49c9d8a23ceb873e/out'
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
warning: build failed, waiting for other jobs to finish...
error: build failed

real	1m7.818s
user	3m56.892s
sys	0m2.909s

