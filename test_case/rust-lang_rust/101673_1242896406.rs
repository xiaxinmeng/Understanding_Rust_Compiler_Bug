plain
......i..........i..........i........................................................... 3960/13508
........................................................................................ 4048/13508
..iii................................................................................... 4136/13508
......................................................i................................. 4224/13508
..................................................................F..F.................. 4312/13508
........................................................................................ 4488/13508
........................................................................................ 4576/13508
........................................................................................ 4664/13508
........................................................................................ 4752/13508
---
.........................iii............................................................ 13464/13508
............................................
failures:

---- [ui] src/test/ui/generator/clone-impl-static.rs stdout ----


- error[E0277]: the trait bound `[static generator@$DIR/clone-impl-static.rs:7:15: 9:6]: Copy` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-static.rs:7:15: 7:29]: Copy` is not satisfied
2   --> $DIR/clone-impl-static.rs:10:16
3    |
4 LL |     check_copy(&gen);
-    |     ---------- ^^^^ the trait `Copy` is not implemented for `[static generator@$DIR/clone-impl-static.rs:7:15: 9:6]`
+    |     ---------- ^^^^ the trait `Copy` is not implemented for `[static generator@$DIR/clone-impl-static.rs:7:15: 7:29]`
6    |     |
7    |     required by a bound introduced by this call
7    |     required by a bound introduced by this call
8    |

12 LL | fn check_copy<T: Copy>(_x: &T) {}
13    |                  ^^^^ required by this bound in `check_copy`
14 
- error[E0277]: the trait bound `[static generator@$DIR/clone-impl-static.rs:7:15: 9:6]: Clone` is not satisfied
+ error[E0277]: the trait bound `[static generator@$DIR/clone-impl-static.rs:7:15: 7:29]: Clone` is not satisfied
16   --> $DIR/clone-impl-static.rs:12:17
17    |
18 LL |     check_clone(&gen);
-    |     ----------- ^^^^ the trait `Clone` is not implemented for `[static generator@$DIR/clone-impl-static.rs:7:15: 9:6]`
+    |     ----------- ^^^^ the trait `Clone` is not implemented for `[static generator@$DIR/clone-impl-static.rs:7:15: 7:29]`
20    |     |
21    |     required by a bound introduced by this call
---
To only update this specific test, also pass `--test-args generator/clone-impl-static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/clone-impl-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl-static" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl-static/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-static.rs:7:15: 7:29]: Copy` is not satisfied
  --> /checkout/src/test/ui/generator/clone-impl-static.rs:10:16
   |
