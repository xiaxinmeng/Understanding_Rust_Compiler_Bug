plain
To only update this specific test, also pass `--test-args manually_drop_attr/manually_drop-bad-item.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/manually_drop_attr/manually_drop-bad-item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/manually_drop_attr/manually_drop-bad-item" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/manually_drop_attr/manually_drop-bad-item/auxiliary"
stdout: none
--- stderr -------------------------------
error: attribute should be applied to a struct or enum
   |
LL | #[manually_drop]
   | ^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^
LL | //~^ ERROR attribute should be applied to a struct or enum
LL | fn foo() {}
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/manually_drop_attr/manually_drop-bad-item.rs:2:11
   |
   |
LL | #![forbid(unused_attributes)]

error: attribute should be applied to a struct or enum
  --> /checkout/src/test/ui/manually_drop_attr/manually_drop-bad-item.rs:3:1
   |
