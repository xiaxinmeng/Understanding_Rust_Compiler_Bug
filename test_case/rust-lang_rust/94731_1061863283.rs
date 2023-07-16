plain
.................................................................................................... 11800/12706
.................................................................................................... 11900/12706
..........................i......................................................................... 12000/12706
.................................................................................................... 12100/12706
....................F.........................F.F................................................... 12200/12706
.................................................................................................... 12400/12706
.................................................................................................... 12500/12706
.................................................................................................... 12600/12706
.iii................................................................................................ 12700/12706
---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-generic-function.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-generic-function" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-generic-function/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/const-generic-function.rs:16:11
   |
   |
LL |     foo::<baz()>(); //~ ERROR invalid const generic expression
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL |     foo::<{ baz() }>(); //~ ERROR invalid const generic expression

error: invalid const generic expression
  --> /checkout/src/test/ui/const-generics/const-generic-function.rs:17:11
   |
   |
LL |     foo::<bar(bar(1, 1), bar(1, 1))>(); //~ ERROR invalid const generic expression
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL |     foo::<{ bar(bar(1, 1), bar(1, 1)) }>(); //~ ERROR invalid const generic expression

error: invalid const generic expression
  --> /checkout/src/test/ui/const-generics/const-generic-function.rs:18:11
   |
   |
LL |     foo::<bar(1, 1)>(); //~ ERROR invalid const generic expression
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL |     foo::<{ bar(1, 1) }>(); //~ ERROR invalid const generic expression

error: invalid const generic expression
  --> /checkout/src/test/ui/const-generics/const-generic-function.rs:19:11
   |
   |
LL |     foo::<bar(FOO, 2)>(); //~ ERROR invalid const generic expression
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL |     foo::<{ bar(FOO, 2) }>(); //~ ERROR invalid const generic expression

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] ui/generic-associated-types/gat-trait-path-parenthesised-args.rs stdout ----
diff of stderr:

- error: lifetime in trait object type must be followed by `+`
-   --> $DIR/gat-trait-path-parenthesised-args.rs:7:29
+ error: expected `while`, `for`, `loop` or `{` after a label
+   --> $DIR/gat-trait-path-parenthesised-args.rs:7:31
3    |
4 LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
-    |                             ^^
+    |                               ^ expected `while`, `for`, `loop` or `{` after a label
- error: parenthesized generic arguments cannot be used in associated type constraints
-   --> $DIR/gat-trait-path-parenthesised-args.rs:7:27
+ error: expected expression, found `)`
+   --> $DIR/gat-trait-path-parenthesised-args.rs:7:31
+   --> $DIR/gat-trait-path-parenthesised-args.rs:7:31
9    |
10 LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
+    |                               ^ expected expression
12 
12 
- error[E0107]: this associated type takes 1 lifetime argument but 0 lifetime arguments were supplied
-   --> $DIR/gat-trait-path-parenthesised-args.rs:7:27
+ error: borrow expressions cannot be annotated with lifetimes
+   --> $DIR/gat-trait-path-parenthesised-args.rs:7:35
15    |
16 LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
-    |                           ^ expected 1 lifetime argument
-    |
- note: associated type defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/gat-trait-path-parenthesised-args.rs:4:8
-    |
- LL |   type Y<'a>;
-    |        ^ --
- help: add missing lifetime argument
-    |
- LL | fn foo<'a>(arg: Box<dyn X<Y('a, 'a) = &'a ()>>) {}
+    |                                   ^--^^^
+    |                                    |
+    |                                    |
+    |                                    annotated with lifetime here
28 
28 
- error[E0107]: this associated type takes 0 generic arguments but 1 generic argument was supplied
+ error: expected one of `>`, a const expression, or lifetime, found `Y`
30   --> $DIR/gat-trait-path-parenthesised-args.rs:7:27
31    |
32 LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}

-    |                           ^---- help: remove these generics
-    |                           expected 0 generic arguments
-    |
- note: associated type defined here, with 0 generic parameters
-   --> $DIR/gat-trait-path-parenthesised-args.rs:4:8
-   --> $DIR/gat-trait-path-parenthesised-args.rs:4:8
-    |
- LL |   type Y<'a>;
-    |        ^
+    |                           ^ expected one of `>`, a const expression, or lifetime
43 error: aborting due to 4 previous errors
44 

