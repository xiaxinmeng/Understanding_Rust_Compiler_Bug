plain
To only update this specific test, also pass `--test-args let-else/let-else-check.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/let-else/let-else-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-check" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-check/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unused variable: `x`
  --> /checkout/src/test/ui/let-else/let-else-check.rs:13:9
   |
LL |     let x = 1; //~ ERROR unused variable: `x`
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
note: the lint level is defined here
  --> /checkout/src/test/ui/let-else/let-else-check.rs:1:9
   |
LL | #![deny(unused_variables)]
---
normalized stderr:
warning: unused variable: `x`
  --> $DIR/let-else.rs:4:14
   |
LL |     let Some(x) = Some(1) else {
   |              ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args let-else/let-else.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/let-else/let-else.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unused variable: `x`
  --> /checkout/src/test/ui/let-else/let-else.rs:4:14
   |
LL |     let Some(x) = Some(1) else {
   |              ^ help: if this is intentional, prefix it with an underscore: `_x`
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args pattern/usefulness/top-level-alternation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/top-level-alternation" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/top-level-alternation/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:6:23
   |
LL |     while let 0..=2 | 1 = 0 {} //~ ERROR unreachable pattern
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:1:9
   |
   |
LL | #![deny(unreachable_patterns)]
   |         ^^^^^^^^^^^^^^^^^^^^

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:7:20
   |
LL |     if let 0..=2 | 1 = 0 {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:11:15
   |
   |
LL |             | 0 => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:16:15
   |
   |
LL |             | Some(0) => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:21:9
   |
   |
LL |         (0, 0) => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:41:9
   |
   |
LL |         _ => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:45:9
   |
   |
LL |         Some(_) => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:46:9
   |
   |
LL |         None => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:51:9
   |
   |
LL |         None | Some(_) => {} //~ ERROR unreachable pattern

error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:55:9
   |
   |
LL |         1..=2 => {}, //~ ERROR unreachable pattern

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: unreachable pattern
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:58:14
  --> /checkout/src/test/ui/pattern/usefulness/top-level-alternation.rs:58:14
   |
LL |     let (0 | 0) = 0 else { return }; //~ ERROR unreachable pattern

error: aborting due to 11 previous errors


