plain
.................................................................................................... 5900/11978
.............................i...................................................................... 6000/11978
.................................................................................................... 6100/11978
...................................i................................................................ 6200/11978
........i.......F............................................F...................................... 6300/11978
.........F..............................................................ii.ii.......i...i........... 6400/11978
.................i....i..................................i...........................i.............. 6600/11978
.................................................................................................... 6700/11978
..........................i......................................................................... 6800/11978
.................................................................................................... 6900/11978
---
.................................................................................................... 11100/11978
.................................................................................................... 11200/11978
..........................................................ii........................................ 11300/11978
.................................................................................................... 11400/11978
.FF..F.......F...FF................................................................................. 11500/11978
.................................................................................................... 11700/11978
.................................................................................................... 11800/11978
.....................................................................i.i............................ 11900/11978
..............................................................................
..............................................................................
failures:

---- [ui] ui/did_you_mean/bad-assoc-ty.rs stdout ----
diff of stderr:

81 LL | type D = (u8, u8)::AssocTy;
82    |          ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(u8, u8) as Trait>::AssocTy`
83 
- error[E0121]: the type placeholder `_` is not allowed within type aliases
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for type aliases
86    |
86    |
87 LL | type E = _::AssocTy;

122 LL | type I = ty!()::AssocTy;
123    |          ^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
124 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
127    |
127    |
128 LL | fn foo<X: K<_, _>>(x: X) {}

135 LL | fn foo<X: K<T, T>, T>(x: X) {}
136    |             ^  ^ ^^^
137 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
140    |
140    |
141 LL | fn bar<F>(_: F) where F: Fn() -> _ {}

146 LL | fn bar<F, T>(_: F) where F: Fn() -> T {}
147    |         ^^^                         ^
148 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
151    |
151    |
152 LL | fn baz<F: Fn() -> _>(_: F) {}

157 LL | fn baz<F: Fn() -> T, T>(_: F) {}
159 
159 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
162    |
162    |
163 LL | struct L<F>(F) where F: Fn() -> _;

168 LL | struct L<F, T>(F) where F: Fn() -> T;
169    |           ^^^                      ^
170 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
173    |
173    |
174 LL | struct M<F> where F: Fn() -> _ {

179 LL | struct M<F, T> where F: Fn() -> T {
180    |           ^^^                   ^
181 
- error[E0121]: the type placeholder `_` is not allowed within enums
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for enums
184    |
184    |
185 LL | enum N<F> where F: Fn() -> _ {

190 LL | enum N<F, T> where F: Fn() -> T {
191    |         ^^^                   ^
192 
- error[E0121]: the type placeholder `_` is not allowed within unions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for unions
195    |
195    |
196 LL | union O<F> where F: Fn() -> _ {

201 LL | union O<F, T> where F: Fn() -> T {
202    |          ^^^                   ^
203 
- error[E0121]: the type placeholder `_` is not allowed within traits
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for traits
206    |
206    |
207 LL | trait P<F> where F: Fn() -> _ {

212 LL | trait P<F, T> where F: Fn() -> T {
213    |          ^^^                   ^
214 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
217    |
217    |
218 LL |     fn foo<F>(_: F) where F: Fn() -> _ {}

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty/bad-assoc-ty.stderr
To only update this specific test, also pass `--test-args did_you_mean/bad-assoc-ty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/bad-assoc-ty/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:1:10
   |
LL | type A = [u8; 4]::AssocTy;
   |          ^^^^^^^^^^^^^^^^ help: try: `<[u8; 4]>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:5:10
   |
   |
LL | type B = [u8]::AssocTy;
   |          ^^^^^^^^^^^^^ help: try: `<[u8]>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:9:10
   |
   |
LL | type C = (u8)::AssocTy;
   |          ^^^^^^^^^^^^^ help: try: `<(u8)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:13:10
   |
   |
LL | type D = (u8, u8)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^ help: try: `<(u8, u8)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:17:10
   |
   |
LL | type E = _::AssocTy;
   |          ^^^^^^^^^^ help: try: `<_>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:21:19
   |
   |
LL | type F = &'static (u8)::AssocTy;
   |                   ^^^^^^^^^^^^^ help: try: `<(u8)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:27:10
   |
   |
LL | type G = dyn 'static + (Send)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `<dyn 'static + (Send)>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:44:10
   |
   |
LL | type I = ty!()::AssocTy;
   |          ^^^^^^^^^^^^^^ help: try: `<ty!()>::AssocTy`
error: missing angle brackets in associated item path
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:37:19
   |
   |
LL |     ($ty: ty) => ($ty::AssocTy);
   |                   ^^^^^^^^^^^^ help: try: `<$ty>::AssocTy`
...
LL | type J = ty!(u8);
   |
   |
   = note: this error originates in the macro `ty` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:1:10
   |
   |
LL | type A = [u8; 4]::AssocTy;
   |          ^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<[u8; 4] as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:5:10
   |
   |
