plain
---- [ui] src/test/ui/lto/issue-100772.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/issue-100772.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-musl" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-musl/native/rust-test-helpers" "-Clinker=x86_64-linux-musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-100772/auxiliary" "-Zsanitizer=cfi" "-Clto"
stdout: none
--- stderr -------------------------------
error: sanitizer is incompatible with statically linked libc, disable it using `-C target-feature=-crt-static`
error: aborting due to previous error
------------------------------------------


