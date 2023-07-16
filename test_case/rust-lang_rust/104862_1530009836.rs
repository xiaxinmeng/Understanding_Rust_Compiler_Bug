plain
..........................................................................i.............  1760/14893
........................................................................................  1848/14893
........................................................................................  1936/14893
........................................................................................  2024/14893
.........................F......i......F..............i...........ii....................  2112/14893
........................................................................................  2288/14893
.............................................i..........................................  2376/14893
........................................................................................  2464/14893
........................................................................................  2552/14893
---
---- [ui] tests/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-registers/auxiliary" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib"
stdout: none
error: requires `panic_bounds_check` lang_item
  --> fake-test-src-base/cmse-nonsecure/cmse-nonsecure-call/params-on-registers.rs:19:9
   |
   |
LL | /         transmute::<usize, extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32) -> u32>(
LL | |             0x10000004,
LL | |         )

error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs stdout ----
diff of stderr:

- error: <unknown>:0:0: in function test i32 (i32, i32, i32, i32, i32): call to non-secure function would require passing arguments on stack
+ error: requires `panic_bounds_check` lang_item
+    |
+ LL | /         transmute::<
+ LL | |             usize,
+ LL | |             usize,
+ LL | |             extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32, u32) -> u32>
+ LL | |         (
+ LL | |             0x10000004,
+ LL | |         )
2 
3 error: aborting due to previous error
4 

---
To only update this specific test, also pass `--test-args cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/params-on-stack/auxiliary" "--target" "thumbv8m.main-none-eabi" "--crate-type" "lib"
stdout: none
error: requires `panic_bounds_check` lang_item
  --> fake-test-src-base/cmse-nonsecure/cmse-nonsecure-call/params-on-stack.rs:19:9
   |
LL | /         transmute::<
LL | /         transmute::<
LL | |             usize,
LL | |             extern "C-cmse-nonsecure-call" fn(u32, u32, u32, u32, u32) -> u32>
LL | |         (
LL | |             0x10000004,
LL | |         )

error: aborting due to previous error
------------------------------------------

