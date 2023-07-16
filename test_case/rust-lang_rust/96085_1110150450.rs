plain

9    |
10 LL | #![deny(unused_crate_dependencies)]
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
-    = help: remove unnecessary dependency `bar`
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-attr/deny-attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/deny-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/deny-attr.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-attr/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-attr/auxiliary/libbar.so"
stdout: none
--- stderr -------------------------------
error: external crate `bar` unused in `deny_attr`: remove the dependency or add `use bar as _;`
   |
LL | #![deny(unused_crate_dependencies)]
   | ^
   |
---
diff of stderr:

5    | ^
6    |
7    = note: requested on the command line with `-D unused-crate-dependencies`
-    = help: remove unnecessary dependency `bar`
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-cmdline/deny-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused-crate-deps/deny-cmdline.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/deny-cmdline.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "--edition=2018" "-Dunused-crate-dependencies" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-cmdline/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/deny-cmdline/auxiliary/libbar.so"
stdout: none
--- stderr -------------------------------
error: external crate `bar` unused in `deny_cmdline`: remove the dependency or add `use bar as _;`
   |
LL | fn main() {}
   | ^
   |
   |
   = note: requested on the command line with `-D unused-crate-dependencies`
error: aborting due to previous error
------------------------------------------


