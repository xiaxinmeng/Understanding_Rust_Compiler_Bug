plain
.................................................................................................... 10100/11424
.................................................................................................... 10200/11424
.................................................................................................... 10300/11424
.............i...................................................................................... 10400/11424
..F.F........FFFF.F..F..F.F.....................FF.FF.F.FFF................F........FF........F.FF.. 10500/11424
..F.FF....................................................F.........................F.........FF.FF. 10600/11424
.F..............F.................FFFFFF.F.......................................................... 10700/11424
.................................................................................................... 10900/11424
.................................................................................................... 11000/11424
.................................................................................................... 11100/11424
.................................................................................................... 11200/11424
.................................................................................................... 11200/11424
.................................................................................................... 11300/11424
..............i.i................................................................................... 11400/11424
........................
failures:

---- [ui] ui/traits/alias/ambiguous.rs stdout ----

1 error[E0034]: multiple applicable items in scope
-   --> $DIR/trait-alias-ambiguous.rs:21:7
+   --> $DIR/ambiguous.rs:21:7
+   --> $DIR/ambiguous.rs:21:7
3    |
4 LL |     t.foo();
5    |       ^^^ multiple `foo` found
6    |
6    |
7 note: candidate #1 is defined in an impl of the trait `A` for the type `u8`
-   --> $DIR/trait-alias-ambiguous.rs:8:9
+   --> $DIR/ambiguous.rs:8:9
9    |
10 LL |         fn foo(&self) {}


12 note: candidate #2 is defined in an impl of the trait `B` for the type `u8`
-   --> $DIR/trait-alias-ambiguous.rs:11:9
+   --> $DIR/ambiguous.rs:11:9
14    |
15 LL |         fn foo(&self) {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous/ambiguous.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/ambiguous.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0034]: multiple applicable items in scope
  --> /checkout/src/test/ui/traits/alias/ambiguous.rs:21:7
   |
LL |     t.foo(); //~ ERROR E0034
   |       ^^^ multiple `foo` found
   |
note: candidate #1 is defined in an impl of the trait `A` for the type `u8`
  --> /checkout/src/test/ui/traits/alias/ambiguous.rs:8:9
   |
LL |         fn foo(&self) {}
   |         ^^^^^^^^^^^^^
note: candidate #2 is defined in an impl of the trait `B` for the type `u8`
  --> /checkout/src/test/ui/traits/alias/ambiguous.rs:11:9
   |
LL |         fn foo(&self) {}
help: disambiguate the associated function for candidate #1
   |
   |
LL |     A::foo(&t); //~ ERROR E0034
help: disambiguate the associated function for candidate #2
   |
   |
LL |     B::foo(&t); //~ ERROR E0034

error: aborting due to previous error

For more information about this error, try `rustc --explain E0034`.
For more information about this error, try `rustc --explain E0034`.

------------------------------------------


---- [ui] ui/traits/alias/impl.rs stdout ----
diff of stderr:

1 error[E0404]: expected trait, found trait alias `DefaultAlias`
-   --> $DIR/trait-alias-impl.rs:5:6
3    |
3    |
4 LL | impl DefaultAlias for () {}
5    |      ^^^^^^^^^^^^ not a trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl/impl.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl/impl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/impl.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0404]: expected trait, found trait alias `DefaultAlias`
   |
   |
LL | impl DefaultAlias for () {} //~ ERROR expected trait, found trait alias
   |      ^^^^^^^^^^^^ not a trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0404`.


------------------------------------------


---- [ui] ui/traits/alias/only-maybe-bound.rs stdout ----

1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-alias-only-maybe-bound.rs:13:12
+   --> $DIR/only-maybe-bound.rs:13:12
+   --> $DIR/only-maybe-bound.rs:13:12
3    |
4 LL | type _T0 = dyn _1;

6 
7 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-alias-only-maybe-bound.rs:19:12
-   --> $DIR/trait-alias-only-maybe-bound.rs:19:12
+   --> $DIR/only-maybe-bound.rs:19:12
9    |
10 LL | type _T1 = dyn _2;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/only-maybe-bound/only-maybe-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/only-maybe-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/only-maybe-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/only-maybe-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/alias/only-maybe-bound.rs:13:12
   |
LL | type _T0 = dyn _1;

error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/alias/only-maybe-bound.rs:19:12
   |
   |
LL | type _T1 = dyn _2;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0224`.
For more information about this error, try `rustc --explain E0224`.

