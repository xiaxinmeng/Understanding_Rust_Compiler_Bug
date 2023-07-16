plain
........................................................................................ 1848/13121
............................................................................i........... 1936/13121
........................................................................................ 2024/13121
........................................................................................ 2112/13121
.......................................F............F................................... 2200/13121
........................................................................................ 2376/13121
........................................................................................ 2464/13121
........................................................................................ 2552/13121
........................................................................................ 2640/13121
---
failures:

---- [ui] src/test/ui/const-generics/issue-73298.rs#full stdout ----

error in revision `full`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-73298.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-73298.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-73298.full/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/issue-73298.rs:4:27
   |
   |
LL | #![cfg_attr(full, feature(const_generics))]
   |
   |
   = note: removed in favor of `#![feature(adt_const_params)]` and `#![feature(generic_const_exprs)]`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0557`.
------------------------------------------
---
warning: the feature `min_const_generics` has been stable since 1.51.0 and no longer requires an attribute to enable
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> $DIR/issue-73298.rs:6:26
   |
LL | #![cfg_attr(min, feature(min_const_generics))]
   |
   = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted




The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-73298.min/issue-73298.min.stderr
To only update this specific test, also pass `--test-args const-generics/issue-73298.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issue-73298.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-73298.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issue-73298.min/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `min_const_generics` has been stable since 1.51.0 and no longer requires an attribute to enable
   |
   |
LL | #![cfg_attr(min, feature(min_const_generics))]
   |
   = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted
