plain
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 485 tests
i.F.FF....................................F......F.FiF....F...........FF........F...........F....... 100/485
F...F...F.F..........F.F.........F........F.............FF..F.....................................F. 200/485
...........................F.F.FF......F..F.....FF................F................................. 300/485
.......................i......F.F..............F.F......................................F.i........F 400/485
............F.......................................FFFF.FF.................F....FF..

---- [rustdoc] rustdoc/associated-consts.rs stdout ----

error: rustdoc failed!
error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/associated-consts/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/associated-consts" "/checkout/src/test/rustdoc/associated-consts.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:5 ~ foo[cf51]::Trait::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Trait::foo`
error: aborting due to previous error


------------------------------------------
---

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:8 ~ foo[cf51]::AsExpression::as_expression)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `AsExpression::as_expression`
error: aborting due to previous error


------------------------------------------
---

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ assoc_types[f68f]::Index::index)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Index::index`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/cross-crate-links.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links/auxiliary/all-item-types/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/cross-crate-links" "/checkout/src/test/rustdoc/auxiliary/all-item-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:5 ~ all_item_types[eb84]::::foo_ffn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `foo_ffn`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/default-trait-method-link.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method-link/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method-link" "/checkout/src/test/rustdoc/default-trait-method-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ foo[cf51]::Foo::req)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Foo::req`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/default-trait-method.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/default-trait-method" "/checkout/src/test/rustdoc/default-trait-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
 --> /checkout/src/test/rustdoc/default-trait-method.rs:1:12
  |
1 | #![feature(specialization)]
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
  = help: consider using `min_specialization` instead, which is more stable and complete
  = help: consider using `min_specialization` instead, which is more stable and complete

error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ default_trait_method[79ed]::Item::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Item::foo`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/deprecated-impls.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/deprecated-impls" "/checkout/src/test/rustdoc/deprecated-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:9 ~ foo[cf51]::Bar::fn_empty_with_doc)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Bar::fn_empty_with_doc`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/doc-assoc-item.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-assoc-item/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-assoc-item" "/checkout/src/test/rustdoc/doc-assoc-item.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:8 ~ doc_assoc_item[6eb6]::Bar::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Bar::foo`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/doc-cfg-simplification.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification" "/checkout/src/test/rustdoc/doc-cfg-simplification.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:46 ~ globuliferous[3676]::ratel::Aposiopesis::meseems)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `ratel::Aposiopesis::meseems`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/doc-cfg.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this attribute can only be applied to a `use` item
   --> /checkout/src/test/rustdoc/doc-cfg.rs:99:7
    |
99  | #[doc(inline, cfg(x))]
    |       ^^^^^^ only applicable on `use` items
100 | #[doc(cfg(y), cfg(z))]
101 | pub fn multiple_attrs() {}
    | ----------------------- not a `use` item
    = note: `#[warn(invalid_doc_attributes)]` on by default
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
    = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
    = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#inline-and-no_inline for more information

