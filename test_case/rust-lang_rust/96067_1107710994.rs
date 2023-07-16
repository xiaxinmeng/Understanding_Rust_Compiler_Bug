plain
iii..................................................................................... 12936/12955
...................
failures:

---- [ui] src/test/ui/extern-flag/no-nounused.rs stdout ----

5    | ^
6    |
6    |
7    = note: requested on the command line with `-D unused-crate-dependencies`
-    = help: remove unnecessary dependency `somedep`
10 error: aborting due to previous error
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/no-nounused/no-nounused.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args extern-flag/no-nounused.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern-flag/no-nounused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/no-nounused" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunstable-options" "-Dunused-crate-dependencies" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/no-nounused/auxiliary" "--extern" "somedep=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/no-nounused/auxiliary/libsomedep.so"
stdout: none
--- stderr -------------------------------
error: external crate `somedep` unused in `no_nounused`: remove the dependency or add `use somedep as _;`
  --> /checkout/src/test/ui/extern-flag/no-nounused.rs:5:1
   |
LL | fn main() { //~ ERROR external crate `somedep` unused in `no_nounused`
   |
   |
   = note: requested on the command line with `-D unused-crate-dependencies`
error: aborting due to previous error
------------------------------------------


