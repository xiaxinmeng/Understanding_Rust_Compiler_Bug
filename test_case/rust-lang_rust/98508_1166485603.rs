plain
........................................................................................ 13112/13115
...
failures:

---- [ui] src/test/ui/const-generics/recur-walk.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/recur-walk.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/recur-walk" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/recur-walk/auxiliary"
stdout: none
--- stderr -------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/recur-walk.rs:13:5
   |
LL |     foo::<LEN>([123]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + 1]:`
note: required by a bound in `foo`
  --> /checkout/src/test/ui/const-generics/recur-walk.rs:6:46
   |
LL | fn foo<const A: usize>(a: [u32; A]) -> [u32; A + 1] {
   |                                              ^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/recur-walk.rs:13:5
   |
   |
LL |     foo::<LEN>([123]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + 1]:`
note: required by a bound in `foo`
  --> /checkout/src/test/ui/const-generics/recur-walk.rs:6:46
   |
LL | fn foo<const A: usize>(a: [u32; A]) -> [u32; A + 1] {
   |                                              ^^^^^ required by this bound in `foo`
error: aborting due to 2 previous errors
------------------------------------------


