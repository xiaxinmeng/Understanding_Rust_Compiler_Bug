
Testing error-index stage2
[01:32:37] doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md
[01:33:11] 
[01:33:11] 
[01:33:11] command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""
[01:33:11] expected success, got: exit code: 1
[01:33:11] 
[01:33:11] stdout ----
[01:33:11] 
[01:33:11] running 677 tests
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0003::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 71) ... ok
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0002::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 55) ... ok
...
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0511 (line 9584) ... ignored
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0508 (line 9469) ... ok
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0512 (line 9621) ... ok
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0509 (line 9525) ... ok
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0509 (line 9551) ... ok
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0516 (line 9649) ... ok
[01:33:11] test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0517 (line 9672) ... ok
[01:33:11] 
[01:33:11] stderr ----
[01:33:11] 
[01:33:11] 
[01:33:11] 
[01:33:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:11] Build completed unsuccessfully in 0:40:44
[01:33:11] make: *** [check] Error 1
[01:33:11] Makefile:52: recipe for target 'check' failed
