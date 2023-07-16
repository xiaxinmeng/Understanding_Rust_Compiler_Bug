plain
.....................................................................................i.............. 8300/11882
.................................................................................................... 8400/11882
.............................................i...................................................... 8500/11882
.................................................................................................... 8600/11882
..................FF..F.F...F.F..FFF.F.FFF.FF.F..................................................... 8700/11882
.................................................................................................... 8900/11882
............................................iiii.iiiii.............................................. 9000/11882
..................ii...............i................................................................ 9100/11882
.................................................................................................... 9200/11882
.................................................................................................... 9200/11882
.................................................................................................... 9300/11882
.................................................................................................... 9400/11882
.................................................................................................... 9500/11882
.................................................................................................... 9600/11882
.................................................................................i......i........... 9700/11882
.................................................................................................... 9800/11882
..........................iiiiiii..iiiiii.i......................................................... 9900/11882
.................................................................................................... 10100/11882
.................................................................................................... 10200/11882
.................................................................................................... 10300/11882
.................................................................................................... 10400/11882
---

---- [ui] ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs stdout ----
diff of stderr:

1 error: cannot borrow value as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:37:9
3    |
3    |
4 LL |         ref foo @ [.., ref mut bar] => (),


8    |         immutable borrow, by `foo`, occurs here
9 
10 error: cannot borrow value as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:121:9
12    |
12    |
13 LL |         ref foo @ Some(box ref mut s) => (),


