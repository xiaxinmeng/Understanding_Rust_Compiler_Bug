plain

---- [ui] tests/ui/test-attrs/tests-listing-format-json-without-unstableopts.rs stdout ----
diff of run.stderr:

- error: The "json" format is only accepted on the nightly compiler
+ error: The "json" format is only accepted on the nightly compiler with -Z unstable-options


The actual run.stderr differed from the expected run.stderr.
The actual run.stderr differed from the expected run.stderr.
Actual run.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json-without-unstableopts/tests-listing-format-json-without-unstableopts.run.stderr
error: 1 errors occurred comparing run output.
status: exit status: 101
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json-without-unstableopts" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/tests-listing-format-json-without-unstableopts/a" "--list" "--format" "json"
stdout: none
--- stderr -------------------------------
error: The "json" format is only accepted on the nightly compiler with -Z unstable-options



failures:
