plain
normalized stderr:
warning: trailing irrefutable pattern in let chain
  --> $DIR/mir_let_chains_drop_order.rs:71:28
   |
LL |                         && let DropLogger { .. } = d(14, None)
   |
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: this pattern will always match
   = help: consider moving it into the body
   = help: consider moving it into the body

warning: irrefutable `if let` patterns
   |
   |
LL |             if let DropLogger { .. } = d(18, None) && let DropLogger { .. } = d(19, None) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
   = note: these patterns will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`
warning: trailing irrefutable pattern in let chain
  --> $DIR/mir_let_chains_drop_order.rs:42:64
   |
   |
LL |                     if let Some(_) = d(2, Some(true)).extra && let DropLogger { .. } = d(3, None) {
   |
   = note: this pattern will always match
   = help: consider moving it into the body


warning: irrefutable `if let` patterns
   |
   |
LL |             if let DropLogger { .. } = d(7, None) && let DropLogger { .. } = d(8, None) {
   |
   |
   = note: these patterns will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`
warning: 4 warnings emitted



---
To only update this specific test, also pass `--test-args mir/mir_let_chains_drop_order.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/mir_let_chains_drop_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/auxiliary"
stdout: none
--- stderr -------------------------------
warning: trailing irrefutable pattern in let chain
   |
   |
LL |                         && let DropLogger { .. } = d(14, None)
   |
   = note: `#[warn(irrefutable_let_patterns)]` on by default
   = note: this pattern will always match
   = help: consider moving it into the body
   = help: consider moving it into the body

warning: irrefutable `if let` patterns
   |
   |
LL |             if let DropLogger { .. } = d(18, None) && let DropLogger { .. } = d(19, None) {
   |
   |
   = note: these patterns will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`
warning: trailing irrefutable pattern in let chain
  --> /checkout/src/test/ui/mir/mir_let_chains_drop_order.rs:42:64
   |
   |
LL |                     if let Some(_) = d(2, Some(true)).extra && let DropLogger { .. } = d(3, None) {
   |
   = note: this pattern will always match
   = help: consider moving it into the body


warning: irrefutable `if let` patterns
   |
   |
LL |             if let DropLogger { .. } = d(7, None) && let DropLogger { .. } = d(8, None) {
   |
   |
   = note: these patterns will always match, so the `if let` is useless
   = help: consider replacing the `if let` with a `let`
warning: 4 warnings emitted
------------------------------------------


