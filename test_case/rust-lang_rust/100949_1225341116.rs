plain
---- [ui] src/test/ui-fulldeps/pprust-expr-roundtrip.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/pprust-expr-roundtrip/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/pprust-expr-roundtrip.rs:54:33
   |
   |
LL |     let path = Path { segments: vec![seg], span: DUMMY_SP, tokens: None };
   |                                 ^^^^^^^^^ expected struct `thin_vec::ThinVec`, found struct `Vec`
   = note: expected struct `thin_vec::ThinVec<rustc_ast::PathSegment>`
              found struct `Vec<rustc_ast::PathSegment>`
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

