plain
..............iii....................................................................... 14080/14116
....................................
failures:

---- [ui] src/test/ui/traits/blame-trait-error-spans-on-exprs.rs stdout ----

- error[E0412]: cannot find type `T` in this scope
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:78:35
-    |
-    |
- LL | impl<A> T2 for ComplexImplExample<T> where Option<A>: Copy {}
-    |      |
-    |      similarly named type parameter `A` defined here
-    |
- help: a type parameter with a similar name exists
- help: a type parameter with a similar name exists
-    |
- LL | impl<A> T2 for ComplexImplExample<A> where Option<A>: Copy {}
- help: you might be missing a type parameter
-    |
-    |
- LL | impl<A, T> T2 for ComplexImplExample<T> where Option<A>: Copy {}
- 
- 
18 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:85:60
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:79:60
20    |
21 LL |     want(Wrapper { value: Burrito { spicy: false, filling: q } });
22    |     ---- required by a bound introduced by this call       ^ the trait `T3` is not implemented for `Q`
42    |             ++++
43 
43 
44 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:89:84
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:83:84
46    |
47 LL |     want(Wrapper { value: BurritoKinds::SmallBurrito { spicy: true, small_filling: q } });
48    |     ---- required by a bound introduced by this call                               ^ the trait `T3` is not implemented for `Q`
68    |             ++++
69 
69 
70 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:93:39
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:87:39
72    |
73 LL |     want(Wrapper { value: Taco(false, q) });
74    |     ----                              ^ the trait `T3` is not implemented for `Q`
96    |             ++++
97 
97 
98 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:97:27
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:91:27
100    |
101 LL |     want(Wrapper { value: TacoKinds::OneTaco(false, q) });
102    |     ----                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `T3` is not implemented for `Q`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
124    |             ++++
125 
125 
126 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:101:74
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:95:74
128    |
129 LL |     want(Wrapper { value: GenericBurrito { spiciness: NotSpicy, filling: q } });
130    |     ---- required by a bound introduced by this call                     ^ the trait `T3` is not implemented for `Q`
150    |             ++++
151 
151 
152 error[E0277]: the trait bound `Q: T2` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:105:14
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:99:14
154    |
155 LL |     want((3, q));
156    |     ----     ^ the trait `T2` is not implemented for `Q`
173    |             ++++
174 
174 
175 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:109:31
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:103:31
177    |
178 LL |     want(Wrapper { value: (3, q) });
179    |     ----                      ^ the trait `T3` is not implemented for `Q`
201    |             ++++
202 
202 
203 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:113:15
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:107:15
205    |
206 LL |     want(((3, q), 5));
207    |     ----      ^ the trait `T3` is not implemented for `Q`
229    |             ++++
230 
230 
231 error[E0277]: the trait bound `Q: T1` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:116:49
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:110:49
233    |
234 LL |     want(DoubleWrapper { item: Wrapper { value: q } });
235    |     ----                                        ^ the trait `T1` is not implemented for `Q`
252    |             ++++
253 
253 
254 error[E0277]: the trait bound `Q: T1` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:119:88
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:113:88
256    |
257 LL |     want(DoubleWrapper { item: Wrapper { value: DoubleWrapper { item: Wrapper { value: q } } } });
258    |     ---- required by a bound introduced by this call                                   ^ the trait `T1` is not implemented for `Q`
275    |             ++++
276 
276 
277 error[E0277]: the trait bound `Q: T3` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:123:27
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:117:27
279    |
280 LL |     want(Wrapper { value: AliasBurrito { spiciness: q, filling: q } });
281    |     ----                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `T3` is not implemented for `Q`
303    |             ++++
304 
304 
305 error[E0277]: the trait bound `Q: T1` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:126:35
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:120:35
307    |
308 LL |     want(Two { a: Two { a: (), b: q }, b: () });
309    |     ----                          ^ the trait `T1` is not implemented for `Q`
326    |             ++++
327 
327 
328 error[E0277]: the trait bound `Q: T1` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:132:59
+   --> $DIR/blame-trait-error-spans-on-exprs.rs:126:59
331 LL |     want(
332    |     ---- required by a bound introduced by this call


350 LL | fn example<Q: T1>(q: Q) {
352 
352 
- error[E0277]: the trait bound `ComplexImplExample<Vec<_>>: T2` is not satisfied
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:136:27
-    |
- LL |     want(Wrapper { value: ComplexImplExample { value: Wrapper { value: vec![] } } });
-    |     ----                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `T2` is not implemented for `ComplexImplExample<Vec<_>>`
-    |     required by a bound introduced by this call
-    |
-    = help: the following other types implement trait `T2`:
-              (A, B)
-              (A, B)
-              Burrito<A>
-              BurritoKinds<D>
-              BurritoTuple<C>
-              GenericBurrito<X, Y>
-              Taco<E>
-              TacoKinds<F>
-              i32
- note: required for `Wrapper<ComplexImplExample<Vec<_>>>` to implement `T1`
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:16:13
-    |
- LL | impl<B: T2> T1 for Wrapper<B> {}
-    |             ^^     ^^^^^^^^^^
- note: required by a bound in `want`
-   --> $DIR/blame-trait-error-spans-on-exprs.rs:53:12
-    |
- LL | fn want<V: T1>(_x: V) {}
-    |            ^^ required by this bound in `want`
+ error: aborting due to 13 previous errors
- error: aborting due to 15 previous errors
- 
- Some errors have detailed explanations: E0277, E0412.
- For more information about an error, try `rustc --explain E0277`.
- For more information about an error, try `rustc --explain E0277`.
+ For more information about this error, try `rustc --explain E0277`.
385 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/blame-trait-error-spans-on-exprs/blame-trait-error-spans-on-exprs.stderr
To only update this specific test, also pass `--test-args traits/blame-trait-error-spans-on-exprs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/blame-trait-error-spans-on-exprs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/blame-trait-error-spans-on-exprs/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:79:60
   |
LL |     want(Wrapper { value: Burrito { spicy: false, filling: q } });
   |     ---- required by a bound introduced by this call       ^ the trait `T3` is not implemented for `Q`
   |
note: required for `Burrito<Q>` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:22:13
   |
LL | impl<A: T3> T2 for Burrito<A> {}
   |             ^^     ^^^^^^^^^^
note: required for `Wrapper<Burrito<Q>>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:83:84
   |
LL |     want(Wrapper { value: BurritoKinds::SmallBurrito { spicy: true, small_filling: q } });
   |     ---- required by a bound introduced by this call                               ^ the trait `T3` is not implemented for `Q`
   |
note: required for `BurritoKinds<Q>` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:32:13
   |
LL | impl<D: T3> T2 for BurritoKinds<D> {}
   |             ^^     ^^^^^^^^^^^^^^^
note: required for `Wrapper<BurritoKinds<Q>>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:87:39
   |
LL |     want(Wrapper { value: Taco(false, q) });
   |     ----                              ^ the trait `T3` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `Taco<Q>` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:35:13
   |
LL | impl<E: T3> T2 for Taco<E> {}
   |             ^^     ^^^^^^^
note: required for `Wrapper<Taco<Q>>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:91:27
   |
LL |     want(Wrapper { value: TacoKinds::OneTaco(false, q) });
   |     ----                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `T3` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `TacoKinds<Q>` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:41:13
   |
LL | impl<F: T3> T2 for TacoKinds<F> {}
   |             ^^     ^^^^^^^^^^^^
note: required for `Wrapper<TacoKinds<Q>>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:95:74
   |
LL |     want(Wrapper { value: GenericBurrito { spiciness: NotSpicy, filling: q } });
   |     ---- required by a bound introduced by this call                     ^ the trait `T3` is not implemented for `Q`
   |
note: required for `GenericBurrito<NotSpicy, Q>` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:47:16
   |
LL | impl<X, Y: T3> T2 for GenericBurrito<X, Y> {}
   |                ^^     ^^^^^^^^^^^^^^^^^^^^
note: required for `Wrapper<GenericBurrito<NotSpicy, Q>>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T2` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:99:14
   |
LL |     want((3, q));
   |     ----     ^ the trait `T2` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `(i32, Q)` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:51:20
   |
LL | impl<A: T2, B: T2> T1 for (A, B) {}
   |                    ^^     ^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T2>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:103:31
   |
LL |     want(Wrapper { value: (3, q) });
   |     ----                      ^ the trait `T3` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `(i32, Q)` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:50:20
   |
LL | impl<A: T3, B: T3> T2 for (A, B) {}
   |                    ^^     ^^^^^^
note: required for `Wrapper<(i32, Q)>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:107:15
   |
LL |     want(((3, q), 5));
   |     ----      ^ the trait `T3` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `(i32, Q)` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:50:20
   |
LL | impl<A: T3, B: T3> T2 for (A, B) {}
   |                    ^^     ^^^^^^
note: required for `((i32, Q), i32)` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:51:20
   |
LL | impl<A: T2, B: T2> T1 for (A, B) {}
   |                    ^^     ^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T1` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:110:49
   |
LL |     want(DoubleWrapper { item: Wrapper { value: q } });
   |     ----                                        ^ the trait `T1` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `DoubleWrapper<Q>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:72:13
   |
LL | impl<T: T1> T1 for DoubleWrapper<T> {}
   |             ^^     ^^^^^^^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T1>(q: Q) {


error[E0277]: the trait bound `Q: T1` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:113:88
   |
LL |     want(DoubleWrapper { item: Wrapper { value: DoubleWrapper { item: Wrapper { value: q } } } });
   |     ---- required by a bound introduced by this call                                   ^ the trait `T1` is not implemented for `Q`
   |
note: required for `DoubleWrapper<Q>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:72:13
   |
LL | impl<T: T1> T1 for DoubleWrapper<T> {}
   |             ^^     ^^^^^^^^^^^^^^^^
   = note: 1 redundant requirement hidden
   = note: required for `DoubleWrapper<DoubleWrapper<Q>>` to implement `T1`
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T1>(q: Q) {


error[E0277]: the trait bound `Q: T3` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:117:27
   |
LL |     want(Wrapper { value: AliasBurrito { spiciness: q, filling: q } });
   |     ----                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `T3` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `GenericBurrito<Q, Q>` to implement `T2`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:47:16
   |
LL | impl<X, Y: T3> T2 for GenericBurrito<X, Y> {}
   |                ^^     ^^^^^^^^^^^^^^^^^^^^
note: required for `Wrapper<GenericBurrito<Q, Q>>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:16:13
   |
LL | impl<B: T2> T1 for Wrapper<B> {}
   |             ^^     ^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T3>(q: Q) {


error[E0277]: the trait bound `Q: T1` is not satisfied
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:120:35
   |
LL |     want(Two { a: Two { a: (), b: q }, b: () });
   |     ----                          ^ the trait `T1` is not implemented for `Q`
   |     required by a bound introduced by this call
   |
   |
note: required for `Two<Two<(), Q>, ()>` to implement `T1`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:66:19
   |
LL | impl<X, Y: T1, Z> T1 for Two<Two<X, Y>, Z> {}
   |                   ^^     ^^^^^^^^^^^^^^^^^
note: required by a bound in `want`
  --> /checkout/src/test/ui/traits/blame-trait-error-spans-on-exprs.rs:53:12
   |
LL | fn want<V: T1>(_x: V) {}
   |            ^^ required by this bound in `want`
help: consider restricting type parameter `Q`
   |
LL | fn example<Q: T1>(q: Q) {


