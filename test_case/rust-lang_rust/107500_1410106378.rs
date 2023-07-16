plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---

---- [ui] tests/ui/future-sizes/large-arg.rs stdout ----
diff of stdout:

- print-type-size type: `[async fn body@$DIR/large-arg.rs:5:21: 7:2]`: 3076 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Suspend0`: 3075 bytes
- print-type-size         field `.__awaitee`: 3075 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Unresumed`: 0 bytes
- print-type-size     variant `Returned`: 0 bytes
- print-type-size     variant `Panicked`: 0 bytes
- print-type-size type: `[async fn body@$DIR/large-arg.rs:9:30: 11:2]`: 3075 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Suspend0`: 3074 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size         field `.__awaitee`: 2050 bytes
- print-type-size     variant `Unresumed`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Returned`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Panicked`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size type: `std::mem::ManuallyDrop<[async fn body@$DIR/large-arg.rs:9:30: 11:2]>`: 3075 bytes, alignment: 1 bytes
- print-type-size     field `.value`: 3075 bytes
- print-type-size type: `std::mem::MaybeUninit<[async fn body@$DIR/large-arg.rs:9:30: 11:2]>`: 3075 bytes, alignment: 1 bytes
- print-type-size     variant `MaybeUninit`: 3075 bytes
- print-type-size         field `.uninit`: 0 bytes
- print-type-size         field `.value`: 3075 bytes
- print-type-size type: `[async fn body@$DIR/large-arg.rs:12:26: 14:2]`: 2050 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Suspend0`: 2049 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size         field `.__awaitee`: 1025 bytes
- print-type-size     variant `Unresumed`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Returned`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Panicked`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size type: `std::mem::ManuallyDrop<[async fn body@$DIR/large-arg.rs:12:26: 14:2]>`: 2050 bytes, alignment: 1 bytes
- print-type-size     field `.value`: 2050 bytes
- print-type-size type: `std::mem::MaybeUninit<[async fn body@$DIR/large-arg.rs:12:26: 14:2]>`: 2050 bytes, alignment: 1 bytes
- print-type-size     variant `MaybeUninit`: 2050 bytes
- print-type-size         field `.uninit`: 0 bytes
- print-type-size         field `.value`: 2050 bytes
- print-type-size type: `[async fn body@$DIR/large-arg.rs:15:26: 17:2]`: 1025 bytes, alignment: 1 bytes
- print-type-size     discriminant: 1 bytes
- print-type-size     variant `Unresumed`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Returned`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size     variant `Panicked`: 1024 bytes
- print-type-size         field `.t`: 1024 bytes, offset: 0 bytes, alignment: 1 bytes
- print-type-size type: `std::mem::ManuallyDrop<[async fn body@$DIR/large-arg.rs:15:26: 17:2]>`: 1025 bytes, alignment: 1 bytes
- print-type-size     field `.value`: 1025 bytes
- print-type-size type: `std::mem::MaybeUninit<[async fn body@$DIR/large-arg.rs:15:26: 17:2]>`: 1025 bytes, alignment: 1 bytes
- print-type-size     variant `MaybeUninit`: 1025 bytes
- print-type-size         field `.uninit`: 0 bytes
- print-type-size         field `.value`: 1025 bytes
56 print-type-size type: `std::task::Poll<[u8; 1024]>`: 1025 bytes, alignment: 1 bytes
57 print-type-size     discriminant: 1 bytes
58 print-type-size     variant `Ready`: 1024 bytes

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/future-sizes/large-arg/large-arg.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args future-sizes/large-arg.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/future-sizes/large-arg.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/future-sizes/large-arg" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/future-sizes/large-arg/auxiliary" "-Z" "print-type-sizes" "--crate-type=lib" "--edition=2021"
--- stdout -------------------------------
print-type-size type: `std::task::Poll<[u8; 1024]>`: 1025 bytes, alignment: 1 bytes
print-type-size     discriminant: 1 bytes
print-type-size     variant `Ready`: 1024 bytes
print-type-size         field `.0`: 1024 bytes
print-type-size     variant `Pending`: 0 bytes
stderr: none