------------------------------------------


---- [ui] ui/traits/alias/syntax-fail.rs stdout ----


1 error: trait aliases cannot be `auto`
-   --> $DIR/trait-alias-syntax-fail.rs:4:1
3    |
3    |
4 LL | auto trait A = Foo;
5    | ^^^^^^^^^^^^^^^^^^^ trait aliases cannot be `auto`
6 
6 
7 error: trait aliases cannot be `unsafe`
-   --> $DIR/trait-alias-syntax-fail.rs:5:1
9    |
9    |
10 LL | unsafe trait B = Foo;
11    | ^^^^^^^^^^^^^^^^^^^^^ trait aliases cannot be `unsafe`
12 
13 error: bounds are not allowed on trait aliases
-   --> $DIR/trait-alias-syntax-fail.rs:7:8
+   --> $DIR/syntax-fail.rs:7:8
+   --> $DIR/syntax-fail.rs:7:8
15    |
16 LL | trait C: Ord = Eq;

18 
19 error: bounds are not allowed on trait aliases
-   --> $DIR/trait-alias-syntax-fail.rs:8:8
-   --> $DIR/trait-alias-syntax-fail.rs:8:8
+   --> $DIR/syntax-fail.rs:8:8
21    |
22 LL | trait D: = Eq;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/syntax-fail/syntax-fail.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/syntax-fail/syntax-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/syntax-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/syntax-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/syntax-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/syntax-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: trait aliases cannot be `auto`
  --> /checkout/src/test/ui/traits/alias/syntax-fail.rs:4:1
   |
LL | auto trait A = Foo; //~ ERROR trait aliases cannot be `auto`
   | ^^^^^^^^^^^^^^^^^^^ trait aliases cannot be `auto`

error: trait aliases cannot be `unsafe`
  --> /checkout/src/test/ui/traits/alias/syntax-fail.rs:5:1
   |
LL | unsafe trait B = Foo; //~ ERROR trait aliases cannot be `unsafe`
   | ^^^^^^^^^^^^^^^^^^^^^ trait aliases cannot be `unsafe`
error: bounds are not allowed on trait aliases
  --> /checkout/src/test/ui/traits/alias/syntax-fail.rs:7:8
   |
   |
LL | trait C: Ord = Eq; //~ ERROR bounds are not allowed on trait aliases

error: bounds are not allowed on trait aliases
  --> /checkout/src/test/ui/traits/alias/syntax-fail.rs:8:8
   |
   |
LL | trait D: = Eq; //~ ERROR bounds are not allowed on trait aliases

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/traits/alias/object-fail.rs stdout ----


1 error[E0038]: the trait `Eq` cannot be made into an object
-   --> $DIR/trait-alias-object-fail.rs:7:13
3    |
3    |
4 LL |     let _: &dyn EqAlias = &123;
5    |             ^^^^^^^^^^^ `Eq` cannot be made into an object

