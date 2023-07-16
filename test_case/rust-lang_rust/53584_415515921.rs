plain
[00:48:11] ....................................................................................................
[00:48:15] ....................................................................................................
[00:48:17] ...i................................................................................................
[00:48:20] ....................................................................................................
[00:48:23] ...................................................iiiiiiiii........................................
[00:48:28] ....................................................................................................
[00:48:32] ....................................................................................................
[00:48:34] ................................i...................................................................
[00:48:37] ..................................................................................i.i..ii...........
---
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0087::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1809) stdout ----
[01:16:02] error[E0107]: wrong number of type arguments: expected 1, found 2
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1813:16
[01:16:02]   |
[01:16:02] 5 |     foo::<f64, bool>(); // error: wrong number of type arguments:
[01:16:02]   |                ^^^^ unexpected type argument
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0087::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1809)' panicked at 'Some expected error codes were not found: ["E0087"]', librustdoc/test.rs:338:9
[01:16:02] 
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0088::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1826) stdout ----
[01:16:02] error[E0107]: wrong number of lifetime arguments: expected 0, found 1
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1830:9
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1830:9
[01:16:02]   |
[01:16:02] 5 |     f::<'static>() // error: wrong number of lifetime arguments:
[01:16:02]   |         ^^^^^^^ unexpected lifetime argument
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0088::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1826)' panicked at 'Some expected error codes were not found: ["E0088"]', librustdoc/test.rs:338:9
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0089::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1873) stdout ----
[01:16:02] error[E0107]: wrong number of type arguments: expected 2, found 1
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1877:5
[01:16:02]   |
[01:16:02]   |
[01:16:02] 5 |     foo::<f64>(); // error: wrong number of type arguments: expected 2, found 1
[01:16:02]   |     ^^^^^^^^^^ expected 2 type arguments
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0089::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1873)' panicked at 'Some expected error codes were not found: ["E0089"]', librustdoc/test.rs:338:9
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0090::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1900) stdout ----
[01:16:02] error[E0107]: wrong number of lifetime arguments: expected 2, found 1
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1904:5
[01:16:02]   |
[01:16:02]   |
[01:16:02] 5 |     foo::<'static>(); // error: wrong number of lifetime arguments:
[01:16:02] 
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0090::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1900)' panicked at 'Some expected error codes were not found: ["E0090"]', librustdoc/test.rs:338:9
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0089::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1884) stdout ----
[01:16:02] error[E0107]: wrong number of type arguments: expected 2, found 1
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1889:5
[01:16:02]   |
[01:16:02]   |
[01:16:02] 6 |     foo::<f64>(x);    // error: wrong number of type arguments:
[01:16:02]   |     ^^^^^^^^^^ expected 2 type arguments
[01:16:02] error[E0308]: mismatched types
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:1889:16
[01:16:02]   |
[01:16:02]   |
[01:16:02] 6 |     foo::<f64>(x);    // error: wrong number of type arguments:
[01:16:02]   |                ^ expected f64, found bool
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0089::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1884)' panicked at 'Some expected error codes were not found: ["E0089"]', librustdoc/test.rs:338:9
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0107 (line 2123) stdout ----
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0107 (line 2123) stdout ----
[01:16:02] error: expected one of `.`, `;`, `?`, `}`, or an operator, found `f`
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:2129:5
[01:16:02]   |
[01:16:02] 5 |     f()                 // error: wrong number of lifetime arguments:
[01:16:02]   |        - expected one of `.`, `;`, `?`, `}`, or an operator here
[01:16:02] 6 |                         //        expected 1, found 0
[01:16:02] 7 |     f::<'static, 'b>() // error: wrong number of lifetime arguments:
[01:16:02]   |     ^ unexpected token
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0107 (line 2123)' panicked at 'Some expected error codes were not found: ["E0088"]', librustdoc/test.rs:338:9
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0243::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3846) stdout ----
[01:16:02] error[E0107]: wrong number of type arguments: expected 1, found 0
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:3849:17
[01:16:02]   |
[01:16:02]   |
[01:16:02] 5 | struct Bar { x: Foo }
[01:16:02]   |                 ^^^ expected 1 type argument
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0243::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3846)' panicked at 'Some expected error codes were not found: ["E0243"]', librustdoc/test.rs:338:9
[01:16:02] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0244::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3861) stdout ----
[01:16:02] error[E0107]: wrong number of type arguments: expected 0, found 2
[01:16:02]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:3864:23
[01:16:02]   |
[01:16:02]   |
[01:16:02] 5 | struct Bar<S, T> { x: Foo<S, T> }
[01:16:02]   |                       ^^^^^^^^^ 2 unexpected type arguments
[01:16:02] 
[01:16:02] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0244::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 3861)' panicked at 'Some expected error codes were not found: ["E0244"]', librustdoc/test.rs:338:9
[01:16:02] 
[01:16:02] failures:
[01:16:02]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0087::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1809)
[01:16:02]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0088::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 1826)
---
[01:16:02] 
[01:16:02] 
[01:16:02] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:16:02] Build completed unsuccessfully in 0:31:48
[01:16:02] Makefile:58: recipe for target 'check' failed
[01:16:02] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b98a7e0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