- For more information about this error, try `rustc --explain E0107`.
---
To only update this specific test, also pass `--test-args generic-associated-types/gat-trait-path-parenthesised-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-parenthesised-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/gat-trait-path-parenthesised-args/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected `while`, `for`, `loop` or `{` after a label
   |
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
   |                               ^ expected `while`, `for`, `loop` or `{` after a label
error: expected expression, found `)`
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:7:31
   |
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
   |                               ^ expected expression
error: borrow expressions cannot be annotated with lifetimes
  --> /checkout/src/test/ui/generic-associated-types/gat-trait-path-parenthesised-args.rs:7:35
   |
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
   |                                   ^--^^^
   |                                    |
   |                                    annotated with lifetime here


error: expected one of `>`, a const expression, or lifetime, found `Y`
   |
   |
LL | fn foo<'a>(arg: Box<dyn X<Y('a) = &'a ()>>) {}
   |                           ^ expected one of `>`, a const expression, or lifetime
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] ui/issues/issue-32995.rs stdout ----
diff of stderr:

+ error: expected one of `)`, `,`, `.`, `?`, or an operator, found `::`
+    |
+    |
+ LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);
+    |                                     -^
+    |                                     |
+    |                                     expected one of `)`, `,`, `.`, `?`, or an operator
+    |                                     help: missing `,`
+ error: invalid const generic expression
+   --> $DIR/issue-32995.rs:14:17
+    |
+    |
+ LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);
+    |
+    |
+ help: expressions must be enclosed in braces to be used as const generic arguments
+    |
+ LL |     let o : Box<{ dyn (::std::marker()::Send) }> = Box::new(1);
+ 
+ 
1 error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
3    |


23    |                         ^^^^^^^^^^^^^ only `Fn` traits may use parentheses
24 
25 error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
-   --> $DIR/issue-32995.rs:14:29
-    |
- LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);
-    |                             ^^^^^^^^ only `Fn` traits may use parentheses
- 
- error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
33    |
33    |
34 LL |     let o : Box<dyn Send + ::std::marker()::Sync> = Box::new(1);

40 LL |     let d : X() = Default::default();
41    |             ^^^ only `Fn` traits may use parentheses
- error: aborting due to 7 previous errors
+ error[E0747]: constant provided when a type was expected
+   --> $DIR/issue-32995.rs:14:17
+    |
+    |
+ LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);
44 
- For more information about this error, try `rustc --explain E0214`.
+ error: aborting due to 9 previous errors
+ 
---
To only update this specific test, also pass `--test-args issues/issue-32995.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-32995.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32995" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-32995/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `)`, `,`, `.`, `?`, or an operator, found `::`
   |
   |
LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);
   |                                     -^
   |                                     |
   |                                     expected one of `)`, `,`, `.`, `?`, or an operator
   |                                     help: missing `,`
error: invalid const generic expression
  --> /checkout/src/test/ui/issues/issue-32995.rs:14:17
   |
   |
LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL |     let o : Box<{ dyn (::std::marker()::Send) }> = Box::new(1);


error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
LL |     let x: usize() = 1;
LL |     let x: usize() = 1;
   |            ^^^^^^^ only `Fn` traits may use parentheses

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
   |
LL |     let b: ::std::boxed()::Box<_> = Box::new(1);
   |                   ^^^^^^^ only `Fn` traits may use parentheses

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
   |
LL |     let p = ::std::str::()::from_utf8(b"foo").unwrap();
   |                    ^^^^^^^ only `Fn` traits may use parentheses

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
   |
LL |     let p = ::std::str::from_utf8::()(b"foo").unwrap();
   |                         ^^^^^^^^^^^^^ only `Fn` traits may use parentheses

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
   |
LL |     let o : Box<dyn Send + ::std::marker()::Sync> = Box::new(1);
   |                                   ^^^^^^^^ only `Fn` traits may use parentheses

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
   |
LL |     let d : X() = Default::default();
   |             ^^^ only `Fn` traits may use parentheses
error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/issues/issue-32995.rs:14:17
   |
   |
