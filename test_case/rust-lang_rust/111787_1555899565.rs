plain
........................................................................................   440/15027
........................................................................................   528/15027
........................................................................................   616/15027
........................................................................................   704/15027
.......................................F.F..............................................   792/15027
........................................................................................   968/15027
....................................i...................................................  1056/15027
........................................................i...............................  1144/15027
........................................................................................  1232/15027
---

---- [ui] tests/ui/async-await/future-sizes/large-arg.rs stdout ----
diff of stdout:

53 print-type-size     variant `MaybeUninit`: 1025 bytes
54 print-type-size         field `.uninit`: 0 bytes
55 print-type-size         field `.value`: 1025 bytes
- print-type-size type: `std::task::Poll<[u8; 1024]>`: 1025 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Ready`: 1024 bytes
- print-type-size         field `.0`: 1024 bytes
- print-type-size     variant `Pending`: 0 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/large-arg/large-arg.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/large-arg/large-arg.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/future-sizes/large-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/future-sizes/large-arg.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/large-arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/large-arg/auxiliary" "-Z" "print-type-sizes" "--crate-type=lib" "--edition=2021"
--- stdout -------------------------------
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:6:21: 8:2]`: 3076 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Suspend0`: 3075 bytes
print-type-size         local `.__awaitee`: 3075 bytes
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:10:30: 12:2]`: 3075 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size     variant `Suspend0`: 3074 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size         local `.__awaitee`: 2050 bytes
print-type-size     variant `Returned`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size     variant `Panicked`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:10:30: 12:2]>`: 3075 bytes, alignment: 1 bytes
print-type-size     field `.value`: 3075 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:10:30: 12:2]>`: 3075 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 3075 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 3075 bytes
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:13:26: 15:2]`: 2050 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size     variant `Suspend0`: 2049 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size         local `.__awaitee`: 1025 bytes
print-type-size     variant `Returned`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size     variant `Panicked`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:13:26: 15:2]>`: 2050 bytes, alignment: 1 bytes
print-type-size     field `.value`: 2050 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:13:26: 15:2]>`: 2050 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 2050 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 2050 bytes
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:16:26: 18:2]`: 1025 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size     variant `Returned`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size     variant `Panicked`: 1024 bytes
print-type-size         upvar `.t`: 1024 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:16:26: 18:2]>`: 1025 bytes, alignment: 1 bytes
print-type-size     field `.value`: 1025 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/async-await/future-sizes/large-arg.rs:16:26: 18:2]>`: 1025 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 1025 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 1025 bytes
stderr: none


---- [ui] tests/ui/async-await/future-sizes/async-awaiting-fut.rs stdout ----
---- [ui] tests/ui/async-await/future-sizes/async-awaiting-fut.rs stdout ----
diff of stdout:

