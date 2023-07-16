plain
Successfully built 487b3e13bf68
Successfully tagged rust-ci:latest
Built container sha256:487b3e13bf68b084c49c63dd628cbb2390c3c916b6081762fef10b4dc1ec8968
Uploading finished image to https://ci-caches.rust-lang.org/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0
upload failed: - to s3://rust-lang-ci-sccache2/docker/93608764ca974412130ba4a304c50b0ea4a89a9b5853d488b55e530021cbeebcb5549b17247c6674f12706902ca4687aae74f60088fb03044e2d4026dd4d59e0 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---
..........................................................i......................................... 5600/12638
.................................................................................................... 5700/12638
.................................................................................................... 5800/12638
.................................................................................................... 5900/12638
......................................................................F............................. 6000/12638
...........................................................................F..F..................... 6100/12638
......................................................................................i............. 6300/12638
.................................................................................................... 6400/12638
...................................................i................................................ 6500/12638
..ii.ii........i...i................................................................................ 6600/12638
---

---- [ui] ui/issues/issue-41394.rs stdout ----
diff of stderr:

- error[E0369]: cannot add `{integer}` to `&str`
-   --> $DIR/issue-41394.rs:2:12
+ error[E0308]: mismatched types
3    |
3    |
4 LL |     A = "" + 1
-    |         -- ^ - {integer}
-    |         &str
+    |              ^ expected `&str`, found integer
8 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0308]: mismatched types
+   --> $DIR/issue-41394.rs:2:9
+    |
+ LL |     A = "" + 1
+    |         ^^^^^^ expected `isize`, found struct `String`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- For more information about this error, try `rustc --explain E0369`.
+ error: aborting due to 2 previous errors
+ 
---
To only update this specific test, also pass `--test-args issues/issue-41394.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-41394.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-41394/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-41394.rs:2:14
   |
LL |     A = "" + 1
   |              ^ expected `&str`, found integer
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-41394.rs:2:9
   |
   |
LL |     A = "" + 1
   |         ^^^^^^ expected `isize`, found struct `String`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/issues/issue-47380.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47380.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47380" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47380/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/issues/issue-47377.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47377.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47377" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47377/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

- error[E0369]: cannot add `&str` to `&str`
-   --> $DIR/issue-39018.rs:2:22
-    |
- LL |     let x = "Hello " + "World!";
-    |             -------- ^ -------- &str
-    |             |        |
-    |             |        `+` cannot be used to concatenate two `&str` strings
-    |             &str
-    |
-    = note: string concatenation requires an owned `String` on the left
- help: create an owned `String` from a string reference
-    |
- LL |     let x = "Hello ".to_owned() + "World!";
- 
- 
16 error[E0369]: cannot add `World` to `World`
18    |

38 LL | | }
39    | |_^
39    | |_^
40 
- error[E0369]: cannot add `String` to `&str`
-   --> $DIR/issue-39018.rs:11:22
+ error[E0308]: mismatched types
43    |
43    |
44 LL |     let x = "Hello " + "World!".to_owned();
-    |             -------- ^ ------------------- String
-    |             |        |
-    |             |        `+` cannot be used to concatenate a `&str` with a `String`
-    |             &str
-    |
- help: create an owned `String` on the left and add a borrow on the right
-    |
- LL |     let x = "Hello ".to_owned() + &"World!".to_owned();
-    |                     +++++++++++   +
+    |                        |
+    |                        expected `&str`, found struct `String`
+    |                        expected `&str`, found struct `String`
+    |                        help: consider borrowing here: `&"World!".to_owned()`
54 
55 error[E0369]: cannot add `&String` to `&String`


166    |             -- ^ - &str
168    |             &&str
- 
- 
- error[E0369]: cannot add `&&str` to `&str`
-   --> $DIR/issue-39018.rs:36:15
172    |
- LL |     let _ = c + &d;
-    |             - ^ -- &&str
-    |             | |
-    |             | `+` cannot be used to concatenate two `&str` strings
-    |             &str
+ help: `+` can be used on `str`, you can dereference `&c`
178    |
-    = note: string concatenation requires an owned `String` on the left
- help: create an owned `String` from a string reference
-    |
- LL |     let _ = c.to_owned() + &d;
-    |              +++++++++++
+ LL |     let _ = *&c + d;
184 
- error[E0369]: cannot add `&str` to `&str`
-   --> $DIR/issue-39018.rs:37:15
-    |
-    |
- LL |     let _ = c + d;
-    |             - ^ - &str
-    |             | |
-    |             | `+` cannot be used to concatenate two `&str` strings
-    |             &str
-    |
-    = note: string concatenation requires an owned `String` on the left
- help: create an owned `String` from a string reference
-    |
- LL |     let _ = c.to_owned() + d;
- 
- error: aborting due to 14 previous errors
+ error: aborting due to 11 previous errors
201 
---
To only update this specific test, also pass `--test-args span/issue-39018.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-39018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-39018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0369]: cannot add `World` to `World`
   |
   |
LL |     let y = World::Hello + World::Goodbye;
   |             ------------ ^ -------------- World
   |             World
   |
   |
note: an implementation of `Add<_>` might be missing for `World`
   |
   |