11    |               ^^^^^^^^^^^^^^^ the trait cannot be made into an object because it uses `Self` as a type parameter
12 
13 error[E0191]: the value of the associated type `Item` (from trait `Iterator`) must be specified
-   --> $DIR/trait-alias-object-fail.rs:9:17
15    |
15    |
16 LL |     let _: &dyn IteratorAlias = &vec![123].into_iter();
17    |                 ^^^^^^^^^^^^^ help: specify the associated type: `IteratorAlias<Item = Type>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/object-fail/object-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/alias/object-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/object-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/object-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/object-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0038]: the trait `Eq` cannot be made into an object
  --> /checkout/src/test/ui/traits/alias/object-fail.rs:7:13
   |
LL |     let _: &dyn EqAlias = &123;
   |             ^^^^^^^^^^^ `Eq` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL | pub trait Eq: PartialEq<Self> {
   |               ^^^^^^^^^^^^^^^ the trait cannot be made into an object because it uses `Self` as a type parameter

error[E0191]: the value of the associated type `Item` (from trait `Iterator`) must be specified
  --> /checkout/src/test/ui/traits/alias/object-fail.rs:9:17
   |
LL |     let _: &dyn IteratorAlias = &vec![123].into_iter();
   |                 ^^^^^^^^^^^^^ help: specify the associated type: `IteratorAlias<Item = Type>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0038, E0191.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/alias/no-duplicates.rs stdout ----


1 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:16:22
3    |
3    |
4 LL | trait _0 = Obj;


16    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
17 
18 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:19:22
20    |
20    |
21 LL | trait _0 = Obj;


35    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
36 
37 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:22:22
39    |
39    |
40 LL | trait _0 = Obj;


57    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
58 
59 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:25:23
61    |
61    |
62 LL | trait _0 = Obj;
63    |            --- additional non-auto trait

73    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
74 
75 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:28:22
77    |
77    |
78 LL | trait _0 = Obj;
79    |            --- first non-auto trait

89    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
90 
91 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:37:17
93    |
93    |
94 LL | trait _0 = Obj;


114    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
115 
116 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:40:22
118    |
118    |
119 LL | trait _0 = Obj;
120    |            --- additional non-auto trait

133    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
134 
135 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:43:23
137    |
137    |
138 LL | trait _0 = Obj;
139    |            --- additional non-auto trait

150    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
151 
152 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:46:17
154    |
154    |
155 LL | trait _0 = Obj;


175    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
176 
177 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:49:22
179    |
179    |
180 LL | trait _0 = Obj;
181    |            --- first non-auto trait

194    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
195 
196 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:52:22
198    |
198    |
199 LL | trait _0 = Obj;
200    |            --- additional non-auto trait

213    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
214 
215 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:55:22
217    |
217    |
218 LL | trait _0 = Obj;
219    |            --- first non-auto trait

234    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
235 
236 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:58:22
238    |
238    |
239 LL | trait _0 = Obj;
240    |            --- additional non-auto trait

255    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
256 
257 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:65:22
259    |
259    |
260 LL | trait _5 = Obj + Send;


272    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
273 
274 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:68:23
276    |
276    |
277 LL | trait _5 = Obj + Send;
278    |            --- additional non-auto trait

286    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
287 
288 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:71:22
290    |
290    |
291 LL | trait _5 = Obj + Send;
292    |            --- first non-auto trait

300    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
301 
302 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:74:36
304    |
304    |
305 LL | trait _5 = Obj + Send;
306    |            --- first non-auto trait

314    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
315 
316 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:81:17
318    |
318    |
319 LL | trait _5 = Obj + Send;


337    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
338 
339 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:84:17
341    |
341    |
342 LL | trait _5 = Obj + Send;


360    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
361 
362 error[E0225]: only auto traits can be used as additional traits in a trait object
-   --> $DIR/trait-alias-no-duplicates.rs:87:24
364    |
364    |
365 LL | trait _5 = Obj + Send;