LL | type B = [u8]::AssocTy;
   |          ^^^^^^^^^^^^^ help: use fully-qualified syntax: `<[u8] as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:9:10
   |
   |
LL | type C = (u8)::AssocTy;
   |          ^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:13:10
   |
   |
LL | type D = (u8, u8)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(u8, u8) as Trait>::AssocTy`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for type aliases
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:17:10
   |
LL | type E = _::AssocTy;
   |          ^ not allowed in type signatures
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:21:19
   |
   |
LL | type F = &'static (u8)::AssocTy;
   |                   ^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:27:10
   |
   |
LL | type G = dyn 'static + (Send)::AssocTy;
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(dyn Send + 'static) as Trait>::AssocTy`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:33:10
   |
   |
LL | type H = Fn(u8) -> (u8)::Output;
   |          ^^^^^^^^^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<(dyn Fn(u8) -> u8 + 'static) as Trait>::Output`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:37:19
   |
   |
LL |     ($ty: ty) => ($ty::AssocTy);
   |                   ^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`
...
LL | type J = ty!(u8);
   |
   |
   = note: this error originates in the macro `ty` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:44:10
   |
   |
LL | type I = ty!()::AssocTy;
   |          ^^^^^^^^^^^^^^ help: use fully-qualified syntax: `<u8 as Trait>::AssocTy`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:49:13
   |
LL | fn foo<X: K<_, _>>(x: X) {}
   |             ^  ^ not allowed in type signatures
   |             not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | fn foo<X: K<T, T>, T>(x: X) {}
   |             ^  ^ ^^^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:52:34
   |
LL | fn bar<F>(_: F) where F: Fn() -> _ {}
   |                                  ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn bar<F, T>(_: F) where F: Fn() -> T {}
   |         ^^^                         ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:55:19
   |
LL | fn baz<F: Fn() -> _>(_: F) {}
   |                   ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn baz<F: Fn() -> T, T>(_: F) {}


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:58:33
   |
LL | struct L<F>(F) where F: Fn() -> _;
   |                                 ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct L<F, T>(F) where F: Fn() -> T;
   |           ^^^                      ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:60:30
   |
LL | struct M<F> where F: Fn() -> _ {
   |                              ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct M<F, T> where F: Fn() -> T {
   |           ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for enums
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:64:28
   |
LL | enum N<F> where F: Fn() -> _ {
   |                            ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | enum N<F, T> where F: Fn() -> T {
   |         ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for unions
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:69:29
   |
LL | union O<F> where F: Fn() -> _ {
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | union O<F, T> where F: Fn() -> T {
   |          ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for traits
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:74:29
   |
LL | trait P<F> where F: Fn() -> _ {
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | trait P<F, T> where F: Fn() -> T {
   |          ^^^                   ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
  --> /checkout/src/test/ui/did_you_mean/bad-assoc-ty.rs:79:38
   |
LL |     fn foo<F>(_: F) where F: Fn() -> _ {}
   |                                      ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn foo<F, T>(_: F) where F: Fn() -> T {}
   |             ^^^                         ^
error: aborting due to 28 previous errors

Some errors have detailed explanations: E0121, E0223.
For more information about an error, try `rustc --explain E0121`.
For more information about an error, try `rustc --explain E0121`.

------------------------------------------


---- [ui] ui/error-codes/E0121.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
2   --> $DIR/E0121.rs:1:13
3    |
4 LL | fn foo() -> _ { 5 }
7    |             not allowed in type signatures
8    |             help: replace with the correct return type: `i32`
9 
9 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
11   --> $DIR/E0121.rs:3:13
12    |
13 LL | static BAR: _ = "test";

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0121/E0121.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0121.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0121.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0121" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0121/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn foo() -> _ { 5 } //~ ERROR E0121
   |             |
   |             not allowed in type signatures
   |             help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL | static BAR: _ = "test"; //~ ERROR E0121
   |             |
   |             not allowed in type signatures
   |             help: replace with the correct type: `&str`

---

---- [ui] ui/fn/issue-80179.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
3    |
3    |
4 LL | fn returns_fn_ptr() -> _ {
7    |                        not allowed in type signatures
8    |                        help: replace with the correct return type: `fn() -> i32`
9 
9 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
12    |
12    |
13 LL | fn returns_closure() -> _ {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/issue-80179/issue-80179.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/issue-80179.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fn/issue-80179.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/issue-80179" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/issue-80179/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn returns_fn_ptr() -> _ {
   |                        |
   |                        not allowed in type signatures
   |                        help: replace with the correct return type: `fn() -> i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn returns_closure() -> _ {
   |                         ^ not allowed in type signatures
   |
   = help: consider using an `Fn`, `FnMut`, or `FnOnce` trait bound
   = note: for more information on `Fn` traits and closure types, see https://doc.rust-lang.org/book/ch13-01-closures.html
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/issues/issue-69396-const-no-type-in-macro.rs stdout ----
diff of stderr:

30    |
31    = note: this error originates in the macro `suite` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
35    |
35    |
36 LL |               const A = "A".$fn();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro/issue-69396-const-no-type-in-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-69396-const-no-type-in-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-69396-const-no-type-in-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0428]: the name `A` is defined multiple times
   |
   |
LL |               const A = "A".$fn();
   |               |
   |               |
   |               `A` redefined here
   |               previous definition of the value `A` here
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   |
   = note: `A` must be defined only once in the value namespace of this module
   = note: this error originates in the macro `suite` (in Nightly builds, run with -Z macro-backtrace for more info)
error: missing type for `const` item
  --> /checkout/src/test/ui/issues/issue-69396-const-no-type-in-macro.rs:4:19
   |
   |
LL |               const A = "A".$fn();
   |                     ^ help: provide a type for the constant: `A: usize`
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   |
   = note: this error originates in the macro `suite` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL |               const A = "A".$fn();
   |                     |
   |                     not allowed in type signatures
   |                     help: replace with the correct type: `bool`
...
...
LL | / suite! {
LL | |     len;
LL | |     is_empty;
LL | | }
   |
   |
   = note: this error originates in the macro `suite` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0121, E0428.
For more information about an error, try `rustc --explain E0121`.
For more information about an error, try `rustc --explain E0121`.

------------------------------------------


---- [ui] ui/issues/issue-74086.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
3    |
3    |
4 LL |     static BUG: fn(_) -> u8 = |_| 8;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-74086/issue-74086.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-74086.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-74086.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-74086" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-74086/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     static BUG: fn(_) -> u8 = |_| 8;
   |                    ^ not allowed in type signatures
error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/issues/issue-81885.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
3    |
3    |
4 LL | const TEST4: fn() -> _ = 42;
5    |                      ^ not allowed in type signatures
6 
6 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
9    |
9    |
10 LL |     const TEST5: fn() -> _ = 42;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81885/issue-81885.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-81885.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-81885.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81885" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-81885/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | const TEST4: fn() -> _ = 42;
   |                      ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     const TEST5: fn() -> _ = 42;
   |                          ^ not allowed in type signatures
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/self/self-infer.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
3    |
3    |
4 LL |     fn f(self: _) {}

9 LL |     fn f<T>(self: T) {}
10    |         ^^^       ^
11 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
14    |
14    |
15 LL |     fn g(self: &_) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self-infer/self-infer.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args self/self-infer.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/self-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self-infer" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/self-infer/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn f(self: _) {} //~ERROR the type placeholder `_` is not allowed within functions
   |                ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn f<T>(self: T) {} //~ERROR the type placeholder `_` is not allowed within functions
   |         ^^^       ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL |     fn g(self: &_) {} //~ERROR the type placeholder `_` is not allowed within functions
   |                 ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn g<T>(self: &T) {} //~ERROR the type placeholder `_` is not allowed within functions
   |         ^^^        ^
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/typeck/issue-80779.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
3    |
3    |
4 LL | pub fn g(_: T<'static>) -> _ {}
7    |                            not allowed in type signatures
8    |                            help: replace with the correct return type: `()`
9 
9 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
12    |
12    |
13 LL | pub fn f<'a>(val: T<'a>) -> _ {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-80779/issue-80779.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-80779.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-80779.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-80779" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-80779/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | pub fn g(_: T<'static>) -> _ {}
   |                            |
   |                            not allowed in type signatures
   |                            help: replace with the correct return type: `()`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | pub fn f<'a>(val: T<'a>) -> _ {
   |                             |
   |                             not allowed in type signatures
   |                             help: replace with the correct return type: `()`

---

---- [ui] ui/typeck/issue-75883.rs stdout ----
diff of stderr:

34 LL |     pub fn interact(&mut self) -> Result<_, E> {
36 
36 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
39    |
39    |
40 LL |     pub fn interact(&mut self) -> Result<_> {
41    |                                          ^ not allowed in type signatures
42 
42 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
45    |
45    |
46 LL |     pub fn run() -> Result<_> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-75883/issue-75883.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-75883.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-75883.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-75883" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-75883/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this enum takes 2 generic arguments but 1 generic argument was supplied
   |
   |
LL |     pub fn run() -> Result<_> {
   |                     ^^^^^^ - supplied 1 generic argument
   |                     expected 2 generic arguments
   |
   |
note: enum defined here, with 2 generic parameters: `T`, `E`
   |
   |
LL | pub enum Result<T, E> {
   |          ^^^^^^ -  -
help: add missing generic argument
   |
LL |     pub fn run() -> Result<_, E> {


error[E0107]: this enum takes 2 generic arguments but 1 generic argument was supplied
   |
   |
LL |     pub fn interact(&mut self) -> Result<_> {
   |                                   ^^^^^^ - supplied 1 generic argument
   |                                   expected 2 generic arguments
   |
   |
note: enum defined here, with 2 generic parameters: `T`, `E`
   |
   |
LL | pub enum Result<T, E> {
   |          ^^^^^^ -  -
help: add missing generic argument
   |
LL |     pub fn interact(&mut self) -> Result<_, E> {


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     pub fn interact(&mut self) -> Result<_> {
   |                                          ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL |     pub fn run() -> Result<_> {
   |                            ^ not allowed in type signatures
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0107, E0121.
For more information about an error, try `rustc --explain E0107`.
For more information about an error, try `rustc --explain E0107`.

------------------------------------------


---- [ui] ui/typeck/issue-83621-placeholder-static-in-extern.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
3    |
3    |
4 LL |     static x: _;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-83621-placeholder-static-in-extern/issue-83621-placeholder-static-in-extern.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-83621-placeholder-static-in-extern.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-83621-placeholder-static-in-extern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-83621-placeholder-static-in-extern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-83621-placeholder-static-in-extern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
   |
   |
LL |     static x: _; //~ ERROR: [E0121]
   |               ^ not allowed in type signatures
error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.


------------------------------------------


---- [ui] ui/typeck/typeck_type_placeholder_item_help.rs stdout ----
diff of stderr:

- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
3    |
3    |
4 LL | fn test1() -> _ { Some(42) }
7    |               not allowed in type signatures
7    |               not allowed in type signatures
8    |               help: replace with the correct return type: `Option<i32>`
9 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
12    |
12    |
13 LL | const TEST2: _ = 42u32;
16    |              not allowed in type signatures
17    |              help: replace with the correct type: `u32`
18 
18 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
21    |
21    |
22 LL | const TEST3: _ = Some(42);
25    |              not allowed in type signatures
25    |              not allowed in type signatures
26    |              help: replace with the correct type: `Option<i32>`
27 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
30    |
30    |
31 LL | const TEST4: fn() -> _ = 42;
32    |                      ^ not allowed in type signatures
33 
33 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
36    |
36    |
37 LL |     const TEST5: _ = 42;
40    |                  not allowed in type signatures
41    |                  help: replace with the correct type: `i32`
42 
42 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
45    |
45    |
46 LL |     const TEST6: _ = 13;

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item_help/typeck_type_placeholder_item_help.stderr
To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item_help.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item_help.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item_help" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item_help/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
   |
   |
LL | fn test1() -> _ { Some(42) }
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace with the correct return type: `Option<i32>`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL | const TEST2: _ = 42u32;
   |              |
   |              not allowed in type signatures
   |              help: replace with the correct type: `u32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL | const TEST3: _ = Some(42);
   |              |
   |              not allowed in type signatures
   |              not allowed in type signatures
   |              help: replace with the correct type: `Option<i32>`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
   |
   |
LL | const TEST4: fn() -> _ = 42;
   |                      ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL |     const TEST5: _ = 42;
   |                  |
   |                  not allowed in type signatures
   |                  help: replace with the correct type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
   |
   |
LL |     const TEST6: _ = 13;
   |                  |
   |                  not allowed in type signatures
   |                  help: replace with the correct type: `i32`

---

53    = note: `#[warn(incomplete_features)]` on by default
54    = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information
55 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
58    |
58    |
59 LL | fn test() -> _ { 5 }
62    |              not allowed in type signatures
63    |              help: replace with the correct return type: `i32`
64 
64 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
67    |
67    |
68 LL | fn test2() -> (_, _) { (5, 5) }

72    |               |not allowed in type signatures
73    |               help: replace with the correct return type: `(i32, i32)`
74 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
77    |
77    |
78 LL | static TEST3: _ = "test";
81    |               not allowed in type signatures
82    |               help: replace with the correct type: `&str`
83 
83 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
86    |
86    |
87 LL | static TEST4: _ = 145;
90    |               not allowed in type signatures
91    |               help: replace with the correct type: `i32`
92 
92 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
95    |
95    |
96 LL | static TEST5: (_, _) = (1, 2);
97    |               ^^^^^^ not allowed in type signatures
98 
98 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
101    |
101    |
102 LL | fn test6(_: _) { }

107 LL | fn test6<T>(_: T) { }
108    |         ^^^    ^
109 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
112    |
112    |
113 LL | fn test6_b<T>(_: _, _: T) { }

118 LL | fn test6_b<T, U>(_: U, _: T) { }
119    |             ^^^     ^
120 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
123    |
123    |
124 LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }

129 LL | fn test6_c<T, K, L, A, B, U>(_: U, _: (T, K, L, A, B)) { }
130    |                         ^^^     ^
131 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
134    |
134    |
135 LL | fn test7(x: _) { let _x: usize = x; }

140 LL | fn test7<T>(x: T) { let _x: usize = x; }
141    |         ^^^    ^
142 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
145    |
145    |
146 LL | fn test8(_f: fn() -> _) { }
149    |                      not allowed in type signatures
150    |                      help: use type parameters instead: `T`
151 
151 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
154    |
154    |
155 LL | fn test8(_f: fn() -> _) { }

160 LL | fn test8<T>(_f: fn() -> T) { }
161    |         ^^^             ^
162 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
165    |
165    |
166 LL | fn test11(x: &usize) -> &_ {

169    |                         |not allowed in type signatures
170    |                         help: replace with the correct return type: `&'static &'static usize`
171 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
174    |
174    |
175 LL | unsafe fn test12(x: *const usize) -> *const *const _ {
178    |                                      |             not allowed in type signatures
178    |                                      |             not allowed in type signatures
179    |                                      help: replace with the correct return type: `*const *const usize`
180 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
183    |
184 LL |     a: _,

203 LL |     static A = 42;
203 LL |     static A = 42;
204    |            ^ help: provide a type for the static variable: `A: i32`
205 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
208    |
208    |
209 LL |     static B: _ = 42;
212    |               not allowed in type signatures
213    |               help: replace with the correct type: `i32`
214 
214 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
217    |
217    |
218 LL |     static C: Option<_> = Some(42);
219    |               ^^^^^^^^^ not allowed in type signatures
220 
220 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
223    |
223    |
224 LL |     fn fn_test() -> _ { 5 }
227    |                     not allowed in type signatures
228    |                     help: replace with the correct return type: `i32`
229 
229 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
232    |
232    |
233 LL |     fn fn_test2() -> (_, _) { (5, 5) }

237    |                      |not allowed in type signatures
238    |                      help: replace with the correct return type: `(i32, i32)`
239 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
242    |
242    |
243 LL |     static FN_TEST3: _ = "test";
246    |                      not allowed in type signatures
247    |                      help: replace with the correct type: `&str`
248 
248 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
251    |
251    |
252 LL |     static FN_TEST4: _ = 145;
255    |                      not allowed in type signatures
256    |                      help: replace with the correct type: `i32`
257 
257 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
260    |
260    |
261 LL |     static FN_TEST5: (_, _) = (1, 2);
262    |                      ^^^^^^ not allowed in type signatures
263 
263 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
266    |
266    |
267 LL |     fn fn_test6(_: _) { }

272 LL |     fn fn_test6<T>(_: T) { }
273    |                ^^^    ^
274 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
277    |
277    |
278 LL |     fn fn_test7(x: _) { let _x: usize = x; }

283 LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
284    |                ^^^    ^
285 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
288    |
288    |
289 LL |     fn fn_test8(_f: fn() -> _) { }
292    |                             not allowed in type signatures
293    |                             help: use type parameters instead: `T`
294 
294 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
297    |
297    |
298 LL |     fn fn_test8(_f: fn() -> _) { }

303 LL |     fn fn_test8<T>(_f: fn() -> T) { }
304    |                ^^^             ^
305 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
308    |
309 LL |         a: _,


328 LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
329    |                  ^ cannot infer type
330 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
333    |
333    |
334 LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
336    |                            |
337    |                            not allowed in type signatures
338 
338 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
341    |
341    |
342 LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }

346    |                             |not allowed in type signatures
347    |                             help: replace with the correct return type: `(i32, i32)`
348 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
351    |
351    |
352 LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
355    |                           |     not allowed in type signatures
355    |                           |     not allowed in type signatures
356    |                           help: replace with the correct return type: `(i32, i32)`
357 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
360    |
360    |
361 LL | struct BadStruct<_>(_);

366 LL | struct BadStruct<T>(T);
367    |                  ^  ^
368 
- error[E0121]: the type placeholder `_` is not allowed within implementations
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for implementations
371    |
371    |
372 LL | impl BadTrait<_> for BadStruct<_> {}

379 LL | impl<T> BadTrait<T> for BadStruct<T> {}
380    |     ^^^          ^                ^
381 
- error[E0121]: the type placeholder `_` is not allowed within opaque types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for opaque types
384    |
384    |
385 LL | fn impl_trait() -> impl BadTrait<_> {
386    |                                  ^ not allowed in type signatures
387 
387 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
390    |
390    |
391 LL | struct BadStruct1<_, _>(_);

396 LL | struct BadStruct1<T, _>(T);
397    |                   ^     ^
398 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
401    |
401    |
402 LL | struct BadStruct2<_, T>(_, T);

407 LL | struct BadStruct2<U, T>(U, T);
408    |                   ^     ^
409 
- error[E0121]: the type placeholder `_` is not allowed within type aliases
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for type aliases
412    |
412    |
413 LL | type X = Box<_>;
414    |              ^ not allowed in type signatures
415 
415 
- error[E0121]: the type placeholder `_` is not allowed within opaque types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for opaque types
418    |
418    |
419 LL | type Y = impl Trait<_>;
420    |                     ^ not allowed in type signatures
421 
421 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
424    |
424    |
425 LL | fn value() -> Option<&'static _> {
428    |               |               not allowed in type signatures
428    |               |               not allowed in type signatures
429    |               help: replace with the correct return type: `Option<&'static u8>`
430 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
433    |
433    |
434 LL | const _: Option<_> = map(value);
437    |          not allowed in type signatures
438    |          help: replace with the correct type: `Option<u8>`
439 
439 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
442    |
442    |
443 LL |     fn method_test1(&self, x: _);

448 LL |     fn method_test1<T>(&self, x: T);
449    |                    ^^^           ^
450 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
453    |
453    |
454 LL |     fn method_test2(&self, x: _) -> _;

461 LL |     fn method_test2<T>(&self, x: T) -> T;
462    |                    ^^^           ^     ^
463 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
466    |
466    |
467 LL |     fn method_test3(&self) -> _;

472 LL |     fn method_test3<T>(&self) -> T;
473    |                    ^^^           ^
474 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
477    |
477    |
478 LL |     fn assoc_fn_test1(x: _);

483 LL |     fn assoc_fn_test1<T>(x: T);
484    |                      ^^^    ^
485 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
488    |
488    |
489 LL |     fn assoc_fn_test2(x: _) -> _;

496 LL |     fn assoc_fn_test2<T>(x: T) -> T;
497    |                      ^^^    ^     ^
498 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
501    |
501    |
502 LL |     fn assoc_fn_test3() -> _;

507 LL |     fn assoc_fn_test3<T>() -> T;
508    |                      ^^^      ^
509 
- error[E0121]: the type placeholder `_` is not allowed within associated types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for associated types
512    |
513 LL |     type B = _;

514    |              ^ not allowed in type signatures
514    |              ^ not allowed in type signatures
515 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
518    |
519 LL |     const C: _;

520    |              ^ not allowed in type signatures
520    |              ^ not allowed in type signatures
521 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
524    |
524    |
525 LL |     const D: _ = 42;
528    |              not allowed in type signatures
529    |              help: replace with the correct type: `i32`
530 
530 
- error[E0121]: the type placeholder `_` is not allowed within associated types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for associated types
533    |
533    |
534 LL |     type F: std::ops::Fn(_);
535    |                          ^ not allowed in type signatures
536 
536 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
539    |
539    |
540 LL |     fn test9(&self) -> _ { () }
543    |                        not allowed in type signatures
544    |                        help: replace with the correct return type: `()`
545 
545 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
548    |
548    |
549 LL |     fn test10(&self, _x : _) { }

554 LL |     fn test10<T>(&self, _x : T) { }
555    |              ^^^             ^
556 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
559    |
559    |
560 LL |     fn clone(&self) -> _ { Test9 }
563    |                        not allowed in type signatures
564    |                        help: replace with the correct return type: `Test9`
565 
565 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
568    |
568    |
569 LL |     fn clone_from(&mut self, other: _) { *self = Test9; }

574 LL |     fn clone_from<T>(&mut self, other: T) { *self = Test9; }
575    |                  ^^^                   ^
576 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
579    |
579    |
580 LL |         fn fn_test9(&self) -> _ { () }
583    |                               not allowed in type signatures
584    |                               help: replace with the correct return type: `()`
585 
585 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
588    |
588    |
589 LL |         fn fn_test10(&self, _x : _) { }

594 LL |         fn fn_test10<T>(&self, _x : T) { }
595    |                     ^^^             ^
596 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
599    |
599    |
600 LL |         fn clone(&self) -> _ { FnTest9 }
603    |                            not allowed in type signatures
603    |                            not allowed in type signatures
604    |                            help: replace with the correct return type: `FnTest9`
605 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
608    |
608    |
609 LL |         fn clone_from(&mut self, other: _) { *self = FnTest9; }

614 LL |         fn clone_from<T>(&mut self, other: T) { *self = FnTest9; }
615    |                      ^^^                   ^
616 
---

44    |                   |
45    |                   first use of `_`
46 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
49    |
49    |
50 LL | fn test() -> _ { 5 }
53    |              not allowed in type signatures
54    |              help: replace with the correct return type: `i32`
55 
55 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
58    |
58    |
59 LL | fn test2() -> (_, _) { (5, 5) }

63    |               |not allowed in type signatures
64    |               help: replace with the correct return type: `(i32, i32)`
65 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
68    |
68    |
69 LL | static TEST3: _ = "test";
72    |               not allowed in type signatures
73    |               help: replace with the correct type: `&str`
74 
74 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
77    |
77    |
78 LL | static TEST4: _ = 145;
81    |               not allowed in type signatures
82    |               help: replace with the correct type: `i32`
83 
83 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
86    |
86    |
87 LL | static TEST5: (_, _) = (1, 2);
88    |               ^^^^^^ not allowed in type signatures
89 
89 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
92    |
92    |
93 LL | fn test6(_: _) { }

98 LL | fn test6<T>(_: T) { }
99    |         ^^^    ^
100 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
103    |
103    |
104 LL | fn test6_b<T>(_: _, _: T) { }

109 LL | fn test6_b<T, U>(_: U, _: T) { }
110    |             ^^^     ^
111 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
114    |
114    |
115 LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }

120 LL | fn test6_c<T, K, L, A, B, U>(_: U, _: (T, K, L, A, B)) { }
121    |                         ^^^     ^
122 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
125    |
125    |
126 LL | fn test7(x: _) { let _x: usize = x; }

131 LL | fn test7<T>(x: T) { let _x: usize = x; }
132    |         ^^^    ^
133 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
136    |
136    |
137 LL | fn test8(_f: fn() -> _) { }
140    |                      not allowed in type signatures
141    |                      help: use type parameters instead: `T`
142 
142 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
145    |
145    |
146 LL | fn test8(_f: fn() -> _) { }

151 LL | fn test8<T>(_f: fn() -> T) { }
152    |         ^^^             ^
153 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
156    |
156    |
157 LL | fn test11(x: &usize) -> &_ {

160    |                         |not allowed in type signatures
161    |                         help: replace with the correct return type: `&'static &'static usize`
162 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
165    |
165    |
166 LL | unsafe fn test12(x: *const usize) -> *const *const _ {
169    |                                      |             not allowed in type signatures
169    |                                      |             not allowed in type signatures
170    |                                      help: replace with the correct return type: `*const *const usize`
171 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
174    |
175 LL |     a: _,

194 LL |     static A = 42;
194 LL |     static A = 42;
195    |            ^ help: provide a type for the static variable: `A: i32`
196 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
199    |
199    |
200 LL |     static B: _ = 42;
203    |               not allowed in type signatures
204    |               help: replace with the correct type: `i32`
205 
205 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
208    |
208    |
209 LL |     static C: Option<_> = Some(42);
210    |               ^^^^^^^^^ not allowed in type signatures
211 
211 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
214    |
214    |
215 LL |     fn fn_test() -> _ { 5 }
218    |                     not allowed in type signatures
219    |                     help: replace with the correct return type: `i32`
220 
220 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
223    |
223    |
224 LL |     fn fn_test2() -> (_, _) { (5, 5) }

228    |                      |not allowed in type signatures
229    |                      help: replace with the correct return type: `(i32, i32)`
230 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
233    |
233    |
234 LL |     static FN_TEST3: _ = "test";
237    |                      not allowed in type signatures
238    |                      help: replace with the correct type: `&str`
239 
239 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
242    |
242    |
243 LL |     static FN_TEST4: _ = 145;
246    |                      not allowed in type signatures
247    |                      help: replace with the correct type: `i32`
248 
248 
- error[E0121]: the type placeholder `_` is not allowed within static variables
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for static variables
251    |
251    |
252 LL |     static FN_TEST5: (_, _) = (1, 2);
253    |                      ^^^^^^ not allowed in type signatures
254 
254 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
257    |
257    |
258 LL |     fn fn_test6(_: _) { }

263 LL |     fn fn_test6<T>(_: T) { }
264    |                ^^^    ^
265 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
268    |
268    |
269 LL |     fn fn_test7(x: _) { let _x: usize = x; }

274 LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
275    |                ^^^    ^
276 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
279    |
279    |
280 LL |     fn fn_test8(_f: fn() -> _) { }
283    |                             not allowed in type signatures
284    |                             help: use type parameters instead: `T`
285 
285 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
288    |
288    |
289 LL |     fn fn_test8(_f: fn() -> _) { }

294 LL |     fn fn_test8<T>(_f: fn() -> T) { }
295    |                ^^^             ^
296 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
299    |
300 LL |         a: _,


319 LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
320    |                  ^ cannot infer type
321 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
324    |
324    |
325 LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
327    |                            |
328    |                            not allowed in type signatures
329 
329 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
332    |
332    |
333 LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }

337    |                             |not allowed in type signatures
338    |                             help: replace with the correct return type: `(i32, i32)`
339 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
342    |
342    |
343 LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
346    |                           |     not allowed in type signatures
346    |                           |     not allowed in type signatures
347    |                           help: replace with the correct return type: `(i32, i32)`
348 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
351    |
351    |
352 LL | struct BadStruct<_>(_);

357 LL | struct BadStruct<T>(T);
358    |                  ^  ^
359 
- error[E0121]: the type placeholder `_` is not allowed within implementations
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for implementations
362    |
362    |
363 LL | impl BadTrait<_> for BadStruct<_> {}

370 LL | impl<T> BadTrait<T> for BadStruct<T> {}
371    |     ^^^          ^                ^
372 
- error[E0121]: the type placeholder `_` is not allowed within opaque types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for opaque types
375    |
375    |
376 LL | fn impl_trait() -> impl BadTrait<_> {
377    |                                  ^ not allowed in type signatures
378 
378 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
381    |
381    |
382 LL | struct BadStruct1<_, _>(_);

387 LL | struct BadStruct1<T, _>(T);
388    |                   ^     ^
389 
- error[E0121]: the type placeholder `_` is not allowed within structs
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for structs
392    |
392    |
393 LL | struct BadStruct2<_, T>(_, T);

398 LL | struct BadStruct2<U, T>(U, T);
399    |                   ^     ^
400 
- error[E0121]: the type placeholder `_` is not allowed within type aliases
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for type aliases
403    |
403    |
404 LL | type X = Box<_>;
405    |              ^ not allowed in type signatures
406 
406 
- error[E0121]: the type placeholder `_` is not allowed within opaque types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for opaque types
409    |
409    |
410 LL | type Y = impl Trait<_>;
411    |                     ^ not allowed in type signatures
412 
412 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
415    |
415    |
416 LL | fn value() -> Option<&'static _> {
419    |               |               not allowed in type signatures
419    |               |               not allowed in type signatures
420    |               help: replace with the correct return type: `Option<&'static u8>`
421 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
424    |
424    |
425 LL | const _: Option<_> = map(value);
428    |          not allowed in type signatures
429    |          help: replace with the correct type: `Option<u8>`
430 
430 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
433    |
433    |
434 LL |     fn method_test1(&self, x: _);

439 LL |     fn method_test1<T>(&self, x: T);
440    |                    ^^^           ^
441 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
444    |
444    |
445 LL |     fn method_test2(&self, x: _) -> _;

452 LL |     fn method_test2<T>(&self, x: T) -> T;
453    |                    ^^^           ^     ^
454 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
457    |
457    |
458 LL |     fn method_test3(&self) -> _;

463 LL |     fn method_test3<T>(&self) -> T;
464    |                    ^^^           ^
465 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
468    |
468    |
469 LL |     fn assoc_fn_test1(x: _);

474 LL |     fn assoc_fn_test1<T>(x: T);
475    |                      ^^^    ^
476 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
479    |
479    |
480 LL |     fn assoc_fn_test2(x: _) -> _;

487 LL |     fn assoc_fn_test2<T>(x: T) -> T;
488    |                      ^^^    ^     ^
489 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
492    |
492    |
493 LL |     fn assoc_fn_test3() -> _;

498 LL |     fn assoc_fn_test3<T>() -> T;
499    |                      ^^^      ^
500 
- error[E0121]: the type placeholder `_` is not allowed within associated types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for associated types
503    |
504 LL |     type B = _;

505    |              ^ not allowed in type signatures
505    |              ^ not allowed in type signatures
506 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
509    |
510 LL |     const C: _;

511    |              ^ not allowed in type signatures
511    |              ^ not allowed in type signatures
512 
- error[E0121]: the type placeholder `_` is not allowed within constants
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for constants
515    |
515    |
516 LL |     const D: _ = 42;
519    |              not allowed in type signatures
520    |              help: replace with the correct type: `i32`
521 
521 
- error[E0121]: the type placeholder `_` is not allowed within associated types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for associated types
524    |
524    |
525 LL |     type F: std::ops::Fn(_);
526    |                          ^ not allowed in type signatures
527 
527 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
530    |
530    |
531 LL |     fn test9(&self) -> _ { () }
534    |                        not allowed in type signatures
535    |                        help: replace with the correct return type: `()`
536 
536 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
539    |
539    |
540 LL |     fn test10(&self, _x : _) { }

545 LL |     fn test10<T>(&self, _x : T) { }
546    |              ^^^             ^
547 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
550    |
550    |
551 LL |     fn clone(&self) -> _ { Test9 }
554    |                        not allowed in type signatures
555    |                        help: replace with the correct return type: `Test9`
556 
556 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
559    |
559    |
560 LL |     fn clone_from(&mut self, other: _) { *self = Test9; }

565 LL |     fn clone_from<T>(&mut self, other: T) { *self = Test9; }
566    |                  ^^^                   ^
567 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
570    |
570    |
571 LL |         fn fn_test9(&self) -> _ { () }
574    |                               not allowed in type signatures
575    |                               help: replace with the correct return type: `()`
576 
576 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
579    |
579    |
580 LL |         fn fn_test10(&self, _x : _) { }

585 LL |         fn fn_test10<T>(&self, _x : T) { }
586    |                     ^^^             ^
587 
- error[E0121]: the type placeholder `_` is not allowed within return types
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for return types
590    |
590    |
591 LL |         fn clone(&self) -> _ { FnTest9 }
594    |                            not allowed in type signatures
594    |                            not allowed in type signatures
595    |                            help: replace with the correct return type: `FnTest9`
596 
- error[E0121]: the type placeholder `_` is not allowed within functions
+ error[E0121]: the type placeholder `_` is not allowed within types on item signatures for functions
599    |
599    |
600 LL |         fn clone_from(&mut self, other: _) { *self = FnTest9; }

605 LL |         fn clone_from<T>(&mut self, other: T) { *self = FnTest9; }
606    |                      ^^^                   ^
607 
- error[E0121]: the type placeholder `_` is not allowed within associated types
---
test result: FAILED. 11864 passed; 13 failed; 101 ignored; 0 measured; 0 filtered out; finished in 122.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:43
