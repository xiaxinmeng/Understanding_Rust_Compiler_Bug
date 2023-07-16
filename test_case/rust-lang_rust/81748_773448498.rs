plain
test /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 35) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/library-features/global-asm.md - The_tracking_issue_for_this_feature_is__ (line 35) stdout ----
error[E0601]: `main` function not found in crate `rust_out`
  --> /checkout/src/doc/unstable-book/src/library-features/global-asm.md:34:1
   |
1  | / #![allow(unused)]
2  | | #![feature(global_asm)]
3  | | #![cfg(any(target_arch = "x86", target_arch = "x86_64"))]
...  |
33 | | }
33 | | }
34 | | } _doctest_main__checkout_src_doc_unstable_book_src_library_features_global_asm_md_35_0() }
   | |___________________________________________________________________________________________^ consider adding a `main` function at the crate level
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
Couldn't compile the test.
---
test result: FAILED. 0 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.08s



command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/library-features/global-asm.md" "--test-args" ""


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 1:48:42
