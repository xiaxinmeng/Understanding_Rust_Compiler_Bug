plain
---- [ui] src/test/ui-fulldeps/pprust-expr-roundtrip.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0063]: missing field `b` in initializer of `rustc_ast::Expr`
   |
   |
LL |     P(Expr { id: DUMMY_NODE_ID, kind, span: DUMMY_SP, attrs: ThinVec::new(), tokens: None })
   |       ^^^^ missing `b`
error[E0063]: missing field `b` in initializer of `rustc_ast::Expr`
  --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:195:15
   |
   |
LL |             P(Expr {
   |               ^^^^ missing `b`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0063`.
------------------------------------------
