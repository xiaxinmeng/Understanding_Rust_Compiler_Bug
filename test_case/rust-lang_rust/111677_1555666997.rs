plain
To only update this specific test, also pass `--test-args consts/const-eval/heap/dealloc_intrinsic_incorrect_layout.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/heap/dealloc_intrinsic_incorrect_layout.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/dealloc_intrinsic_incorrect_layout" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/dealloc_intrinsic_incorrect_layout/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/heap/dealloc_intrinsic_incorrect_layout.rs:8:5
   |
LL |     intrinsics::const_deallocate(ptr, 4, 2);
---

error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/heap/dealloc_intrinsic_incorrect_layout.rs:25:5
   |
LL |     intrinsics::const_deallocate(ptr, 4, 3);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid align passed to `const_deallocate`: 3 is not a power of 2
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
