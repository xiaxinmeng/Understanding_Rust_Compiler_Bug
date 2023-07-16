plain
...............................iii...................................................... 13200/13250
..................................................
failures:

---- [ui] src/test/ui/kw-in-trait-bounds.rs stdout ----

1 error: expected identifier, found keyword `fn`
2   --> $DIR/kw-in-trait-bounds.rs:3:10
3    |
3    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
6    |
6    |
7 help: escape `fn` to use it as an identifier
8    |
8    |
- LL | fn _f<F: r#fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: r#fn(), G>(_: impl fn(), _: &dyn fn())
11 
12 error: expected identifier, found keyword `fn`

13   --> $DIR/kw-in-trait-bounds.rs:3:27
13   --> $DIR/kw-in-trait-bounds.rs:3:27
14    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
17    |
17    |
18 help: escape `fn` to use it as an identifier
19    |
19    |
- LL | fn _f<F: fn(), G>(_: impl r#fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl r#fn(), _: &dyn fn())
22 
23 error: expected identifier, found keyword `fn`

24   --> $DIR/kw-in-trait-bounds.rs:3:41
24   --> $DIR/kw-in-trait-bounds.rs:3:41
25    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
28    |
28    |
29 help: escape `fn` to use it as an identifier
30    |
30    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn r#fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn r#fn())
33 
34 error: expected identifier, found keyword `fn`

45 error: expected identifier, found keyword `struct`
45 error: expected identifier, found keyword `struct`
46   --> $DIR/kw-in-trait-bounds.rs:24:10
47    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
50    |
50    |
51 help: escape `struct` to use it as an identifier
52    |
52    |
- LL | fn _g<A: r#struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: r#struct, B>(_: impl struct, _: &dyn struct)
55 
56 error: expected identifier, found keyword `struct`

57   --> $DIR/kw-in-trait-bounds.rs:24:29
57   --> $DIR/kw-in-trait-bounds.rs:24:29
58    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
61    |
61    |
62 help: escape `struct` to use it as an identifier
63    |
63    |
- LL | fn _g<A: struct, B>(_: impl r#struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl r#struct, _: &dyn struct)
66 
67 error: expected identifier, found keyword `struct`

68   --> $DIR/kw-in-trait-bounds.rs:24:45
68   --> $DIR/kw-in-trait-bounds.rs:24:45
69    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
72    |
72    |
73 help: escape `struct` to use it as an identifier
74    |
74    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn r#struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn r#struct)
77 
78 error: expected identifier, found keyword `struct`


89 error[E0405]: cannot find trait `r#fn` in this scope
90   --> $DIR/kw-in-trait-bounds.rs:3:10
91    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
93    |          ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
95   ::: $SRC_DIR/core/src/ops/function.rs:LL:COL


111 error[E0405]: cannot find trait `r#fn` in this scope
112   --> $DIR/kw-in-trait-bounds.rs:3:27
113    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
115    |                           ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
117   ::: $SRC_DIR/core/src/ops/function.rs:LL:COL


122 error[E0405]: cannot find trait `r#fn` in this scope
123   --> $DIR/kw-in-trait-bounds.rs:3:41
124    |
- LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn()) 
+ LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
126    |                                         ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
128   ::: $SRC_DIR/core/src/ops/function.rs:LL:COL


133 error[E0405]: cannot find trait `r#struct` in this scope
134   --> $DIR/kw-in-trait-bounds.rs:24:10
135    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
137    |          ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
139 LL | trait Struct {}


151 error[E0405]: cannot find trait `r#struct` in this scope
152   --> $DIR/kw-in-trait-bounds.rs:24:29
153    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
155    |                             ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
157 LL | trait Struct {}


160 error[E0405]: cannot find trait `r#struct` in this scope
161   --> $DIR/kw-in-trait-bounds.rs:24:45
162    |
- LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct) 
+ LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
164    |                                             ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
166 LL | trait Struct {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kw-in-trait-bounds/kw-in-trait-bounds.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args kw-in-trait-bounds.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kw-in-trait-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kw-in-trait-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kw-in-trait-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `fn`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:3:10
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
   |
   |
help: escape `fn` to use it as an identifier
   |
LL | fn _f<F: r#fn(), G>(_: impl fn(), _: &dyn fn())

error: expected identifier, found keyword `fn`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:3:27
   |
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
   |
   |
help: escape `fn` to use it as an identifier
   |
LL | fn _f<F: fn(), G>(_: impl r#fn(), _: &dyn fn())

error: expected identifier, found keyword `fn`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:3:41
   |
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
   |
   |
help: escape `fn` to use it as an identifier
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn r#fn())

error: expected identifier, found keyword `fn`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:17:4
   |
   |
LL | G: fn(),
   |
   |
help: escape `fn` to use it as an identifier
   |
LL | G: r#fn(),

error: expected identifier, found keyword `struct`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:24:10
   |
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
   |
   |
help: escape `struct` to use it as an identifier
   |
LL | fn _g<A: r#struct, B>(_: impl struct, _: &dyn struct)

error: expected identifier, found keyword `struct`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:24:29
   |
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
   |
   |
help: escape `struct` to use it as an identifier
   |
LL | fn _g<A: struct, B>(_: impl r#struct, _: &dyn struct)

error: expected identifier, found keyword `struct`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:24:45
   |
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
   |
   |
help: escape `struct` to use it as an identifier
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn r#struct)

error: expected identifier, found keyword `struct`
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:38:8
   |
   |
LL |     B: struct,
   |        ^^^^^^ expected identifier, found keyword
   |
help: escape `struct` to use it as an identifier
   |
LL |     B: r#struct,


error[E0405]: cannot find trait `r#fn` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:3:10
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
   |          ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
  ::: /checkout/library/core/src/ops/function.rs:74:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here

error[E0405]: cannot find trait `r#fn` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:17:4
   |
LL | G: fn(),
   |    ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
  ::: /checkout/library/core/src/ops/function.rs:74:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here

error[E0405]: cannot find trait `r#fn` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:3:27
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
   |                           ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
  ::: /checkout/library/core/src/ops/function.rs:74:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here

error[E0405]: cannot find trait `r#fn` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:3:41
   |
LL | fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
   |                                         ^^ help: a trait with a similar name exists (notice the capitalization): `Fn`
  ::: /checkout/library/core/src/ops/function.rs:74:1
   |
   |
LL | pub trait Fn<Args>: FnMut<Args> {
   | ------------------------------- similarly named trait `Fn` defined here

error[E0405]: cannot find trait `r#struct` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:24:10
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
   |          ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
LL | trait Struct {}
   | ------------ similarly named trait `Struct` defined here


error[E0405]: cannot find trait `r#struct` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:38:8
LL |     B: struct,
LL |     B: struct,
   |        ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
LL | trait Struct {}
   | ------------ similarly named trait `Struct` defined here


error[E0405]: cannot find trait `r#struct` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:24:29
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
   |                             ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
LL | trait Struct {}
   | ------------ similarly named trait `Struct` defined here


error[E0405]: cannot find trait `r#struct` in this scope
  --> /checkout/src/test/ui/kw-in-trait-bounds.rs:24:45
   |
LL | fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
   |                                             ^^^^^^ help: a trait with a similar name exists (notice the capitalization): `Struct`
LL | trait Struct {}
   | ------------ similarly named trait `Struct` defined here

error: aborting due to 16 previous errors
