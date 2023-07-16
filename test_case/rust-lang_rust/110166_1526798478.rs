plain
-    = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/70861>
+    = note: for more information, see issue #70861 <https://github.com/rust-lang/rust/issues/70861>
9    = note: `#[warn(pointer_structural_match)]` on by default
10 
11 warning: function pointers and unsized pointers in patterns behave unpredictably and should not be relied upon. See https://github.com/rust-lang/rust/issues/70861 for details.
15    |         ^^^
16    |
17    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #62411 <https://github.com/rust-lang/rust/issues/70861>
---
To only update this specific test, also pass `--test-args consts/const_in_pattern/issue-44333.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const_in_pattern/issue-44333.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-44333/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_in_pattern/issue-44333/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function pointers and unsized pointers in patterns behave unpredictably and should not be relied upon. See https://github.com/rust-lang/rust/issues/70861 for details.
  --> fake-test-src-base/consts/const_in_pattern/issue-44333.rs:19:9
   |
LL |         FOO => println!("foo"), //~ WARN pointers in patterns behave unpredictably
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #70861 <https://github.com/rust-lang/rust/issues/70861>
   = note: `#[warn(pointer_structural_match)]` on by default
   = note: `#[warn(pointer_structural_match)]` on by default

warning: function pointers and unsized pointers in patterns behave unpredictably and should not be relied upon. See https://github.com/rust-lang/rust/issues/70861 for details.
  --> fake-test-src-base/consts/const_in_pattern/issue-44333.rs:21:9
   |
LL |         BAR => println!("bar"), //~ WARN pointers in patterns behave unpredictably
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #70861 <https://github.com/rust-lang/rust/issues/70861>

---
To only update this specific test, also pass `--test-args rfc-1445-restrict-constants-in-patterns/issue-63479-match-fnptr.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfc-1445-restrict-constants-in-patterns/issue-63479-match-fnptr.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/issue-63479-match-fnptr/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1445-restrict-constants-in-patterns/issue-63479-match-fnptr/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function pointers and unsized pointers in patterns behave unpredictably and should not be relied upon. See https://github.com/rust-lang/rust/issues/70861 for details.
  --> fake-test-src-base/rfc-1445-restrict-constants-in-patterns/issue-63479-match-fnptr.rs:35:7
   |
LL |     B(TEST) => println!("matched"),
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #70861 <https://github.com/rust-lang/rust/issues/70861>
   = note: `#[warn(pointer_structural_match)]` on by default
