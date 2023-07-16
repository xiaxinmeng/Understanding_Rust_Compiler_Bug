plain

running 13240 tests
........................................................................................ 88/13240
......................................................................iiiiiiiiiiiiii.... 176/13240
...............i.iF................ii...i............................................... 264/13240
........................................................................................ 440/13240
........................................................................................ 528/13240
........................................................................................ 616/13240
........................................................................................ 704/13240
---

---- [ui] src/test/ui/asm/issue-99071.rs stdout ----
diff of stderr:

+ '+atomics-32' is not a recognized feature for this target (ignoring feature)
+ '+atomics-32' is not a recognized feature for this target (ignoring feature)
1 error: cannot use register `r8`: high registers (r8+) can only be used as clobbers in Thumb-1 code
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99071/issue-99071.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/issue-99071.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/issue-99071.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99071" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv6m-none-eabi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/issue-99071/auxiliary"
stdout: none
--- stderr -------------------------------
'+atomics-32' is not a recognized feature for this target (ignoring feature)
'+atomics-32' is not a recognized feature for this target (ignoring feature)
error: cannot use register `r8`: high registers (r8+) can only be used as clobbers in Thumb-1 code
   |
   |
LL |         asm!("", in("r8") 0);

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-37131.rs stdout ----
diff of stderr:

+ '+atomics-32' is not a recognized feature for this target (ignoring feature)
+ '+atomics-32' is not a recognized feature for this target (ignoring feature)
2    |
2    |
3    = note: the `thumbv6m-none-eabi` target may not be installed

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131/issue-37131.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-37131.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37131.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=thumbv6m-none-eabi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131/auxiliary"
stdout: none
--- stderr -------------------------------
'+atomics-32' is not a recognized feature for this target (ignoring feature)
'+atomics-32' is not a recognized feature for this target (ignoring feature)
   |
   |
   = note: the `thumbv6m-none-eabi` target may not be installed
   = help: consider downloading the target with `rustup target add thumbv6m-none-eabi`
   = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: requires `sized` lang_item

error: aborting due to 2 previous errors

