plain
...................................................iii.................................. 12848/12918
......................................................................
failures:

---- [ui] src/test/ui/generator/clone-impl.rs stdout ----

133    |
133    |
134 LL | fn check_copy<T: Copy>(_x: &T) {}
135    |                  ^^^^ required by this bound in `check_copy`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ help: consider annotating `NonClone` with `#[derive(Copy)]`
+    |
+ LL | #[derive(Copy)]
136 
136 
137 error[E0277]: the trait bound `NonClone: Clone` is not satisfied in `[generator@$DIR/clone-impl.rs:62:25: 65:6]`
138   --> $DIR/clone-impl.rs:68:5
157    |
157    |
158 LL | fn check_clone<T: Clone>(_x: &T) {}
159    |                   ^^^^^ required by this bound in `check_clone`
+ help: consider annotating `NonClone` with `#[derive(Clone)]`
+ LL | #[derive(Clone)]
+    |
160 
161 error: aborting due to 6 previous errors
---
To only update this specific test, also pass `--test-args generator/clone-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/clone-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 41:6]`
  --> /checkout/src/test/ui/generator/clone-impl.rs:42:5
   |
LL |       let gen_clone_0 = move || {
   |  _______________________-
LL | |         let v = vec!['a'];
LL | |         yield;
LL | |         drop(v);
LL | |         drop(clonable_0);
LL | |     };
   | |_____- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 41:6]`
LL |       check_copy(&gen_clone_0);
   |       ^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 41:6]`, the trait `Copy` is not implemented for `Vec<u32>`
note: captured value does not implement `Copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:40:14
   |
   |
LL |         drop(clonable_0);
   |              ^^^^^^^^^^ has type `Vec<u32>` which does not implement `Copy`
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:72:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 41:6]`
  --> /checkout/src/test/ui/generator/clone-impl.rs:42:5
   |
LL |       let gen_clone_0 = move || {
   |  _______________________-
LL | |         let v = vec!['a'];
LL | |         yield;
LL | |         drop(v);
LL | |         drop(clonable_0);
LL | |     };
   | |_____- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 41:6]`
LL |       check_copy(&gen_clone_0);
   |       ^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 41:6]`, the trait `Copy` is not implemented for `Vec<char>`
   |
note: generator does not implement `Copy` as this value is used across a yield
  --> /checkout/src/test/ui/generator/clone-impl.rs:38:9
   |
LL |         let v = vec!['a'];
   |             - has type `Vec<char>` which does not implement `Copy`
LL |         yield;
   |         ^^^^^ yield occurs here, with `v` maybe used later
LL |     };
LL |     };
   |     - `v` is later dropped here
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:72:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 57:6]`
  --> /checkout/src/test/ui/generator/clone-impl.rs:58:5
   |
LL |       let gen_clone_1 = move || {
   |  _______________________-
LL | |         let v = vec!['a'];
LL | |         /*
LL | |         let n = NonClone;
...  |
LL | |         drop(clonable_1);
LL | |     };
   | |_____- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 57:6]`
LL |       check_copy(&gen_clone_1);
   |       ^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 57:6]`, the trait `Copy` is not implemented for `Vec<u32>`
note: captured value does not implement `Copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:56:14
   |
   |
LL |         drop(clonable_1);
   |              ^^^^^^^^^^ has type `Vec<u32>` which does not implement `Copy`
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:72:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 57:6]`
  --> /checkout/src/test/ui/generator/clone-impl.rs:58:5
   |
LL |       let gen_clone_1 = move || {
   |  _______________________-
LL | |         let v = vec!['a'];
LL | |         /*
LL | |         let n = NonClone;
...  |
LL | |         drop(clonable_1);
LL | |     };
   | |_____- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 57:6]`
LL |       check_copy(&gen_clone_1);
   |       ^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 57:6]`, the trait `Copy` is not implemented for `Vec<char>`
   |
note: generator does not implement `Copy` as this value is used across a yield
  --> /checkout/src/test/ui/generator/clone-impl.rs:52:9
   |
LL |         let v = vec!['a'];
   |             - has type `Vec<char>` which does not implement `Copy`
LL |         yield;
LL |         yield;
   |         ^^^^^ yield occurs here, with `v` maybe used later
LL |     };
LL |     };
   |     - `v` is later dropped here
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:72:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `NonClone: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 65:6]`
  --> /checkout/src/test/ui/generator/clone-impl.rs:66:5
LL |       let gen_non_clone = move || {
   |  _________________________-
LL | |         yield;
LL | |         yield;
LL | |         drop(non_clonable);
LL | |     };
   | |_____- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 65:6]`
LL |       check_copy(&gen_non_clone);
   |       ^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 65:6]`, the trait `Copy` is not implemented for `NonClone`
note: captured value does not implement `Copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:64:14
   |
   |
LL |         drop(non_clonable);
   |              ^^^^^^^^^^^^ has type `NonClone` which does not implement `Copy`
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:72:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`
help: consider annotating `NonClone` with `#[derive(Copy)]`
   |
LL | #[derive(Copy)]


error[E0277]: the trait bound `NonClone: Clone` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 65:6]`
  --> /checkout/src/test/ui/generator/clone-impl.rs:68:5
LL |       let gen_non_clone = move || {
   |  _________________________-
LL | |         yield;
LL | |         yield;
LL | |         drop(non_clonable);
LL | |     };
   | |_____- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 65:6]`
...
LL |       check_clone(&gen_non_clone);
   |       ^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 65:6]`, the trait `Clone` is not implemented for `NonClone`
note: captured value does not implement `Clone`
  --> /checkout/src/test/ui/generator/clone-impl.rs:64:14
   |
   |
LL |         drop(non_clonable);
   |              ^^^^^^^^^^^^ has type `NonClone` which does not implement `Clone`
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl.rs:73:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`
help: consider annotating `NonClone` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 6 previous errors
