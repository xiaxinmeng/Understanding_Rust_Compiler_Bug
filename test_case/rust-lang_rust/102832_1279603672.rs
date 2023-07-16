plain
........................................................................................ 8008/13670
........................................................................................ 8096/13670
........................................................................................ 8184/13670
......ii...............i......i..ii..................................................... 8272/13670
..................................F...F................................................. 8360/13670
........................................................................................ 8536/13670
........................................................................................ 8624/13670
.......................................................i..ii............................ 8712/13670
..................................ii.................................................... 8800/13670
---
---- [ui] src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive-link-attr/auxiliary"
stdout: none
--- stderr -------------------------------
error: the linking modifiers `+bundle` and `+whole-archive` are not compatible with each other when generating rlibs
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive.rs stdout ----
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/native-library-link-flags/mix-bundle-and-whole-archive.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "rlib" "-l" "static:+bundle,+whole-archive=mylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/native-library-link-flags/mix-bundle-and-whole-archive/auxiliary"
stdout: none
--- stderr -------------------------------
error: the linking modifiers `+bundle` and `+whole-archive` are not compatible with each other when generating rlibs
error: aborting due to previous error
------------------------------------------


