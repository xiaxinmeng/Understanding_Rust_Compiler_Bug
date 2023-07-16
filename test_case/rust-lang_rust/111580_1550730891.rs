plain

---- [ui] tests/ui/limits/issue-69485-var-size-diffs-too-large.rs stdout ----
diff of stderr:

- error: values of the type `[u8; usize::MAX]` are too big for the current architecture
+ error: failed to get layout for Bug: values of the type `[u8; usize::MAX]` are too big for the current architecture
3    |
3    |
4 LL |     Bug::V([0; !0]);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-69485-var-size-diffs-too-large/issue-69485-var-size-diffs-too-large.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args limits/issue-69485-var-size-diffs-too-large.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/limits/issue-69485-var-size-diffs-too-large.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-69485-var-size-diffs-too-large" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-69485-var-size-diffs-too-large/auxiliary" "-Zmir-opt-level=0"
stdout: none
--- stderr -------------------------------
error: failed to get layout for Bug: values of the type `[u8; usize::MAX]` are too big for the current architecture
  --> fake-test-src-base/limits/issue-69485-var-size-diffs-too-large.rs:6:5
   |
LL |     Bug::V([0; !0]); //~ ERROR are too big for the current

error: aborting due to previous error
------------------------------------------

