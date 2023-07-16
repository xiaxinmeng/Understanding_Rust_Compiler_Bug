plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/core-no-fp-fmt-parse/core-no-fp-fmt-parse  --edition=2018 --crate-type=rlib ../../../../library/core/src/lib.rs --cfg no_fp_fmt_parse
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error[E0501]: cannot borrow `self.iter` as mutable because previous closure requires unique access
  --> ../../../../library/core/src/iter/adapters/reject.rs:50:9
   |
50 |         self.iter.find(|v| !(self.predicate)(v))
   |         ^^^^^^^^^^----^---^^----------------^^^^
   |         |         |    |    |
   |         |         |    |    first borrow occurs due to use of `self` in closure
   |         |         |    closure construction occurs here
   |         |         first borrow later used by call
   |         second borrow occurs here

error[E0500]: closure requires unique access to `self` but it is already borrowed
  --> ../../../../library/core/src/iter/adapters/reject.rs:50:24
   |
50 |         self.iter.find(|v| !(self.predicate)(v))
   |         ---------------^^^----------------------
   |         |         |    |    |
   |         |         |    |    second borrow occurs due to use of `self` in closure
   |         |         |    closure construction occurs here
   |         |         first borrow later used by call
   |         borrow occurs here

error[E0501]: cannot borrow `self.iter` as mutable because previous closure requires unique access
   --> ../../../../library/core/src/iter/adapters/reject.rs:106:9
    |
106 |         self.iter.rfind(|v| !(self.predicate)(v))
    |         ^^^^^^^^^^-----^---^^----------------^^^^
    |         |         |     |    |
    |         |         |     |    first borrow occurs due to use of `self` in closure
    |         |         |     closure construction occurs here
    |         |         first borrow later used by call
    |         second borrow occurs here

error[E0500]: closure requires unique access to `self` but it is already borrowed
   --> ../../../../library/core/src/iter/adapters/reject.rs:106:25
    |
106 |         self.iter.rfind(|v| !(self.predicate)(v))
    |         ----------------^^^----------------------
    |         |         |     |    |
    |         |         |     |    second borrow occurs due to use of `self` in closure
    |         |         |     closure construction occurs here
    |         |         first borrow later used by call
    |         borrow occurs here
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0500, E0501.
For more information about an error, try `rustc --explain E0500`.
For more information about an error, try `rustc --explain E0500`.
make: *** [Makefile:4: all] Error 1
------------------------------------------



