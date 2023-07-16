plain
---- [ui] src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-90875.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-90875.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-90875" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-90875/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0601]: `main` function not found in crate `issue_90875`
   |
LL | }
LL | }
   |  ^ consider adding a `main` function to `/checkout/src/test/ui/higher-rank-trait-bounds/normalize-under-binder/issue-90875.rs`
error: aborting due to previous error
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

For more information about this error, try `rustc --explain E0601`.