error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:14 ~ doc_cfg[437f]::wasi_only::Wasm32Only::wasi_and_wasm32_only_function)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `wasi_only::Wasm32Only::wasi_and_wasm32_only_function`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/empty-impls.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-impls/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/empty-impls" "/checkout/src/test/rustdoc/empty-impls.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:8 ~ foo[cf51]::NotEmpty::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `NotEmpty::foo`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/extern-method.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/extern-method" "/checkout/src/test/rustdoc/extern-method.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:8 ~ extern_method[ed5f]::Bar::bar)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Bar::bar`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/ffi.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/ffi" "/checkout/src/test/rustdoc/ffi.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:8 ~ ffi[b9eb]::::another)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `another`
error: aborting due to previous error


------------------------------------------
---

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:11 ~ foreigntype_reexport[e7f8]::sub2::::f)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `sub2::f`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/hidden-trait-methods-with-document-hidden-items.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-methods-with-document-hidden-items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-methods-with-document-hidden-items" "/checkout/src/test/rustdoc/hidden-trait-methods-with-document-hidden-items.rs" "-Z" "unstable-options" "--document-hidden-items"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ foo[cf51]::Trait::f)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: Unrecognized option: 'document-hidden-items'
---
---- [rustdoc] rustdoc/hidden-trait-methods.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-methods/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/hidden-trait-methods" "/checkout/src/test/rustdoc/hidden-trait-methods.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:7 ~ foo[cf51]::Trait::g)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Trait::g`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/index-page.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/index-page/auxiliary/all-item-types/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/index-page" "/checkout/src/test/rustdoc/auxiliary/all-item-types.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:5 ~ all_item_types[eb84]::::foo_ffn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `foo_ffn`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/inline_cross/assoc-items.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items/auxiliary/assoc-items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_cross/assoc-items" "/checkout/src/test/rustdoc/inline_cross/auxiliary/assoc-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:15 ~ assoc_items[e5b9]::MyTrait::method_no_default)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `MyTrait::method_no_default`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/inline_local/glob-extern-no-defaults.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern-no-defaults/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern-no-defaults" "/checkout/src/test/rustdoc/inline_local/glob-extern-no-defaults.rs" "--no-defaults"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the `no-defaults` flag is deprecated
  = note: see issue #44136 <https://github.com/rust-lang/rust/issues/44136> for more information
  = help: you may want to use --document-private-items


error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ foo[cf51]::mod1::::private_fn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: Unrecognized option: 'no-defaults'
---
---- [rustdoc] rustdoc/inline_local/glob-extern.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/inline_local/glob-extern" "/checkout/src/test/rustdoc/inline_local/glob-extern.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:5 ~ foo[cf51]::mod1::::public_fn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `mod1::public_fn`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/associated-items.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/associated-items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/associated-items" "/checkout/src/test/rustdoc/intra-doc/associated-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `intra_doc_link_resolution_failure` has been renamed to `rustdoc::broken_intra_doc_links`
  |
1 | #![deny(intra_doc_link_resolution_failure)]
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use the new name: `rustdoc::broken_intra_doc_links`
  |
  |
  = note: `#[warn(renamed_and_removed_lints)]` on by default

