plain
failures:

---- [ui] src/test/ui/lint/must_not_suspend/ref-drop-tracking.rs stdout ----
normalized stderr:
error: reference to `Umm` held across a suspend point, but should not be
   |
LL |         let guard = &mut self.u;
   |             ^^^^^
LL |
LL |
LL |         other().await;
   |                ------ the value is held across this suspend point
note: the lint level is defined here
  --> $DIR/ref-drop-tracking.rs:4:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
LL |         let guard = &mut self.u;
   |             ^^^^^
   |             ^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
LL |         let guard = &mut self.u;
   |             ^^^^^

---
To only update this specific test, also pass `--test-args lint/must_not_suspend/ref-drop-tracking.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/ref-drop-tracking.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/ref-drop-tracking" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Zdrop-tracking" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/ref-drop-tracking/auxiliary"
stdout: none
--- stderr -------------------------------
error: reference to `Umm` held across a suspend point, but should not be
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across
LL |
LL |         other().await;
LL |         other().await;
   |                ------ the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/ref-drop-tracking.rs:4:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across
   |             ^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across

error: aborting due to previous error
------------------------------------------

