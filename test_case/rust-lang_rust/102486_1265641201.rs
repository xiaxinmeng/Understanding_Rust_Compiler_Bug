plain

---- [ui] src/test/ui/consts/const_in_pattern/issue-73431.rs stdout ----
diff of stderr:

- 2:rustcWARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.
+ WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/issue-73431.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_in_pattern/issue-73431.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_in_pattern/issue-73431.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-73431/auxiliary"
stdout: none
--- stderr -------------------------------
WARN rustc_mir_build::thir::pattern::const_to_pat MIR const-checker found novel structural match violation. See #73448.



failures:
