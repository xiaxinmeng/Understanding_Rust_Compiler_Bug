plain
failures:

---- [ui] src/test/ui/rfc-2294-if-let-guard/run-pass.rs stdout ----
normalized stderr:
warning: irrefutable `if let` guard pattern
   |
   |
LL |         () | () if let x = 0 => {
   |
   |
   = note: this pattern will always match, so the guard is useless
   = help: consider removing the guard and adding a `let` inside the match arm
   = note: `#[warn(irrefutable_let_patterns)]` on by default
warning: path statement with no effect
  --> $DIR/run-pass.rs:35:13
   |
LL |             x;
---
To only update this specific test, also pass `--test-args rfc-2294-if-let-guard/run-pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2294-if-let-guard/run-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/run-pass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2294-if-let-guard/run-pass/auxiliary"
stdout: none
--- stderr -------------------------------
warning: irrefutable `if let` guard pattern
   |
   |
LL |         () | () if let x = 0 => {
   |
   |
   = note: this pattern will always match, so the guard is useless
   = help: consider removing the guard and adding a `let` inside the match arm
   = note: `#[warn(irrefutable_let_patterns)]` on by default
warning: path statement with no effect
  --> /checkout/src/test/ui/rfc-2294-if-let-guard/run-pass.rs:35:13
   |
LL |             x;