LL |     check_copy(&gen);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |     ---------- ^^^^ the trait `Copy` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-static.rs:7:15: 7:29]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_copy`
  --> /checkout/src/test/ui/generator/clone-impl-static.rs:16:18
  --> /checkout/src/test/ui/generator/clone-impl-static.rs:16:18
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `[static generator@/checkout/src/test/ui/generator/clone-impl-static.rs:7:15: 7:29]: Clone` is not satisfied
  --> /checkout/src/test/ui/generator/clone-impl-static.rs:12:17
   |
LL |     check_clone(&gen);
   |     ----------- ^^^^ the trait `Clone` is not implemented for `[static generator@/checkout/src/test/ui/generator/clone-impl-static.rs:7:15: 7:29]`
   |     required by a bound introduced by this call
   |
note: required by a bound in `check_clone`
  --> /checkout/src/test/ui/generator/clone-impl-static.rs:17:19
  --> /checkout/src/test/ui/generator/clone-impl-static.rs:17:19
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/generator/clone-impl.rs stdout ----
diff of stderr:

- error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:36:23: 41:6]`
-   --> $DIR/clone-impl.rs:42:5
+ error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:36:23: 36:30]`
3    |
3    |
- LL |       let gen_clone_0 = move || {
-    |  _______________________-
- LL | |         let v = vec!['a'];
- LL | |         yield;
- LL | |         drop(v);
- LL | |         drop(clonable_0);
- LL | |     };
-    | |_____- within this `[generator@$DIR/clone-impl.rs:36:23: 41:6]`
- LL |       check_copy(&gen_clone_0);
-    |       ^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:36:23: 41:6]`, the trait `Copy` is not implemented for `Vec<u32>`
+ LL |     let gen_clone_0 = move || {
+    |                       ------- within this `[generator@$DIR/clone-impl.rs:36:23: 36:30]`
+ ...
+ LL |     check_copy(&gen_clone_0);
+    |                ^^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:36:23: 36:30]`, the trait `Copy` is not implemented for `Vec<u32>`
15 note: captured value does not implement `Copy`
16   --> $DIR/clone-impl.rs:40:14


23 LL | fn check_copy<T: Copy>(_x: &T) {}
24    |                  ^^^^ required by this bound in `check_copy`
25 
- error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:36:23: 41:6]`
-   --> $DIR/clone-impl.rs:42:5
+ error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:36:23: 36:30]`
28    |
28    |
- LL |       let gen_clone_0 = move || {
-    |  _______________________-
- LL | |         let v = vec!['a'];
- LL | |         yield;
- LL | |         drop(v);
- LL | |         drop(clonable_0);
- LL | |     };
-    | |_____- within this `[generator@$DIR/clone-impl.rs:36:23: 41:6]`
- LL |       check_copy(&gen_clone_0);
-    |       ^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:36:23: 41:6]`, the trait `Copy` is not implemented for `Vec<char>`
+ LL |     let gen_clone_0 = move || {
+    |                       ------- within this `[generator@$DIR/clone-impl.rs:36:23: 36:30]`
+ ...
+ LL |     check_copy(&gen_clone_0);
+    |                ^^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:36:23: 36:30]`, the trait `Copy` is not implemented for `Vec<char>`
39    |
40 note: generator does not implement `Copy` as this value is used across a yield


53 LL | fn check_copy<T: Copy>(_x: &T) {}
54    |                  ^^^^ required by this bound in `check_copy`
55 
- error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:46:23: 57:6]`
-   --> $DIR/clone-impl.rs:58:5
+ error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:46:23: 46:30]`
58    |
58    |
- LL |       let gen_clone_1 = move || {
-    |  _______________________-
- LL | |         let v = vec!['a'];
- LL | |         /*
- LL | |         let n = NonClone;
- ...  |
- LL | |         drop(clonable_1);
- LL | |     };
-    | |_____- within this `[generator@$DIR/clone-impl.rs:46:23: 57:6]`
- LL |       check_copy(&gen_clone_1);
-    |       ^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:46:23: 57:6]`, the trait `Copy` is not implemented for `Vec<u32>`
+ LL |     let gen_clone_1 = move || {
+    |                       ------- within this `[generator@$DIR/clone-impl.rs:46:23: 46:30]`
+ ...
+ LL |     check_copy(&gen_clone_1);
+    |                ^^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:46:23: 46:30]`, the trait `Copy` is not implemented for `Vec<u32>`
71 note: captured value does not implement `Copy`
72   --> $DIR/clone-impl.rs:56:14


79 LL | fn check_copy<T: Copy>(_x: &T) {}
80    |                  ^^^^ required by this bound in `check_copy`
81 
- error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:46:23: 57:6]`
-   --> $DIR/clone-impl.rs:58:5
+ error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:46:23: 46:30]`
84    |
84    |
- LL |       let gen_clone_1 = move || {
-    |  _______________________-
- LL | |         let v = vec!['a'];
- LL | |         /*
- LL | |         let n = NonClone;
- ...  |
- LL | |         drop(clonable_1);
- LL | |     };
-    | |_____- within this `[generator@$DIR/clone-impl.rs:46:23: 57:6]`
- LL |       check_copy(&gen_clone_1);
-    |       ^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:46:23: 57:6]`, the trait `Copy` is not implemented for `Vec<char>`
+ LL |     let gen_clone_1 = move || {
+    |                       ------- within this `[generator@$DIR/clone-impl.rs:46:23: 46:30]`
+ ...
+ LL |     check_copy(&gen_clone_1);
+    |                ^^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:46:23: 46:30]`, the trait `Copy` is not implemented for `Vec<char>`
96    |
97 note: generator does not implement `Copy` as this value is used across a yield


111 LL | fn check_copy<T: Copy>(_x: &T) {}
112    |                  ^^^^ required by this bound in `check_copy`
113 
- error[E0277]: the trait bound `NonClone: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:62:25: 65:6]`
-   --> $DIR/clone-impl.rs:66:5
+ error[E0277]: the trait bound `NonClone: Copy` is not satisfied in `[generator@$DIR/clone-impl.rs:62:25: 62:32]`
116    |
116    |
- LL |       let gen_non_clone = move || {
- LL | |         yield;
- LL | |         yield;
- LL | |         drop(non_clonable);
- LL | |     };
-    | |_____- within this `[generator@$DIR/clone-impl.rs:62:25: 65:6]`
- LL |       check_copy(&gen_non_clone);
-    |       ^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:62:25: 65:6]`, the trait `Copy` is not implemented for `NonClone`
+ LL |     let gen_non_clone = move || {
+    |                         ------- within this `[generator@$DIR/clone-impl.rs:62:25: 62:32]`
+ ...
+ LL |     check_copy(&gen_non_clone);
+    |                ^^^^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:62:25: 62:32]`, the trait `Copy` is not implemented for `NonClone`
126 note: captured value does not implement `Copy`
127   --> $DIR/clone-impl.rs:64:14

138 LL | #[derive(Copy)]
138 LL | #[derive(Copy)]
139    |
140 
- error[E0277]: the trait bound `NonClone: Clone` is not satisfied in `[generator@$DIR/clone-impl.rs:62:25: 65:6]`
-   --> $DIR/clone-impl.rs:68:5
+ error[E0277]: the trait bound `NonClone: Clone` is not satisfied in `[generator@$DIR/clone-impl.rs:62:25: 62:32]`
143    |
143    |
- LL |       let gen_non_clone = move || {
- LL | |         yield;
- LL | |         yield;
- LL | |         drop(non_clonable);
- LL | |     };
-    | |_____- within this `[generator@$DIR/clone-impl.rs:62:25: 65:6]`
+ LL |     let gen_non_clone = move || {
+    |                         ------- within this `[generator@$DIR/clone-impl.rs:62:25: 62:32]`
150 ...
- LL |       check_clone(&gen_non_clone);
-    |       ^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:62:25: 65:6]`, the trait `Clone` is not implemented for `NonClone`
+ LL |     check_clone(&gen_non_clone);
+    |                 ^^^^^^^^^^^^^^ within `[generator@$DIR/clone-impl.rs:62:25: 62:32]`, the trait `Clone` is not implemented for `NonClone`
154 note: captured value does not implement `Clone`
155   --> $DIR/clone-impl.rs:64:14



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl/clone-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generator/clone-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/clone-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/clone-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 36:30]`
   |
   |
LL |     let gen_clone_0 = move || {
   |                       ------- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 36:30]`
