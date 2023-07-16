plain
---- [ui] src/test/ui-fulldeps/pprust-expr-roundtrip.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:116:23
   |
   |
LL |                     g(ExprKind::Closure(
LL |                         CaptureBy::Value,
   |                         ---------------- an argument of type `ClosureBinder` is missing
   |
note: tuple variant defined here
note: tuple variant defined here
  --> /checkout/compiler/rustc_ast/src/ast.rs:1393:5
   |
LL |     Closure(ClosureBinder, CaptureBy, Async, Movability, P<FnDecl>, P<Expr>, Span),
help: provide the argument
   |
   |
LL |                     g(ExprKind::Closure(/* ClosureBinder */, CaptureBy::Value, Async::No, Movability::Movable, decl.clone(), e, DUMMY_SP))

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
