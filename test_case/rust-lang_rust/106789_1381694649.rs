plain

---- [ui] checkout/tests/ui/save-analysis/emit-notifications.rs stdout ----
diff of stderr:

- {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/save-analysis/libemit_notifications.json","emit":"save-analysis"}
+ {"artifact":"$TEST_BUILD_DIR/save-analysis/suggestions-not-always-applicable.json","emit":"save-analysis"}
2 {"artifact":"$TEST_BUILD_DIR/save-analysis/emit-notifications/libemit_notifications.rlib","emit":"link"}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications/emit-notifications.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications/emit-notifications.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args save-analysis/emit-notifications.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/save-analysis/emit-notifications.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/emit-notifications/auxiliary" "-Zsave-analysis" "--json" "artifacts" "--crate-type" "rlib" "--error-format=json"
stdout: none
stderr: none


failures:
    [ui] checkout/tests/ui/save-analysis/emit-notifications.rs