383    = note: auto-traits like `Send` and `Sync` are traits that have special properties; for more information on them, visit <https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits>
---
1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-macro-matcher.rs:11:8
+   --> $DIR/macro-matcher.rs:11:8
3    |
4 LL |     m!(dyn 'static +);

6 
6 
7 error[E0038]: the trait `Copy` cannot be made into an object
-   --> $DIR/trait-object-macro-matcher.rs:8:8
9    |
9    |
10 LL |     m!(dyn Copy + Send + 'static);
11    |        ^^^^^^^^^^^^^^^^^^^^^^^^^ `Copy` cannot be made into an object

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher/macro-matcher.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher/macro-matcher.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/macro-matcher.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/macro-matcher.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/macro-matcher/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/macro-matcher.rs:11:8
   |
LL |     m!(dyn 'static +); //~ ERROR at least one trait is required for an object type


error[E0038]: the trait `Copy` cannot be made into an object
   |
   |
LL |     m!(dyn Copy + Send + 'static);
   |        ^^^^^^^^^^^^^^^^^^^^^^^^^ `Copy` cannot be made into an object
   |
   = note: the trait cannot be made into an object because it requires `Self: Sized`
   = note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0038, E0224.
For more information about an error, try `rustc --explain E0038`.
For more information about an error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/object/safety.rs stdout ----


1 error[E0038]: the trait `Tr` cannot be made into an object
-   --> $DIR/trait-object-safety.rs:15:22
3    |
3    |
4 LL |     let _: &dyn Tr = &St;
5    |                      ^^^ `Tr` cannot be made into an object
6    |
6    |
7 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-object-safety.rs:4:8
9    |
9    |
10 LL | trait Tr {
11    |       -- this trait cannot be made into an object...
23    |              ^^^^^^^^^^^^^^^^^
24 
24 
25 error[E0038]: the trait `Tr` cannot be made into an object
-   --> $DIR/trait-object-safety.rs:15:12
27    |
27    |
28 LL |     let _: &dyn Tr = &St;
29    |            ^^^^^^^ `Tr` cannot be made into an object
30    |
30    |
31 note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
-   --> $DIR/trait-object-safety.rs:4:8
33    |
33    |
34 LL | trait Tr {
35    |       -- this trait cannot be made into an object...

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety/safety.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety/safety.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/safety.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/safety.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/safety/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0038]: the trait `Tr` cannot be made into an object
  --> /checkout/src/test/ui/traits/object/safety.rs:15:22
   |
LL |     let _: &dyn Tr = &St; //~ ERROR E0038
   |                      ^^^ `Tr` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/object/safety.rs:4:8
   |
LL | trait Tr {
   |       -- this trait cannot be made into an object...
LL |     fn foo();
   |        ^^^ ...because associated function `foo` has no `self` parameter
   = note: required because of the requirements on the impl of `CoerceUnsized<&dyn Tr>` for `&St`
   = note: required by cast to type `&dyn Tr`
help: consider turning `foo` into a method by giving it a `&self` argument
   |
LL |     fn foo(&self);
   |            ^^^^^
help: alternatively, consider constraining `foo` so it does not apply to trait objects
   |
LL |     fn foo() where Self: Sized;


error[E0038]: the trait `Tr` cannot be made into an object
  --> /checkout/src/test/ui/traits/object/safety.rs:15:12
   |
LL |     let _: &dyn Tr = &St; //~ ERROR E0038
   |            ^^^^^^^ `Tr` cannot be made into an object
   |
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/object/safety.rs:4:8
   |
LL | trait Tr {
   |       -- this trait cannot be made into an object...
LL |     fn foo();
   |        ^^^ ...because associated function `foo` has no `self` parameter
help: consider turning `foo` into a method by giving it a `&self` argument
   |
LL |     fn foo(&self);
   |            ^^^^^
help: alternatively, consider constraining `foo` so it does not apply to trait objects
   |
LL |     fn foo() where Self: Sized;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0038`.
For more information about this error, try `rustc --explain E0038`.

------------------------------------------


---- [ui] ui/traits/object/vs-lifetime.rs stdout ----

1 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-vs-lifetime.rs:9:23
+   --> $DIR/vs-lifetime.rs:9:23
+   --> $DIR/vs-lifetime.rs:9:23
3    |
4 LL |     let _: S<'static, dyn 'static +>;

6 
6 
7 error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
-   --> $DIR/trait-object-vs-lifetime.rs:11:12
+   --> $DIR/vs-lifetime.rs:11:12
9    |
10 LL |     let _: S<'static, 'static>;
11    |            ^        --------- help: remove this lifetime argument
13    |            expected 1 lifetime argument
14    |
14    |
15 note: struct defined here, with 1 lifetime parameter: `'a`
-   --> $DIR/trait-object-vs-lifetime.rs:4:8
+   --> $DIR/vs-lifetime.rs:4:8
17    |
18 LL | struct S<'a, T>(&'a u8, T);
19    |        ^ --
20 
20 
21 error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
-   --> $DIR/trait-object-vs-lifetime.rs:11:12
+   --> $DIR/vs-lifetime.rs:11:12
23    |
24 LL |     let _: S<'static, 'static>;
25    |            ^ expected 1 type argument
26    |
26    |
27 note: struct defined here, with 1 type parameter: `T`
-   --> $DIR/trait-object-vs-lifetime.rs:4:8
+   --> $DIR/vs-lifetime.rs:4:8
29    |
30 LL | struct S<'a, T>(&'a u8, T);
31    |        ^     -
35    |                              ^^^
36 
37 error[E0224]: at least one trait is required for an object type
-   --> $DIR/trait-object-vs-lifetime.rs:14:14
-   --> $DIR/trait-object-vs-lifetime.rs:14:14
+   --> $DIR/vs-lifetime.rs:14:14
39    |
40 LL |     let _: S<dyn 'static +, 'static>;

42 
42 
43 error[E0747]: type provided when a lifetime was expected
-   --> $DIR/trait-object-vs-lifetime.rs:14:14
+   --> $DIR/vs-lifetime.rs:14:14
45    |
46 LL |     let _: S<dyn 'static +, 'static>;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime/vs-lifetime.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/vs-lifetime.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/vs-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/vs-lifetime/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:9:23
   |
LL |     let _: S<'static, dyn 'static +>;


error[E0107]: this struct takes 1 lifetime argument but 2 lifetime arguments were supplied
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:11:12
   |
LL |     let _: S<'static, 'static>;
   |            ^        --------- help: remove this lifetime argument
   |            expected 1 lifetime argument
   |
   |
note: struct defined here, with 1 lifetime parameter: `'a`
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:4:8
   |
LL | struct S<'a, T>(&'a u8, T);
   |        ^ --

error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:11:12
   |
LL |     let _: S<'static, 'static>;
   |            ^ expected 1 type argument
   |
note: struct defined here, with 1 type parameter: `T`
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:4:8
   |
LL | struct S<'a, T>(&'a u8, T);
   |        ^     -
help: add missing type argument
   |
LL |     let _: S<'static, 'static, T>;

error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:14:14
   |
   |
LL |     let _: S<dyn 'static +, 'static>;


error[E0747]: type provided when a lifetime was expected
  --> /checkout/src/test/ui/traits/object/vs-lifetime.rs:14:14
   |
LL |     let _: S<dyn 'static +, 'static>;

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0107, E0224, E0747.
---

---- [ui] ui/traits/object/with-self-in-projection-output-bad.rs stdout ----
diff of stderr:

1 error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
-   --> $DIR/trait-object-with-self-in-projection-output-bad.rs:45:21
3    |
4 LL |     type Output;
4 LL |     type Output;
5    |     ------------ `Output` defined here

8    |                     ^^^^^^^^^^^^^^^^^^ help: specify the associated type: `Helper<Target=i32, Output = Type>`
9 
10 error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
-   --> $DIR/trait-object-with-self-in-projection-output-bad.rs:48:21
12    |
13 LL |     type Output;
13 LL |     type Output;
14    |     ------------ `Output` defined here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad/with-self-in-projection-output-bad.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad/with-self-in-projection-output-bad.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/object/with-self-in-projection-output-bad.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/with-self-in-projection-output-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/with-self-in-projection-output-bad/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
   |
LL |     type Output;
LL |     type Output;
   |     ------------ `Output` defined here
...
LL |     let _x: Box<dyn Helper<Target=i32>> = Box::new(2u32);
   |                     ^^^^^^^^^^^^^^^^^^ help: specify the associated type: `Helper<Target=i32, Output = Type>`

error[E0191]: the value of the associated type `Output` (from trait `Base`) must be specified
   |
LL |     type Output;
LL |     type Output;
   |     ------------ `Output` defined here
...
LL |     let _y: Box<dyn NormalizableHelper<Target=i32>> = Box::new(2u32);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: specify the associated type: `NormalizableHelper<Target=i32, Output = Type>`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0191`.


------------------------------------------


---- [ui] ui/traits/repeated-superambig.rs stdout ----


1 error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:26:7
+   --> $DIR/repeated-superambig.rs:26:7
3    |
4 LL |     c.same_as(22)
5    |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`
6 
6 
7 error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:30:7
+   --> $DIR/repeated-superambig.rs:30:7
9    |
10 LL |     c.same_as(22)
11    |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `C`
16    |                               ^^^^^^^^^^^^^^^^
17 
17 
18 error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:34:5
+   --> $DIR/repeated-superambig.rs:34:5
20    |
21 LL |     fn same_as(&self, t: T) -> bool;
22    |     -------------------------------- required by `CompareTo::same_as`

25    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`
26 
27 error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:38:5
+   --> $DIR/repeated-superambig.rs:38:5
29    |
30 LL |     fn same_as(&self, t: T) -> bool;
31    |     -------------------------------- required by `CompareTo::same_as`
39    |                               ^^^^^^^^^^^^^^^^
40 
40 
41 error[E0277]: the trait bound `i64: CompareTo<i32>` is not satisfied
-   --> $DIR/repeated-supertrait-ambig.rs:42:23
+   --> $DIR/repeated-superambig.rs:42:23
43    |
44 LL |     assert_eq!(22_i64.same_as(22), true);
45    |                       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `i64`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/repeated-superambig/repeated-superambig.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/repeated-superambig.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/repeated-superambig.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/repeated-superambig" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/repeated-superambig/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:26:7
   |
LL |     c.same_as(22) //~ ERROR `dyn CompareToInts: CompareTo<i32>` is not satisfied
   |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`

error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:30:7
   |
LL |     c.same_as(22) //~ ERROR `C: CompareTo<i32>` is not satisfied
   |       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `C`
help: consider further restricting this bound
   |
   |
LL | fn with_trait<C:CompareToInts + CompareTo<i32>>(c: &C) -> bool {


error[E0277]: the trait bound `dyn CompareToInts: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:34:5
   |
LL |     fn same_as(&self, t: T) -> bool;
   |     -------------------------------- required by `CompareTo::same_as`
...
LL |     CompareToInts::same_as(c, 22) //~ ERROR `dyn CompareToInts: CompareTo<i32>` is not satisfied
   |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `CompareTo<i32>` is not implemented for `dyn CompareToInts`

error[E0277]: the trait bound `C: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:38:5
   |
LL |     fn same_as(&self, t: T) -> bool;
   |     -------------------------------- required by `CompareTo::same_as`
...
LL |     CompareTo::same_as(c, 22) //~ ERROR `C: CompareTo<i32>` is not satisfied
   |     ^^^^^^^^^^^^^^^^^^ the trait `CompareTo<i32>` is not implemented for `C`
help: consider further restricting this bound
   |
   |
LL | fn with_ufcs2<C:CompareToInts + CompareTo<i32>>(c: &C) -> bool {


error[E0277]: the trait bound `i64: CompareTo<i32>` is not satisfied
  --> /checkout/src/test/ui/traits/repeated-superambig.rs:42:23
   |
LL |     assert_eq!(22_i64.same_as(22), true); //~ ERROR `i64: CompareTo<i32>` is not satisfied
   |                       ^^^^^^^ the trait `CompareTo<i32>` is not implemented for `i64`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <i64 as CompareTo<i64>>
             <i64 as CompareTo<u64>>
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/wf-object/maybe-bound.rs stdout ----


1 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:5:15
3    |
3    |
4 LL | type _0 = dyn ?Sized + Foo;

6 
6 
7 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:8:21
9    |
9    |
10 LL | type _1 = dyn Foo + ?Sized;

12 
12 
13 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:11:21
15    |
15    |
16 LL | type _2 = dyn Foo + ?Sized + ?Sized;

18 
18 
19 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:11:30
21    |
21    |
22 LL | type _2 = dyn Foo + ?Sized + ?Sized;

24 
24 
25 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-maybe-bound.rs:15:15
27    |
27    |
28 LL | type _3 = dyn ?Sized + Foo;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/maybe-bound/maybe-bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/maybe-bound/maybe-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/wf-object/maybe-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/wf-object/maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/maybe-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/maybe-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `?Trait` is not permitted in trait object types
  --> /checkout/src/test/ui/traits/wf-object/maybe-bound.rs:5:15
   |
LL | type _0 = dyn ?Sized + Foo;


error: `?Trait` is not permitted in trait object types
  --> /checkout/src/test/ui/traits/wf-object/maybe-bound.rs:8:21
   |
LL | type _1 = dyn Foo + ?Sized;


error: `?Trait` is not permitted in trait object types
  --> /checkout/src/test/ui/traits/wf-object/maybe-bound.rs:11:21
   |
LL | type _2 = dyn Foo + ?Sized + ?Sized;


error: `?Trait` is not permitted in trait object types
  --> /checkout/src/test/ui/traits/wf-object/maybe-bound.rs:11:30
   |
LL | type _2 = dyn Foo + ?Sized + ?Sized;


error: `?Trait` is not permitted in trait object types
  --> /checkout/src/test/ui/traits/wf-object/maybe-bound.rs:15:15
   |
LL | type _3 = dyn ?Sized + Foo;

error: aborting due to 5 previous errors



------------------------------------------


---- [ui] ui/traits/suggest-deferences/issue-62530.rs stdout ----


1 error[E0277]: the trait bound `&String: SomeTrait` is not satisfied
-   --> $DIR/trait-suggest-deferences-issue-62530.rs:13:26
3    |
3    |
4 LL | fn takes_type_parameter<T>(_x: T) where T: SomeTrait {}
5    |                                            --------- required by this bound in `takes_type_parameter`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530/issue-62530.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530/issue-62530.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/issue-62530.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/issue-62530.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-62530/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&String: SomeTrait` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/issue-62530.rs:13:26
   |
LL | fn takes_type_parameter<T>(_x: T) where T: SomeTrait {}
   |                                            --------- required by this bound in `takes_type_parameter`
...
LL |     takes_type_parameter(&string);  // Error
   |                          |
   |                          |
   |                          the trait `SomeTrait` is not implemented for `&String`
   |                          help: consider adding dereference here: `&*string`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/suggest-deferences/issue-39029.rs stdout ----


1 error[E0277]: the trait bound `NoToSocketAddrs: ToSocketAddrs` is not satisfied
-   --> $DIR/trait-suggest-deferences-issue-39029.rs:16:37
3    |
3    |
4 LL |     let _errors = TcpListener::bind(&bad);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029/issue-39029.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029/issue-39029.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/issue-39029.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/issue-39029.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/issue-39029/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `NoToSocketAddrs: ToSocketAddrs` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/issue-39029.rs:16:37
   |
LL |     let _errors = TcpListener::bind(&bad);
   |                                     |
   |                                     |
   |                                     the trait `ToSocketAddrs` is not implemented for `NoToSocketAddrs`
   |                                     help: consider adding dereference here: `&*bad`
  ::: /checkout/library/std/src/net/tcp.rs:696:20
   |
   |
LL |     pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
   |                    ------------- required by this bound in `TcpListener::bind`
   |
   = note: required because of the requirements on the impl of `ToSocketAddrs` for `&NoToSocketAddrs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/suggest-deferences/multiple-1.rs stdout ----


1 error[E0277]: the trait bound `&mut Baz: Happy` is not satisfied
-   --> $DIR/trait-suggest-deferences-multiple-1.rs:52:9
3    |
3    |
4 LL | fn foo<T>(_: T) where T: Happy {}
5    |                          ----- required by this bound in `foo`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-1/multiple-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/multiple-1.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/multiple-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&mut Baz: Happy` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/multiple-1.rs:52:9
   |
LL | fn foo<T>(_: T) where T: Happy {}
   |                          ----- required by this bound in `foo`
...
LL |     foo(&mut baz);
   |         ^^^^^^^^ the trait `Happy` is not implemented for `&mut Baz`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/suggest-deferences/multiple-0.rs stdout ----


1 error[E0277]: the trait bound `&Baz: Happy` is not satisfied
-   --> $DIR/trait-suggest-deferences-multiple-0.rs:34:9
3    |
3    |
4 LL | fn foo<T>(_: T) where T: Happy {}
5    |                          ----- required by this bound in `foo`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-0/multiple-0.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/suggest-deferences/multiple-0.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/suggest-deferences/multiple-0.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-0" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/suggest-deferences/multiple-0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&Baz: Happy` is not satisfied
  --> /checkout/src/test/ui/traits/suggest-deferences/multiple-0.rs:34:9
   |
LL | fn foo<T>(_: T) where T: Happy {}
   |                          ----- required by this bound in `foo`
...
LL |     foo(&baz);
   |         |
   |         |
   |         the trait `Happy` is not implemented for `&Baz`
   |         help: consider adding dereference here: `&***baz`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/traits/wf-object/only-maybe-bound.rs stdout ----


1 error: `?Trait` is not permitted in trait object types
-   --> $DIR/wf-trait-object-only-maybe-bound.rs:3:15
+   --> $DIR/only-maybe-bound.rs:3:15
3    |
4 LL | type _0 = dyn ?Sized;

6 
7 error[E0224]: at least one trait is required for an object type
-   --> $DIR/wf-trait-object-only-maybe-bound.rs:3:11
-   --> $DIR/wf-trait-object-only-maybe-bound.rs:3:11
+   --> $DIR/only-maybe-bound.rs:3:11
9    |
10 LL | type _0 = dyn ?Sized;


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/only-maybe-bound/only-maybe-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/wf-object/only-maybe-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/wf-object/only-maybe-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/only-maybe-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/wf-object/only-maybe-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `?Trait` is not permitted in trait object types
  --> /checkout/src/test/ui/traits/wf-object/only-maybe-bound.rs:3:15
   |
LL | type _0 = dyn ?Sized;

error[E0224]: at least one trait is required for an object type
  --> /checkout/src/test/ui/traits/wf-object/only-maybe-bound.rs:3:11
   |
   |
LL | type _0 = dyn ?Sized;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0224`.
For more information about this error, try `rustc --explain E0224`.

------------------------------------------


---- [ui] ui/traits/wf-object/no-duplicates.rs stdout ----
---
    [ui] ui/traits/default-method/xc.rs
    [ui] ui/traits/duplicate-methods.rs
    [ui] ui/traits/impl-can-not-have-untraitful-items.rs
    [ui] ui/traits/impl-of-supertrait-has-wrong-lifetime-parameters.rs
    [ui] ui/traits/inductive-overflow/lifetime.rs
    [ui] ui/traits/inductive-overflow/simultaneous.rs
    [ui] ui/traits/inductive-overflow/superauto-trait.rs
    [ui] ui/traits/inductive-overflow/supertrait.rs
    [ui] ui/traits/inductive-overflow/two-traits.rs
    [ui] ui/traits/object/auto-dedup-in-impl.rs
    [ui] ui/traits/object/macro-matcher.rs
    [ui] ui/traits/object/safety.rs
    [ui] ui/traits/object/vs-lifetime-2.rs
---
test result: FAILED. 11290 passed; 42 failed; 92 ignored; 0 measured; 0 filtered out; finished in 134.21s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:21