61 print-type-size     variant `MaybeUninit`: 1 bytes
62 print-type-size         field `.uninit`: 0 bytes
63 print-type-size         field `.value`: 1 bytes
- print-type-size type: `std::task::Poll<()>`: 1 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Ready`: 0 bytes
- print-type-size         field `.0`: 0 bytes
- print-type-size     variant `Pending`: 0 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/async-awaiting-fut/async-awaiting-fut.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/async-awaiting-fut/async-awaiting-fut.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/future-sizes/async-awaiting-fut.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/async-await/future-sizes/async-awaiting-fut.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/async-awaiting-fut" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/future-sizes/async-awaiting-fut/auxiliary" "-Z" "print-type-sizes" "--crate-type" "lib" "--edition=2021"
--- stdout -------------------------------
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:21:21: 24:2]`: 3078 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Suspend0`: 3077 bytes
print-type-size         local `.__awaitee`: 3077 bytes
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:10:64: 19:2]`: 3077 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 1025 bytes
print-type-size         upvar `.fut`: 1025 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size     variant `Suspend0`: 2052 bytes
print-type-size         upvar `.fut`: 1025 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size         padding: 1 bytes
print-type-size         local `.fut`: 1025 bytes, alignment: 1 bytes
print-type-size         local `..generator_field4`: 1 bytes
print-type-size         local `.__awaitee`: 1 bytes
print-type-size     variant `Suspend1`: 3076 bytes
print-type-size         upvar `.fut`: 1025 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size         padding: 1026 bytes
print-type-size         local `..generator_field4`: 1 bytes, alignment: 1 bytes
print-type-size         local `.__awaitee`: 1025 bytes
print-type-size     variant `Suspend2`: 2052 bytes
print-type-size         upvar `.fut`: 1025 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size         padding: 1 bytes
print-type-size         local `.fut`: 1025 bytes, alignment: 1 bytes
print-type-size         local `..generator_field4`: 1 bytes
print-type-size         local `.__awaitee`: 1 bytes
print-type-size     variant `Returned`: 1025 bytes
print-type-size         upvar `.fut`: 1025 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size     variant `Panicked`: 1025 bytes
print-type-size         upvar `.fut`: 1025 bytes, offset: 0 bytes, alignment: 1 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:10:64: 19:2]>`: 3077 bytes, alignment: 1 bytes
print-type-size     field `.value`: 3077 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:10:64: 19:2]>`: 3077 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 3077 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 3077 bytes
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:8:35: 8:37]`: 1025 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 1024 bytes
print-type-size         upvar `.arg`: 1024 bytes
print-type-size     variant `Returned`: 1024 bytes
print-type-size         upvar `.arg`: 1024 bytes
print-type-size     variant `Panicked`: 1024 bytes
print-type-size         upvar `.arg`: 1024 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:8:35: 8:37]>`: 1025 bytes, alignment: 1 bytes
print-type-size     field `.value`: 1025 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:8:35: 8:37]>`: 1025 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 1025 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 1025 bytes
print-type-size type: `[async fn body@fake-test-src-base/async-await/future-sizes/async-awaiting-fut.rs:6:17: 6:19]`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size type: `std::mem::ManuallyDrop<bool>`: 1 bytes, alignment: 1 bytes
print-type-size     field `.value`: 1 bytes
print-type-size type: `std::mem::MaybeUninit<bool>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 1 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 1 bytes
stderr: none


---- [ui] tests/ui/print_type_sizes/async.rs stdout ----
---- [ui] tests/ui/print_type_sizes/async.rs stdout ----
diff of stdout:

27 print-type-size     variant `MaybeUninit`: 1 bytes
28 print-type-size         field `.uninit`: 0 bytes
29 print-type-size         field `.value`: 1 bytes
- print-type-size type: `std::task::Poll<()>`: 1 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Ready`: 0 bytes
- print-type-size         field `.0`: 0 bytes
- print-type-size     variant `Pending`: 0 bytes


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/async/async.stdout
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/async/async.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args print_type_sizes/async.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/print_type_sizes/async.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/print_type_sizes/async/auxiliary" "-Z" "print-type-sizes" "--crate-type" "lib" "--edition=2021"
--- stdout -------------------------------
print-type-size type: `[async fn body@fake-test-src-base/print_type_sizes/async.rs:10:36: 13:2]`: 16386 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 8192 bytes
print-type-size         upvar `.arg`: 8192 bytes
print-type-size     variant `Suspend0`: 16385 bytes
print-type-size         upvar `.arg`: 8192 bytes
print-type-size         local `.arg`: 8192 bytes
print-type-size         local `.__awaitee`: 1 bytes
print-type-size     variant `Returned`: 8192 bytes
print-type-size         upvar `.arg`: 8192 bytes
print-type-size     variant `Panicked`: 8192 bytes
print-type-size         upvar `.arg`: 8192 bytes
print-type-size type: `std::mem::ManuallyDrop<[u8; 8192]>`: 8192 bytes, alignment: 1 bytes
print-type-size     field `.value`: 8192 bytes
print-type-size type: `std::mem::MaybeUninit<[u8; 8192]>`: 8192 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 8192 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 8192 bytes
print-type-size type: `[async fn body@fake-test-src-base/print_type_sizes/async.rs:8:17: 8:19]`: 1 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Unresumed`: 0 bytes
print-type-size     variant `Returned`: 0 bytes
print-type-size     variant `Panicked`: 0 bytes
print-type-size type: `std::mem::ManuallyDrop<[async fn body@fake-test-src-base/print_type_sizes/async.rs:8:17: 8:19]>`: 1 bytes, alignment: 1 bytes
print-type-size     field `.value`: 1 bytes
print-type-size type: `std::mem::MaybeUninit<[async fn body@fake-test-src-base/print_type_sizes/async.rs:8:17: 8:19]>`: 1 bytes, alignment: 1 bytes
print-type-size     variant `MaybeUninit`: 1 bytes
print-type-size         field `.uninit`: 0 bytes
print-type-size         field `.value`: 1 bytes
stderr: none



