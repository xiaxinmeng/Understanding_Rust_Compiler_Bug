plain
..............................................i.........................................  7480/15022
.................................i......................................................  7568/15022
........................................................................................  7656/15022
........................................................................................  7744/15022
....................................i....i..iii..........F.F............................  7832/15022
..........................i....................................iiiiiiiii................  8008/15022
...................................................i....................................  8096/15022
........................................................................................  8184/15022
.............................................i..........................................  8272/15022
---

---- [ui] tests/ui/limits/issue-15919-32.rs stdout ----
diff of stderr:

- error: values of the type `[usize; usize::MAX]` are too big for the current architecture
+ error: failed to get layout for [usize; usize::MAX]: values of the type `[usize; usize::MAX]` are too big for the current architecture
3    |
3    |
4 LL |     let x = [0usize; 0xffff_ffff];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-15919-32/issue-15919-32.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args limits/issue-15919-32.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/limits/issue-15919-32.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-15919-32" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-15919-32/auxiliary"
stdout: none
--- stderr -------------------------------
error: failed to get layout for [usize; usize::MAX]: values of the type `[usize; usize::MAX]` are too big for the current architecture
  --> fake-test-src-base/limits/issue-15919-32.rs:9:9
   |
LL |     let x = [0usize; 0xffff_ffff]; //~ ERROR too big

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/limits/huge-array-simple-32.rs stdout ----
diff of stderr:

- error: values of the type `[u8; 2147516416]` are too big for the current architecture
+ error: failed to get layout for [u8; 2147516416]: values of the type `[u8; 2147516416]` are too big for the current architecture
2   --> $DIR/huge-array-simple-32.rs:10:9
3    |
4 LL |     let _fat: [u8; (1<<31)+(1<<15)] =

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/huge-array-simple-32/huge-array-simple-32.stderr
To only update this specific test, also pass `--test-args limits/huge-array-simple-32.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/limits/huge-array-simple-32.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/huge-array-simple-32" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/huge-array-simple-32/auxiliary"
stdout: none
--- stderr -------------------------------
error: failed to get layout for [u8; 2147516416]: values of the type `[u8; 2147516416]` are too big for the current architecture
  --> fake-test-src-base/limits/huge-array-simple-32.rs:10:9
   |
LL |     let _fat: [u8; (1<<31)+(1<<15)] = //~ ERROR too big for the current architecture

error: aborting due to previous error
------------------------------------------

