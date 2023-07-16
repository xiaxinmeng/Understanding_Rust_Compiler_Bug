plain
diff of stderr:

1 warning: ignoring --out-dir flag due to -o flag
2 
- error: the generated executable for the input file "$DIR/non-ice-error-on-worker-io-fail.rs" conflicts with the existing directory "/does-not-exist/output"
+ error: io error modifying /does-not-exist/
5 error: aborting due to previous error; 1 warning emitted
6 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/io-checks/non-ice-error-on-worker-io-fail/non-ice-error-on-worker-io-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args io-checks/non-ice-error-on-worker-io-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/io-checks/non-ice-error-on-worker-io-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/io-checks/non-ice-error-on-worker-io-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/io-checks/non-ice-error-on-worker-io-fail/auxiliary" "-o" "/does-not-exist/output"
stdout: none
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag

error: couldn't create a temp dir: No such file or directory (os error 2) at path "/does-not-exist/rmetazUHTYT"
error: aborting due to previous error; 1 warning emitted
------------------------------------------