LL |     let o : Box<dyn (::std::marker()::Send)> = Box::new(1);

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0214, E0747.
Some errors have detailed explanations: E0214, E0747.
For more information about an error, try `rustc --explain E0214`.
------------------------------------------


---- [ui] ui/lifetimes/lifetime-bound-will-change-warning.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/lifetimes/auxiliary/lifetime_bound_will_change_warning_lib.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/auxiliary/lifetime_bound_will_change_warning_lib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lifetimes/auxiliary/lifetime_bound_will_change_warning_lib.rs:9:24
   |
   |
LL | pub fn ref_obj(x: &Box<Fn()>) {
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL | pub fn ref_obj(x: &Box<{ Fn() }>) {

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/lifetimes/auxiliary/lifetime_bound_will_change_warning_lib.rs:6:21
   |
   |
LL | pub fn just_ref(x: &Fn()) {
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub fn just_ref(x: &Fn()) {
LL + pub fn just_ref(x: &dyn Fn()) {

error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/lifetimes/auxiliary/lifetime_bound_will_change_warning_lib.rs:9:24
   |
   |
LL | pub fn ref_obj(x: &Box<Fn()>) {

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0747`.
---

6    |          |
7    |          unclosed delimiter
8 
- error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `;`
+ error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `;`
11    |
11    |
12 LL | impl W <s(f;Y(;]
-    |            ^ expected one of 7 possible tokens
+    |            ^
+    |            |
+    |            expected one of 8 possible tokens
+    |            expected one of 8 possible tokens
+    |            help: missing `,`
14 
+ error: expected one of `)`, `,`, `.`, `?`, or an operator, found `Y`
+    |
+    |
+ LL | impl W <s(f;Y(;]
+    |             |
+    |             |
+    |             expected one of `)`, `,`, `.`, `?`, or an operator
+    |             help: missing `,`
+ error: invalid const generic expression
+   --> $DIR/issue-63116.rs:3:9
+    |
+    |
+ LL | impl W <s(f;Y(;]
+    |
+    |
+ help: expressions must be enclosed in braces to be used as const generic arguments
+    |
+ LL | impl W <{ s(f;Y(;] }
+ 
15 error: mismatched closing delimiter: `]`
16   --> $DIR/issue-63116.rs:3:14
17    |
17    |

20    |              |
21    |              unclosed delimiter
22 
- error: aborting due to 3 previous errors
+ error: expected one of `,`, `.`, `:`, `=`, `>`, `?`, or an operator, found `<eof>`
+    |
+    |
+ LL | impl W <s(f;Y(;]
+    |                  ^ expected one of 7 possible tokens
+ error: aborting due to 6 previous errors
24 
25 

---
To only update this specific test, also pass `--test-args parser/issues/issue-63116.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-63116.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-63116" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-63116/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:18
   |
   |
LL | impl W <s(f;Y(;]
   |          -       ^
   |          unclosed delimiter


error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `;`
   |
   |
LL | impl W <s(f;Y(;]
   |            |
   |            expected one of 8 possible tokens
   |            help: missing `,`


error: expected one of `)`, `,`, `.`, `?`, or an operator, found `Y`
   |
   |
LL | impl W <s(f;Y(;]
   |             |
   |             |
   |             expected one of `)`, `,`, `.`, `?`, or an operator
   |             help: missing `,`
error: invalid const generic expression
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:9
   |
   |
LL | impl W <s(f;Y(;]
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL | impl W <{ s(f;Y(;] }

error: mismatched closing delimiter: `]`
  --> /checkout/src/test/ui/parser/issues/issue-63116.rs:3:14
   |
   |
LL | impl W <s(f;Y(;]
   |              ^ ^ mismatched closing delimiter
   |              unclosed delimiter


error: expected one of `,`, `.`, `:`, `=`, `>`, `?`, or an operator, found `<eof>`
   |
   |
LL | impl W <s(f;Y(;]
   |                  ^ expected one of 7 possible tokens
error: aborting due to 6 previous errors
------------------------------------------



---- [ui] ui/typeid-intrinsic.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/auxiliary/typeid-intrinsic-aux1.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/auxiliary/typeid-intrinsic-aux1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeid-intrinsic/auxiliary" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeid-intrinsic/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/auxiliary/typeid-intrinsic-aux1.rs:12:18
   |
   |
LL | pub type I = Box<Fn()>;
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL | pub type I = Box<{ Fn() }>;

error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/auxiliary/typeid-intrinsic-aux1.rs:12:18
   |
   |
LL | pub type I = Box<Fn()>;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/auxiliary/typeid-intrinsic-aux1.rs:13:24
   |
   |
LL | pub type I32Iterator = Iterator<Item=i32>;
   |
   = note: `#[warn(bare_trait_objects)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
   |
LL - pub type I32Iterator = Iterator<Item=i32>;
LL + pub type I32Iterator = dyn Iterator<Item=i32>;

warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/auxiliary/typeid-intrinsic-aux1.rs:14:24
   |
   |
LL | pub type U32Iterator = Iterator<Item=u32>;
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
help: use `dyn`
   |
LL - pub type U32Iterator = Iterator<Item=u32>;
LL + pub type U32Iterator = dyn Iterator<Item=u32>;

error: aborting due to 2 previous errors; 2 warnings emitted

For more information about this error, try `rustc --explain E0747`.
For more information about this error, try `rustc --explain E0747`.
------------------------------------------


---- [ui] ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1.rs stdout ----
diff of stderr:

- error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
+ error: invalid const generic expression
3    |
3    |
4 LL |     let x: Box<Bar()> = panic!();

-    |                ^^^^^ only `Fn` traits may use parentheses
+    |
+    |
+ help: expressions must be enclosed in braces to be used as const generic arguments
+    |
+ LL |     let x: Box<{ Bar() }> = panic!();
6 
6 
- error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
+ error[E0747]: constant provided when a type was expected
9    |
9    |
10 LL |     let x: Box<Bar()> = panic!();
-    |                ^^^ expected 1 generic argument
-    |
-    |
- note: struct defined here, with 1 generic parameter: `A`
-   --> $DIR/unboxed-closure-sugar-used-on-struct-1.rs:3:8
-    |
- LL | struct Bar<A> {
-    |        ^^^ -
- help: add missing generic argument
-    |
- LL |     let x: Box<Bar(A)> = panic!();
+    |                ^^^^^
22 
23 error: aborting due to 2 previous errors
24 
---
27 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1/unboxed-closure-sugar-used-on-struct-1.stderr
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closure-sugar-used-on-struct-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1.rs:8:16
   |
   |
LL |     let x: Box<Bar()> = panic!();
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL |     let x: Box<{ Bar() }> = panic!();

error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct-1.rs:8:16
   |
   |
LL |     let x: Box<Bar()> = panic!();

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0747`.
For more information about this error, try `rustc --explain E0747`.
------------------------------------------


---- [ui] ui/unboxed-closures/unboxed-closure-sugar-used-on-struct.rs stdout ----
diff of stderr:

- error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
+ error: invalid const generic expression
3    |
3    |
4 LL | fn foo(b: Box<Bar()>) {

-    |               ^^^^^ only `Fn` traits may use parentheses
+    |
+    |
+ help: expressions must be enclosed in braces to be used as const generic arguments
+    |
+ LL | fn foo(b: Box<{ Bar() }>) {
6 
6 
- error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
+ error[E0747]: constant provided when a type was expected
9    |
9    |
10 LL | fn foo(b: Box<Bar()>) {
-    |               ^^^ expected 1 generic argument
-    |
-    |
- note: struct defined here, with 1 generic parameter: `A`
-   --> $DIR/unboxed-closure-sugar-used-on-struct.rs:3:8
-    |
- LL | struct Bar<A> {
-    |        ^^^ -
- help: add missing generic argument
-    |
- LL | fn foo(b: Box<Bar(A)>) {
+    |               ^^^^^
22 
23 error: aborting due to 2 previous errors
24 
---
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closure-sugar-used-on-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct.rs:7:15
   |
   |
LL | fn foo(b: Box<Bar()>) {
   |
   |
help: expressions must be enclosed in braces to be used as const generic arguments
   |
LL | fn foo(b: Box<{ Bar() }>) {

error[E0747]: constant provided when a type was expected
  --> /checkout/src/test/ui/unboxed-closures/unboxed-closure-sugar-used-on-struct.rs:7:15
   |
   |
LL | fn foo(b: Box<Bar()>) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0747`.
