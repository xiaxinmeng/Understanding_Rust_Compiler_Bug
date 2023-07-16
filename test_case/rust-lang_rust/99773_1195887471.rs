plain

---- [ui] src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr.rs stdout ----
diff of stderr:

- error: the linking modifiers `+bundle` and `+whole-archive` are not compatible with each other when generating rlibs
- 
3 error: could not find native static library `mylib`, perhaps an -L flag is missing?
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
6 
7 
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunstable-options" "--crate-type" "rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr/auxiliary"
stdout: none
--- stderr -------------------------------
error: could not find native static library `mylib`, perhaps an -L flag is missing?
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive.rs stdout ----
diff of stderr:

- error: the linking modifiers `+bundle` and `+whole-archive` are not compatible with each other when generating rlibs
- 
3 error: could not find native static library `mylib`, perhaps an -L flag is missing?
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
6 
7 
---
To only update this specific test, also pass `--test-args native-library-link-flags/mix-bundle-and-whole-archive.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-l" "static:+bundle,+whole-archive=mylib" "-Zunstable-options" "--crate-type" "rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive/auxiliary"
stdout: none
--- stderr -------------------------------
error: could not find native static library `mylib`, perhaps an -L flag is missing?
error: aborting due to previous error
------------------------------------------


