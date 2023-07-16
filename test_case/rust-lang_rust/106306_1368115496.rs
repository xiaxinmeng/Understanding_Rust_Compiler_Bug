plain

---- [ui] src/test/ui/crate-loading/crateresolve1.rs stdout ----
diff of stderr:

- error[E0464]: multiple candidates for `dylib` dependency `crateresolve1` found
+ error[E0464]: multiple candidates for `rlib` dependency `crateresolve1` found
3    |
4 LL | extern crate crateresolve1;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/crateresolve1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve1.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0464]: multiple candidates for `rlib` dependency `crateresolve1` found
   |
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.rlib
   = note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.rlib
   = note: candidate #3: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.rlib
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0464.rs stdout ----
diff of stderr:

- error[E0464]: multiple candidates for `dylib` dependency `crateresolve1` found
+ error[E0464]: multiple candidates for `rlib` dependency `crateresolve1` found
2   --> $DIR/E0464.rs:11:1
4 LL | extern crate crateresolve1;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/E0464.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0464.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0464.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0464]: multiple candidates for `rlib` dependency `crateresolve1` found
   |
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: candidate #1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary/libcrateresolve1-1.rlib
   = note: candidate #2: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary/libcrateresolve1-2.rlib
   = note: candidate #3: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary/libcrateresolve1-3.rlib
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.
------------------------------------------