17    |         immutable borrow, by `foo`, occurs here
18 
19 error[E0382]: borrow of moved value: `x`
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:19:5
21    |
21    |
22 LL | fn bindings_after_at_slice_patterns_move_binding(x: [String; 4]) {
23    |                                                  - move occurs because `x` has type `[String; 4]`, which does not implement the `Copy` trait

29    |     ^^ value borrowed here after move
30 
31 error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:29:5
33    |
33    |
34 LL |         ref mut foo @ [.., _] => Some(foo),
35    |         --------------------- mutable borrow occurs here

41    |          - mutable borrow later used here
42 
43 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:51:5
45    |
45    |
46 LL |         [ref foo @ .., ref bar] => Some(foo),
47    |          ------------ immutable borrow occurs here

53    |          - immutable borrow later used here
54 
55 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:63:5
57    |
57    |
58 LL |         ref foo @ [.., ref bar] => Some(foo),
59    |         ----------------------- immutable borrow occurs here

65    |          - immutable borrow later used here
66 
67 error[E0382]: borrow of moved value: `x`
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:77:5
69    |
69    |
70 LL | fn bindings_after_at_or_patterns_move(x: Option<Test>) {
71    |                                       - move occurs because `x` has type `Option<Test>`, which does not implement the `Copy` trait

80    |     ^^ value borrowed here after move
81 
82 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:87:5
84    |
84    |
85 LL |         ref foo @ Some(Test::Foo | Test::Bar) => Some(foo),
86    |         ------------------------------------- immutable borrow occurs here

92    |          - immutable borrow later used here
93 
94 error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:99:5
96    |
96    |
97 LL |         ref mut foo @ Some(Test::Foo | Test::Bar) => Some(foo),
98    |         ----------------------------------------- mutable borrow occurs here

104    |          - mutable borrow later used here
105 
106 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:113:5
108    |
108    |
109 LL |         ref foo @ Some(box ref s) => Some(foo),
110    |         ------------------------- immutable borrow occurs here

116    |          - immutable borrow later used here
117 
118 error[E0382]: borrow of moved value: `x`
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:135:5
120    |
120    |
121 LL | fn bindings_after_at_slice_patterns_or_patterns_moves(x: [Option<Test>; 4]) {
122    |                                                       - move occurs because `x` has type `[Option<Test>; 4]`, which does not implement the `Copy` trait

131    |     ^^ value borrowed here after move
132 
133 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:145:5
135    |
135    |
136 LL |         ref a @ [ref b @ .., Some(Test::Foo | Test::Bar)] => Some(a),
137    |         ------------------------------------------------- immutable borrow occurs here

143    |          - immutable borrow later used here
144 
145 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:157:5
147    |
147    |
148 LL |         ref a @ [ref b @ .., Some(Test::Foo | Test::Bar)] => Some(b),
149    |                  ---------- immutable borrow occurs here

155    |          - immutable borrow later used here
156 
157 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:171:5
159    |
159    |
160 LL |         [_, ref a @ Some(box ref b), ..] => Some(a),
161    |             ----------------------- immutable borrow occurs here

167    |          - immutable borrow later used here
168 
169 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:187:5
171    |
171    |
172 LL |         [_, ref a @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
173    |             ------------------------------------------- immutable borrow occurs here

179    |          - immutable borrow later used here
180 
181 error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:201:5
183    |
183    |
184 LL |         [_, ref mut a @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
185    |             ----------------------------------------------- mutable borrow occurs here

191    |          - mutable borrow later used here
192 
193 error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
-   --> $DIR/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs:215:5
195    |
195    |
196 LL |         ref a @ [_, ref b @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
197    |         ------------------------------------------------------------ immutable borrow occurs here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns/bindings-after-at-or-patterns-slice-patterns-box-patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/bindings-after-at-or-patterns-slice-patterns-box-patterns/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ [.., ref mut bar] => (),
   |         |              |
   |         |              |
   |         |              mutable borrow, by `bar`, occurs here
   |         immutable borrow, by `foo`, occurs here

error: cannot borrow value as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ Some(box ref mut s) => (),
   |         |                  |
   |         |                  |
   |         |                  mutable borrow, by `s`, occurs here
   |         immutable borrow, by `foo`, occurs here

error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn bindings_after_at_slice_patterns_move_binding(x: [String; 4]) {
   |                                                  - move occurs because `x` has type `[String; 4]`, which does not implement the `Copy` trait
LL |     match x {
LL |         a @ [.., _] => (),
   |         ----------- value moved here
LL |     &x;
LL |     &x;
   |     ^^ value borrowed here after move

error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
   |
LL |         ref mut foo @ [.., _] => Some(foo),
   |         --------------------- mutable borrow occurs here
LL |     &x;
LL |     &x;
   |     ^^ immutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - mutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         [ref foo @ .., ref bar] => Some(foo),
   |          ------------ immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ [.., ref bar] => Some(foo),
   |         ----------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn bindings_after_at_or_patterns_move(x: Option<Test>) {
   |                                       - move occurs because `x` has type `Option<Test>`, which does not implement the `Copy` trait
LL |     match x {
LL |         foo @ Some(Test::Foo | Test::Bar) => (),
   |         |
   |         value moved here
   |         value moved here
...
...
LL |     &x;
   |     ^^ value borrowed here after move

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ Some(Test::Foo | Test::Bar) => Some(foo),
   |         ------------------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
   |
LL |         ref mut foo @ Some(Test::Foo | Test::Bar) => Some(foo),
   |         ----------------------------------------- mutable borrow occurs here
LL |     &x;
LL |     &x;
   |     ^^ immutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - mutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref foo @ Some(box ref s) => Some(foo),
   |         ------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0382]: borrow of moved value: `x`
   |
   |
LL | fn bindings_after_at_slice_patterns_or_patterns_moves(x: [Option<Test>; 4]) {
   |                                                       - move occurs because `x` has type `[Option<Test>; 4]`, which does not implement the `Copy` trait
LL |     match x {
LL |         a @ [.., Some(Test::Foo | Test::Bar)] => (),
   |         |
   |         value moved here
   |         value moved here
...
...
LL |     &x;
   |     ^^ value borrowed here after move

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref a @ [ref b @ .., Some(Test::Foo | Test::Bar)] => Some(a),
   |         ------------------------------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref a @ [ref b @ .., Some(Test::Foo | Test::Bar)] => Some(b),
   |                  ---------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         [_, ref a @ Some(box ref b), ..] => Some(a),
   |             ----------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         [_, ref a @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
   |             ------------------------------------------- immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here

error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
   |
   |
LL |         [_, ref mut a @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
   |             ----------------------------------------------- mutable borrow occurs here
LL |     &x;
LL |     &x;
   |     ^^ immutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - mutable borrow later used here

error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
   |
   |
LL |         ref a @ [_, ref b @ Some(box Test::Foo | box Test::Bar), ..] => Some(a),
   |         ------------------------------------------------------------ immutable borrow occurs here
LL |     &mut x;
LL |     &mut x;
   |     ^^^^^^ mutable borrow occurs here
LL |     drop(r);
LL |     drop(r);
   |          - immutable borrow later used here
error: aborting due to 17 previous errors

Some errors have detailed explanations: E0382, E0502.
For more information about an error, try `rustc --explain E0382`.
---
1 error[E0382]: use of partially moved value
-   --> $DIR/bind-by-move-no-subbindings-fun-param.rs:9:6
+   --> $DIR/bind-by-move-no-subbindings-fun-param.rs:7:6
3    |
4 LL | fn f(a @ A(u): A) -> Box<u8> {
5    |      ^^^^^^-^

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param/bind-by-move-no-subbindings-fun-param.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of partially moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-no-subbindings-fun-param.rs:7:6
   |
LL | fn f(a @ A(u): A) -> Box<u8> {
   |      ^^^^^^-^
   |      |     value partially moved here
   |      |     value partially moved here
   |      value used here after partial move
   |
   = note: partial move occurs because value has type `Box<u8>`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.


------------------------------------------


---- [ui] ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs stdout ----
diff of stderr:

1 error: cannot move out of value because it is borrowed
-   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:14:14
3    |
3    |
4 LL |         Some(ref _y @ _z) => {}
5    |              ------^^^--

8    |              value borrowed, by `_y`, here
9 
10 error: borrow of moved value
-   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:21:14
12    |
12    |
13 LL |         Some(_z @ ref _y) => {}
14    |              --^^^------

18    |              move occurs because `_z` has type `X` which does not implement the `Copy` trait
19 
20 error: cannot move out of value because it is borrowed
-   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:28:14
22    |
22    |
23 LL |         Some(ref mut _y @ _z) => {}
24    |              ----------^^^--

27    |              value borrowed, by `_y`, here
28 
29 error: borrow of moved value
-   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:35:14
31    |
31    |
32 LL |         Some(_z @ ref mut _y) => {}
33    |              --^^^----------

37    |              move occurs because `_z` has type `X` which does not implement the `Copy` trait
38 
39 error[E0382]: borrow of moved value
-   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:14:14
41    |
41    |
42 LL |         Some(ref _y @ _z) => {}

52    |                       ^^^
53 
53 
54 error[E0382]: borrow of moved value
-   --> $DIR/bind-by-move-neither-can-live-while-the-other-survives-1.rs:28:14
56    |
56    |
57 LL |         Some(ref mut _y @ _z) => {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1/bind-by-move-neither-can-live-while-the-other-survives-1.stderr
To only update this specific test, also pass `--test-args pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/bind-by-move-neither-can-live-while-the-other-survives-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot move out of value because it is borrowed
   |
   |
LL |         Some(ref _y @ _z) => {} //~ ERROR cannot move out of value because it is borrowed
   |              ------^^^--
   |              |        |
   |              |        value moved into `_z` here
   |              value borrowed, by `_y`, here

error: borrow of moved value
   |
   |
LL |         Some(_z @ ref _y) => {}
   |              --^^^------
   |              |    |
   |              |    value borrowed here after move
   |              value moved into `_z` here
   |              move occurs because `_z` has type `X` which does not implement the `Copy` trait

error: cannot move out of value because it is borrowed
   |
   |
LL |         Some(ref mut _y @ _z) => {} //~ ERROR cannot move out of value because it is borrowed
   |              ----------^^^--
   |              |            |
   |              |            value moved into `_z` here
   |              value borrowed, by `_y`, here

error: borrow of moved value
   |
   |
LL |         Some(_z @ ref mut _y) => {}
   |              --^^^----------
   |              |    |
   |              |    value borrowed here after move
   |              value moved into `_z` here
   |              move occurs because `_z` has type `X` which does not implement the `Copy` trait

error[E0382]: borrow of moved value
   |
   |
LL |         Some(ref _y @ _z) => {} //~ ERROR cannot move out of value because it is borrowed
   |              |        |
   |              |        value moved here
   |              |        value moved here
   |              value borrowed here after move
   |
   = note: move occurs because value has type `X`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `x.0`
   |
LL |         Some(ref _y @ ref _z) => {} //~ ERROR cannot move out of value because it is borrowed


error[E0382]: borrow of moved value
   |
   |
LL |         Some(ref mut _y @ _z) => {} //~ ERROR cannot move out of value because it is borrowed
   |              |            |
   |              |            value moved here
   |              |            value moved here
   |              value borrowed here after move
   |
   = note: move occurs because value has type `X`, which does not implement the `Copy` trait
help: borrow this field in the pattern to avoid moving `x.0`
   |
LL |         Some(ref mut _y @ ref _z) => {} //~ ERROR cannot move out of value because it is borrowed

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0382`.
---
1 error[E0382]: use of moved value
-   --> $DIR/borrowck-move-and-move.rs:13:9
+   --> $DIR/borrowck-move-and-move.rs:11:9
3    |
4 LL |     let a @ b = U;
5    |         ^^^^-   - move occurs because value has type `U`, which does not implement the `Copy` trait

8    |         value used here after move
10 error[E0382]: use of partially moved value
-   --> $DIR/borrowck-move-and-move.rs:15:9
+   --> $DIR/borrowck-move-and-move.rs:13:9
12    |
12    |
13 LL |     let a @ (b, c) = (U, U);
14    |         ^^^^^^^^-^

19    = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
21 error[E0382]: use of partially moved value
-   --> $DIR/borrowck-move-and-move.rs:17:9
+   --> $DIR/borrowck-move-and-move.rs:15:9
23    |
23    |
24 LL |     let a @ (b, c) = (u(), u());
25    |         ^^^^^^^^-^

30    = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
32 error[E0382]: use of moved value
-   --> $DIR/borrowck-move-and-move.rs:20:16
+   --> $DIR/borrowck-move-and-move.rs:18:16
34    |
34    |
35 LL |     match Ok(U) {
36    |           ----- move occurs because value has type `Result<U, U>`, which does not implement the `Copy` trait
41    |         value moved here
42 
43 error[E0382]: use of moved value
-   --> $DIR/borrowck-move-and-move.rs:20:29
-   --> $DIR/borrowck-move-and-move.rs:20:29
+   --> $DIR/borrowck-move-and-move.rs:18:29
45    |
46 LL |     match Ok(U) {
47    |           ----- move occurs because value has type `Result<U, U>`, which does not implement the `Copy` trait
52    |                     value moved here
53 
54 error[E0382]: use of partially moved value
-   --> $DIR/borrowck-move-and-move.rs:27:9
-   --> $DIR/borrowck-move-and-move.rs:27:9
+   --> $DIR/borrowck-move-and-move.rs:25:9
56    |
57 LL |         xs @ [a, .., b] => {}
58    |         ^^^^^^^^^^^^^-^

63    = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
65 error[E0382]: use of partially moved value
-   --> $DIR/borrowck-move-and-move.rs:31:9
+   --> $DIR/borrowck-move-and-move.rs:29:9
67    |
67    |
68 LL |         xs @ [_, ys @ .., _] => {}


74    = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
76 error[E0382]: use of moved value
-   --> $DIR/borrowck-move-and-move.rs:24:12
+   --> $DIR/borrowck-move-and-move.rs:22:12
78    |
78    |
79 LL |     fn fun(a @ b: U) {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-move-and-move/borrowck-move-and-move.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-move-and-move/borrowck-move-and-move.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-move-and-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-move-and-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-move-and-move/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:11:9
   |
LL |     let a @ b = U; //~ ERROR use of moved value
   |         ^^^^-   - move occurs because value has type `U`, which does not implement the `Copy` trait
   |         |   |
   |         |   value moved here
   |         value used here after move
error[E0382]: use of partially moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:13:9
   |
   |
LL |     let a @ (b, c) = (U, U); //~ ERROR use of partially moved value
   |         ^^^^^^^^-^
   |         |       value partially moved here
   |         |       value partially moved here
   |         value used here after partial move
   |
   = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
error[E0382]: use of partially moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:15:9
   |
   |
LL |     let a @ (b, c) = (u(), u()); //~ ERROR use of partially moved value
   |         ^^^^^^^^-^
   |         |       value partially moved here
   |         |       value partially moved here
   |         value used here after partial move
   |
   = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
error[E0382]: use of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:18:16
   |
   |
LL |     match Ok(U) {
   |           ----- move occurs because value has type `Result<U, U>`, which does not implement the `Copy` trait
LL |         a @ Ok(b) | a @ Err(b) => {} //~ ERROR use of moved value
   |         -------^-
   |         |      |
   |         |      value used here after move
   |         value moved here
error[E0382]: use of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:18:29
   |
   |
LL |     match Ok(U) {
   |           ----- move occurs because value has type `Result<U, U>`, which does not implement the `Copy` trait
LL |         a @ Ok(b) | a @ Err(b) => {} //~ ERROR use of moved value
   |                     --------^-
   |                     |       |
   |                     |       value used here after move
   |                     value moved here
error[E0382]: use of partially moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:25:9
   |
   |
LL |         xs @ [a, .., b] => {} //~ ERROR use of partially moved value
   |         ^^^^^^^^^^^^^-^
   |         |            value partially moved here
   |         |            value partially moved here
   |         value used here after partial move
   |
   = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
error[E0382]: use of partially moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:29:9
   |
   |
LL |         xs @ [_, ys @ .., _] => {} //~ ERROR use of partially moved value
   |         |        |
   |         |        value partially moved here
   |         |        value partially moved here
   |         value used here after partial move
   |
   = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
error[E0382]: use of moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/borrowck-move-and-move.rs:22:12
   |
   |
LL |     fn fun(a @ b: U) {} //~ ERROR use of moved value
   |            |   |
   |            |   value moved here
   |            |   value moved here
   |            value used here after move
   |            move occurs because value has type `U`, which does not implement the `Copy` trait
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0382`.


------------------------------------------


---- [ui] ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs stdout ----
diff of stderr:

1 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse-promotion.rs:8:9
3    |
3    |
4 LL |     let a @ ref b = U;
5    |         -^^^-----

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion/borrowck-pat-by-move-and-ref-inverse-promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse-promotion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: borrow of moved value
   |
   |
LL |     let a @ ref b = U; //~ ERROR borrow of moved value
   |         -^^^-----
   |         |   |
   |         |   value borrowed here after move
   |         value moved into `a` here
   |         move occurs because `a` has type `U` which does not implement the `Copy` trait
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/pattern/bindings-after-at/borrowck-pat-by-move-and-ref-inverse.rs stdout ----
diff of stderr:

1 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:24:9
3    |
3    |
4 LL |     let a @ ref b = U;
5    |         -^^^-----

9    |         move occurs because `a` has type `U` which does not implement the `Copy` trait
10 
11 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:26:9
13    |
13    |
14 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);
15    |         -^^^^^^^^^^^^---------^^^^^^-----^

20    |         move occurs because `a` has type `(U, U)` which does not implement the `Copy` trait
21 
22 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:26:14
24    |
24    |
25 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);
26    |              -----^^^---------

30    |              move occurs because `b` has type `U` which does not implement the `Copy` trait
31 
32 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:26:33
34    |
34    |
35 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);
36    |                                 -^^^-----

40    |                                 move occurs because `d` has type `U` which does not implement the `Copy` trait
41 
42 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:31:9
44    |
44    |
45 LL |     let a @ [ref mut b, ref c] = [U, U];
46    |         -^^^^---------^^-----^

51    |         move occurs because `a` has type `[U; 2]` which does not implement the `Copy` trait
52 
53 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:33:9
55    |
55    |
56 LL |     let a @ ref b = u();
57    |         -^^^-----

61    |         move occurs because `a` has type `U` which does not implement the `Copy` trait
62 
63 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:35:9
65    |
65    |
66 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
67    |         -^^^^^^^^^^^^---------^^^^^^-----^

72    |         move occurs because `a` has type `(U, U)` which does not implement the `Copy` trait
73 
74 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:35:14
76    |
76    |
77 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
78    |              -----^^^---------

82    |              move occurs because `b` has type `U` which does not implement the `Copy` trait
83 
84 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:35:33
86    |
86    |
87 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
88    |                                 -^^^-----

92    |                                 move occurs because `d` has type `U` which does not implement the `Copy` trait
93 
94 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:40:9
96    |
96    |
97 LL |     let a @ [ref mut b, ref c] = [u(), u()];
98    |         -^^^^---------^^-----^

103    |         move occurs because `a` has type `[U; 2]` which does not implement the `Copy` trait
104 
105 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:44:9
107    |
107    |
108 LL |         a @ Some(ref b) => {}
109    |         -^^^^^^^^-----^

113    |         move occurs because `a` has type `Option<U>` which does not implement the `Copy` trait
114 
115 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:49:9
117    |
117    |
118 LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
119    |         -^^^^^^^^^^^^^^^^^---------^^^^^^-----^^

124    |         move occurs because `a` has type `Option<(U, U)>` which does not implement the `Copy` trait
125 
126 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:49:19
128    |
128    |
129 LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
130    |                   -----^^^---------

134    |                   move occurs because `b` has type `U` which does not implement the `Copy` trait
135 
136 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:49:38
138    |
138    |
139 LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
140    |                                      -^^^-----

144    |                                      move occurs because `d` has type `U` which does not implement the `Copy` trait
145 
146 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:57:9
148    |
148    |
149 LL |         mut a @ Some([ref b, ref mut c]) => {}
150    |         -----^^^^^^^^^-----^^---------^^

155    |         move occurs because `a` has type `Option<[U; 2]>` which does not implement the `Copy` trait
156 
157 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:63:9
159    |
159    |
160 LL |         a @ Some(ref b) => {}
161    |         -^^^^^^^^-----^

165    |         move occurs because `a` has type `Option<U>` which does not implement the `Copy` trait
166 
167 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:69:9
169    |
169    |
170 LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
171    |         -^^^^^^^^^^^^^^^^^---------^^^^^^-----^^

176    |         move occurs because `a` has type `Option<(U, U)>` which does not implement the `Copy` trait
177 
178 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:69:19
180    |
180    |
181 LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
182    |                   -----^^^---------

186    |                   move occurs because `b` has type `U` which does not implement the `Copy` trait
187 
188 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:69:38
190    |
190    |
191 LL |         a @ Some((mut b @ ref mut c, d @ ref e)) => {}
192    |                                      -^^^-----

196    |                                      move occurs because `d` has type `U` which does not implement the `Copy` trait
197 
198 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:77:9
200    |
200    |
201 LL |         mut a @ Some([ref b, ref mut c]) => {}
202    |         -----^^^^^^^^^-----^^---------^^

207    |         move occurs because `a` has type `Option<[U; 2]>` which does not implement the `Copy` trait
208 
209 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:13:11
211    |
211    |
212 LL |     fn f1(a @ ref b: U) {}
213    |           -^^^-----

217    |           move occurs because `a` has type `U` which does not implement the `Copy` trait
218 
219 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:16:11
221    |
221    |
222 LL |     fn f2(mut a @ (b @ ref c, mut d @ ref e): (U, U)) {}


228    |           move occurs because `a` has type `(U, U)` which does not implement the `Copy` trait
229 
230 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:16:20
232    |
232    |
233 LL |     fn f2(mut a @ (b @ ref c, mut d @ ref e): (U, U)) {}
234    |                    -^^^-----

238    |                    move occurs because `b` has type `U` which does not implement the `Copy` trait
239 
240 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:16:31
242    |
242    |
243 LL |     fn f2(mut a @ (b @ ref c, mut d @ ref e): (U, U)) {}
244    |                               -----^^^-----

248    |                               move occurs because `d` has type `U` which does not implement the `Copy` trait
249 
250 error: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:21:11
252    |
252    |
253 LL |     fn f3(a @ [ref mut b, ref c]: [U; 2]) {}
254    |           -^^^^---------^^-----^

259    |           move occurs because `a` has type `[U; 2]` which does not implement the `Copy` trait
261 error[E0382]: use of partially moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:26:9
+   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:24:9
263    |
263    |
264 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (U, U);


270    = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
272 error[E0382]: use of partially moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:35:9
+   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:33:9
274    |
274    |
275 LL |     let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());


281    = note: partial move occurs because value has type `U`, which does not implement the `Copy` trait
283 error[E0382]: use of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:49:38
+   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:47:38
285    |
285    |
286 LL |     match Some((U, U)) {
287    |           ------------ move occurs because value has type `Option<(U, U)>`, which does not implement the `Copy` trait
292    |         value moved here
293 
293 
294 error[E0382]: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:57:30
296    |
296    |
297 LL |     match Some([U, U]) {
298    |           ------------ move occurs because value has type `Option<[U; 2]>`, which does not implement the `Copy` trait
303    |         value moved here
304 
304 
305 error[E0382]: borrow of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:63:18
307    |
307    |
308 LL |     match Some(u()) {
309    |           --------- move occurs because value has type `Option<U>`, which does not implement the `Copy` trait
314    |         value moved here
315 
316 error[E0382]: use of moved value
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:69:38
-   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:69:38
+   --> $DIR/borrowck-pat-by-move-and-ref-inverse.rs:67:38
318    |
319 LL |     match Some((u(), u())) {
320    |           ---------------- move occurs because value has type `Option<(U, U)>`, which does not implement the `Copy` trait
325    |         value moved here
326 
326 
---

---- [ui] ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs stdout ----
diff of stderr:

1 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:28:9
3    |
3    |
4 LL |     let ref mut a @ ref mut b = U;
5    |         ---------^^^---------

8    |         first mutable borrow, by `a`, occurs here
9 
10 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:31:9
12    |
12    |
13 LL |     let ref mut a @ ref mut b = U;
14    |         ---------^^^---------

17    |         first mutable borrow, by `a`, occurs here
18 
19 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:35:9
21    |
21    |
22 LL |     let ref mut a @ ref mut b = U;
23    |         ---------^^^---------

26    |         first mutable borrow, by `a`, occurs here
27 
28 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:38:9
30    |
30    |
31 LL |     let ref mut a @ ref mut b = U;
32    |         ---------^^^---------

35    |         first mutable borrow, by `a`, occurs here
36 
37 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:41:9
39    |
39    |
40 LL |     let ref mut a @ ref mut b = U;
41    |         ---------^^^---------

44    |         first mutable borrow, by `a`, occurs here
45 
46 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:46:9
48    |
48    |
49 LL |       let ref mut a @ (
50    |           ^--------
66    | |_____^
67 
67 
68 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:56:9
70    |
70    |
71 LL |       let ref mut a @ (
72    |           ^--------
88    | |_________^
89 
89 
90 error: borrow of moved value
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:66:9
92    |
92    |
93 LL |     let a @ (ref mut b, ref mut c) = (U, U);
94    |         -^^^^---------^^---------^

99    |         move occurs because `a` has type `(U, U)` which does not implement the `Copy` trait
100 
101 error: borrow of moved value
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:69:9
103    |
103    |
104 LL |     let a @ (b, [c, d]) = &mut val; // Same as ^--
105    |         -^^^^-^^^-^^-^^

111    |         move occurs because `a` has type `&mut (U, [U; 2])` which does not implement the `Copy` trait
112 
113 error: borrow of moved value
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:72:9
115    |
115    |
116 LL |     let a @ &mut ref mut b = &mut U;
117    |         -^^^^^^^^---------

121    |         move occurs because `a` has type `&mut U` which does not implement the `Copy` trait
122 
123 error: borrow of moved value
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:74:9
125    |
125    |
126 LL |     let a @ &mut (ref mut b, ref mut c) = &mut (U, U);
127    |         -^^^^^^^^^---------^^---------^

132    |         move occurs because `a` has type `&mut (U, U)` which does not implement the `Copy` trait
133 
134 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:78:9
136    |
136    |
137 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


141    |         first mutable borrow, by `a`, occurs here
142 
143 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:78:37
145    |
145    |
146 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


150    |                                     first mutable borrow, by `a`, occurs here
151 
152 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:84:9
154    |
154    |
155 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


159    |         first mutable borrow, by `a`, occurs here
160 
161 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:84:37
163    |
163    |
164 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


168    |                                     first mutable borrow, by `a`, occurs here
169 
170 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:91:9
172    |
172    |
173 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


177    |         first mutable borrow, by `a`, occurs here
178 
179 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:91:37
181    |
181    |
182 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


186    |                                     first mutable borrow, by `a`, occurs here
187 
188 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:103:9
190    |
190    |
191 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


195    |         first mutable borrow, by `a`, occurs here
196 
197 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:103:37
199    |
199    |
200 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


204    |                                     first mutable borrow, by `a`, occurs here
205 
206 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:10:11
208    |
208    |
209 LL |     fn f1(ref mut a @ ref mut b: U) {}
210    |           ---------^^^---------

213    |           first mutable borrow, by `a`, occurs here
214 
215 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:12:11
217    |
217    |
218 LL |     fn f2(ref mut a @ ref mut b: U) {}
219    |           ---------^^^---------

222    |           first mutable borrow, by `a`, occurs here
223 
224 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:15:9
226    |
226    |
227 LL |           ref mut a @ [
228    |           ^--------
240    | |_________^
241 
241 
242 error: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:23:22
244    |
244    |
245 LL |     fn f4_also_moved(ref mut a @ ref mut b @ c: U) {}
246    |                      ---------^^^-------------

250    |                      first mutable borrow, by `a`, occurs here
251 
252 error: cannot move out of value because it is borrowed
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:23:34
254    |
254    |
255 LL |     fn f4_also_moved(ref mut a @ ref mut b @ c: U) {}
256    |                                  ---------^^^-

259    |                                  value borrowed, by `b`, here
260 
261 error[E0499]: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:31:9
263    |
263    |
264 LL |     let ref mut a @ ref mut b = U;


271    |          - first borrow later used here
272 
273 error[E0499]: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:41:9
275    |
275    |
276 LL |     let ref mut a @ ref mut b = U;


283    |     ------ first borrow later used here
284 
285 error[E0499]: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:91:24
287    |
287    |
288 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


295    |             ----------- first borrow later used here
296 
297 error[E0499]: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:91:53
299    |
299    |
300 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


307    |             ----------- first borrow later used here
308 
309 error[E0499]: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:103:24
311    |
311    |
312 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


319    |                  - first borrow later used here
320 
321 error[E0499]: cannot borrow value as mutable more than once at a time
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:103:53
323    |
323    |
324 LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {


331    |                  - first borrow later used here
332 
333 error[E0382]: borrow of moved value
-   --> $DIR/borrowck-pat-ref-mut-twice.rs:23:34
335    |
335    |
336 LL |     fn f4_also_moved(ref mut a @ ref mut b @ c: U) {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice/borrowck-pat-ref-mut-twice.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice/borrowck-pat-ref-mut-twice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/borrowck-pat-ref-mut-twice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |     let ref mut a @ ref mut b = U;
   |         ---------^^^---------
   |         |           |
   |         |           another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |       let ref mut a @ (
   |           ^--------
   |           |
   |  _________first mutable borrow, by `a`, occurs here
   | |
LL | |     //~^ ERROR cannot borrow value as mutable more than once at a time
LL | |         ref mut b,
   | |         --------- another mutable borrow, by `b`, occurs here
LL | |         [
LL | |             ref mut c,
   | |             --------- another mutable borrow, by `c`, occurs here
LL | |             ref mut d,
   | |             --------- another mutable borrow, by `d`, occurs here
LL | |             ref e,
   | |             ----- also borrowed as immutable, by `e`, here
LL | |         ]
LL | |     ) = (U, [U, U, U]);


error: cannot borrow value as mutable more than once at a time
   |
   |
LL |       let ref mut a @ (
   |           ^--------
   |           |
   |  _________first mutable borrow, by `a`, occurs here
   | |
LL | |         //~^ ERROR cannot borrow value as mutable more than once at a time
LL | |             ref mut b,
   | |             --------- another mutable borrow, by `b`, occurs here
LL | |             [
LL | |                 ref mut c,
   | |                 --------- another mutable borrow, by `c`, occurs here
LL | |                 ref mut d,
   | |                 --------- another mutable borrow, by `d`, occurs here
LL | |                 ref e,
   | |                 ----- also borrowed as immutable, by `e`, here
LL | |             ]
LL | |         ) = (u(), [u(), u(), u()]);


error: borrow of moved value
   |
   |
LL |     let a @ (ref mut b, ref mut c) = (U, U);
   |         -^^^^---------^^---------^
   |         |    |          |
   |         |    |          value borrowed here after move
   |         |    value borrowed here after move
   |         value moved into `a` here
   |         move occurs because `a` has type `(U, U)` which does not implement the `Copy` trait

error: borrow of moved value
   |
   |
LL |     let a @ (b, [c, d]) = &mut val; // Same as ^--
   |         -^^^^-^^^-^^-^^
   |         |    |   |  |
   |         |    |   |  value borrowed here after move
   |         |    |   value borrowed here after move
   |         |    value borrowed here after move
   |         value moved into `a` here
   |         move occurs because `a` has type `&mut (U, [U; 2])` which does not implement the `Copy` trait

error: borrow of moved value
   |
   |
LL |     let a @ &mut ref mut b = &mut U;
   |         -^^^^^^^^---------
   |         |        |
   |         |        value borrowed here after move
   |         value moved into `a` here
   |         move occurs because `a` has type `&mut U` which does not implement the `Copy` trait

error: borrow of moved value
   |
   |
LL |     let a @ &mut (ref mut b, ref mut c) = &mut (U, U);
   |         -^^^^^^^^^---------^^---------^
   |         |         |          |
   |         |         |          value borrowed here after move
   |         |         value borrowed here after move
   |         value moved into `a` here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |         move occurs because `a` has type `&mut (U, U)` which does not implement the `Copy` trait

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
   |                                     first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |         |              |
   |         |              |
   |         |              another mutable borrow, by `b`, occurs here
   |         first mutable borrow, by `a`, occurs here

error: cannot borrow value as mutable more than once at a time
   |
   |
LL |         ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {
   |                                     |               |
   |                                     |               |
   |                                     |               another mutable borrow, by `b`, occurs here
---
To only update this specific test, also pass `--test-args pattern/bindings-after-at/nested-binding-modes-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/nested-binding-modes-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/nested-binding-modes-ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/nested-binding-modes-ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0614]: type `{integer}` cannot be dereferenced
   |
LL |     *is_val;
   |     ^^^^^^^


error[E0614]: type `{integer}` cannot be dereferenced
   |
LL |     *is_val;
   |     ^^^^^^^

---
1 error[E0382]: use of partially moved value
-   --> $DIR/copy-and-move-mixed.rs:14:9
+   --> $DIR/copy-and-move-mixed.rs:12:9
3    |
4 LL |     let a @ NC(b, c @ NC(d, e)) = NC(C, NC(C, C));


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/copy-and-move-mixed/copy-and-move-mixed.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/copy-and-move-mixed/copy-and-move-mixed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/copy-and-move-mixed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/copy-and-move-mixed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/copy-and-move-mixed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/copy-and-move-mixed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0382]: use of partially moved value
  --> /checkout/src/test/ui/pattern/bindings-after-at/copy-and-move-mixed.rs:12:9
   |
LL |     let a @ NC(b, c @ NC(d, e)) = NC(C, NC(C, C));
   |         |         |
   |         |         value partially moved here
   |         |         value partially moved here
   |         value used here after partial move
   |
   = note: partial move occurs because value has type `NC<C, C>`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.


------------------------------------------


---- [ui] ui/pattern/bindings-after-at/nested-type-ascription-syntactically-invalid.rs stdout ----
diff of stderr:

1 error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `@`
-   --> $DIR/nested-type-ascription-syntactically-invalid.rs:19:15
3    |
3    |
4 LL |     let a: u8 @ b = 0;
5    |               ^ expected one of 7 possible tokens
6 
6 
7 error: expected one of `)`, `,`, `@`, or `|`, found `:`
-   --> $DIR/nested-type-ascription-syntactically-invalid.rs:25:15
9    |
9    |
10 LL |     let a @ (b: u8);
11    |               ^ expected one of `)`, `,`, `@`, or `|`
12 
12 
13 error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `)`
-   --> $DIR/nested-type-ascription-syntactically-invalid.rs:25:19
15    |
15    |
16 LL |     let a @ (b: u8);
17    |                   ^ expected one of 7 possible tokens
18 
18 
19 error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `@`
-   --> $DIR/nested-type-ascription-syntactically-invalid.rs:32:15
21    |
21    |
22 LL |     let a: T1 @ Outer(b: T2);
23    |               ^ expected one of 7 possible tokens

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/nested-type-ascription-syntactically-invalid/nested-type-ascription-syntactically-invalid.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/nested-type-ascription-syntactically-invalid.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/nested-type-ascription-syntactically-invalid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/nested-type-ascription-syntactically-invalid" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/nested-type-ascription-syntactically-invalid/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `@`
   |
   |
LL |     let a: u8 @ b = 0;
   |               ^ expected one of 7 possible tokens

error: expected one of `)`, `,`, `@`, or `|`, found `:`
   |
   |
LL |     let a @ (b: u8);
   |               ^ expected one of `)`, `,`, `@`, or `|`

error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `)`
   |
   |
LL |     let a @ (b: u8);
   |                   ^ expected one of 7 possible tokens

error: expected one of `!`, `(`, `+`, `::`, `;`, `<`, or `=`, found `@`
   |
   |
LL |     let a: T1 @ Outer(b: T2);
   |               ^ expected one of 7 possible tokens
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/pattern/bindings-after-at/pat-at-same-name-both.rs stdout ----
diff of stderr:

1 error[E0415]: identifier `a` is bound more than once in this parameter list
-   --> $DIR/pat-at-same-name-both.rs:7:14
3    |
3    |
4 LL |     fn f(a @ a @ a: ()) {}
5    |              ^ used as parameter more than once
6 
6 
7 error[E0415]: identifier `a` is bound more than once in this parameter list
-   --> $DIR/pat-at-same-name-both.rs:7:18
9    |
9    |
10 LL |     fn f(a @ a @ a: ()) {}
11    |                  ^ used as parameter more than once
12 
12 
13 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:12:20
15    |
15    |
16 LL |         Ok(a @ b @ a)
17    |                    ^ used in a pattern more than once
18 
18 
19 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:14:23
21    |
21    |
22 LL |         | Err(a @ b @ a)
23    |                       ^ used in a pattern more than once
24 
24 
25 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:19:13
27    |
27    |
28 LL |     let a @ a @ a = ();
29    |             ^ used in a pattern more than once
30 
30 
31 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:19:17
33    |
33    |
34 LL |     let a @ a @ a = ();
35    |                 ^ used in a pattern more than once
36 
36 
37 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:22:21
39    |
39    |
40 LL |     let ref a @ ref a = ();
41    |                     ^ used in a pattern more than once
42 
42 
43 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:24:29
45    |
45    |
46 LL |     let ref mut a @ ref mut a = ();
47    |                             ^ used in a pattern more than once
48 
48 
49 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:27:17
51    |
51    |
52 LL |     let a @ (Ok(a) | Err(a)) = Ok(());
53    |                 ^ used in a pattern more than once
54 
54 
55 error[E0416]: identifier `a` is bound more than once in the same pattern
-   --> $DIR/pat-at-same-name-both.rs:27:26
57    |
57    |
58 LL |     let a @ (Ok(a) | Err(a)) = Ok(());
59    |                          ^ used in a pattern more than once

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/pat-at-same-name-both/pat-at-same-name-both.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/bindings-after-at/pat-at-same-name-both.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pattern/bindings-after-at/pat-at-same-name-both.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/pat-at-same-name-both" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/bindings-after-at/pat-at-same-name-both/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0415]: identifier `a` is bound more than once in this parameter list
   |
   |
LL |     fn f(a @ a @ a: ()) {}
   |              ^ used as parameter more than once

error[E0415]: identifier `a` is bound more than once in this parameter list
   |
   |
LL |     fn f(a @ a @ a: ()) {}
   |                  ^ used as parameter more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |         Ok(a @ b @ a)
   |                    ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |         | Err(a @ b @ a)
   |                       ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |     let a @ a @ a = ();
   |             ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |     let a @ a @ a = ();
   |                 ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |     let ref a @ ref a = ();
   |                     ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |     let ref mut a @ ref mut a = ();
   |                             ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |     let a @ (Ok(a) | Err(a)) = Ok(());
   |                 ^ used in a pattern more than once

error[E0416]: identifier `a` is bound more than once in the same pattern
   |
   |
LL |     let a @ (Ok(a) | Err(a)) = Ok(());
   |                          ^ used in a pattern more than once
error: aborting due to 10 previous errors

Some errors have detailed explanations: E0415, E0416.
For more information about an error, try `rustc --explain E0415`.
---
test result: FAILED. 11768 passed; 17 failed; 97 ignored; 0 measured; 0 filtered out; finished in 122.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:55
