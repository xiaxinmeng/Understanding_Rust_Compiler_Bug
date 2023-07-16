
$ time cargo build
...
   Compiling clippy_lints v0.0.212 (/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints)
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' in pwd='/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy' with args: '--edition=2018 --crate-name clippy_lints /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/lib.rs --color always --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=3d6e9f7e0458b600 -C extra-filename=-3d6e9f7e0458b600 --out-dir /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps -C incremental=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/incremental -L dependency=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps --extern cargo_metadata=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libcargo_metadata-8df71ebc3ec8c7b1.rlib --extern if_chain=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libif_chain-18472c918195eb10.rlib --extern itertools=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libitertools-63f847f4b07a7a94.rlib --extern lazy_static=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/liblazy_static-e134885038065a59.rlib --extern matches=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libmatches-ff9c7cc1d9e9c702.rlib --extern pulldown_cmark=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libpulldown_cmark-b5927e461cff9f13.rlib --extern quine_mc_cluskey=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libquine_mc_cluskey-c716d5d0c4b4bb66.rlib --extern regex_syntax=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libregex_syntax-af255f912840c0b8.rlib --extern semver=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libsemver-d2211cb1aad9a77c.rlib --extern serde=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libserde-31302d9e7cf738a8.rlib --extern serde_derive=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libserde_derive-46f8713235b67e51.so --extern smallvec=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libsmallvec-b10874f5e464b3e5.rlib --extern toml=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libtoml-619b7a67cda19680.rlib --extern unicode_normalization=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/libunicode_normalization-ae27b76e45d9c760.rlib --extern url=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/deps/liburl-899a520053e9b032.rlib -L native=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/target/debug/build/backtrace-sys-4034742239ad3cea/out'
error[E0277]: `syntax_pos::symbol::LocalInternedString` cannot be shared between threads safely
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/lib.rs:352:34
    |
352 |     reg.register_early_lint_pass(box enum_variants::EnumVariantNames::new(conf.enum_variant_name_threshold));
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `syntax_pos::symbol::LocalInternedString` cannot be shared between threads safely
    |
    = help: within `(syntax_pos::symbol::LocalInternedString, std::string::String)`, the trait `std::marker::Sync` is not implemented for `syntax_pos::symbol::LocalInternedString`
    = note: required because it appears within the type `(syntax_pos::symbol::LocalInternedString, std::string::String)`
    = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<(syntax_pos::symbol::LocalInternedString, std::string::String)>`
    = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax_pos::symbol::LocalInternedString, std::string::String)>`
    = note: required because it appears within the type `std::vec::Vec<(syntax_pos::symbol::LocalInternedString, std::string::String)>`
    = note: required because it appears within the type `enum_variants::EnumVariantNames`
    = note: required for the cast to the object type `dyn rustc::lint::EarlyLintPass + std::marker::Send + std::marker::Sync`

error[E0277]: `syntax_pos::symbol::LocalInternedString` cannot be sent between threads safely
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/lib.rs:352:34
    |
352 |     reg.register_early_lint_pass(box enum_variants::EnumVariantNames::new(conf.enum_variant_name_threshold));
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `syntax_pos::symbol::LocalInternedString` cannot be sent between threads safely
    |
    = help: within `(syntax_pos::symbol::LocalInternedString, std::string::String)`, the trait `std::marker::Send` is not implemented for `syntax_pos::symbol::LocalInternedString`
    = note: required because it appears within the type `(syntax_pos::symbol::LocalInternedString, std::string::String)`
    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(syntax_pos::symbol::LocalInternedString, std::string::String)>`
    = note: required because it appears within the type `alloc::raw_vec::RawVec<(syntax_pos::symbol::LocalInternedString, std::string::String)>`
    = note: required because it appears within the type `std::vec::Vec<(syntax_pos::symbol::LocalInternedString, std::string::String)>`
    = note: required because it appears within the type `enum_variants::EnumVariantNames`
    = note: required for the cast to the object type `dyn rustc::lint::EarlyLintPass + std::marker::Send + std::marker::Sync`

error[E0308]: mismatched types
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/consts.rs:159:63
    |
159 |         LitKind::ByteStr(ref s) => Constant::Binary(Rc::clone(s)),
    |                                                               ^ expected struct `std::rc::Rc`, found struct `std::sync::Arc`
    |
    = note: expected type `&std::rc::Rc<std::vec::Vec<u8>>`
               found type `&std::sync::Arc<std::vec::Vec<u8>>`

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
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/utils/mod.rs:226:50
    |
226 |             for item in mem::replace(&mut items, Rc::new(vec![])).iter() {
    |                                                  ^^^^^^^^^^^^^^^ expected struct `std::sync::Arc`, found struct `std::rc::Rc`
    |
    = note: expected type `std::sync::Arc<std::vec::Vec<rustc::hir::def::Export>>`
               found type `std::rc::Rc<std::vec::Vec<_>>`

error[E0599]: no method named `predicate_must_hold` found for type `rustc::infer::InferCtxt<'_, '_, '_>` in the current scope
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/utils/mod.rs:274:30
    |
274 |         .enter(|infcx| infcx.predicate_must_hold(&obligation))
    |                              ^^^^^^^^^^^^^^^^^^^
    |
    = help: did you mean `predicate_may_hold`?

error[E0599]: no method named `moves_by_default` found for type `&'tcx rustc::ty::TyS<'tcx>` in the current scope
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/utils/mod.rs:887:9
    |
887 |     !ty.moves_by_default(cx.tcx.global_tcx(), cx.param_env, DUMMY_SP)
    |         ^^^^^^^^^^^^^^^^

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

error[E0599]: no method named `moves_by_default` found for type `&rustc::ty::TyS<'_>` in the current scope
   --> /home/xftroxgpx/build/2nonpkgs/rust.stuff/rust-clippy/clippy_lints/src/question_mark.rs:116:17
    |
116 |         expr_ty.moves_by_default(cx.tcx, cx.param_env, expression.span)
    |                 ^^^^^^^^^^^^^^^^

error: aborting due to 10 previous errors

Some errors occurred: E0277, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
error: Could not compile `clippy_lints`.

To learn more, run the command again with --verbose.

real	2m55.131s
user	8m24.497s
sys	0m7.911s
