plain
      Memory: 14 GB
      System Firmware Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMpJpCYqo9KS
      Provisioning UDID: 4203018E-580F-C1B5-9525-B745CECA79EB

hw.ncpu: 3
hw.byteorder: 1234
---
failures:

---- [incremental] src/test/incremental/type_alias_cross_crate/b.rs stdout ----

error in revision `rpass3`: compilation failed!
status: exit status: 1
command: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/runner/work/rust/rust/src/test/incremental/type_alias_cross_crate/b.rs" "-Zthreads=1" "--target=x86_64-apple-darwin" "--cfg" "rpass3" "-C" "incremental=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/incremental/type_alias_cross_crate/b/b.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/incremental/type_alias_cross_crate/b/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/Users/runner/work/rust/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/test/incremental/type_alias_cross_crate/b/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unused variable: `x`
   |
   |
LL |     let x: a::Y = 'c';
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default


warning: function `use_X` should have a snake case name
   |
   |
LL | pub fn use_X() -> u32 {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_x`
   = note: `#[warn(non_snake_case)]` on by default


warning: function `use_Y` should have a snake case name
   |
   |
LL | pub fn use_Y() {
   |        ^^^^^ help: convert the identifier to snake case (notice the capitalization): `use_y`

error: `typeck(use_X)` should be clean but is not
   |
   |
LL | pub fn use_X() -> u32 {

error: aborting due to previous error; 3 warnings emitted
------------------------------------------