error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:17 ~ associated_items[312d]::T2::ambiguous_method)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `T2::ambiguous_method`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/basic.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/basic/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/basic" "/checkout/src/test/rustdoc/intra-doc/basic.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:17 ~ basic[c0d8]::ThisTrait::this_associated_method)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `ThisTrait::this_associated_method`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/cross-crate/traits.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/traits/auxiliary/traits/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/cross-crate/traits" "/checkout/src/test/rustdoc/intra-doc/cross-crate/auxiliary/traits.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ inner[73f7]::SomeTrait::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `SomeTrait::foo`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/intra-doc/trait-impl.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/trait-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/trait-impl" "/checkout/src/test/rustdoc/intra-doc/trait-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:12 ~ foo[cf51]::MyTrait::trait_fn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `MyTrait::trait_fn`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-20727-2.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-2" "/checkout/src/test/rustdoc/issue-20727-2.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:7 ~ issue_20727_2[c8c2]::Add::add)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Add::add`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-20727-3.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-3/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-3" "/checkout/src/test/rustdoc/issue-20727-3.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:7 ~ issue_20727_3[f52a]::Deref2::deref)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Deref2::deref`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-20727-4.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727-4" "/checkout/src/test/rustdoc/issue-20727-4.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:10 ~ issue_20727_4[71b1]::IndexMut::index_mut)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `IndexMut::index_mut`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-20727.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-20727" "/checkout/src/test/rustdoc/issue-20727.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ issue_20727[ccfd]::Deref::deref)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Deref::deref`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-22038.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-22038/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-22038" "/checkout/src/test/rustdoc/issue-22038.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ issue_22038[5cb0]::#1::foo2)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `foo2`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-25001.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25001" "/checkout/src/test/rustdoc/issue-25001.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:9 ~ issue_25001[13b8]::Bar::quux)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Bar::quux`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-28478.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28478/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-28478" "/checkout/src/test/rustdoc/issue-28478.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ issue_28478[ed2f]::Bar::bar)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Bar::bar`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-29503.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-29503" "/checkout/src/test/rustdoc/issue-29503.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:7 ~ issue_29503[39f0]::MyTrait::my_string)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `MyTrait::my_string`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-34274.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34274/auxiliary/issue-34274/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-34274" "/checkout/src/test/rustdoc/auxiliary/issue-34274.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ issue_34274[d21a]::::extern_c_fn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `extern_c_fn`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-85454.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-85454/auxiliary/issue-85454/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-85454" "/checkout/src/test/rustdoc/auxiliary/issue-85454.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:9 ~ issue_85454[386a]::Try::from_output)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Try::from_output`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/issue-89309-heading-levels.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-89309-heading-levels/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-89309-heading-levels" "/checkout/src/test/rustdoc/issue-89309-heading-levels.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ foo[cf51]::Read::read)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Read::read`
error: aborting due to previous error


---

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:7 ~ foo[cf51]::foo::bar::Foo::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `foo::bar::Foo::foo`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/sidebar-items.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-items/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/sidebar-items" "/checkout/src/test/rustdoc/sidebar-items.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: variant `foo` should have an upper camel case name
   |
36 |     foo,
36 |     foo,
   |     ^^^ help: convert the identifier to upper camel case (notice the capitalization): `Foo`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: variant `bar` should have an upper camel case name
   |
37 |     bar,
37 |     bar,
   |     ^^^ help: convert the identifier to upper camel case: `Bar`

error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:7 ~ foo[cf51]::Foo::bar)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Foo::bar`
error: aborting due to previous error; 2 warnings emitted


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/titles.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/titles/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/titles" "/checkout/src/test/rustdoc/titles.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `doc(primitive)` should never have been stable
   |
   |
37 | #[doc(primitive = "bool")]
   |
   = note: `#[warn(invalid_doc_attributes)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>

error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ foo[cf51]::::foo_ffn)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `foo_ffn`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/toggle-trait-fn.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/toggle-trait-fn/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/toggle-trait-fn" "/checkout/src/test/rustdoc/toggle-trait-fn.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ foo[cf51]::Foo::not_documented)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Foo::not_documented`
error: aborting due to previous error


------------------------------------------
---

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:142 ~ toggle_item_contents[e628]::BigTrait::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `BigTrait::foo`
error: aborting due to previous error


------------------------------------------
---

------------------------------------------
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:6 ~ trait_impl_items_links_and_anchors[1ae2]::MyTrait::trait_function)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `MyTrait::trait_function`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/trait-impl.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-impl/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-impl" "/checkout/src/test/rustdoc/trait-impl.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ trait_impl[d1de]::Trait::a)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Trait::a`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/trait-src-link.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-src-link/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/trait-src-link" "/checkout/src/test/rustdoc/trait-src-link.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ quix[ea01]::Foo::required)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Foo::required`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/variadic.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/variadic/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/variadic" "/checkout/src/test/rustdoc/variadic.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:4 ~ variadic[b0aa]::::foo)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `foo`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [rustdoc] rustdoc/visibility.rs stdout ----

error: rustdoc failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/visibility/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/visibility" "/checkout/src/test/rustdoc/visibility.rs" "--document-private-items"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:36 ~ foo[cf51]::PubTrait::function)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: Unrecognized option: 'document-private-items'
---
---- [rustdoc] rustdoc/where.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/where" "/checkout/src/test/rustdoc/where.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_ty_utils/src/ty.rs:493:9: asyncness: expected fn-like node but got `DefId(0:11 ~ foo[cf51]::Bravo::get)`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (774a43360 2021-11-25) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [asyncness] checking if the function is async: `Bravo::get`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 432 passed; 49 failed; 4 ignored; 0 measured; 0 filtered out; finished in 54.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:16:59