LL | enum World {
   | ^^^^^^^^^^ must implement `Add<_>`
  --> /checkout/library/core/src/ops/arith.rs:100:1
   |
   |
LL | / pub trait Add<Rhs = Self> {
LL | |     /// The resulting type after applying the `+` operator.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Output;
...  |
LL | |     fn add(self, rhs: Rhs) -> Self::Output;
LL | | }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/issue-39018.rs:11:24
   |
   |
LL |     let x = "Hello " + "World!".to_owned();
   |                        |
   |                        expected `&str`, found struct `String`
   |                        expected `&str`, found struct `String`
   |                        help: consider borrowing here: `&"World!".to_owned()`

error[E0369]: cannot add `&String` to `&String`
   |
   |
LL |     let _ = &a + &b; //~ ERROR cannot add
   |             -- ^ -- &String
   |             |  |
   |             |  `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: remove the borrow to obtain an owned `String`
   |
LL -     let _ = &a + &b; //~ ERROR cannot add
LL +     let _ = a + &b; //~ ERROR cannot add


error[E0369]: cannot add `String` to `&String`
   |
   |
LL |     let _ = &a + b; //~ ERROR cannot add
   |             -- ^ - String
   |             |  |
   |             |  `+` cannot be used to concatenate a `&str` with a `String`
   |             &String
   |
help: remove the borrow on the left and add one on the right
   |
LL -     let _ = &a + b; //~ ERROR cannot add
LL +     let _ = a + &b; //~ ERROR cannot add

error[E0308]: mismatched types
  --> /checkout/src/test/ui/span/issue-39018.rs:29:17
   |
   |
LL |     let _ = a + b; //~ ERROR mismatched types
   |                 |
   |                 expected `&str`, found struct `String`
   |                 help: consider borrowing here: `&b`


error[E0369]: cannot add `String` to `&String`
   |
   |
LL |     let _ = e + b; //~ ERROR cannot add
   |             - ^ - String
   |             | |
   |             | `+` cannot be used to concatenate a `&str` with a `String`
   |             &String
   |
help: create an owned `String` on the left and add a borrow on the right
   |
LL |     let _ = e.to_owned() + &b; //~ ERROR cannot add
   |              +++++++++++   +

error[E0369]: cannot add `&String` to `&String`
   |
   |
LL |     let _ = e + &b; //~ ERROR cannot add
   |             - ^ -- &String
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = e.to_owned() + &b; //~ ERROR cannot add


error[E0369]: cannot add `&str` to `&String`
   |
   |
LL |     let _ = e + d; //~ ERROR cannot add
   |             - ^ - &str
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = e.to_owned() + d; //~ ERROR cannot add


error[E0369]: cannot add `&&str` to `&String`
   |
   |
LL |     let _ = e + &d; //~ ERROR cannot add
   |             - ^ -- &&str
   |             | |
   |             | `+` cannot be used to concatenate two `&str` strings
   |             &String
   |
   = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
   |
LL |     let _ = e.to_owned() + &d; //~ ERROR cannot add


error[E0369]: cannot add `&&str` to `&&str`
   |
   |
LL |     let _ = &c + &d; //~ ERROR cannot add
   |             -- ^ -- &&str
   |             &&str


error[E0369]: cannot add `&str` to `&&str`
   |
   |
LL |     let _ = &c + d; //~ ERROR cannot add
   |             -- ^ - &str
   |             &&str
   |
   |
help: `+` can be used on `str`, you can dereference `&c`
   |
LL |     let _ = *&c + d; //~ ERROR cannot add

error: aborting due to 11 previous errors

Some errors have detailed explanations: E0308, E0369.
---
---- [ui] ui/terminal-width/non-1-width-unicode-multiline-label.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/terminal-width/non-1-width-unicode-multiline-label.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/terminal-width/non-1-width-unicode-multiline-label/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

---- [ui] ui/typeck/issue-79040.rs stdout ----
diff of stderr:

- error[E0369]: cannot add `{integer}` to `&str`
-   --> $DIR/issue-79040.rs:2:25
+ error[E0308]: mismatched types
3    |
3    |
4 LL |     const FOO = "hello" + 1;
-    |                 ------- ^ - {integer}
-    |                 &str
+    |                           ^ expected `&str`, found integer
8 
9 error: missing type for `const` item
9 error: missing type for `const` item
10   --> $DIR/issue-79040.rs:2:11

11    |
12 LL |     const FOO = "hello" + 1;
-    |           ^^^ help: provide a type for the item: `FOO: <type>`
+    |           ^^^ help: provide a type for the constant: `FOO: String`
15 error: aborting due to 2 previous errors
16 

- For more information about this error, try `rustc --explain E0369`.
---
To only update this specific test, also pass `--test-args typeck/issue-79040.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-79040.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-79040/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-79040.rs:2:27
   |
LL |     const FOO = "hello" + 1; //~ ERROR cannot add `{integer}` to `&str`
   |                           ^ expected `&str`, found integer
error: missing type for `const` item
  --> /checkout/src/test/ui/typeck/issue-79040.rs:2:11
   |
   |
LL |     const FOO = "hello" + 1; //~ ERROR cannot add `{integer}` to `&str`
   |           ^^^ help: provide a type for the constant: `FOO: String`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.

