plain
..........................................................................i............. 2904/13038
.........................................i.............................................. 2992/13038
........................................................................................ 3080/13038
........................................................................................ 3168/13038
..iiiii..F.............................................................................. 3256/13038
........................................................................................ 3432/13038
........................................................................................ 3520/13038
........................................................................................ 3608/13038
........................................................................................ 3696/13038
---
........................................................................................ 4224/13038
........................................................................................ 4312/13038
........................................................................................ 4400/13038
........................................................................................ 4488/13038
..............................................F.......F.....F........................... 4576/13038
........................................................................................ 4752/13038
........................................................................................ 4840/13038
........................................................................................ 4928/13038
........................................................i............................... 5016/13038
---
---- [ui] src/test/ui/anon-params/anon-params-edition-hygiene.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params/anon-params-edition-hygiene.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-edition-hygiene/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `:`, `@`, or `|`, found `)`
   |
   |
LL | generate_trait_2015!(u8);
   | ^^^^^^^^^^^^^^^^^^^^^^^^ expected one of `:`, `@`, or `|`
   |
   = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
   = note: this error originates in the macro `generate_trait_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
help: if this is a `self` type, give it a parameter name
   |
LL | generate_trait_2015!(self: u8);
help: if this is a parameter name, give it a type
   |
   |
LL | generate_trait_2015!(u8: TypeName);
help: if this is a type, explicitly ignore the parameter name
   |
   |
LL | generate_trait_2015!(_: u8);

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/asm/x86_64/interpolated-idents.rs stdout ----
diff of stderr:

1 error: the `nomem` and `readonly` options are mutually exclusive
-   --> $DIR/interpolated-idents.rs:13:13
3    |
3    |
4 LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack, $att_syntax));
+    |                                                                                                    ^
6 ...
6 ...
7 LL | /     m!(in out lateout inout inlateout const sym
8 LL | |        pure nomem readonly preserves_flags

12    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
13 
14 error: the `pure` and `noreturn` options are mutually exclusive
-   --> $DIR/interpolated-idents.rs:13:13
16    |
16    |
17 LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack, $att_syntax));
+    |                                                                                                    ^
19 ...
19 ...
20 LL | /     m!(in out lateout inout inlateout const sym
21 LL | |        pure nomem readonly preserves_flags

25    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
26 
27 error: asm outputs are not allowed with the `noreturn` option
-   --> $DIR/interpolated-idents.rs:10:32
29    |
29    |
30 LL |               asm!("", $in(x) x, $out(x) x, $lateout(x) x, $inout(x) x, $inlateout(x) x,
+    |                                          ^              ^            ^                ^
32 ...
32 ...
33 LL |       m!(in out lateout inout inlateout const sym


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/interpolated-idents/interpolated-idents.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/interpolated-idents/interpolated-idents.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args asm/x86_64/interpolated-idents.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/x86_64/interpolated-idents.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/interpolated-idents" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/x86_64/interpolated-idents/auxiliary"
stdout: none
--- stderr -------------------------------
error: the `nomem` and `readonly` options are mutually exclusive
   |
   |
LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack, $att_syntax));
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack att_syntax options);
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error: the `pure` and `noreturn` options are mutually exclusive
   |
   |
LL |               $options($pure, $nomem, $readonly, $preserves_flags, $noreturn, $nostack, $att_syntax));
...
...
LL | /     m!(in out lateout inout inlateout const sym
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack att_syntax options);
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error: asm outputs are not allowed with the `noreturn` option
   |
   |
LL |               asm!("", $in(x) x, $out(x) x, $lateout(x) x, $inout(x) x, $inlateout(x) x,
   |                                          ^              ^            ^                ^
...
LL |       m!(in out lateout inout inlateout const sym
   | |_____|
   | |_____|
   | |_____|
   | |
   | |
LL | |        pure nomem readonly preserves_flags
LL | |        noreturn nostack att_syntax options);
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   | |___________________________________________|
   | |___________________________________________in this macro invocation
   | |___________________________________________in this macro invocation
   | |___________________________________________in this macro invocation
   | |___________________________________________in this macro invocation
   |                                             in this macro invocation
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 3 previous errors
------------------------------------------



---- [ui] src/test/ui/borrowck/move-error-snippets.rs stdout ----
diff of stderr:

1 error[E0507]: cannot move out of static item `D`
-   --> $DIR/move-error-snippets-ext.rs:5:17
3    |
- LL |         let a = $c;
-    |                 ^^
-    |                 |
-    |                 |
-    |                 move occurs because `D` has type `A`, which does not implement the `Copy` trait
-    |                 help: consider borrowing here: `&$c`
-   ::: $DIR/move-error-snippets.rs:21:1
-    |
-    |
+ LL |             aaa!(D);
+    |                  |
+    |                  |
+    |                  move occurs because `D` has type `A`, which does not implement the `Copy` trait
+    |                  help: consider borrowing here: `&D`
+ ...
12 LL | sss!();
14    |


-    = note: this error originates in the macro `aaa` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `sss` (in Nightly builds, run with -Z macro-backtrace for more info)
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/move-error-snippets.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/move-error-snippets.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-snippets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0507]: cannot move out of static item `D`
   |
   |
LL |             aaa!(D);         //~ ERROR cannot move
   |                  |
   |                  |
   |                  move occurs because `D` has type `A`, which does not implement the `Copy` trait
   |                  help: consider borrowing here: `&D`
...
LL | sss!();
   |
   |
   = note: this error originates in the macro `sss` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0507`.
