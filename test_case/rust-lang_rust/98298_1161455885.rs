plain
........................................................................................ 1584/13089
........................................................................................ 1672/13089
.....................i.....ii........................................................... 1760/13089
........................................................................................ 1848/13089
...........................................................................i..........F. 1936/13089
F..F......F............................................................................. 2024/13089
..................................................................F.............F....... 2112/13089
...............................F........................................................ 2200/13089
........................................................................................ 2376/13089
........................................................................................ 2464/13089
........................................................................................ 2552/13089
........................................................................F............... 2640/13089
........................................................................F............... 2640/13089
........................................................................................ 2728/13089
........................................................................................ 2816/13089
........................................................................................ 2904/13089
..................................i.....................................F............... 2992/13089
i............................................................................F.......... 3080/13089
.........................................FF............................................. 3168/13089
......................................F.............iiiii......................F........ 3256/13089
........................................................................................ 3432/13089
........................................................................................ 3520/13089
........................................................................................ 3608/13089
..........................................F............................................. 3696/13089
---
.....................i.................................................................. 12320/13089
........................................................................................ 12408/13089
........................................................................................ 12496/13089
........................................................................................ 12584/13089
....................................................................F.F................. 12672/13089
........................................................................................ 12848/13089
........................................................................................ 12936/13089
...............................................iii...................................... 13024/13089
.................................................................
.................................................................
failures:

---- [ui] src/test/ui/associated-item/associated-item-enum.rs stdout ----
diff of stderr:

2   --> $DIR/associated-item-enum.rs:17:11
3    |
4 LL | enum Enum { Variant }
-    | --------- variant or associated item `mispellable` not found here
+    |      ---- variant or associated item `mispellable` not found here
6 ...
7 LL |     Enum::mispellable();

14   --> $DIR/associated-item-enum.rs:18:11
15    |
15    |
16 LL | enum Enum { Variant }
-    | --------- variant or associated item `mispellable_trait` not found here
+    |      ---- variant or associated item `mispellable_trait` not found here
18 ...
19 LL |     Enum::mispellable_trait();

26   --> $DIR/associated-item-enum.rs:19:11
27    |
27    |
28 LL | enum Enum { Variant }
-    | --------- variant or associated item `MISPELLABLE` not found here
+    |      ---- variant or associated item `MISPELLABLE` not found here
30 ...
31 LL |     Enum::MISPELLABLE;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-enum/associated-item-enum.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-enum/associated-item-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-item/associated-item-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-item/associated-item-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-item/associated-item-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no variant or associated item named `mispellable` found for enum `Enum` in the current scope
   |
   |
LL | enum Enum { Variant }
   |      ---- variant or associated item `mispellable` not found here
...
LL |     Enum::mispellable(); //~ ERROR no variant or associated item
   |           |
   |           variant or associated item not found in `Enum`
   |           help: there is an associated function with a similar name: `misspellable`


error[E0599]: no variant or associated item named `mispellable_trait` found for enum `Enum` in the current scope
   |
   |
LL | enum Enum { Variant }
   |      ---- variant or associated item `mispellable_trait` not found here
...
LL |     Enum::mispellable_trait(); //~ ERROR no variant or associated item
   |           |
   |           variant or associated item not found in `Enum`
   |           help: there is an associated function with a similar name: `misspellable`


error[E0599]: no variant or associated item named `MISPELLABLE` found for enum `Enum` in the current scope
   |
   |
LL | enum Enum { Variant }
   |      ---- variant or associated item `MISPELLABLE` not found here
...
LL |     Enum::MISPELLABLE; //~ ERROR no variant or associated item
   |           |
   |           variant or associated item not found in `Enum`
   |           help: there is an associated constant with a similar name: `MISSPELLABLE`

---
diff of stderr:

2   --> $DIR/pin-needed-to-poll.rs:42:20
3    |
4 LL | struct Sleep;
-    | ------------- method `poll` not found for this
+    |        ----- method `poll` not found for this
6 ...
7 LL |         self.sleep.poll(cx)
8    |                    ^^^^ method not found in `Sleep`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/pin-needed-to-poll/pin-needed-to-poll.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/pin-needed-to-poll.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/pin-needed-to-poll.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/pin-needed-to-poll" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/pin-needed-to-poll/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `poll` found for struct `Sleep` in the current scope
   |
LL | struct Sleep;
LL | struct Sleep;
   |        ----- method `poll` not found for this
...
LL |         self.sleep.poll(cx)
   |                    ^^^^ method not found in `Sleep`
  ::: /checkout/library/core/src/future/future.rs:104:8
   |
   |
LL |     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
   |        ---- the method is available for `Pin<&mut Sleep>` here
help: consider wrapping the receiver expression with the appropriate type
   |
   |
LL |         Pin::new(&mut self.sleep).poll(cx)
   |         +++++++++++++           +
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/bogus-tag.rs stdout ----
diff of stderr:

2   --> $DIR/bogus-tag.rs:7:16
3    |
4 LL | enum Color { Rgb(isize, isize, isize), Rgba(isize, isize, isize, isize), }
-    | ---------- variant or associated item `Hsl` not found here
+    |      ----- variant or associated item `Hsl` not found here
6 ...
7 LL |         Color::Hsl(h, s, l) => { println!("hsl"); }


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-tag/bogus-tag.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-tag/bogus-tag.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args bogus-tag.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/bogus-tag.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-tag" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/bogus-tag/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no variant or associated item named `Hsl` found for enum `Color` in the current scope
   |
   |
LL | enum Color { Rgb(isize, isize, isize), Rgba(isize, isize, isize, isize), }
   |      ----- variant or associated item `Hsl` not found here