...
LL |     check_copy(&gen_clone_0);
   |                ^^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 36:30]`, the trait `Copy` is not implemented for `Vec<u32>`
note: captured value does not implement `Copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:40:14
   |
   |
LL |         drop(clonable_0);
   |              ^^^^^^^^^^ has type `Vec<u32>` which does not implement `Copy`
note: required by a bound in `check_copy`
   |
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 36:30]`
   |
   |
LL |     let gen_clone_0 = move || {
   |                       ------- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 36:30]`
...
LL |     check_copy(&gen_clone_0);
   |                ^^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:36:23: 36:30]`, the trait `Copy` is not implemented for `Vec<char>`
   |
note: generator does not implement `Copy` as this value is used across a yield
   |
   |
LL |         let v = vec!['a'];
   |             - has type `Vec<char>` which does not implement `Copy`
LL |         yield;
   |         ^^^^^ yield occurs here, with `v` maybe used later
LL |     };
LL |     };
   |     - `v` is later dropped here
note: required by a bound in `check_copy`
   |
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `Vec<u32>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 46:30]`
   |
   |
LL |     let gen_clone_1 = move || {
   |                       ------- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 46:30]`
...
LL |     check_copy(&gen_clone_1);
   |                ^^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 46:30]`, the trait `Copy` is not implemented for `Vec<u32>`
note: captured value does not implement `Copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:56:14
   |
   |
LL |         drop(clonable_1);
   |              ^^^^^^^^^^ has type `Vec<u32>` which does not implement `Copy`
note: required by a bound in `check_copy`
   |
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `Vec<char>: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 46:30]`
   |
   |
LL |     let gen_clone_1 = move || {
   |                       ------- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 46:30]`
...
LL |     check_copy(&gen_clone_1);
   |                ^^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:46:23: 46:30]`, the trait `Copy` is not implemented for `Vec<char>`
   |
note: generator does not implement `Copy` as this value is used across a yield
   |
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
   |
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`

error[E0277]: the trait bound `NonClone: Copy` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 62:32]`
   |
LL |     let gen_non_clone = move || {
LL |     let gen_non_clone = move || {
   |                         ------- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 62:32]`
...
LL |     check_copy(&gen_non_clone);
   |                ^^^^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 62:32]`, the trait `Copy` is not implemented for `NonClone`
note: captured value does not implement `Copy`
  --> /checkout/src/test/ui/generator/clone-impl.rs:64:14
   |
   |
LL |         drop(non_clonable);
   |              ^^^^^^^^^^^^ has type `NonClone` which does not implement `Copy`
note: required by a bound in `check_copy`
   |
   |
LL | fn check_copy<T: Copy>(_x: &T) {}
   |                  ^^^^ required by this bound in `check_copy`
help: consider annotating `NonClone` with `#[derive(Copy)]`
LL | #[derive(Copy)]
   |


error[E0277]: the trait bound `NonClone: Clone` is not satisfied in `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 62:32]`
   |
LL |     let gen_non_clone = move || {
LL |     let gen_non_clone = move || {
   |                         ------- within this `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 62:32]`
...
LL |     check_clone(&gen_non_clone);
   |                 ^^^^^^^^^^^^^^ within `[generator@/checkout/src/test/ui/generator/clone-impl.rs:62:25: 62:32]`, the trait `Clone` is not implemented for `NonClone`
note: captured value does not implement `Clone`
  --> /checkout/src/test/ui/generator/clone-impl.rs:64:14
   |
   |
LL |         drop(non_clonable);
   |              ^^^^^^^^^^^^ has type `NonClone` which does not implement `Clone`
note: required by a bound in `check_clone`
   |
   |
LL | fn check_clone<T: Clone>(_x: &T) {}
   |                   ^^^^^ required by this bound in `check_clone`
help: consider annotating `NonClone` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 6 previous errors