------------------------------------------
---
1 error[E0308]: mismatched types
-   --> $DIR/enum-discr-type-err.rs:18:21
+   --> $DIR/enum-discr-type-err.rs:18:25
3    |
4 LL |               $( $v = $s::V, )*
-    |                       ^^^^^ expected `isize`, found `i32`
+    |                           ^ expected `isize`, found `i32`
7 LL | / mac! {
7 LL | / mac! {
8 LL | |     A = F,

13    = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
15 error[E0308]: mismatched types
-   --> $DIR/enum-discr-type-err.rs:18:21
+   --> $DIR/enum-discr-type-err.rs:18:25
17    |
17    |
18 LL |               $( $v = $s::V, )*
-    |                       ^^^^^ expected `isize`, found `i32`
+    |                           ^ expected `isize`, found `i32`
21 LL | / mac! {
21 LL | / mac! {
22 LL | |     A = F,

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/enum-discr-type-err/enum-discr-type-err.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/enum-discr-type-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/enum-discr-type-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/enum-discr-type-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/enum-discr-type-err/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/enum-discr-type-err.rs:18:25
   |
   |
LL |               $( $v = $s::V, )*
   |                           ^ expected `isize`, found `i32`
LL | / mac! {
LL | / mac! {
LL | |     A = F,
LL | |     B = T,
LL | | }
   |
   |
   = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/consts/enum-discr-type-err.rs:18:25
   |
   |
LL |               $( $v = $s::V, )*
   |                           ^ expected `isize`, found `i32`
LL | / mac! {
LL | / mac! {
LL | |     A = F,
LL | |     B = T,
LL | | }
   |
   |
   = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/did_you_mean/recursion_limit.rs stdout ----
diff of stderr:

6    |
7    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]` attribute to your crate (`recursion_limit`)
8 note: required because it appears within the type `J`
-   --> $DIR/recursion_limit.rs:24:9
10    |
10    |
- LL | link! { J, K }
-    |         ^
+ LL |         enum $id { $id($t) }
13 note: required because it appears within the type `I`
-   --> $DIR/recursion_limit.rs:23:9
+   --> $DIR/recursion_limit.rs:11:14
15    |
15    |
- LL | link! { I, J }
-    |         ^
+ LL |         enum $id { $id($t) }
18 note: required because it appears within the type `H`
-   --> $DIR/recursion_limit.rs:22:9
+   --> $DIR/recursion_limit.rs:11:14
20    |
20    |
- LL | link! { H, I }
-    |         ^
+ LL |         enum $id { $id($t) }
23 note: required because it appears within the type `G`
-   --> $DIR/recursion_limit.rs:21:9
+   --> $DIR/recursion_limit.rs:11:14
25    |
25    |
- LL | link! { G, H }
-    |         ^
+ LL |         enum $id { $id($t) }
28 note: required because it appears within the type `F`
-   --> $DIR/recursion_limit.rs:20:9
+   --> $DIR/recursion_limit.rs:11:14
30    |
30    |
- LL | link! { F, G }
-    |         ^
+ LL |         enum $id { $id($t) }
33 note: required because it appears within the type `E`
-   --> $DIR/recursion_limit.rs:19:9
+   --> $DIR/recursion_limit.rs:11:14
35    |
35    |
- LL | link! { E, F }
-    |         ^
+ LL |         enum $id { $id($t) }
38 note: required because it appears within the type `D`
-   --> $DIR/recursion_limit.rs:18:9
+   --> $DIR/recursion_limit.rs:11:14
40    |
40    |
- LL | link! { D, E }
-    |         ^
+ LL |         enum $id { $id($t) }
43 note: required because it appears within the type `C`
-   --> $DIR/recursion_limit.rs:17:9
+   --> $DIR/recursion_limit.rs:11:14
45    |
45    |
- LL | link! { C, D }
-    |         ^
+ LL |         enum $id { $id($t) }
48 note: required because it appears within the type `B`
-   --> $DIR/recursion_limit.rs:16:9
+   --> $DIR/recursion_limit.rs:11:14
50    |
50    |
- LL | link! { B, C }
-    |         ^
+ LL |         enum $id { $id($t) }
53 note: required because it appears within the type `A`
-   --> $DIR/recursion_limit.rs:15:9
+   --> $DIR/recursion_limit.rs:11:14
55    |
55    |
- LL | link! { A, B }
-    |         ^
+ LL |         enum $id { $id($t) }
58 note: required by a bound in `is_send`
59   --> $DIR/recursion_limit.rs:31:14
60    |

---
To only update this specific test, also pass `--test-args did_you_mean/recursion_limit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/recursion_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `K: Send`
   |
   |
LL |     is_send::<A>(); //~ ERROR overflow evaluating the requirement
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "20"]` attribute to your crate (`recursion_limit`)
note: required because it appears within the type `J`
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `I`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `H`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `G`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `F`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `E`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `D`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `C`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `B`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required because it appears within the type `A`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:11:14
   |
   |
LL |         enum $id { $id($t) }
note: required by a bound in `is_send`
  --> /checkout/src/test/ui/did_you_mean/recursion_limit.rs:31:14
   |
   |
LL | fn is_send<T:Send>() { }
   |              ^^^^ required by this bound in `is_send`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/fmt/ifmt-unimpl.rs stdout ----
diff of stderr:

18 note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`
19   --> $SRC_DIR/core/src/fmt/mod.rs:LL:COL
20    |
- LL |     arg_new!(new_upper_hex, UpperHex);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
+ LL |         pub fn $f<'b, T: $t>(x: &'b T) -> ArgumentV1<'_> {
+    |                          ^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
24 
25 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/ifmt-unimpl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/ifmt-unimpl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/fmt/ifmt-unimpl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: UpperHex` is not satisfied
   |
   |
LL |     format!("{:X}", "3");
   |                     ^^^ the trait `UpperHex` is not implemented for `str`
   |
   = help: the following other types implement trait `UpperHex`:
             &T
             NonZeroI128
             NonZeroI16
             NonZeroI32
             NonZeroI64
             NonZeroI64
             NonZeroI8
             NonZeroIsize
           and 21 others
   = note: required because of the requirements on the impl of `UpperHex` for `&str`
note: required by a bound in `ArgumentV1::<'a>::new_upper_hex`
   |
   |
LL |         pub fn $f<'b, T: $t>(x: &'b T) -> ArgumentV1<'_> {
   |                          ^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
---
diff of stderr:

2   --> $DIR/duplicate_lifetimes.rs:8:14
3    |
4 LL |     fn g<$a, 'a>() {}
-    |              ^^ declared twice
+    |          --  ^^ declared twice
+    |          previous declaration here
6 ...
6 ...
7 LL | m!('a);
-    | |  |
-    | |  previous declaration here
-    | in this macro invocation
+    | ------ in this macro invocation
+    | ------ in this macro invocation
12    |
13    = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

16   --> $DIR/duplicate_lifetimes.rs:13:14
17    |
17    |
18 LL |     fn h<$a, 'a>() {}
-    |              ^^ declared twice
+    |          --  ^^ declared twice
+    |          previous declaration here
20 ...
20 ...
21 LL | n!('a);
-    | |  |
-    | |  previous declaration here
-    | in this macro invocation
+    | ------ in this macro invocation
+    | ------ in this macro invocation
26    |
27    = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/duplicate_lifetimes/duplicate_lifetimes.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/duplicate_lifetimes/duplicate_lifetimes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/duplicate_lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/duplicate_lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/duplicate_lifetimes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/duplicate_lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0263]: lifetime name `'a` declared twice in the same scope
   |
   |
LL |     fn g<$a, 'a>() {} //~ ERROR lifetime name `'a` declared twice
   |          --  ^^ declared twice
   |          previous declaration here
...
...
LL | m!('a);
   |
   |
   = note: this error originates in the macro `m` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0263]: lifetime name `'a` declared twice in the same scope
   |
   |
LL |     fn h<$a, 'a>() {} //~ ERROR lifetime name `'a` declared twice
   |          --  ^^ declared twice
   |          previous declaration here
...
...
LL | n!('a);
   |
   |
   = note: this error originates in the macro `n` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0263`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/hygiene/fields-definition.rs stdout ----
diff of stderr:

1 error[E0124]: field `a` is already declared
-   --> $DIR/fields-definition.rs:14:13
3    |
4 LL |             a: u8,
4 LL |             a: u8,
5    |             ----- `a` first declared here
6 LL |             $a: u8,
6 LL |             $a: u8,
-    |             ^^^^^^ field already declared
+    |                 ^^ field already declared
8 ...
9 LL | legacy!(a);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/fields-definition.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/fields-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/fields-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/fields-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0124]: field `a` is already declared
   |
LL |             a: u8,
LL |             a: u8,
   |             ----- `a` first declared here
LL |             $a: u8, //~ ERROR field `a` is already declared
   |                 ^^ field already declared
...
LL | legacy!(a);
   |
   |
   = note: this error originates in the macro `legacy` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0124`.
------------------------------------------
---
1 error[E0382]: use of moved value: `foo.x`
-   --> $DIR/fields-move.rs:18:9
+   --> $DIR/fields-move.rs:18:14
3    |
4 LL |         $foo.x
-    |         ^^^^^^ value used here after move
+    |              ^ value used here after move
6 ...
7 LL |     assert_two_copies(copy_modern!(foo), foo.x);
8    |                                          ----- value moved here
16   --> $DIR/fields-move.rs:28:42
17    |
17    |
18 LL |    $foo.x
-    |    ------ value moved here
+    |         - value moved here
20 ...
21 LL |     assert_two_copies(copy_modern!(foo), foo.x);
22    |                                          ^^^^^ value used here after move
27   --> $DIR/fields-move.rs:29:42
28    |
28    |
29 LL |         $foo.x
-    |         ------ value moved here
+    |              - value moved here
31 ...
32 LL |     assert_two_copies(copy_legacy!(foo), foo.x);
33    |                                          ^^^^^ value used here after move

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-move/fields-move.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/fields-move.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/fields-move.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-move" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/fields-move/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `foo.x`
   |
   |
LL |         $foo.x //~ ERROR use of moved value: `foo.x`
   |              ^ value used here after move
...
LL |     assert_two_copies(copy_modern!(foo), foo.x); //~ ERROR use of moved value: `foo.x`
   |                                          ----- value moved here
LL |     assert_two_copies(copy_legacy!(foo), foo.x); //~ ERROR use of moved value: `foo.x`
   |
   |
   = note: move occurs because `foo.x` has type `NonCopy`, which does not implement the `Copy` trait
   = note: this error originates in the macro `copy_legacy` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0382]: use of moved value: `foo.x`
  --> /checkout/src/test/ui/hygiene/fields-move.rs:28:42
   |
   |
LL |    $foo.x
   |         - value moved here
...
LL |     assert_two_copies(copy_modern!(foo), foo.x); //~ ERROR use of moved value: `foo.x`
   |                                          ^^^^^ value used here after move
   |
   = note: move occurs because `foo.x` has type `NonCopy`, which does not implement the `Copy` trait
error[E0382]: use of moved value: `foo.x`
  --> /checkout/src/test/ui/hygiene/fields-move.rs:29:42
   |
   |
LL |         $foo.x //~ ERROR use of moved value: `foo.x`
   |              - value moved here
...
LL |     assert_two_copies(copy_legacy!(foo), foo.x); //~ ERROR use of moved value: `foo.x`
   |                                          ^^^^^ value used here after move
   |
   = note: move occurs because `foo.x` has type `NonCopy`, which does not implement the `Copy` trait
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0382`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-25386.rs stdout ----
diff of stderr:

10    = note: this error originates in the macro `check_ptr_exist` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
12 error[E0616]: field `name` of struct `CObj` is private
-   --> $DIR/issue-25386.rs:26:43
14    |
14    |
- LL |     println!("{}", check_ptr_exist!(item, name));
-    |                                           ^^^^ private field
+ LL |         (*$var.c_object).$member.is_some()
17 
18 error: aborting due to 2 previous errors
19 

---
To only update this specific test, also pass `--test-args issues/issue-25386.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-25386.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25386" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-25386/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0616]: field `c_object` of struct `Item` is private
   |
   |
LL |         (*$var.c_object).$member.is_some()
...
...
LL |     println!("{}", check_ptr_exist!(item, name));
   |
   = note: this error originates in the macro `check_ptr_exist` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0616]: field `name` of struct `CObj` is private
   |
   |
LL |         (*$var.c_object).$member.is_some()

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0616`.
---
5    |         --          ^^^^^^ expected `{`
6    |         |
7    |         this `if` expression has a condition, but no block
- ...
- LL |     get_opt!(bar, foo);
11    |
-    = note: this error originates in the macro `get_opt` (in Nightly builds, run with -Z macro-backtrace for more info)
13 help: try placing this code inside a block
14    |
14    |
- LL |         if $tgt.has_{ $field() } {}
-    |                     +          +
+ LL |         if $tgt.has_$field{ () } {}
17 
18 error: aborting due to previous error
19 

---
To only update this specific test, also pass `--test-args issues/issue-39848.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-39848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-39848/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected `{`, found `foo`
   |
   |
LL |         if $tgt.has_$field() {} //~ ERROR expected `{`, found `foo`
   |         |
   |         this `if` expression has a condition, but no block
   |
help: try placing this code inside a block
help: try placing this code inside a block
   |
LL |         if $tgt.has_$field{ () } {} //~ ERROR expected `{`, found `foo`

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-42954.rs stdout ----
diff of stderr:

12    = note: this error originates in the macro `is_plainly_printable` (in Nightly builds, run with -Z macro-backtrace for more info)
14    |
14    |
- LL |         ($i as u32) < 0
-    |         +         +
+ LL |         $i as (u32) < 0
+    |               +   +
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42954/issue-42954.stderr
diff of fixed:

4 
5 macro_rules! is_plainly_printable {
6     ($i: ident) => {
-         ($i as u32) < 0 //~ `<` is interpreted as a start of generic arguments
+         $i as (u32) < 0 //~ `<` is interpreted as a start of generic arguments
9 }
10 



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42954/issue-42954.fixed
To only update this specific test, also pass `--test-args issues/issue-42954.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42954.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42954" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42954/auxiliary"
stdout: none
--- stderr -------------------------------
error: `<` is interpreted as a start of generic arguments for `u32`, not a comparison
   |
   |
LL |         $i as u32 < 0 //~ `<` is interpreted as a start of generic arguments
   |                   ^ - interpreted as generic arguments
   |                   not interpreted as comparison
...
...
LL |     is_plainly_printable!(c);
   |
   |
   = note: this error originates in the macro `is_plainly_printable` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
   |
LL |         $i as (u32) < 0 //~ `<` is interpreted as a start of generic arguments
   |               +   +
error: aborting due to previous error
------------------------------------------


---
To only update this specific test, also pass `--test-args macros/macro-parameter-span.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-parameter-span.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-parameter-span" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-parameter-span/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `x` in this scope
   |
LL |         $id
   |         ^^^ not found in this scope

---

---- [ui] src/test/ui/macros/span-covering-argument-1.rs stdout ----
diff of stderr:

1 error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
3    |
3    |
- LL |             let $s = 0;
-    |                 -- help: consider changing this to be mutable: `mut foo`
6 LL |             *&mut $s = 0;
-    |              ^^^^^^^ cannot borrow as mutable
+    |              ^ cannot borrow as mutable
8 ...
9 LL |     bad!(foo whatever);
+    |     ------------------
+    |     |    |
+    |     |    help: consider changing this to be mutable: `mut foo`
+    |     in this macro invocation
+    |     in this macro invocation
11    |
12    = note: this error originates in the macro `bad` (in Nightly builds, run with -Z macro-backtrace for more info)


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/span-covering-argument-1/span-covering-argument-1.stderr
To only update this specific test, also pass `--test-args macros/span-covering-argument-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/span-covering-argument-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/span-covering-argument-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/span-covering-argument-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow `foo` as mutable, as it is not declared as mutable
   |
   |
LL |             *&mut $s = 0;
   |              ^ cannot borrow as mutable
...
LL |     bad!(foo whatever);
   |     |    |
   |     |    help: consider changing this to be mutable: `mut foo`
   |     in this macro invocation
   |
   |
   = note: this error originates in the macro `bad` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/modules/issue-56411.rs stdout ----
diff of stderr:

4 LL |             mod $name;
5    |             ---------- previous definition of the module `issue_56411_aux` here
6 LL |             pub use self::$name;
+    |                     ^^^^
8    |                     |
8    |                     |
9    |                     `issue_56411_aux` reimported here
10    |                     you can use `as` to change the binding name of the import
19   --> $DIR/issue-56411.rs:6:21
20    |
20    |
21 LL |             pub use self::$name;
-    |                     ^^^^^^^^^^^ re-export of crate public `issue_56411_aux`
+    |                     ^^^^ re-export of crate public `issue_56411_aux`
23 ...
24 LL | import!(("issue-56411-aux.rs", issue_56411_aux));


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411/issue-56411.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411/issue-56411.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args modules/issue-56411.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/modules/issue-56411.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/modules/issue-56411/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0255]: the name `issue_56411_aux` is defined multiple times
   |
LL |             mod $name;
LL |             mod $name;
   |             ---------- previous definition of the module `issue_56411_aux` here
LL |             pub use self::$name;
   |                     |
   |                     |
   |                     `issue_56411_aux` reimported here
   |                     you can use `as` to change the binding name of the import
...
LL | import!(("issue-56411-aux.rs", issue_56411_aux));
   |
   |
   = note: `issue_56411_aux` must be defined only once in the type namespace of this module
   = note: this error originates in the macro `import` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0365]: `issue_56411_aux` is only public within the crate, and cannot be re-exported outside
   |
   |
LL |             pub use self::$name;
   |                     ^^^^ re-export of crate public `issue_56411_aux`
...
LL | import!(("issue-56411-aux.rs", issue_56411_aux));
   |
   |
   = note: consider declaring type or module `issue_56411_aux` with `pub`
   = note: this error originates in the macro `import` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0255, E0365.
For more information about an error, try `rustc --explain E0255`.
---
diff of stderr:

2   --> $DIR/decl-macro-illegal-copy.rs:21:9
3    |
4 LL |     $wrapper.inner
-    |     -------------- value moved here
+    |              ----- value moved here
7 LL |         wrapper.inner,
7 LL |         wrapper.inner,
8    |         ^^^^^^^^^^^^^ value used here after move

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/decl-macro-illegal-copy/decl-macro-illegal-copy.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/decl-macro-illegal-copy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/decl-macro-illegal-copy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/decl-macro-illegal-copy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/decl-macro-illegal-copy/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `wrapper.inner`
   |
   |
LL |     $wrapper.inner
   |              ----- value moved here
LL |         wrapper.inner,
LL |         wrapper.inner,
   |         ^^^^^^^^^^^^^ value used here after move
   |
   = note: move occurs because `wrapper.inner` has type `NonCopy`, which does not implement the `Copy` trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/parser/issues/issue-73568-lifetime-after-mut.rs stdout ----
diff of stderr:

14   --> $DIR/issue-73568-lifetime-after-mut.rs:6:22
15    |
16 LL |         fn w<$lt>(w: &mut $lt i32) {}
-    |                      ^^^^^^^^ help: place the lifetime before `mut`: `&$lt mut`
+    |                      ^ help: place the lifetime before `mut`: `&'a mut`
18 ...
19 LL | mac!('a);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-73568-lifetime-after-mut/issue-73568-lifetime-after-mut.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-73568-lifetime-after-mut/issue-73568-lifetime-after-mut.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issues/issue-73568-lifetime-after-mut.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-73568-lifetime-after-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-73568-lifetime-after-mut" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-73568-lifetime-after-mut/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime must precede `mut`
   |
   |
LL | fn x<'a>(x: &mut 'a i32){} //~ ERROR lifetime must precede `mut`
   |             ^^^^^^^ help: place the lifetime before `mut`: `&'a mut`

error[E0178]: expected a path on the left-hand side of `+`, not `&mut 'a`
   |
   |
LL | fn y<'a>(y: &mut 'a + Send) {
   |             ^^^^^^^^^^^^^^ help: try adding parentheses: `&mut ('a + Send)`
error: lifetime must precede `mut`
  --> /checkout/src/test/ui/parser/issues/issue-73568-lifetime-after-mut.rs:6:22
   |
   |
LL |         fn w<$lt>(w: &mut $lt i32) {}
   |                      ^ help: place the lifetime before `mut`: `&'a mut`
...
LL | mac!('a);
   |
   |
   = note: this error originates in the macro `mac` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0423]: expected value, found trait `Send`
  --> /checkout/src/test/ui/parser/issues/issue-73568-lifetime-after-mut.rs:17:28
   |
   |
LL |     let z = y as &mut 'a + Send;

error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/parser/issues/issue-73568-lifetime-after-mut.rs:14:18
   |
   |
LL | fn y<'a>(y: &mut 'a + Send) {

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0178, E0224, E0423.
Some errors have detailed explanations: E0178, E0224, E0423.
For more information about an error, try `rustc --explain E0178`.
------------------------------------------


---- [ui] src/test/ui/proc-macro/capture-macro-rules-invoke.rs stdout ----
diff of stdout:

121         spacing: Alone,
122         span: $DIR/capture-macro-rules-invoke.rs:14:62: 14:63 (#8),
-     Group {
-         delimiter: None,
-         delimiter: None,
-         stream: TokenStream [
-             Punct {
-                 ch: '\'',
-                 spacing: Joint,
-                 span: $DIR/capture-macro-rules-invoke.rs:43:13: 43:15 (#0),
-             Ident {
-                 ident: "a",
-                 ident: "a",
-                 span: $DIR/capture-macro-rules-invoke.rs:43:13: 43:15 (#0),
-         ],
-         ],
-         span: $DIR/capture-macro-rules-invoke.rs:15:29: 15:38 (#8),
+     Punct {
+         ch: '\'',
+         spacing: Joint,
+         span: $DIR/capture-macro-rules-invoke.rs:43:13: 43:15 (#0),
+     Ident {
+         ident: "a",
+         ident: "a",
+         span: $DIR/capture-macro-rules-invoke.rs:43:13: 43:15 (#0),
139     Punct {
139     Punct {
140         ch: ',',

The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/capture-macro-rules-invoke/capture-macro-rules-invoke.stdout
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/capture-macro-rules-invoke.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/capture-macro-rules-invoke" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "span-debug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/capture-macro-rules-invoke/auxiliary"
--- stdout -------------------------------
PRINT-BANG INPUT (DISPLAY): self
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
                ident: "self",
                ident: "self",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:36:24: 36:28 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:21:21: 21:26 (#4),
]
]
PRINT-BANG INPUT (DISPLAY): 1 + 1, { "a" }, let a = 1;, String, my_name, 'a, my_val = 30,
std::option::Option, pub(in some::path) , [a b c], -30
PRINT-BANG RE-COLLECTED (DISPLAY): 1 + 1, { "a" }, let a = 1, String, my_name, 'a, my_val = 30,
std :: option :: Option, pub(in some :: path), [a b c], - 30
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
            Literal {
                symbol: "1",
                suffix: None,
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:38:13: 38:14 (#0),
            Punct {
            Punct {
                ch: '+',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:38:15: 38:16 (#0),
            Literal {
                kind: Integer,
                symbol: "1",
                suffix: None,
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:38:17: 38:18 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:29: 14:34 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:34: 14:35 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
            Group {
                delimiter: Brace,
                stream: TokenStream [
                    Literal {
                        kind: Str,
                        symbol: "a",
                        suffix: None,
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:39:15: 39:18 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:39:13: 39:20 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:36: 14:42 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:42: 14:43 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
            Ident {
                ident: "let",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:40:13: 40:16 (#0),
            Ident {
                ident: "a",
                ident: "a",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:40:17: 40:18 (#0),
            Punct {
            Punct {
                ch: '=',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:40:19: 40:20 (#0),
            Literal {
                kind: Integer,
                symbol: "1",
                suffix: None,
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:40:21: 40:22 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:44: 14:49 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:49: 14:50 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
                ident: "String",
                ident: "String",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:41:13: 41:19 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:51: 14:54 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:54: 14:55 (#8),
    Ident {
    Ident {
        ident: "my_name",
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:42:13: 42:20 (#0),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:14:62: 14:63 (#8),
    Punct {
    Punct {
        ch: '\'',
        spacing: Joint,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:43:13: 43:15 (#0),
    Ident {
        ident: "a",
        ident: "a",
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:43:13: 43:15 (#0),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:38: 15:39 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
            Ident {
                ident: "my_val",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:44:13: 44:19 (#0),
            Punct {
            Punct {
                ch: '=',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:44:20: 44:21 (#0),
            Literal {
                kind: Integer,
                symbol: "30",
                suffix: None,
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:44:22: 44:24 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:40: 15:45 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:45: 15:46 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
                ident: "std",
                ident: "std",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:13: 45:16 (#0),
            Punct {
                ch: ':',
                spacing: Joint,
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:16: 45:18 (#0),
            Punct {
                ch: ':',
                spacing: Alone,
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:16: 45:18 (#0),
            Ident {
                ident: "option",
                ident: "option",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:18: 45:24 (#0),
            Punct {
                ch: ':',
                spacing: Joint,
                spacing: Joint,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:24: 45:26 (#0),
            Punct {
                ch: ':',
                spacing: Alone,
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:24: 45:26 (#0),
            Ident {
                ident: "Option",
                ident: "Option",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:45:26: 45:32 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:47: 15:52 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:52: 15:53 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
            Ident {
                ident: "pub",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:13: 46:16 (#0),
            Group {
                delimiter: Parenthesis,
                delimiter: Parenthesis,
                stream: TokenStream [
                        ident: "in",
                        ident: "in",
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:17: 46:19 (#0),
                    Ident {
                    Ident {
                        ident: "some",
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:20: 46:24 (#0),
                    Punct {
                        ch: ':',
                        spacing: Joint,
                        spacing: Joint,
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:24: 46:26 (#0),
                    Punct {
                        ch: ':',
                        spacing: Alone,
                        spacing: Alone,
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:24: 46:26 (#0),
                    Ident {
                        ident: "path",
                        ident: "path",
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:26: 46:30 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:46:16: 46:31 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:54: 15:58 (#8),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:58: 15:59 (#8),
    Group {
        delimiter: Bracket,
        delimiter: Bracket,
        stream: TokenStream [
                ident: "a",
                ident: "a",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:47:15: 47:16 (#0),
            Ident {
                ident: "b",
                ident: "b",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:47:17: 47:18 (#0),
            Ident {
                ident: "c",
                ident: "c",
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:47:19: 47:20 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:47:13: 47:22 (#0),
    Punct {
    Punct {
        ch: ',',
        spacing: Alone,
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:63: 15:64 (#8),
    Group {
        delimiter: None,
        delimiter: None,
        stream: TokenStream [
            Punct {
                ch: '-',
                spacing: Alone,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:48:13: 48:14 (#0),
            Literal {
                kind: Integer,
                symbol: "30",
                suffix: None,
                suffix: None,
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:48:14: 48:16 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:15:65: 15:69 (#8),
]
]
PRINT-BANG INPUT (DISPLAY): (a, b)
PRINT-BANG INPUT (DEBUG): TokenStream [
    Group {
        delimiter: None,
        stream: TokenStream [
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [
                        ident: "a",
                        ident: "a",
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:52:27: 52:28 (#0),
                    Punct {
                    Punct {
                        ch: ',',
                        spacing: Alone,
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:52:28: 52:29 (#0),
                    Ident {
                        ident: "b",
                        ident: "b",
                        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:52:30: 52:31 (#0),
                ],
                ],
                span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:52:26: 52:32 (#0),
        ],
        ],
        span: /checkout/src/test/ui/proc-macro/capture-macro-rules-invoke.rs:27:21: 27:25 (#12),
]
------------------------------------------
stderr: none



---- [ui] src/test/ui/proc-macro/input-interpolated.rs stdout ----
diff of stdout:

13     },
14     Ident {
15         ident: "A",
-         span: #0 bytes(503..504),
+         span: #0 bytes(422..424),
18     Punct {
19         ch: ':',

49     },
49     },
50     Ident {
51         ident: "A",
-         span: #0 bytes(503..504),
+         span: #0 bytes(475..477),
54     Group {
55         delimiter: Brace,



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/input-interpolated/input-interpolated.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/input-interpolated.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/input-interpolated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/input-interpolated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/input-interpolated/auxiliary"
--- stdout -------------------------------
PRINT-BANG INPUT (DISPLAY): A
