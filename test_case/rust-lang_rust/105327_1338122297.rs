plain
---- [ui] src/test/ui/impl-trait/in-trait/where-clause.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/in-trait/where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/where-clause" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/where-clause/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
error: `impl` item return type captures lifetime that doesn't appear in `trait` item bounds
   |
   |
LL |     fn foo<'a>(&'a self) -> impl Debug
   |                             ---------- type declared not to capture lifetimes
...
LL |     fn foo<'a>(&'a self) -> impl Debug
   |            |
   |            |
   |            type `impl Debug` captures lifetime `'a` defined here
error: aborting due to previous error
------------------------------------------