...
LL |         Color::Hsl(h, s, l) => { println!("hsl"); }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/issue-18343.rs:7:7
3    |
4 LL | struct Obj<F> where F: FnMut() -> u32 {
-    | ------------------------------------- method `closure` not found for this
+    |        --- method `closure` not found for this
6 ...
7 LL |     o.closure();
8    |       ^^^^^^^ field, not a method

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-18343/issue-18343.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args confuse-field-and-method/issue-18343.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/confuse-field-and-method/issue-18343.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-18343" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-18343/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnMut() -> u32 {
   |        --- method `closure` not found for this
...
LL |     o.closure();
   |       ^^^^^^^ field, not a method
   |
help: to call the function stored in `closure`, surround the field access with parentheses
   |
LL |     (o.closure)();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/issue-32128.rs:12:10
3    |
4 LL | struct Example {
-    | -------------- method `example` not found for this
+    |        ------- method `example` not found for this
6 ...
7 LL |     demo.example(1);
8    |          ^^^^^^^ field, not a method

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-32128/issue-32128.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args confuse-field-and-method/issue-32128.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/confuse-field-and-method/issue-32128.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-32128" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-32128/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `example` found for struct `Example` in the current scope
   |
LL | struct Example {
LL | struct Example {
   |        ------- method `example` not found for this
...
LL |     demo.example(1);
   |          ^^^^^^^ field, not a method
   |
help: to call the function stored in `example`, surround the field access with parentheses
   |
LL |     (demo.example)(1);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/private-field.rs:16:23
3    |
4 LL |     pub struct Dog {
-    |     -------------- method `dog_age` not found for this
+    |                --- method `dog_age` not found for this
6 ...
7 LL |     let dog_age = dog.dog_age();
8    |                       ^^^^^^^ private field, not a method

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/private-field/private-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args confuse-field-and-method/private-field.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/confuse-field-and-method/private-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/private-field" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/private-field/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `dog_age` found for struct `Dog` in the current scope
   |
LL |     pub struct Dog {
LL |     pub struct Dog {
   |                --- method `dog_age` not found for this
...
LL |     let dog_age = dog.dog_age(); //~ ERROR no method
   |                       ^^^^^^^ private field, not a method
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/confuse-field-and-method/issue-2392.rs stdout ----
diff of stderr:

2   --> $DIR/issue-2392.rs:36:15
3    |
4 LL | struct Obj<F> where F: FnOnce() -> u32 {
-    | -------------------------------------- method `closure` not found for this
+    |        --- method `closure` not found for this
6 ...
7 LL |     o_closure.closure();
8    |               ^^^^^^^ field, not a method
16   --> $DIR/issue-2392.rs:38:15
17    |
17    |
18 LL | struct Obj<F> where F: FnOnce() -> u32 {
-    | -------------------------------------- method `not_closure` not found for this
+    |        --- method `not_closure` not found for this
20 ...
21 LL |     o_closure.not_closure();
22    |               ^^^^^^^^^^^-- help: remove the arguments
27   --> $DIR/issue-2392.rs:42:12
28    |
28    |
29 LL | struct Obj<F> where F: FnOnce() -> u32 {
-    | -------------------------------------- method `closure` not found for this
+    |        --- method `closure` not found for this
31 ...
32 LL |     o_func.closure();
33    |            ^^^^^^^ field, not a method
41   --> $DIR/issue-2392.rs:45:14
42    |
43 LL | struct BoxedObj {
43 LL | struct BoxedObj {
-    | --------------- method `boxed_closure` not found for this
+    |        -------- method `boxed_closure` not found for this
45 ...
46 LL |     boxed_fn.boxed_closure();
47    |              ^^^^^^^^^^^^^ field, not a method
55   --> $DIR/issue-2392.rs:48:19
56    |
57 LL | struct BoxedObj {
57 LL | struct BoxedObj {
-    | --------------- method `boxed_closure` not found for this
+    |        -------- method `boxed_closure` not found for this
59 ...
60 LL |     boxed_closure.boxed_closure();
61    |                   ^^^^^^^^^^^^^ field, not a method
69   --> $DIR/issue-2392.rs:53:12
70    |
70    |
71 LL | struct Obj<F> where F: FnOnce() -> u32 {
-    | -------------------------------------- method `closure` not found for this
+    |        --- method `closure` not found for this
74 LL |     w.wrap.closure();
75    |            ^^^^^^^ field, not a method

83   --> $DIR/issue-2392.rs:55:12
83   --> $DIR/issue-2392.rs:55:12
84    |
85 LL | struct Obj<F> where F: FnOnce() -> u32 {
-    | -------------------------------------- method `not_closure` not found for this
+    |        --- method `not_closure` not found for this
87 ...
88 LL |     w.wrap.not_closure();
89    |            ^^^^^^^^^^^-- help: remove the arguments
94   --> $DIR/issue-2392.rs:58:24
95    |
95    |
96 LL | struct Obj<F> where F: FnOnce() -> u32 {
-    | -------------------------------------- method `closure` not found for this
+    |        --- method `closure` not found for this
99 LL |     check_expression().closure();
100    |                        ^^^^^^^ field, not a method

108   --> $DIR/issue-2392.rs:64:31
108   --> $DIR/issue-2392.rs:64:31
109    |
110 LL | struct FuncContainer {
-    | -------------------- method `f1` not found for this
+    |        ------------- method `f1` not found for this
112 ...
113 LL |             (*self.container).f1(1);
114    |                               ^^ field, not a method
122   --> $DIR/issue-2392.rs:65:31
123    |
123    |
124 LL | struct FuncContainer {
-    | -------------------- method `f2` not found for this
+    |        ------------- method `f2` not found for this
126 ...
127 LL |             (*self.container).f2(1);
128    |                               ^^ field, not a method
136   --> $DIR/issue-2392.rs:66:31
137    |
137    |
138 LL | struct FuncContainer {
-    | -------------------- method `f3` not found for this
+    |        ------------- method `f3` not found for this
140 ...
141 LL |             (*self.container).f3(1);
142    |                               ^^ field, not a method

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-2392/issue-2392.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args confuse-field-and-method/issue-2392.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/confuse-field-and-method/issue-2392.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-2392" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/confuse-field-and-method/issue-2392/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnOnce() -> u32 {
   |        --- method `closure` not found for this
...
LL |     o_closure.closure(); //~ ERROR no method named `closure` found
   |               ^^^^^^^ field, not a method
   |
help: to call the function stored in `closure`, surround the field access with parentheses
   |
LL |     (o_closure.closure)(); //~ ERROR no method named `closure` found


error[E0599]: no method named `not_closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnOnce() -> u32 {
   |        --- method `not_closure` not found for this
...
LL |     o_closure.not_closure();
   |               ^^^^^^^^^^^-- help: remove the arguments
   |               field, not a method


error[E0599]: no method named `closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnOnce() -> u32 {
   |        --- method `closure` not found for this
...
LL |     o_func.closure(); //~ ERROR no method named `closure` found
   |            ^^^^^^^ field, not a method
   |
help: to call the function stored in `closure`, surround the field access with parentheses
   |
LL |     (o_func.closure)(); //~ ERROR no method named `closure` found


error[E0599]: no method named `boxed_closure` found for struct `BoxedObj` in the current scope
   |
LL | struct BoxedObj {
LL | struct BoxedObj {
   |        -------- method `boxed_closure` not found for this
...
LL |     boxed_fn.boxed_closure();//~ ERROR no method named `boxed_closure` found
   |              ^^^^^^^^^^^^^ field, not a method
   |
help: to call the function stored in `boxed_closure`, surround the field access with parentheses
   |
LL |     (boxed_fn.boxed_closure)();//~ ERROR no method named `boxed_closure` found


error[E0599]: no method named `boxed_closure` found for struct `BoxedObj` in the current scope
   |
LL | struct BoxedObj {
LL | struct BoxedObj {
   |        -------- method `boxed_closure` not found for this
...
LL |     boxed_closure.boxed_closure();//~ ERROR no method named `boxed_closure` found
   |                   ^^^^^^^^^^^^^ field, not a method
   |
help: to call the function stored in `boxed_closure`, surround the field access with parentheses
   |
LL |     (boxed_closure.boxed_closure)();//~ ERROR no method named `boxed_closure` found


error[E0599]: no method named `closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnOnce() -> u32 {
   |        --- method `closure` not found for this
...
LL |     w.wrap.closure();//~ ERROR no method named `closure` found
   |            ^^^^^^^ field, not a method
   |
help: to call the function stored in `closure`, surround the field access with parentheses
   |
LL |     (w.wrap.closure)();//~ ERROR no method named `closure` found


error[E0599]: no method named `not_closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnOnce() -> u32 {
   |        --- method `not_closure` not found for this
...
LL |     w.wrap.not_closure();
   |            ^^^^^^^^^^^-- help: remove the arguments
   |            field, not a method


error[E0599]: no method named `closure` found for struct `Obj` in the current scope
   |
   |
LL | struct Obj<F> where F: FnOnce() -> u32 {
   |        --- method `closure` not found for this
...
LL |     check_expression().closure();//~ ERROR no method named `closure` found
   |                        ^^^^^^^ field, not a method
   |
help: to call the function stored in `closure`, surround the field access with parentheses
   |
LL |     (check_expression().closure)();//~ ERROR no method named `closure` found


error[E0599]: no method named `f1` found for struct `FuncContainer` in the current scope
   |
   |
LL | struct FuncContainer {
   |        ------------- method `f1` not found for this
...
LL |             (*self.container).f1(1); //~ ERROR no method named `f1` found
   |                               ^^ field, not a method
   |
help: to call the function stored in `f1`, surround the field access with parentheses
   |
LL |             ((*self.container).f1)(1); //~ ERROR no method named `f1` found


error[E0599]: no method named `f2` found for struct `FuncContainer` in the current scope
   |
   |
LL | struct FuncContainer {
   |        ------------- method `f2` not found for this
...
LL |             (*self.container).f2(1); //~ ERROR no method named `f2` found
   |                               ^^ field, not a method
   |
help: to call the function stored in `f2`, surround the field access with parentheses
   |
LL |             ((*self.container).f2)(1); //~ ERROR no method named `f2` found


error[E0599]: no method named `f3` found for struct `FuncContainer` in the current scope
   |
   |
LL | struct FuncContainer {
   |        ------------- method `f3` not found for this
...
LL |             (*self.container).f3(1); //~ ERROR no method named `f3` found
   |                               ^^ field, not a method
   |
help: to call the function stored in `f3`, surround the field access with parentheses
   |
LL |             ((*self.container).f3)(1); //~ ERROR no method named `f3` found

error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

8   --> $DIR/issue-69654.rs:17:10
9    |
10 LL | struct Foo<const N: usize> {}
-    | -------------------------- function or associated item `foo` not found for this
+    |        --- function or associated item `foo` not found for this
13 LL |     Foo::foo();
13 LL |     Foo::foo();
14    |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-69654/issue-69654.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-69654.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-69654.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-69654" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-69654/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0423]: expected value, found type parameter `T`
   |
   |
LL | impl<T> Bar<T> for [u8; T] {}


error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
   |
   |
LL | struct Foo<const N: usize> {}
   |        --- function or associated item `foo` not found for this
LL |     Foo::foo();
LL |     Foo::foo();
   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `[u8; _]: Bar<[(); _]>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.
For more information about an error, try `rustc --explain E0423`.
------------------------------------------


---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-80742.rs stdout ----
diff of stderr:

15 error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
17    |
17    |
- LL | / struct Inline<T>
- LL | | where
- LL | |     [u8; size_of::<T>() + 1]: ,
- LL | | {
- LL | |     _phantom: PhantomData<T>,
- LL | |     buf: [u8; size_of::<T>() + 1],
- LL | | }
-    | |_- function or associated item `new` not found for this
+ LL | struct Inline<T>
+    |        ------ function or associated item `new` not found for this
26 ...
- LL |       let dst = Inline::<dyn Debug>::new(0);
-    |                                      ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
+ LL |     let dst = Inline::<dyn Debug>::new(0);
+    |                                    ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
30   ::: $SRC_DIR/core/src/fmt/mod.rs:LL:COL
31    |

- LL |   pub trait Debug {
- LL |   pub trait Debug {
-    |   --------------- doesn't satisfy `dyn Debug: Sized`
+ LL | pub trait Debug {
+    | --------------- doesn't satisfy `dyn Debug: Sized`
35    = note: the following trait bounds were not satisfied:
35    = note: the following trait bounds were not satisfied:
36            `dyn Debug: Sized`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742/issue-80742.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-80742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     size_of called on unsized type `dyn Debug`
   |     inside `std::mem::size_of::<dyn Debug>` at /checkout/library/core/src/mem/mod.rs:311:5
  ::: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:22:10
   |
   |
LL |     [u8; size_of::<T>() + 1]: ,
   |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:22:10

error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
   |
LL | struct Inline<T>
   |        ------ function or associated item `new` not found for this
...
...
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |                                    ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/fmt/mod.rs:657:1
   |
LL | pub trait Debug {
LL | pub trait Debug {
   | --------------- doesn't satisfy `dyn Debug: Sized`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Debug: Sized`

error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
   |
LL |     intrinsics::size_of::<T>()
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     |
   |     size_of called on unsized type `dyn Debug`
   |     inside `std::mem::size_of::<dyn Debug>` at /checkout/library/core/src/mem/mod.rs:311:5
  ::: /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:14:10
   |
   |
LL |     [u8; size_of::<T>() + 1]: ,
   |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:14:10
error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:30:15
   |
   |
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |
   = help: the trait `Sized` is not implemented for `dyn Debug`
note: required by a bound in `Inline`
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:12:15
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:12:15
   |
LL | struct Inline<T>
   |               ^ required by this bound in `Inline`
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Inline<T: ?Sized>

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0080, E0277, E0599.
---
18 LL | struct S;
-    | --------- method `f` not found for this
+    |        - method `f` not found for this
20 ...
21 LL |     S.f::<0>();
22    |       ^ method not found in `S`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/invalid-const-arg-for-type-param.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/invalid-const-arg-for-type-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-const-arg-for-type-param/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     let _: u32 = 5i32.try_into::<32>().unwrap();
   |                       ^^^^^^^^------ help: remove these generics
   |                       expected 0 generic arguments
   |
note: associated function defined here, with 0 generic parameters
  --> /checkout/library/core/src/convert/mod.rs:400:8
---
   |
LL | struct S;
   |        - method `f` not found for this
...
LL |     S.f::<0>();
   |       ^ method not found in `S`

error[E0107]: this struct takes 0 generic arguments but 1 generic argument was supplied
   |
   |
LL |     S::<0>;
   |     ^----- help: remove these generics
   |     expected 0 generic arguments
   |
note: struct defined here, with 0 generic parameters
  --> /checkout/src/test/ui/const-generics/invalid-const-arg-for-type-param.rs:3:8
---
diff of stderr:

2   --> $DIR/const-needs_drop-monomorphic.rs:11:46
3    |
4 LL | struct Bool<const B: bool> {}
-    | -------------------------- function or associated item `assert` not found for this
+    |        ---- function or associated item `assert` not found for this
6 ...
7 LL |     Bool::<{ std::mem::needs_drop::<T>() }>::assert();
8    |                                              ^^^^^^ function or associated item cannot be called on `Bool<{ std::mem::needs_drop::<T>() }>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-needs_drop-monomorphic/const-needs_drop-monomorphic.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-needs_drop-monomorphic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-needs_drop-monomorphic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-needs_drop-monomorphic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-needs_drop-monomorphic/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no function or associated item named `assert` found for struct `Bool<{ std::mem::needs_drop::<T>() }>` in the current scope
   |
   |
LL | struct Bool<const B: bool> {}
   |        ---- function or associated item `assert` not found for this
...
LL |     Bool::<{ std::mem::needs_drop::<T>() }>::assert();
   |                                              ^^^^^^ function or associated item cannot be called on `Bool<{ std::mem::needs_drop::<T>() }>` due to unsatisfied trait bounds
error: unconstrained generic constant
  --> /checkout/src/test/ui/consts/const-needs_drop-monomorphic.rs:11:5
   |
   |
LL |     Bool::<{ std::mem::needs_drop::<T>() }>::assert();
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { std::mem::needs_drop::<T>() }]:`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
---
4 LL | struct Foo {
-    | ---------- method `clone` not found for this
+    |        --- method `clone` not found for this
6 ...
7 LL |     let _y = x.clone();
8    |                ^^^^^ method not found in `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/copy-a-resource/copy-a-resource.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args copy-a-resource.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/copy-a-resource.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/copy-a-resource" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/copy-a-resource/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/copy-a-resource.rs:18:16
   |
LL | struct Foo {
   |        --- method `clone` not found for this
   |        --- method `clone` not found for this
...
LL |     let _y = x.clone();
   |                ^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/derives/derive-assoc-type-not-impl.rs stdout ----
diff of stderr:

3    |
4 LL | struct Bar<T: Foo> {
-    | |
-    | method `clone` not found for this
+    | |      |
+    | |      method `clone` not found for this
+    | |      method `clone` not found for this
8    | doesn't satisfy `Bar<NotClone>: Clone`
9 ...
10 LL | struct NotClone;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl/derive-assoc-type-not-impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/derive-assoc-type-not-impl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derive-assoc-type-not-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derive-assoc-type-not-impl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for struct `Bar<NotClone>`, but its trait bounds were not satisfied
   |
   |
LL | struct Bar<T: Foo> {
   | |      |
   | |      method `clone` not found for this
   | |      method `clone` not found for this
   | doesn't satisfy `Bar<NotClone>: Clone`
...
LL | struct NotClone;
   | ---------------- doesn't satisfy `NotClone: Clone`
...
LL |     Bar::<NotClone> { x: 1 }.clone(); //~ ERROR
   |                              ^^^^^ method cannot be called on `Bar<NotClone>` due to unsatisfied trait bounds
   |
note: trait bound `NotClone: Clone` was not satisfied
   |
LL | #[derive(Clone)]
   |          ^^^^^ unsatisfied trait bound introduced in this `derive` macro
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NotClone: Clone`
           which is required by `Bar<NotClone>: Clone`
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
help: consider annotating `NotClone` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to previous error
---

---- [ui] src/test/ui/derives/issue-91550.rs stdout ----
diff of stderr:

25    | --------------------- doesn't satisfy `NoDerives: Eq`
26 LL |
27 LL | struct Object<T>(T);
-    | -------------------- method `use_eq` not found for this
+    |        ------ method `use_eq` not found for this
29 ...
30 LL |     foo.use_eq();
31    |         ^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds

44    | --------------------- doesn't satisfy `NoDerives: Ord`
45 LL |
46 LL | struct Object<T>(T);
-    | -------------------- method `use_ord` not found for this
+    |        ------ method `use_ord` not found for this
48 ...
49 LL |     foo.use_ord();
50    |         ^^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds

66    | doesn't satisfy `NoDerives: PartialOrd`
67 LL |
68 LL | struct Object<T>(T);
-    | -------------------- method `use_ord_and_partial_ord` not found for this
+    |        ------ method `use_ord_and_partial_ord` not found for this
70 ...
71 LL |     foo.use_ord_and_partial_ord();
72    |         ^^^^^^^^^^^^^^^^^^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91550/issue-91550.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/issue-91550.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-91550.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91550" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91550/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `insert` exists for struct `HashSet<Value>`, but its trait bounds were not satisfied
   |
LL | struct Value(u32);
   | ------------------
   | |
   | |
   | doesn't satisfy `Value: Eq`
   | doesn't satisfy `Value: Hash`
...
LL |     hs.insert(Value(0)); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `Value: Eq`
           `Value: Hash`
help: consider annotating `Value` with `#[derive(Eq, Hash, PartialEq)]`
   |
LL | #[derive(Eq, Hash, PartialEq)]


error[E0599]: the method `use_eq` exists for struct `Object<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | --------------------- doesn't satisfy `NoDerives: Eq`
LL |
LL | struct Object<T>(T);
   |        ------ method `use_eq` not found for this
...
LL |     foo.use_eq(); //~ ERROR
   |         ^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NoDerives: Eq`
help: consider annotating `NoDerives` with `#[derive(Eq, PartialEq)]`
   |
LL | #[derive(Eq, PartialEq)]


error[E0599]: the method `use_ord` exists for struct `Object<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | --------------------- doesn't satisfy `NoDerives: Ord`
LL |
LL | struct Object<T>(T);
   |        ------ method `use_ord` not found for this
...
LL |     foo.use_ord(); //~ ERROR
   |         ^^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NoDerives: Ord`
help: consider annotating `NoDerives` with `#[derive(Eq, Ord, PartialEq, PartialOrd)]`
   |
LL | #[derive(Eq, Ord, PartialEq, PartialOrd)]


error[E0599]: the method `use_ord_and_partial_ord` exists for struct `Object<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | |
   | |
   | doesn't satisfy `NoDerives: Ord`
   | doesn't satisfy `NoDerives: PartialOrd`
LL |
LL | struct Object<T>(T);
   |        ------ method `use_ord_and_partial_ord` not found for this
...
LL |     foo.use_ord_and_partial_ord(); //~ ERROR
   |         ^^^^^^^^^^^^^^^^^^^^^^^ method cannot be called on `Object<NoDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `NoDerives: Ord`
           `NoDerives: PartialOrd`
help: consider annotating `NoDerives` with `#[derive(Eq, Ord, PartialEq, PartialOrd)]`
   |
LL | #[derive(Eq, Ord, PartialEq, PartialOrd)]

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

37    | --------------------- doesn't satisfy `NoDerives: Clone`
38 ...
39 LL | struct Object<T, A>(T, A);
-    | -------------------------- method `use_clone` not found for this
+    |        ------ method `use_clone` not found for this
41 ...
42 LL |     foo.use_clone();
43    |         ^^^^^^^^^ method cannot be called on `Object<NoDerives, SomeDerives>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492/issue-91492.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args derives/issue-91492.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-91492.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-91492/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<NoDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | --------------------- doesn't satisfy `NoDerives: Clone`
LL | fn fun1(foo: &mut Vec<NoDerives>, bar: &[NoDerives]) {
LL |     foo.extend_from_slice(bar); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
           `NoDerives: Clone`
           `NoDerives: Clone`
help: consider annotating `NoDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `extend_from_slice` exists for mutable reference `&mut Vec<SomeDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct SomeDerives;
   | ----------------------- doesn't satisfy `SomeDerives: Clone`
LL | fn fun2(foo: &mut Vec<SomeDerives>, bar: &[SomeDerives]) {
LL |     foo.extend_from_slice(bar); //~ ERROR
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `SomeDerives: Clone`
help: consider annotating `SomeDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |


error[E0599]: the method `use_clone` exists for struct `Object<NoDerives, SomeDerives>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct NoDerives;
   | --------------------- doesn't satisfy `NoDerives: Clone`
...
LL | struct Object<T, A>(T, A);
   |        ------ method `use_clone` not found for this
...
LL |     foo.use_clone(); //~ ERROR
   |         ^^^^^^^^^ method cannot be called on `Object<NoDerives, SomeDerives>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
           `NoDerives: Clone`
           `NoDerives: Clone`
help: consider annotating `NoDerives` with `#[derive(Clone)]`
LL | #[derive(Clone)]
   |

error: aborting due to 3 previous errors
---
diff of stderr:

88   --> $DIR/issue-40006.rs:38:7
89    |
90 LL | struct S;
-    | --------- method `hello_method` not found for this
+    |        - method `hello_method` not found for this
92 ...
93 LL |     S.hello_method();
94    |       ^^^^^^^^^^^^ method not found in `S`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006/issue-40006.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-40006.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40006.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40006/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `!` or `::`, found `}`
   |
   |
LL | impl dyn A {
   |            - while parsing this item list starting here
LL |     Y
   |      - expected one of `!` or `::`
LL | } //~ ERROR expected one of `!` or `::`, found `}`
   | |
   | unexpected token
   | unexpected token
   | the item list ends here

error: expected one of `!` or `::`, found `(`
   |
   |
LL | trait X {
   |         - while parsing this item list starting here
LL |     X() {} //~ ERROR expected one of `!` or `::`, found `(`
   |      ^ expected one of `!` or `::`
LL | }
LL | }
   | - the item list ends here

error: expected one of `!` or `::`, found `(`
   |
   |
LL | trait A {
   |         - while parsing this item list starting here
LL |     X() {} //~ ERROR expected one of `!` or `::`, found `(`
   |      ^ expected one of `!` or `::`
LL | }
   | - the item list ends here

error: expected one of `!` or `[`, found `#`
   |
   |
LL |     fn xxx() { ### } //~ ERROR expected
   |                 ^ expected one of `!` or `[`

error: expected one of `!` or `::`, found `=`
   |
LL | trait C {
   |         - while parsing this item list starting here
   |         - while parsing this item list starting here
LL |     L = M; //~ ERROR expected one of `!` or `::`, found `=`
   |       ^ expected one of `!` or `::`
LL | }
   | - the item list ends here

error: expected one of `!` or `::`, found `=`
   |
   |
LL | trait D {
   |         - while parsing this item list starting here
LL |     Z = { 2 + 3 }; //~ ERROR expected one of `!` or `::`, found `=`
   |       ^ expected one of `!` or `::`
LL | }
   | - the item list ends here

error: expected one of `!` or `::`, found `(`
   |
LL | trait E {
   |         - while parsing this item list starting here
   |         - while parsing this item list starting here
LL |     ::Y (); //~ ERROR expected one of
   |         ^ expected one of `!` or `::`
LL | }
   | - the item list ends here

error: missing `fn` for method definition
   |
LL | impl S {
   |        - while parsing this item list starting here
   |        - while parsing this item list starting here
LL |     pub hello_method(&self) { //~ ERROR missing
...
LL | }
LL | }
   | - the item list ends here
   |
help: add `fn` here to parse `hello_method` as a public method
   |
LL |     pub fn hello_method(&self) { //~ ERROR missing


error[E0599]: no method named `hello_method` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   |        - method `hello_method` not found for this
...
LL |     S.hello_method(); //~ no method named `hello_method` found
   |       ^^^^^^^^^^^^ method not found in `S`
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/dont-suggest-private-trait-method.rs stdout ----
diff of stderr:

2   --> $DIR/dont-suggest-private-trait-method.rs:4:8
3    |
4 LL | struct T;
-    | --------- function or associated item `new` not found for this
+    |        - function or associated item `new` not found for this
7 LL |     T::new();
8    |        ^^^ function or associated item not found in `T`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dont-suggest-private-trait-method/dont-suggest-private-trait-method.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dont-suggest-private-trait-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dont-suggest-private-trait-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dont-suggest-private-trait-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dont-suggest-private-trait-method/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no function or associated item named `new` found for struct `T` in the current scope
   |
LL | struct T;
LL | struct T;
   |        - function or associated item `new` not found for this
LL |     T::new();
   |        ^^^ function or associated item not found in `T`

error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/E0599.rs:4:20
3    |
4 LL | struct Foo;
-    | ----------- associated item `NotEvenReal` not found for this
+    |        --- associated item `NotEvenReal` not found for this
6 ...
7 LL |     || if let Foo::NotEvenReal() = Foo {};
8    |                    ^^^^^^^^^^^ associated item not found in `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0599/E0599.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0599.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0599.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0599" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0599/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no associated item named `NotEvenReal` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   |        --- associated item `NotEvenReal` not found for this
...
LL |     || if let Foo::NotEvenReal() = Foo {}; //~ ERROR E0599
   |                    ^^^^^^^^^^^ associated item not found in `Foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
---
5    | ---------
-    | |
-    | method `f` not found for this
+    | |      |
+    | |      method `f` not found for this
8    | doesn't satisfy `<S as X>::Y<i32> = i32`
9    | doesn't satisfy `S: M`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate/method-unsatified-assoc-type-predicate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate/method-unsatified-assoc-type-predicate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/method-unsatified-assoc-type-predicate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/method-unsatified-assoc-type-predicate/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `f` exists for struct `S`, but its trait bounds were not satisfied
   |
LL | struct S;
   | ---------
   | |      |
   | |      |
   | |      method `f` not found for this
   | doesn't satisfy `<S as X>::Y<i32> = i32`
   | doesn't satisfy `S: M`
LL |     a.f();
   |       ^ method cannot be called on `S` due to unsatisfied trait bounds
   |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: trait bound `<S as X>::Y<i32> = i32` was not satisfied
   |
   |
LL | impl<T: X<Y<i32> = i32>> M for T {}
   |           ^^^^^^^^^^^^   -     -
   |           unsatisfied trait bound introduced here

error: aborting due to previous error

---
---- [ui] src/test/ui/hrtb/issue-30786.rs stdout ----
diff of stderr:

3    |
4 LL | pub struct Map<S, F> {
-    | |
-    | |
-    | method `filterx` not found for this
+    | |          |
+    | |          method `filterx` not found for this
8    | doesn't satisfy `_: StreamExt`
9 ...
10 LL |     let filter = map.filterx(|x: &_| true);
28    |
28    |
29 LL | pub struct Filter<S, F> {
-    | |
-    | |
-    | method `countx` not found for this
+    | |          |
+    | |          method `countx` not found for this
33    | doesn't satisfy `_: StreamExt`
34 ...
35 LL |     let count = filter.countx();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/issue-30786.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hrtb/issue-30786.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/issue-30786.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/issue-30786/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Map<S, F> {
   | |          |
   | |          |
   | |          method `filterx` not found for this
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>: Stream`
      `&'a mut &mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>: Stream`
      `&'a mut Map<Repeat, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:117:27: 117:36]>: Stream`
   |
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
help: one of the expressions' fields has a method of the same name
   |
LL |     let filter = map.stream.filterx(|x: &_| true);


error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>`, but its trait bounds were not satisfied
   |
   |
LL | pub struct Filter<S, F> {
   | |          |
   | |          |
   | |          method `countx` not found for this
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |                        ^^^^^^ method cannot be called on `Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>: Stream`
      `&'a mut &mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>: Stream`
      `&'a mut Filter<Map<Repeat, for<'r> fn(&'r u64) -> &'r u64 {identity::<u64>}>, [closure@/checkout/src/test/ui/hrtb/issue-30786.rs:129:30: 129:42]>: Stream`
   |
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
help: one of the expressions' fields has a method of the same name
   |
LL |     let count = filter.stream.countx();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
---
diff of stderr:

2   --> $DIR/method-suggestion-no-duplication.rs:7:15
3    |
4 LL | struct Foo;
-    | ----------- method `is_empty` not found for this
+    |        --- method `is_empty` not found for this
6 ...
7 LL |     foo(|s| s.is_empty());
8    |               ^^^^^^^^ method not found in `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/method-suggestion-no-duplication/method-suggestion-no-duplication.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/method-suggestion-no-duplication.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/method-suggestion-no-duplication.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/method-suggestion-no-duplication" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/method-suggestion-no-duplication/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `is_empty` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   |        --- method `is_empty` not found for this
...
LL |     foo(|s| s.is_empty());
   |               ^^^^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `is_empty`, perhaps you need to implement it:
           candidate #1: `ExactSizeIterator`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/infinite/infinite-autoderef.rs stdout ----
diff of stderr:

43   --> $DIR/infinite-autoderef.rs:25:9
44    |
45 LL | struct Foo;
-    | ----------- method `bar` not found for this
+    |        --- method `bar` not found for this
48 LL |     Foo.bar();
49    |         ^^^ method not found in `Foo`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/infinite-autoderef.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-autoderef.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zdeduplicate-diagnostics=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-autoderef/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:19:13
   |
LL |         x = Box::new(x);
LL |         x = Box::new(x);
   |             ^^^^^^^^^^^ cyclic type of infinite size
help: consider unboxing the value
   |
   |
LL |         x = *Box::new(x);


error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
LL |     Foo.foo;
   |     ^^^^^^^ deref recursion limit reached
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
LL |     Foo.foo;
   |         ^^^ deref recursion limit reached
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)
error[E0609]: no field `foo` on type `Foo`
  --> /checkout/src/test/ui/infinite/infinite-autoderef.rs:24:9
   |
LL |     Foo.foo;
LL |     Foo.foo;
   |         ^^^ unknown field

error[E0055]: reached the recursion limit while auto-dereferencing `Foo`
   |
LL |     Foo.bar();
   |         ^^^ deref recursion limit reached
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`infinite_autoderef`)

error[E0599]: no method named `bar` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   |        --- method `bar` not found for this
LL |     Foo.bar();
   |         ^^^ method not found in `Foo`

error: aborting due to 6 previous errors
---
51    |
52 LL | pub struct Foo;
53    | ---------------
-    | |
-    | method `take` not found for this
+    | |          |
+    | |          method `take` not found for this
56    | doesn't satisfy `Foo: Iterator`
58 LL |      .take()


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/method-call-err-msg.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-call-err-msg.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-call-err-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-err-msg/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:13:7
   |
   |
LL |     x.zero(0)   //~ ERROR this function takes 0 arguments but 1 argument was supplied
   |       ^^^^ - argument unexpected
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:5:8
   |
   |
LL |     fn zero(self) -> Foo { self }
help: remove the extra argument
   |
   |
LL |     x.zero()   //~ ERROR this function takes 0 arguments but 1 argument was supplied

error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:14:7
   |
   |
LL |      .one()     //~ ERROR this function takes 1 argument but 0 arguments were supplied
   |       ^^^-- an argument of type `isize` is missing
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:6:8
   |
   |
LL |     fn one(self, _: isize) -> Foo { self }
help: provide the argument
   |
   |
LL |      .one(/* isize */)     //~ ERROR this function takes 1 argument but 0 arguments were supplied

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:15:7
   |
   |
LL |      .two(0);   //~ ERROR this function takes 2 arguments but 1 argument was supplied
   |       ^^^--- an argument of type `isize` is missing
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:7:8
   |
   |
LL |     fn two(self, _: isize, _: isize) -> Foo { self }
help: provide the argument
   |
   |
LL |      .two(0, /* isize */);   //~ ERROR this function takes 2 arguments but 1 argument was supplied

error[E0599]: `Foo` is not an iterator
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:19:7
   |
   |
LL | pub struct Foo;
   | ---------------
   | |          |
   | |          method `take` not found for this
   | doesn't satisfy `Foo: Iterator`
...
LL |      .take()    //~ ERROR not an iterator
   |       ^^^^ `Foo` is not an iterator
   = note: the following trait bounds were not satisfied:
           `Foo: Iterator`
           which is required by `&mut Foo: Iterator`
note: the following trait must be implemented
note: the following trait must be implemented
  --> /checkout/library/core/src/iter/traits/iterator.rs:66:1
   |
LL | / pub trait Iterator {
LL | |     /// The type of the elements being iterated over.
LL | |     #[stable(feature = "rust1", since = "1.0.0")]
LL | |     type Item;
LL | |     }
LL | | }
   | |_^
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `take`, perhaps you need to implement it:
           candidate #1: `Iterator`
error[E0061]: this function takes 3 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:21:7
   |
   |
LL |     y.three::<usize>(); //~ ERROR this function takes 3 arguments but 0 arguments were supplied
   |       ^^^^^^^^^^^^^^-- three arguments of type `usize`, `usize`, and `usize` are missing
note: associated function defined here
  --> /checkout/src/test/ui/methods/method-call-err-msg.rs:8:8
   |
   |
LL |     fn three<T>(self, _: T, _: T, _: T) -> Foo { self }
help: provide the arguments
   |
   |
LL |     y.three::<usize>(/* usize */, /* usize */, /* usize */); //~ ERROR this function takes 3 arguments but 0 arguments were supplied

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0061, E0599.
---
diff of stderr:

2   --> $DIR/method-not-found-generic-arg-elision.rs:82:23
3    |
4 LL | struct Point<T> {
-    | --------------- method `distance` not found for this
+    |        ----- method `distance` not found for this
6 ...
7 LL |     let d = point_i32.distance();
8    |                       ^^^^^^^^ method not found in `Point<i32>`
14   --> $DIR/method-not-found-generic-arg-elision.rs:84:23
15    |
16 LL | struct Point<T> {
16 LL | struct Point<T> {
-    | --------------- method `other` not found for this
+    |        ----- method `other` not found for this
18 ...
19 LL |     let d = point_i32.other();
20    |                       ^^^^^ method not found in `Point<i32>`
29   --> $DIR/method-not-found-generic-arg-elision.rs:90:13
30    |
30    |
31 LL | struct Wrapper<T>(T);
-    | --------------------- method `method` not found for this
+    |        ------- method `method` not found for this
33 ...
34 LL |     wrapper.method();
35    |             ^^^^^^ method not found in `Wrapper<bool>`
45   --> $DIR/method-not-found-generic-arg-elision.rs:92:13
46    |
46    |
47 LL | struct Wrapper<T>(T);
-    | --------------------- method `other` not found for this
+    |        ------- method `other` not found for this
50 LL |     wrapper.other();
50 LL |     wrapper.other();
51    |             ^^^^^ method not found in `Wrapper<bool>`
54   --> $DIR/method-not-found-generic-arg-elision.rs:96:13
55    |
55    |
56 LL | struct Wrapper2<'a, T, const C: usize> {
-    | -------------------------------------- method `method` not found for this
+    |        -------- method `method` not found for this
58 ...
59 LL |     wrapper.method();
60    |             ^^^^^^ method not found in `Wrapper2<'_, bool, 3_usize>`
68   --> $DIR/method-not-found-generic-arg-elision.rs:98:13
69    |
69    |
70 LL | struct Wrapper2<'a, T, const C: usize> {
-    | -------------------------------------- method `other` not found for this
+    |        -------- method `other` not found for this
73 LL |     wrapper.other();
73 LL |     wrapper.other();
74    |             ^^^^^ method not found in `Wrapper2<'_, bool, 3_usize>`
83   --> $DIR/method-not-found-generic-arg-elision.rs:104:7
84    |
84    |
85 LL | struct Struct<T>{
-    | ---------------- method `method` not found for this
+    |        ------ method `method` not found for this
87 ...
88 LL |     s.method();
89    |       ^^^^^^ method cannot be called on `Struct<f64>` due to unsatisfied trait bounds

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/method-not-found-generic-arg-elision.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args methods/method-not-found-generic-arg-elision.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-not-found-generic-arg-elision/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `distance` found for struct `Point<i32>` in the current scope
   |
LL | struct Point<T> {
LL | struct Point<T> {
   |        ----- method `distance` not found for this
...
LL |     let d = point_i32.distance();
   |                       ^^^^^^^^ method not found in `Point<i32>`
   = note: the method was found for
   = note: the method was found for
           - `Point<f64>`

error[E0599]: no method named `other` found for struct `Point` in the current scope
   |
LL | struct Point<T> {
LL | struct Point<T> {
   |        ----- method `other` not found for this
...
LL |     let d = point_i32.other();
   |                       ^^^^^ method not found in `Point<i32>`
error[E0599]: no method named `extend` found for struct `Map` in the current scope
  --> /checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:87:29
   |
   |
LL |     v.iter().map(|x| x * x).extend(std::iter::once(100));
   |                             ^^^^^^ method not found in `Map<std::slice::Iter<'_, i32>, [closure@/checkout/src/test/ui/methods/method-not-found-generic-arg-elision.rs:87:18: 87:27]>`

error[E0599]: no method named `method` found for struct `Wrapper<bool>` in the current scope
   |
   |
LL | struct Wrapper<T>(T);
   |        ------- method `method` not found for this
...
LL |     wrapper.method();
   |             ^^^^^^ method not found in `Wrapper<bool>`
   = note: the method was found for
   = note: the method was found for
           - `Wrapper<i8>`
           - `Wrapper<i16>`
           - `Wrapper<i32>`
           - `Wrapper<i64>`
           and 2 more types

error[E0599]: no method named `other` found for struct `Wrapper` in the current scope
   |
   |
LL | struct Wrapper<T>(T);
   |        ------- method `other` not found for this
LL |     wrapper.other();
LL |     wrapper.other();
   |             ^^^^^ method not found in `Wrapper<bool>`

error[E0599]: no method named `method` found for struct `Wrapper2<'_, bool, 3_usize>` in the current scope
   |
   |
LL | struct Wrapper2<'a, T, const C: usize> {
   |        -------- method `method` not found for this
...
LL |     wrapper.method();
   |             ^^^^^^ method not found in `Wrapper2<'_, bool, 3_usize>`
   = note: the method was found for
   = note: the method was found for
           - `Wrapper2<'a, i8, C>`
           - `Wrapper2<'a, i16, C>`
           - `Wrapper2<'a, i32, C>`

error[E0599]: no method named `other` found for struct `Wrapper2` in the current scope
   |
   |
LL | struct Wrapper2<'a, T, const C: usize> {
   |        -------- method `other` not found for this
LL |     wrapper.other();
LL |     wrapper.other();
   |             ^^^^^ method not found in `Wrapper2<'_, bool, 3_usize>`

error[E0599]: no method named `not_found` found for struct `Vec<{integer}>` in the current scope
   |
LL |     a.not_found();
LL |     a.not_found();
   |       ^^^^^^^^^ method not found in `Vec<{integer}>`

error[E0599]: the method `method` exists for struct `Struct<f64>`, but its trait bounds were not satisfied
   |
   |
LL | struct Struct<T>{
   |        ------ method `method` not found for this
...
LL |     s.method();
   |       ^^^^^^ method cannot be called on `Struct<f64>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `f64: Eq`
           `f64: Ord`
error: aborting due to 9 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
---
4 LL | struct Foo {
-    | ---------- method `clone` not found for this
+    |        --- method `clone` not found for this
6 ...
7 LL |     let _y = x.clone();
8    |                ^^^^^ method not found in `Foo`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/noncopyable-class/noncopyable-class.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args noncopyable-class.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/noncopyable-class.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/noncopyable-class" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/noncopyable-class/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/noncopyable-class.rs:34:16
   |
LL | struct Foo {
   |        --- method `clone` not found for this
   |        --- method `clone` not found for this
...
LL |     let _y = x.clone(); //~ ERROR no method named `clone` found
   |                ^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `clone`, perhaps you need to implement it:
           candidate #1: `Clone`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/parser/emoji-identifiers.rs stdout ----
diff of stderr:

77   --> $DIR/emoji-identifiers.rs:9:8
78    |
79 LL | struct 👀;
-    | ---------- function or associated item `full_of✨` not found for this
+    |        -- function or associated item `full_of✨` not found for this
81 ...
82 LL |     👀::full_of✨()


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/emoji-identifiers.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/emoji-identifiers.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/emoji-identifiers.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/emoji-identifiers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/auxiliary"
stdout: none
--- stderr -------------------------------
error: unknown start of token: \u{2796}
   |
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
help: Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
   |
LL |     let _ = i_like_to_😄_a_lot() - 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error[E0425]: cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   | ----------------------------- similarly named function `i_like_to_😅_a_lot` defined here
...
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |             ^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `i_like_to_😅_a_lot`
error: Ferris cannot be used as an identifier
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:17:9
   |
   |
LL |     let 🦀 = 1;//~ ERROR Ferris cannot be used as an identifier
   |         ^^ help: try using their name instead: `ferris`
LL |     dbg!(🦀);


error: identifiers cannot contain emoji: `ABig👩👩👧👧Family`
   |
   |
LL | struct ABig👩👩👧👧Family; //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `👀`
   |
   |
LL | struct 👀; //~ ERROR identifiers cannot contain emoji
LL | impl 👀 {
   |      ^^
   |      ^^
LL |     fn full_of_✨() -> 👀 { //~ ERROR identifiers cannot contain emoji
LL |         👀
   |         ^^
...
...
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   |                            ^^
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`


error: identifiers cannot contain emoji: `full_of_✨`
   |
   |
LL |     fn full_of_✨() -> 👀 { //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `i_like_to_😅_a_lot`
   |
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `full_of✨`
   |
   |
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`


error: identifiers cannot contain emoji: `i_like_to_😄_a_lot`
   |
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error[E0599]: no function or associated item named `full_of✨` found for struct `👀` in the current scope
   |
   |
LL | struct 👀; //~ ERROR identifiers cannot contain emoji
   |        -- function or associated item `full_of✨` not found for this
...
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`
   |         |
   |         function or associated item not found in `👀`
   |         function or associated item not found in `👀`
   |         help: there is an associated function with a similar name: `full_of_✨`
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
---
diff of stderr:

2   --> $DIR/issue-87932.rs:13:8
3    |
4 LL | pub struct A {}
-    | ------------ function or associated item `deserialize` not found for this
+    |            - function or associated item `deserialize` not found for this
7 LL |     A::deserialize();
8    |        ^^^^^^^^^^^ function or associated item not found in `A`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-87932/issue-87932.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rust-2018/uniform-paths/issue-87932.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/uniform-paths/issue-87932.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-87932" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-87932/auxiliary" "--extern" "issue_87932_a=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/uniform-paths/issue-87932/auxiliary/libissue_87932_a.so"
stdout: none
--- stderr -------------------------------
error[E0599]: no function or associated item named `deserialize` found for struct `A` in the current scope
   |
LL | pub struct A {}
LL | pub struct A {}
   |            - function or associated item `deserialize` not found for this
LL |     A::deserialize();
   |        ^^^^^^^^^^^ function or associated item not found in `A`
   |
   = help: items from traits can only be used if the trait is in scope
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use <crate::A as issue_87932_a::Deserialize>::deserialize::_a::Deserialize;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
4 LL | struct A;
-    | --------- method `foo` not found for this
+    |        - method `foo` not found for this
6 ...
7 LL |     fn foo(self: Box<Self>) {}
8    |        --- the method is available for `Box<A>` here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/point-at-arbitrary-self-type-method/point-at-arbitrary-self-type-method.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/point-at-arbitrary-self-type-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/point-at-arbitrary-self-type-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/point-at-arbitrary-self-type-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/point-at-arbitrary-self-type-method/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `foo` found for struct `A` in the current scope
   |
LL | struct A;
   |        - method `foo` not found for this
...
...
LL |     fn foo(self: Box<Self>) {}
   |        --- the method is available for `Box<A>` here
...
LL |     A.foo(); //~ ERROR E0599
   |       ^^^ method not found in `A`
help: consider wrapping the receiver expression with the appropriate type
   |
   |
LL |     Box::new(A).foo(); //~ ERROR E0599
   |     +++++++++ +
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/self/point-at-arbitrary-self-type-trait-method.rs stdout ----
diff of stderr:

6    |              |
7    |              the method is available for `Box<A>` here
8 LL | struct A;
-    | --------- method `foo` not found for this
+    |        - method `foo` not found for this
11 LL |     A.foo()
12    |       ^^^ method not found in `A`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/point-at-arbitrary-self-type-trait-method/point-at-arbitrary-self-type-trait-method.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/point-at-arbitrary-self-type-trait-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/point-at-arbitrary-self-type-trait-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/point-at-arbitrary-self-type-trait-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/point-at-arbitrary-self-type-trait-method/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `foo` found for struct `A` in the current scope
   |
   |
LL | trait B { fn foo(self: Box<Self>); }
   |              ---       --------- the method might not be found because of this arbitrary self type
   |              |
   |              the method is available for `Box<A>` here
LL | struct A;
   |        - method `foo` not found for this
...
LL |     A.foo() //~ ERROR E0599
   |       ^^^ method not found in `A`
help: consider wrapping the receiver expression with the appropriate type
   |
   |
LL |     Box::new(A).foo() //~ ERROR E0599
   |     +++++++++ +
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/span/issue-7575.rs stdout ----
diff of stderr:

42   --> $DIR/issue-7575.rs:62:30
43    |
44 LL | struct Myisize(isize);
-    | ---------------------- method `fff` not found for this
+    |        ------- method `fff` not found for this
46 ...
47 LL |     u.f8(42) + u.f9(342) + m.fff(42)
48    |                            --^^^

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/issue-7575.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-7575.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-7575.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-7575/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `f9` found for type `usize` in the current scope
   |
   |
LL |     u.f8(42) + u.f9(342) + m.fff(42)
   |                  ^^ this is an associated function, not a method
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: candidate #1 is defined in the trait `CtxtFn`
   |
   |
LL |     fn f9(_: usize) -> usize;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #2 is defined in the trait `OtherTrait`
   |
   |
LL |     fn f9(_: usize) -> usize;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^
note: candidate #3 is defined in the trait `UnusedTrait`
   |
   |
LL |     fn f9(_: usize) -> usize;
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `f9`, perhaps you need to implement one of them:
           candidate #1: `CtxtFn`
           candidate #2: `OtherTrait`
           candidate #3: `UnusedTrait`
help: disambiguate the associated function for candidate #1
   |
LL |     u.f8(42) + <usize as CtxtFn>::f9(u, 342) + m.fff(42)
help: disambiguate the associated function for candidate #2
   |
   |
LL |     u.f8(42) + <usize as OtherTrait>::f9(u, 342) + m.fff(42)
help: disambiguate the associated function for candidate #3
   |
   |
LL |     u.f8(42) + <usize as UnusedTrait>::f9(u, 342) + m.fff(42)


error[E0599]: no method named `fff` found for struct `Myisize` in the current scope
   |
   |
LL | struct Myisize(isize);
   |        ------- method `fff` not found for this
...
LL |     u.f8(42) + u.f9(342) + m.fff(42)
   |                            --^^^
   |                            | this is an associated function, not a method
   |                            | this is an associated function, not a method
   |                            help: use associated function syntax instead: `Myisize::fff`
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `Myisize`
   |
   |
LL |     fn fff(i: isize) -> isize {

error[E0599]: no method named `is_str` found for type parameter `T` in the current scope
  --> /checkout/src/test/ui/span/issue-7575.rs:70:7
   |
   |
LL | fn param_bound<T: ManyImplTrait>(t: T) -> bool {
   |                - method `is_str` not found for this
LL |     t.is_str()
   |       ^^^^^^ this is an associated function, not a method
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in the trait `ManyImplTrait`
   |
LL |     fn is_str() -> bool {
   |     ^^^^^^^^^^^^^^^^^^^
   = help: items from traits can only be used if the type parameter is bounded by the trait
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: disambiguate the associated function for the candidate
   |
LL |     <T as ManyImplTrait>::is_str(t)

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs stdout ----
diff of stderr:

13    |
14 LL | struct MyStruct;
-    | |
-    | |
-    | method `foo_one` not found for this
+    | |      |
+    | |      method `foo_one` not found for this
18    | doesn't satisfy `MyStruct: Foo`
19 ...
20 LL |     println!("{}", MyStruct.foo_one());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented/specialization-trait-not-implemented.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args specialization/defaultimpl/specialization-trait-not-implemented.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/specialization/defaultimpl/specialization-trait-not-implemented/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `specialization` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(specialization)] //~ WARN the feature `specialization` is incomplete
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #31844 <https://github.com/rust-lang/rust/issues/31844> for more information
   = help: consider using `min_specialization` instead, which is more stable and complete
   = help: consider using `min_specialization` instead, which is more stable and complete

error[E0599]: the method `foo_one` exists for struct `MyStruct`, but its trait bounds were not satisfied
   |
   |
LL | struct MyStruct;
   | |      |
   | |      |
   | |      method `foo_one` not found for this
   | doesn't satisfy `MyStruct: Foo`
...
LL |     println!("{}", MyStruct.foo_one());
   |                             ^^^^^^^ method cannot be called on `MyStruct` due to unsatisfied trait bounds
   |
note: trait bound `MyStruct: Foo` was not satisfied
   |
   |
LL | default impl<T> Foo for T {
note: the following trait must be implemented
  --> /checkout/src/test/ui/specialization/defaultimpl/specialization-trait-not-implemented.rs:7:1
   |
LL | / trait Foo {
LL | / trait Foo {
LL | |     fn foo_one(&self) -> &'static str;
LL | |     fn foo_two(&self) -> &'static str;
LL | | }
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Foo` defines an item `foo_one`, perhaps you need to implement it
   |
LL | trait Foo {
   | ^^^^^^^^^

---
---- [ui] src/test/ui/union/union-derive-clone.rs#mirunsafeck stdout ----
diff of stderr:

3    |
4 LL | union U5<T> {
-    | |
-    | method `clone` not found for this
+    | |     |
+    | |     method `clone` not found for this
+    | |     method `clone` not found for this
8    | doesn't satisfy `U5<CloneNoCopy>: Clone`
9 ...
10 LL | struct CloneNoCopy;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck/union-derive-clone.mirunsafeck.stderr
To only update this specific test, also pass `--test-args union/union-derive-clone.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for union `U5<CloneNoCopy>`, but its trait bounds were not satisfied
   |
   |
LL | union U5<T> {
   | |     |
   | |     method `clone` not found for this
   | |     method `clone` not found for this
   | doesn't satisfy `U5<CloneNoCopy>: Clone`
...
LL | struct CloneNoCopy;
   | ------------------- doesn't satisfy `CloneNoCopy: Copy`
...
LL |     let w = u.clone(); //~ ERROR the method
   |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds
   |
note: trait bound `CloneNoCopy: Copy` was not satisfied
   |
LL | #[derive(Clone, Copy)]
   |          ^^^^^ unsatisfied trait bound introduced in this `derive` macro
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `CloneNoCopy: Copy`
           which is required by `U5<CloneNoCopy>: Clone`
help: consider annotating `CloneNoCopy` with `#[derive(Clone, Copy)]`
LL | #[derive(Clone, Copy)]
   |


error[E0277]: the trait bound `U1: Copy` is not satisfied
   |
   |
LL | #[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
   |          ^^^^^ the trait `Copy` is not implemented for `U1`
   |
note: required by a bound in `AssertParamIsCopy`
   |
   |
LL | pub struct AssertParamIsCopy<T: Copy + ?Sized> {
   |                                 ^^^^ required by this bound in `AssertParamIsCopy`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `U1` with `#[derive(Copy)]`
   |
LL | #[derive(Copy)]

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
---
---- [ui] src/test/ui/union/union-derive-clone.rs#thirunsafeck stdout ----
diff of stderr:

3    |
4 LL | union U5<T> {
-    | |
-    | method `clone` not found for this
+    | |     |
+    | |     method `clone` not found for this
+    | |     method `clone` not found for this
8    | doesn't satisfy `U5<CloneNoCopy>: Clone`
9 ...
10 LL | struct CloneNoCopy;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck/union-derive-clone.thirunsafeck.stderr
To only update this specific test, also pass `--test-args union/union-derive-clone.rs`


error in revision `thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/union/union-derive-clone.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/union/union-derive-clone.thirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `clone` exists for union `U5<CloneNoCopy>`, but its trait bounds were not satisfied
   |
   |
LL | union U5<T> {
   | |     |
   | |     method `clone` not found for this
   | |     method `clone` not found for this
   | doesn't satisfy `U5<CloneNoCopy>: Clone`
...
LL | struct CloneNoCopy;
   | ------------------- doesn't satisfy `CloneNoCopy: Copy`
...
LL |     let w = u.clone(); //~ ERROR the method
   |               ^^^^^ method cannot be called on `U5<CloneNoCopy>` due to unsatisfied trait bounds
   |
note: trait bound `CloneNoCopy: Copy` was not satisfied
   |
LL | #[derive(Clone, Copy)]
   |          ^^^^^ unsatisfied trait bound introduced in this `derive` macro
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `CloneNoCopy: Copy`
           which is required by `U5<CloneNoCopy>: Clone`
help: consider annotating `CloneNoCopy` with `#[derive(Clone, Copy)]`
LL | #[derive(Clone, Copy)]
   |


error[E0277]: the trait bound `U1: Copy` is not satisfied
   |
   |
LL | #[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
   |          ^^^^^ the trait `Copy` is not implemented for `U1`
   |
note: required by a bound in `AssertParamIsCopy`
   |
   |
LL | pub struct AssertParamIsCopy<T: Copy + ?Sized> {
   |                                 ^^^^ required by this bound in `AssertParamIsCopy`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `U1` with `#[derive(Copy)]`
   |
LL | #[derive(Copy)]

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0599.
