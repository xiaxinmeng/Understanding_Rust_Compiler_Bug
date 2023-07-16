plain
[01:20:07] test [pretty] run-pass/associated-types/associated-types-where-clause-impl-ambiguity.rs ... FAILED
[01:20:07] test [pretty] run-pass/associated-item-long-paths.rs ... ok
[01:20:07] test [pretty] run-pass/assoc-oddities-3.rs ... ok
[01:20:07] test [pretty] run-pass/associated-types/associated-types-from-supertrait.rs ... ok
[01:20:08] ERROR 2019-06-25T06:44:15Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![feature(cfg_target_has_atomic)]\n#![feature(integer_atomics)]\n\nuse std::mem::{align_of, size_of};\nuse std::sync::atomic::*;\n\nfn main() {\n\n    #[cfg(target_has_atomic = \"8\")]\n    assert_eq!(align_of :: < AtomicBool > (  ) , size_of :: < AtomicBool > (\n               ));\n\n    #[cfg(target_has_atomic = \"ptr\")]\n    assert_eq!(align_of :: < AtomicPtr < u8 >> (  ) , size_of :: < AtomicPtr <\n               u8 >> (  ));\n\n    #[cfg(target_has_atomic = \"8\")]\n    assert_eq!(align_of :: < AtomicU8 > (  ) , size_of :: < AtomicU8 > (  ));\n\n    #[cfg(target_has_atomic = \"8\")]\n    assert_eq!(align_of :: < AtomicI8 > (  ) , size_of :: < AtomicI8 > (  ));\n\n    #[cfg(target_has_atomic = \"16\")]\n    assert_eq!(align_of :: < AtomicU16 > (  ) , size_of :: < AtomicU16 > (\n               ));\n\n    #[cfg(target_has_atomic = \"16\")]\n    assert_eq!(align_of :: < AtomicI16 > (  ) , size_of :: < AtomicI16 > (\n               ));\n\n    #[cfg(target_has_atomic = \"32\")]\n    assert_eq!(align_of :: < AtomicU32 > (  ) , size_of :: < AtomicU32 > (\n               ));\n\n    #[cfg(target_has_atomic = \"32\")]\n    assert_eq!(align_of :: < AtomicI32 > (  ) , size_of :: < AtomicI32 > (\n               ));\n\n    #[cfg(target_has_atomic = \"64\")]\n    assert_eq!(align_of :: < AtomicU64 > (  ) , size_of :: < AtomicU64 > (\n               ));\n\n    #[cfg(target_has_atomic = \"64\")]\n    assert_eq!(align_of :: < AtomicI64 > (  ) , size_of :: < AtomicI64 > (\n               ));\n\n    #[cfg(target_has_atomic = \"128\")]\n    assert_eq!(align_of :: < AtomicU128 > (  ) , size_of :: < AtomicU128 > (\n               ));\n\n    #[cfg(target_has_atomic = \"128\")]\n    assert_eq!(align_of :: < AtomicI128 > (  ) , size_of :: < AtomicI128 > (\n               ));\n\n    #[cfg(target_has_atomic = \"ptr\")]\n    assert_eq!(align_of :: < AtomicUsize > (  ) , size_of :: < AtomicUsize > (\n                ));\n\n    #[cfg(target_has_atomic = \"ptr\")]\n    assert_eq!(align_of :: < AtomicIsize > (  ) , size_of :: < AtomicIsize > (\n                ));\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n#![feature(cfg_target_has_atomic)]\n#![feature(integer_atomics)]\n\nuse std::mem::{align_of, size_of};\nuse std::sync::atomic::*;\n\nfn main() {\n\n    #[cfg(target_has_atomic = \"8\")]\n    assert_eq!(align_of :: < AtomicBool > (  ) , size_of :: < AtomicBool > (\n               ));\n    #[cfg(target_has_atomic = \"ptr\")]\n    assert_eq!(align_of :: < AtomicPtr < u8 >> (  ) , size_of :: < AtomicPtr <\n               u8 >> (  ));\n\n    #[cfg(target_has_atomic = \"8\")]\n    assert_eq!(align_of :: < AtomicU8 > (  ) , size_of :: < AtomicU8 > (  ));\n\n    #[cfg(target_has_atomic = \"8\")]\n    assert_eq!(align_of :: < AtomicI8 > (  ) , size_of :: < AtomicI8 > (  ));\n\n    #[cfg(target_has_atomic = \"16\")]\n    assert_eq!(align_of :: < AtomicU16 > (  ) , size_of :: < AtomicU16 > (\n               ));\n\n    #[cfg(target_has_atomic = \"16\")]\n    assert_eq!(align_of :: < AtomicI16 > (  ) , size_of :: < AtomicI16 > (\n               ));\n\n    #[cfg(target_has_atomic = \"32\")]\n    assert_eq!(align_of :: < AtomicU32 > (  ) , size_of :: < AtomicU32 > (\n               ));\n\n    #[cfg(target_has_atomic = \"32\")]\n    assert_eq!(align_of :: < AtomicI32 > (  ) , size_of :: < AtomicI32 > (\n               ));\n\n    #[cfg(target_has_atomic = \"64\")]\n    assert_eq!(align_of :: < AtomicU64 > (  ) , size_of :: < AtomicU64 > (\n               ));\n\n    #[cfg(target_has_atomic = \"64\")]\n    assert_eq!(align_of :: < AtomicI64 > (  ) , size_of :: < AtomicI64 > (\n               ));\n\n    #[cfg(target_has_atomic = \"128\")]\n    assert_eq!(align_of :: < AtomicU128 > (  ) , size_of :: < AtomicU128 > (\n               ));\n    #[cfg(target_has_atomic = \"128\")]\n    assert_eq!(align_of :: < AtomicI128 > (  ) , size_of :: < AtomicI128 > (\n               ));\n    #[cfg(target_has_atomic = \"ptr\")]\n    assert_eq!(align_of :: < AtomicUsize > (  ) , size_of :: < AtomicUsize > (\n                ));\n\n    #[cfg(target_has_atomic = \"ptr\")]\n    assert_eq!(align_of :: < AtomicIsize > (  ) , size_of :: < AtomicIsize > (\n                ));\n}\n\n------------------------------------------\n\n"
[01:20:08] test [pretty] run-pass/atomic-compare_exchange.rs ... ok
[01:20:08] test [pretty] run-pass/atomic-access-bool.rs ... ok
[01:20:08] test [pretty] run-pass/attr-main-2.rs ... ok
[01:20:08] test [pretty] run-pass/atomic-print.rs ... ok
---
[01:20:16] test [pretty] run-pass/dynamically-sized-types/dst-tuple-sole.rs ... FAILED
[01:20:16] test [pretty] run-pass/dynamically-sized-types/dst-tuple.rs ... FAILED
[01:20:16] test [pretty] run-pass/diverging-fallback-option.rs ... ok
[01:20:16] test [pretty] run-pass/double-ref.rs ... ok
[01:20:16] ERROR 2019-06-25T06:44:24Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![allow(unused_mut)]\n#![allow(unused_assignments)]\n#![allow(unused_variables)]\n// edition:2015\n// aux-build:edition-kw-macro-2015.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2015;\n\npub fn check_async() {\n    let mut async = 1; // OK\n    let mut async = 1; // OK\n\n    async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n    async = consumes_async_raw!(r#async); // OK\n\n    if passes_ident!(async) == 1 { } // OK\n    if passes_ident!(r#async) == 1 { } // OK\n    one_async::async(); // OK\n    one_async::async(); // OK\n    two_async::async(); // OK\n    two_async::async(); // OK\n}\n\nmod one_async {\n    produces_async!{ } // OK\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![allow(unused_mut)]\n#![allow(unused_assignments)]\n#![allow(unused_variables)]\n// edition:2015\n// aux-build:edition-kw-macro-2015.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2015;\n\npub fn check_async() {\n    let mut async = 1; // OK\n    let mut async = 1; // OK\n\n    async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n\n    async = consumes_async_raw!(r#async); // OK\n\n    if passes_ident!(async) == 1 { } // OK\n    if passes_ident!(r#async) == 1 { } // OK\n    one_async::async(); // OK\n    one_async::async(); // OK\n    two_async::async(); // OK\n    two_async::async(); // OK\n}\n\nmod one_async {\n    produces_async!{ } // OK\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\n\n"
[01:20:17] test [pretty] run-pass/early-ret-binop-add.rs ... ok
[01:20:17] test [pretty] run-pass/early-vtbl-resolution.rs ... ok
[01:20:17] test [pretty] run-pass/early-vtbl-resolution.rs ... ok
[01:20:17] ERROR 2019-06-25T06:44:24Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![allow(unused_assignments)]\n// edition:2018\n// aux-build:edition-kw-macro-2015.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2015;\n\npub fn check_async() {\n    // let mut async = 1; // ERROR, reserved\n    let mut r#async = 1; // OK\n\n    r#async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n    r#async = consumes_async_raw!(r#async); // OK\n\n    // if passes_ident!(async) == 1 {} // ERROR, reserved\n    if passes_ident!(r#async) == 1 {\n    } // OK\n      // one_async::async(); // ERROR, reserved\n\n    one_async::r#async(); // OK\n                          // two_async::async(); // ERROR, reserved\n\n    two_async::r#async(); // OK\n}\n\nmod one_async {\n    produces_async!{ } // OK\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![allow(unused_assignments)]\n// edition:2018\n// aux-build:edition-kw-macro-2015.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2015;\n\npub fn check_async() {\n    // let mut async = 1; // ERROR, reserved\n    let mut r#async = 1; // OK\n\n    r#async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n\n    r#async = consumes_async_raw!(r#async); // OK\n\n    // if passes_ident!(async) == 1 {} // ERROR, reserved\n    if passes_ident!(r#async) == 1 {\n    } // OK\n      // one_async::async(); // ERROR, reserved\n\n\n    one_async::r#async(); // OK\n                          // two_async::async(); // ERROR, reserved\n\n\n    two_async::r#async(); // OK\n}\n\nmod one_async {\n    produces_async!{ } // OK\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\n\n"
[01:20:17] test [pretty] run-pass/edition-keywords-2018-2015.rs ... FAILED
[01:20:17] ERROR 2019-06-25T06:44:24Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![allow(unused_mut)]\n#![allow(unused_assignments)]\n#![allow(unused_variables)]\n// edition:2015\n// aux-build:edition-kw-macro-2018.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2018;\n\npub fn check_async() {\n    let mut async = 1; // OK\n    let mut async = 1; // OK\n\n    async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n    async = consumes_async_raw!(r#async); // OK\n\n    if passes_ident!(async) == 1 { } // OK\n    if passes_ident!(r#async) == 1 {\n    } // OK\n      // one_async::async(); // ERROR, unresolved name\n      // one_async::r#async(); // ERROR, unresolved name\n\n    two_async::async(); // OK\n    two_async::async(); // OK\n}\n\nmod one_async {\n    // produces_async! {} // ERROR, reserved\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![allow(unused_mut)]\n#![allow(unused_assignments)]\n#![allow(unused_variables)]\n// edition:2015\n// aux-build:edition-kw-macro-2018.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2018;\n\npub fn check_async() {\n    let mut async = 1; // OK\n    let mut async = 1; // OK\n\n    async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n\n    async = consumes_async_raw!(r#async); // OK\n\n    if passes_ident!(async) == 1 { } // OK\n    if passes_ident!(r#async) == 1 {\n    } // OK\n      // one_async::async(); // ERROR, unresolved name\n      // one_async::r#async(); // ERROR, unresolved name\n\n\n    two_async::async(); // OK\n    two_async::async(); // OK\n}\n\nmod one_async {\n    // produces_async! {} // ERROR, reserved\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\n\n"
[01:20:17] test [pretty] run-pass/edition-keywords-2015-2018.rs ... FAILED
[01:20:17] ERROR 2019-06-25T06:44:24Z: compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n#![allow(unused_assignments)]\n// edition:2018\n// aux-build:edition-kw-macro-2018.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2018;\n\npub fn check_async() {\n    // let mut async = 1; // ERROR, reserved\n    let mut r#async = 1; // OK\n\n    r#async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n    r#async = consumes_async_raw!(r#async); // OK\n\n    // if passes_ident!(async) == 1 {} // ERROR, reserved\n    if passes_ident!(r#async) == 1 {\n    } // OK\n      // one_async::async(); // ERROR, reserved\n      // one_async::r#async(); // ERROR, unresolved name\n      // two_async::async(); // ERROR, reserved\n\n    two_async::r#async(); // OK\n}\n\nmod one_async {\n    // produces_async! {} // ERROR, reserved\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\nactual:\n------------------------------------------\n#![allow(unused_assignments)]\n// edition:2018\n// aux-build:edition-kw-macro-2018.rs\n\n#[macro_use]\nextern crate edition_kw_macro_2018;\n\npub fn check_async() {\n    // let mut async = 1; // ERROR, reserved\n    let mut r#async = 1; // OK\n\n    r#async =\n        consumes_async!(async); // OK\n                                // r#async = consumes_async!(r#async); // ERROR, not a match\n                                // r#async = consumes_async_raw!(async); // ERROR, not a match\n\n\n    r#async = consumes_async_raw!(r#async); // OK\n\n    // if passes_ident!(async) == 1 {} // ERROR, reserved\n    if passes_ident!(r#async) == 1 {\n    } // OK\n      // one_async::async(); // ERROR, reserved\n      // one_async::r#async(); // ERROR, unresolved name\n      // two_async::async(); // ERROR, reserved\n\n\n    two_async::r#async(); // OK\n}\n\nmod one_async {\n    // produces_async! {} // ERROR, reserved\n}\nmod two_async {\n    produces_async_raw!{ } // OK\n}\n\nfn main() { }\n\n------------------------------------------\n\n"
[01:20:17] test [pretty] run-pass/empty-allocation-non-null.rs ... ok
[01:20:17] test [pretty] run-pass/else-if.rs ... ok
[01:20:17] test [pretty] run-pass/duplicated-external-mods.rs ... ok
[01:20:17] test [pretty] run-pass/empty-type-parameter-list.rs ... ok
---
[01:20:55] test [pretty] run-pass/z-crate-attr.rs ... ok
[01:20:55] 
[01:20:55] failures:
[01:20:55] 
[01:20:55] ---- [pretty] run-pass/allocator/custom.rs stdout ----
[01:20:55] thread '[pretty] run-pass/allocator/custom.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] 
[01:20:55] ---- [pretty] run-pass/allocator/xcrate-use.rs stdout ----
[01:20:55] ---- [pretty] run-pass/allocator/xcrate-use.rs stdout ----
[01:20:55] thread '[pretty] run-pass/allocator/xcrate-use.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/allocator/xcrate-use2.rs stdout ----
[01:20:55] ---- [pretty] run-pass/allocator/xcrate-use2.rs stdout ----
[01:20:55] thread '[pretty] run-pass/allocator/xcrate-use2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/arr_cycle.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/arr_cycle.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/arr_cycle.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/array_const_index-1.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/array_const_index-1.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/array_const_index-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/box-of-array-of-drop-1.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/box-of-array-of-drop-1.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/box-of-array-of-drop-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/box-of-array-of-drop-2.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/box-of-array-of-drop-2.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/box-of-array-of-drop-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/cast-in-array-size.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/cast-in-array-size.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/cast-in-array-size.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/check-static-mut-slices.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/check-static-mut-slices.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/check-static-mut-slices.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/check-static-slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/check-static-slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/check-static-slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/copy-out-of-array-1.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/copy-out-of-array-1.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/copy-out-of-array-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/destructure-array-1.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/destructure-array-1.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/destructure-array-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/empty-mutable-vec.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/empty-mutable-vec.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/empty-mutable-vec.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/estr-slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/estr-slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/estr-slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/evec-slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/evec-slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/evec-slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/fixed_length_copy.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/fixed_length_copy.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/fixed_length_copy.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/huge-largest-array.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/huge-largest-array.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/huge-largest-array.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/ivec-pass-by-value.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/ivec-pass-by-value.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/ivec-pass-by-value.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/mutability-inherits-through-fixed-length-vec.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/mutability-inherits-through-fixed-length-vec.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/mutability-inherits-through-fixed-length-vec.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/mutable-alias-vec.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/mutable-alias-vec.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/mutable-alias-vec.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/nested-vec-1.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/nested-vec-1.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/nested-vec-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/nested-vec-2.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/nested-vec-2.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/nested-vec-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/nested-vec-3.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/nested-vec-3.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/nested-vec-3.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/new-style-fixed-length-vec.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/new-style-fixed-length-vec.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/new-style-fixed-length-vec.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/rcvr-borrowed-to-slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/rcvr-borrowed-to-slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/rcvr-borrowed-to-slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/repeated-vector-syntax.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/repeated-vector-syntax.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/repeated-vector-syntax.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/show-boxed-slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/show-boxed-slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/show-boxed-slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-2.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-2.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/slice-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-of-zero-size-elements.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-of-zero-size-elements.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/slice-of-zero-size-elements.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-panic-1.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-panic-1.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/slice-panic-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-panic-2.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice-panic-2.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/slice-panic-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice_binary_search.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/slice_binary_search.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/slice_binary_search.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/variance-vec-covariant.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/variance-vec-covariant.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/variance-vec-covariant.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-concat.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-concat.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-concat.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-dst.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-dst.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-dst.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-fixed-length.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-fixed-length.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-fixed-length.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-growth.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-growth.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-growth.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-late-init.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-late-init.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-late-init.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-no-std.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-no-std.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-macro-no-std.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-repeat.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-repeat.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-macro-repeat.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-rvalue-scope.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-rvalue-scope.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-macro-rvalue-scope.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-with-brackets.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-with-brackets.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-macro-with-brackets.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-with-trailing-comma.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-macro-with-trailing-comma.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-macro-with-trailing-comma.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-autoslice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-autoslice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-matching-autoslice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-fixed.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-fixed.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-matching-fixed.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-fold.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-fold.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-matching-fold.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-legal-tail-element-borrow.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching-legal-tail-element-borrow.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-matching-legal-tail-element-borrow.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-matching.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-matching.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-push.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-push.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-push.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-repeat-with-cast.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-repeat-with-cast.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-repeat-with-cast.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-slice-drop.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-slice-drop.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-slice-drop.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-slice.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-slice.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-tail-matching.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-tail-matching.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-tail-matching.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-to_str.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec-to_str.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec-to_str.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec_cycle.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec_cycle.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec_cycle.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec_cycle_wrapped.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vec_cycle_wrapped.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vec_cycle_wrapped.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vector-no-ann-2.rs stdout ----
[01:20:55] ---- [pretty] run-pass/array-slice-vec/vector-no-ann-2.rs stdout ----
[01:20:55] thread '[pretty] run-pass/array-slice-vec/vector-no-ann-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-const-eval.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-const-eval.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-const-eval.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-cross-crate-const-eval.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-cross-crate-const-eval.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-cross-crate-const-eval.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-cross-crate-defaults.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-cross-crate-defaults.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-cross-crate-defaults.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-cross-crate.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-cross-crate.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-cross-crate.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-in-global-const.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-in-global-const.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-in-global-const.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-inherent-impl.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-inherent-impl.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-inherent-impl.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-marks-live-code.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-marks-live-code.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-marks-live-code.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-match-patterns.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-match-patterns.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-match-patterns.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-outer-ty-refs.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-outer-ty-refs.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-outer-ty-refs.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-overwrite-default.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-overwrite-default.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-overwrite-default.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-public-impl.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-public-impl.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-public-impl.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-range-match-patterns.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-range-match-patterns.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-range-match-patterns.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-resolution-order.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-resolution-order.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-resolution-order.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-self-type.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-self-type.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-self-type.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-type-parameters.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-type-parameters.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-type-parameters.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-ufcs-infer-trait.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-ufcs-infer-trait.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-ufcs-infer-trait.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-use-default.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-use-default.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-use-default.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-use-impl-of-same-trait.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const-use-impl-of-same-trait.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const-use-impl-of-same-trait.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-consts/associated-const.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-consts/associated-const.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-basic.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-basic.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-basic.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-binding-in-trait.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-binding-in-trait.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-binding-in-trait.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-binding-in-where-clause.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-binding-in-where-clause.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-binding-in-where-clause.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-bound.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-bound.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-bound.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-cc.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-cc.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-cc.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-conditional-dispatch.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-conditional-dispatch.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-conditional-dispatch.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-constant-type.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-constant-type.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-constant-type.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-doubleendediterator-object.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-doubleendediterator-object.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-doubleendediterator-object.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-duplicate-binding-in-env-hrtb.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-duplicate-binding-in-env.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-duplicate-binding-in-env.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-duplicate-binding-in-env.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-enum-field-named.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-enum-field-named.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-enum-field-named.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-enum-field-numbered.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-enum-field-numbered.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-enum-field-numbered.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-eq-obj.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-eq-obj.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-eq-obj.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-impl-redirect.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-impl-redirect.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-impl-redirect.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-bound-type-arg.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-bound-type-arg.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-in-bound-type-arg.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-default-method.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-default-method.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-in-default-method.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-fn.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-fn.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-in-fn.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-impl-generics.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-impl-generics.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-in-impl-generics.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-inherent-method.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-in-inherent-method.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-in-inherent-method.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-issue-20220.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-issue-20220.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-issue-20220.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-issue-20371.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-issue-20371.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-issue-20371.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-issue-21212.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-issue-21212.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-issue-21212.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-iterator-binding.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-iterator-binding.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-iterator-binding.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-method.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-method.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-method.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-nested-projections.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-nested-projections.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-nested-projections.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-normalize-in-bounds-binding.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-normalize-in-bounds-ufcs.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-in-bounds.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-in-bounds.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-normalize-in-bounds.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-unifield-struct.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-normalize-unifield-struct.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-normalize-unifield-struct.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-project-from-type-param-via-bound-in-where.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-project-from-type-param-via-bound-in-where.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-project-from-type-param-via-bound-in-where.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-projection-bound-in-supertraits.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-from-known-type-in-impl.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-from-known-type-in-impl.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-projection-from-known-type-in-impl.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-in-object-type.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-in-object-type.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-projection-in-object-type.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-in-supertrait.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-in-supertrait.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-projection-in-supertrait.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-in-where-clause.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-in-where-clause.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-projection-in-where-clause.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-to-unrelated-trait.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-projection-to-unrelated-trait.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-projection-to-unrelated-trait.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-qualified-path-with-trait-with-type-parameters.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-qualified-path-with-trait-with-type-parameters.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-qualified-path-with-trait-with-type-parameters.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-ref-from-struct.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-ref-from-struct.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-ref-from-struct.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-ref-in-struct-literal.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-ref-in-struct-literal.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-ref-in-struct-literal.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-region-erasure-issue-20582.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-region-erasure-issue-20582.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-region-erasure-issue-20582.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-resolve-lifetime.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-resolve-lifetime.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-resolve-lifetime.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-return.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-return.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-return.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-simple.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-simple.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-simple.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-stream.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-stream.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-stream.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-struct-field-named.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-struct-field-named.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-struct-field-named.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-struct-field-numbered.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-struct-field-numbered.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-struct-field-numbered.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-sugar-path.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-sugar-path.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-sugar-path.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-where-clause-impl-ambiguity.rs stdout ----
[01:20:55] ---- [pretty] run-pass/associated-types/associated-types-where-clause-impl-ambiguity.rs stdout ----
[01:20:55] thread '[pretty] run-pass/associated-types/associated-types-where-clause-impl-ambiguity.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:55] ---- [pretty] run-pass/atomic-alignment.rs stdout ----
[01:20:55] 
[01:20:55] error: pretty-printed source does not match expected source
[01:20:55] expected:
[01:20:55] expected:
[01:20:55] ------------------------------------------
[01:20:55] #![feature(cfg_target_has_atomic)]
[01:20:55] #![feature(integer_atomics)]
[01:20:55] 
[01:20:55] use std::mem::{align_of, size_of};
[01:20:55] use std::sync::atomic::*;
[01:20:55] fn main() {
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "8")]
[01:20:55]     assert_eq!(align_of :: < AtomicBool > (  ) , size_of :: < AtomicBool > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "ptr")]
[01:20:55]     assert_eq!(align_of :: < AtomicPtr < u8 >> (  ) , size_of :: < AtomicPtr <
[01:20:55]                u8 >> (  ));
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "8")]
[01:20:55]     assert_eq!(align_of :: < AtomicU8 > (  ) , size_of :: < AtomicU8 > (  ));
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "8")]
[01:20:55]     assert_eq!(align_of :: < AtomicI8 > (  ) , size_of :: < AtomicI8 > (  ));
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "16")]
[01:20:55]     assert_eq!(align_of :: < AtomicU16 > (  ) , size_of :: < AtomicU16 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "16")]
[01:20:55]     assert_eq!(align_of :: < AtomicI16 > (  ) , size_of :: < AtomicI16 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "32")]
[01:20:55]     assert_eq!(align_of :: < AtomicU32 > (  ) , size_of :: < AtomicU32 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "32")]
[01:20:55]     assert_eq!(align_of :: < AtomicI32 > (  ) , size_of :: < AtomicI32 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "64")]
[01:20:55]     assert_eq!(align_of :: < AtomicU64 > (  ) , size_of :: < AtomicU64 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "64")]
[01:20:55]     assert_eq!(align_of :: < AtomicI64 > (  ) , size_of :: < AtomicI64 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "128")]
[01:20:55]     assert_eq!(align_of :: < AtomicU128 > (  ) , size_of :: < AtomicU128 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "128")]
[01:20:55]     assert_eq!(align_of :: < AtomicI128 > (  ) , size_of :: < AtomicI128 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "ptr")]
[01:20:55]     assert_eq!(align_of :: < AtomicUsize > (  ) , size_of :: < AtomicUsize > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "ptr")]
[01:20:55]     assert_eq!(align_of :: < AtomicIsize > (  ) , size_of :: < AtomicIsize > (
[01:20:55] }
[01:20:55] 
[01:20:55] ------------------------------------------
[01:20:55] actual:
[01:20:55] actual:
[01:20:55] ------------------------------------------
[01:20:55] #![feature(cfg_target_has_atomic)]
[01:20:55] #![feature(integer_atomics)]
[01:20:55] 
[01:20:55] use std::mem::{align_of, size_of};
[01:20:55] use std::sync::atomic::*;
[01:20:55] fn main() {
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "8")]
[01:20:55]     assert_eq!(align_of :: < AtomicBool > (  ) , size_of :: < AtomicBool > (
[01:20:55]                ));
[01:20:55]     #[cfg(target_has_atomic = "ptr")]
[01:20:55]     assert_eq!(align_of :: < AtomicPtr < u8 >> (  ) , size_of :: < AtomicPtr <
[01:20:55]                u8 >> (  ));
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "8")]
[01:20:55]     assert_eq!(align_of :: < AtomicU8 > (  ) , size_of :: < AtomicU8 > (  ));
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "8")]
[01:20:55]     assert_eq!(align_of :: < AtomicI8 > (  ) , size_of :: < AtomicI8 > (  ));
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "16")]
[01:20:55]     assert_eq!(align_of :: < AtomicU16 > (  ) , size_of :: < AtomicU16 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "16")]
[01:20:55]     assert_eq!(align_of :: < AtomicI16 > (  ) , size_of :: < AtomicI16 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "32")]
[01:20:55]     assert_eq!(align_of :: < AtomicU32 > (  ) , size_of :: < AtomicU32 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "32")]
[01:20:55]     assert_eq!(align_of :: < AtomicI32 > (  ) , size_of :: < AtomicI32 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "64")]
[01:20:55]     assert_eq!(align_of :: < AtomicU64 > (  ) , size_of :: < AtomicU64 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "64")]
[01:20:55]     assert_eq!(align_of :: < AtomicI64 > (  ) , size_of :: < AtomicI64 > (
[01:20:55] 
[01:20:55] 
[01:20:55]     #[cfg(target_has_atomic = "128")]
---
[01:20:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:20:58] 
[01:20:58] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:20:58] 
[01:20:58] note: compiler flags: -Z no-codegen -Z unstable-options -Z fuel=foo=0 -C rpath -C debuginfo=0
[01:20:58] 
[01:20:58] ------------------------------------------
[01:20:58] 
[01:20:58] 
[01:20:58] 
[01:20:58] ---- [pretty] run-pass/optimization-fuel-1.rs stdout ----
[01:20:58] 
[01:20:58] error: pretty-printed source does not typecheck
[01:20:58] status: exit code: 101
[01:20:58] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-codegen" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-1/optimization-fuel-1.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/optimization-fuel-1/auxiliary.pretty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "human" "-Z" "fuel=foo=1"
[01:20:58] ------------------------------------------
[01:20:58] 
[01:20:58] ------------------------------------------
[01:20:58] stderr:
---
[01:20:58] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:20:58] 
[01:20:58] note: rustc 1.37.0-dev running on x86_64-unknown-linux-gnu
[01:20:58] 
[01:20:58] note: compiler flags: -Z no-codegen -Z unstable-options -Z fuel=foo=1 -C rpath -C debuginfo=0
[01:20:58] 
[01:20:58] ------------------------------------------
[01:20:58] 
[01:20:58] 
[01:20:58] 
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-count.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-autoderef-count.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-indexing.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-indexing.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-autoderef-indexing.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-order.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-order.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-autoderef-order.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-vtable.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-vtable.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-autoderef-vtable.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-xcrate.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef-xcrate.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-autoderef-xcrate.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-autoderef.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-autoderef.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-object-one-arg.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-object-one-arg.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-calls-object-one-arg.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-object-two-args.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-object-two-args.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-calls-object-two-args.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-object-zero-args.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-object-zero-args.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-calls-object-zero-args.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-param-vtables.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-param-vtables.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-calls-param-vtables.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-simple.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-simple.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-calls-simple.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-zero-args.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-calls-zero-args.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-calls-zero-args.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-deref-count.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-deref-count.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-deref-count.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-deref.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-deref.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-deref.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index-assoc-list.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index-assoc-list.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-index-assoc-list.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index-autoderef.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index-autoderef.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-index-autoderef.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index-in-field.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index-in-field.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-index-in-field.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded-index.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded-index.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded_deref_with_ref_pattern.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded_deref_with_ref_pattern.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded_deref_with_ref_pattern.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded_deref_with_ref_pattern_issue15609.rs stdout ----
[01:20:58] ---- [pretty] run-pass/overloaded/overloaded_deref_with_ref_pattern_issue15609.rs stdout ----
[01:20:58] thread '[pretty] run-pass/overloaded/overloaded_deref_with_ref_pattern_issue15609.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/packed/packed-struct-borrow-element.rs stdout ----
[01:20:58] ---- [pretty] run-pass/packed/packed-struct-borrow-element.rs stdout ----
[01:20:58] thread '[pretty] run-pass/packed/packed-struct-borrow-element.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/packed/packed-struct-drop-aligned.rs stdout ----
[01:20:58] ---- [pretty] run-pass/packed/packed-struct-drop-aligned.rs stdout ----
[01:20:58] thread '[pretty] run-pass/packed/packed-struct-drop-aligned.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:58] ---- [pretty] run-pass/packed/packed-struct-generic-layout.rs stdout ----
[01:20:58] ---- [pretty] run-pass/packed/packed-struct-generic-layout.rs stdout ----
[01:20:58] thread '[pretty] run-pass/packed/packed-struct-generic-layout.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-generic-size.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-generic-size.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-generic-size.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-layout.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-layout.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-layout.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-match.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-match.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-match.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-optimized-enum.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-optimized-enum.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-optimized-enum.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-size-xc.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-size-xc.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-size-xc.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-size.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-size.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-size.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-vec.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-struct-vec.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-struct-vec.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-tuple-struct-layout.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-tuple-struct-layout.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-tuple-struct-layout.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/packed/packed-tuple-struct-size.rs stdout ----
[01:20:59] ---- [pretty] run-pass/packed/packed-tuple-struct-size.rs stdout ----
[01:20:59] thread '[pretty] run-pass/packed/packed-tuple-struct-size.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panic-runtime/abort-link-to-unwinding-crates.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panic-runtime/abort-link-to-unwinding-crates.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panic-runtime/abort-link-to-unwinding-crates.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panic-runtime/abort.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panic-runtime/abort.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panic-runtime/abort.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panic-runtime/link-to-abort.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panic-runtime/link-to-abort.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panic-runtime/link-to-abort.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panic-runtime/link-to-unwind.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panic-runtime/link-to-unwind.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panic-runtime/link-to-unwind.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panic-runtime/lto-abort.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panic-runtime/lto-abort.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panic-runtime/lto-abort.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panic-runtime/lto-unwind.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panic-runtime/lto-unwind.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panic-runtime/lto-unwind.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panics/panic-handler-chain.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panics/panic-handler-chain.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panics/panic-handler-chain.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panics/panic-handler-flail-wildly.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panics/panic-handler-flail-wildly.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panics/panic-handler-flail-wildly.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panics/panic-handler-set-twice.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panics/panic-handler-set-twice.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panics/panic-handler-set-twice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panics/panic-in-dtor-drops-fields.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panics/panic-in-dtor-drops-fields.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panics/panic-in-dtor-drops-fields.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panics/panic-recover-propagate.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panics/panic-recover-propagate.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panics/panic-recover-propagate.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/panics/panic-safe.rs stdout ----
[01:20:59] ---- [pretty] run-pass/panics/panic-safe.rs stdout ----
[01:20:59] thread '[pretty] run-pass/panics/panic-safe.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/priv-impl-prim-ty.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/priv-impl-prim-ty.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/priv-impl-prim-ty.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/privacy-ns.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/privacy-ns.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/privacy-ns.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/privacy-reexport.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/privacy-reexport.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/privacy-reexport.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/privacy1.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/privacy1.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/privacy1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/private-class-field.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/private-class-field.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/private-class-field.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/private-method.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/private-method.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/private-method.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/pub-extern-privacy.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/pub-extern-privacy.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/pub-extern-privacy.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/pub-use-xcrate.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/pub-use-xcrate.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/pub-use-xcrate.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/privacy/pub_use_mods_xcrate_exe.rs stdout ----
[01:20:59] ---- [pretty] run-pass/privacy/pub_use_mods_xcrate_exe.rs stdout ----
[01:20:59] thread '[pretty] run-pass/privacy/pub_use_mods_xcrate_exe.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-envs.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-envs.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-envs.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-exit.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-exit.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-exit.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-remove-from-env.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-remove-from-env.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-remove-from-env.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-sigpipe.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-sigpipe.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-sigpipe.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-spawn-nonexistent.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-spawn-nonexistent.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-spawn-nonexistent.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-spawn-with-unicode-params.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-spawn-with-unicode-params.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-spawn-with-unicode-params.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/process/process-status-inherits-stdin.rs stdout ----
[01:20:59] ---- [pretty] run-pass/process/process-status-inherits-stdin.rs stdout ----
[01:20:59] thread '[pretty] run-pass/process/process-status-inherits-stdin.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-addr-of-interior-of-unique-box.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-addr-of-interior-of-unique-box.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-addr-of-interior-of-unique-box.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-addr-of-ret.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-addr-of-ret.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-addr-of-ret.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-assoc-type-region-bound.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-assoc-type-region-bound.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-assoc-type-region-bound.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-assoc-type-static-bound.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-assoc-type-static-bound.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-assoc-type-static-bound.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-at.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-at.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-borrow-at.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-evec-fixed.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-evec-fixed.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-borrow-evec-fixed.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-evec-uniq.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-evec-uniq.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-borrow-evec-uniq.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-uniq.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-borrow-uniq.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-borrow-uniq.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-bot.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-bot.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-bot.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-bound-lists-feature-gate-2.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-bound-lists-feature-gate-2.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-bound-lists-feature-gate-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-bound-lists-feature-gate.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-bound-lists-feature-gate.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-bound-lists-feature-gate.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-close-over-type-parameter-successfully.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-close-over-type-parameter-successfully.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-close-over-type-parameter-successfully.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-copy-closure.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-copy-closure.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-copy-closure.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-creating-enums2.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-creating-enums2.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-creating-enums2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-creating-enums5.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-creating-enums5.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-creating-enums5.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-debruijn-of-object.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-debruijn-of-object.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-debruijn-of-object.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-autofn.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-autofn.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-dependent-autofn.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-addr-of.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-addr-of.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-dependent-addr-of.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-autoslice.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-autoslice.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-dependent-autoslice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-lifetime-in-assoc-fn.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-lifetime-in-assoc-fn.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-early-bound-lifetime-in-assoc-fn.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-trait-param.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-trait-param.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-early-bound-trait-param.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-let-ref.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-dependent-let-ref.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-dependent-let-ref.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-used-in-bound.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-used-in-bound.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-early-bound-used-in-bound.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-used-in-bound-method.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-used-in-bound-method.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-early-bound-used-in-bound-method.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-used-in-type-param.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-early-bound-used-in-type-param.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-early-bound-used-in-type-param.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-escape-into-other-fn.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-escape-into-other-fn.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-escape-into-other-fn.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-expl-self.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-expl-self.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-expl-self.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-fn-subtyping.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-fn-subtyping.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-fn-subtyping.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-free-region-outlives-static-outlives-free-region.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-free-region-outlives-static-outlives-free-region.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-free-region-outlives-static-outlives-free-region.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope-addr-of.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope-addr-of.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-borrow-scope-addr-of.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope-view.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope-view.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-borrow-scope-view.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-fn-subtyping-2.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-fn-subtyping-2.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-fn-subtyping-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope-within-loop-ok.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope-within-loop-ok.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-borrow-scope-within-loop-ok.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-borrow-scope.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-borrow-scope.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-call.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-call.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-call.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-call-2.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-call-2.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-call-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-reborrow-ref-mut-recurse.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-reborrow-ref-mut-recurse.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-reborrow-ref-mut-recurse.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-contravariance-due-to-ret.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-contravariance-due-to-ret.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-contravariance-due-to-ret.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-static-from-proc.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-static-from-proc.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-static-from-proc.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-region-in-fn-but-not-type.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-infer-region-in-fn-but-not-type.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-infer-region-in-fn-but-not-type.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-issue-22246.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-issue-22246.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-issue-22246.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-lifetime-nonfree-late-bound.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-lifetime-nonfree-late-bound.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-lifetime-nonfree-late-bound.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-issue-21422.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-issue-21422.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-issue-21422.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-lifetime-static-items-enclosing-scopes.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-lifetime-static-items-enclosing-scopes.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-lifetime-static-items-enclosing-scopes.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-link-fn-args.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-link-fn-args.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-link-fn-args.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-lub-ref-ref-rc.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-lub-ref-ref-rc.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-lub-ref-ref-rc.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-mock-codegen.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-mock-codegen.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-mock-codegen.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-no-variance-from-fn-generics.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-no-variance-from-fn-generics.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-no-variance-from-fn-generics.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-nullary-variant.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-nullary-variant.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-nullary-variant.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-params.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-params.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-params.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-reassign-let-bound-pointer.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-reassign-let-bound-pointer.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-reassign-let-bound-pointer.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-reassign-match-bound-pointer.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-reassign-match-bound-pointer.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-reassign-match-bound-pointer.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-refcell.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-refcell.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-refcell.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-relate-bound-regions-on-closures-to-inference-variables.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-relate-bound-regions-on-closures-to-inference-variables.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-relate-bound-regions-on-closures-to-inference-variables.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-return-interior-of-option.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-return-interior-of-option.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-return-interior-of-option.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-scope-chain-example.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-scope-chain-example.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-scope-chain-example.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-self-impls.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-self-impls.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-self-impls.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-self-in-enums.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-self-in-enums.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-self-in-enums.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-simple.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-simple.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-simple.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-static-bound.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-static-bound.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-static-bound.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-static-closure.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-static-closure.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-static-closure.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-trait-object-1.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-trait-object-1.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-trait-object-1.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-variance-contravariant-use-contravariant.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-variance-contravariant-use-contravariant.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-variance-contravariant-use-contravariant.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-variance-covariant-use-covariant.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-variance-covariant-use-covariant.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-variance-covariant-use-covariant.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/regions/regions-no-bound-in-argument-cleanup.rs stdout ----
[01:20:59] ---- [pretty] run-pass/regions/regions-no-bound-in-argument-cleanup.rs stdout ----
[01:20:59] thread '[pretty] run-pass/regions/regions-no-bound-in-argument-cleanup.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1014-2.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1014-2.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1014-2.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1014.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1014.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1014.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1789-as-cell/from-mut.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1789-as-cell/from-mut.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1789-as-cell/from-mut.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-box-dyn-error.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-box-dyn-error.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-box-dyn-error.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-empty.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-empty.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-empty.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-exitcode.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-exitcode.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-exitcode.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-impl-termination.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-impl-termination.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-impl-termination.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-result-box-error_ok.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-result-box-error_ok.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-result-box-error_ok.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-result.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-result.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-result.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-str.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-str.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-1937-termination-trait/termination-trait-for-str.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/box.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/box.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/box.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/constref.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/constref.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/constref.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/enum.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/enum.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/enum.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/for.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/for.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/for.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/general.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/general.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/general.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/lit.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/lit.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/lit.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/range.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/range.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/range.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/ref-region.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/ref-region.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/ref-region.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/reset-mode.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/reset-mode.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/reset-mode.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/struct.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/struct.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/struct.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/tuple-struct.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/tuple-struct.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/tuple-struct.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/slice.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/slice.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/slice.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2126-crate-paths/crate-path-absolute.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2126-crate-paths/crate-path-visibility-ambiguity.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/basic.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/basic.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/basic.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/tuple.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2005-default-binding-mode/tuple.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2005-default-binding-mode/tuple.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/whitelisted.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/whitelisted.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/whitelisted.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/attr.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/attr.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2151-raw-identifiers/attr.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/test.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/test.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2126-extern-absolute-paths/test.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/items.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/items.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2151-raw-identifiers/items.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/basic.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/basic.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2151-raw-identifiers/basic.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/macros.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2151-raw-identifiers/macros.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2151-raw-identifiers/macros.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2175-or-if-while-let/basic.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2175-or-if-while-let/basic.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2175-or-if-while-let/basic.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2302-self-struct-ctor.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2302-self-struct-ctor.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2302-self-struct-ctor.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1445/eq-allows-match-on-ty-in-macro.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1445/eq-allows-match-on-ty-in-macro.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc1445/eq-allows-match-on-ty-in-macro.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2421-unreserve-pure-offsetof-sizeof-alignof.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc-2421-unreserve-pure-offsetof-sizeof-alignof.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc-2421-unreserve-pure-offsetof-sizeof-alignof.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1445/eq-allows-match.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1445/eq-allows-match.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc1445/eq-allows-match.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1717/library-override.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1717/library-override.rs stdout ----
[01:20:59] thread '[pretty] run-pass/rfcs/rfc1717/library-override.rs' panicked at '`run-pass` header is only supported in UI tests', src/tools/compiletest/src/header.rs:599:17
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1857-drop-order.rs stdout ----
[01:20:59] ---- [pretty] run-pass/rfcs/rfc1857-drop-order.rs stdout ----
---
[01:21:01] test result: FAILED. 610 passed; 2262 failed; 50 ignored; 0 measured; 0 filtered out
[01:21:01] 
[01:21:01] 
[01:21:01] 
[01:21:01] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-pass" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:21:01] 
[01:21:01] 
[01:21:01] 
[01:21:01] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-pass/pretty src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/test/run-pass-fulldeps/pretty src/tools/cargo src/tools/cargotest
[01:21:01] Build completed unsuccessfully in 1:17:18
[01:21:01] Makefile:50: recipe for target 'check-aux' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0a236836
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jun 25 06:45:09 UTC 2019
---
travis_time:end:00968bf6:start=1561445110717944394,finish=1561445110724746672,duration=6802278
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1877f000
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1120c61d
travis_time:start:1120c61d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f5ceead
$ dmesg | grep -i kill
