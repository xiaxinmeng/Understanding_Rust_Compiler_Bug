
$ time cargo build
...
   Compiling clippy v0.0.212 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy)
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy' with args: '--edition=2018 --crate-name build_script_build /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/build.rs --color always --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=01aa6e327713cacf -C extra-filename=-01aa6e327713cacf --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/build/clippy-01aa6e327713cacf -C incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/incremental -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps --extern rustc_tools_util=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/librustc_tools_util-356afa5dd41fee1a.rlib'
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

real	0m26.782s
user	0m29.112s
sys	0m0.460s
