plain

error[E0412]: cannot find type `Ty` in this scope
 --> src/pass_by_value.rs:19:31
  |
5 | fn ty_pass_by_ref<'a>(ty: &'a Ty<'a>) -> Option<&'a Ty<'a>> {
  |
help: consider importing one of these items
  |
2 | use rustc_ast::Ty;
---

error[E0412]: cannot find type `Ty` in this scope
 --> src/pass_by_value.rs:19:53
  |
5 | fn ty_pass_by_ref<'a>(ty: &'a Ty<'a>) -> Option<&'a Ty<'a>> {
  |
help: consider importing one of these items
  |
2 | use rustc_ast::Ty;
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc_lint" "--" "--quiet"


Build completed unsuccessfully in 0:24:49
