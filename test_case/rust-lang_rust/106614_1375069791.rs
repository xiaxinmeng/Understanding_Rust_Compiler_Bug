plain
........................................................................................ 13640/14116
........................................................................................ 13728/14116
........................................................................................ 13816/14116
........................................................................................ 13904/14116
........................................................................FF.............. 13992/14116
F.F.F.F..FF.F.i.ii...................................................................... 14080/14116
failures:

---- [ui] src/test/ui/feature-gates/stability-attribute-consistency.rs stdout ----
diff of stderr:
---
To only update this specific test, also pass `--test-args feature-gates/stability-attribute-consistency.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/stability-attribute-consistency.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/stability-attribute-consistency" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/stability-attribute-consistency/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0711]: feature `foo` is declared stable since 1.29.0, but was previously declared stable since 1.0.0
   |
LL | #[stable(feature = "foo", since = "1.29.0")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

---
To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-sanity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/auxiliary"
stdout: none
--- stderr -------------------------------
error: multiple `deprecated` attributes
   |
   |
LL | #[deprecated(since = "b", note = "text")] //~ ERROR multiple `deprecated` attributes
   |
note: attribute also specified here
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:61:1
   |
   |
LL | #[deprecated(since = "b", note = "text")]


error[E0541]: unknown meta item 'reason'
   |
   |
LL |     #[stable(feature = "a", since = "b", reason)] //~ ERROR unknown meta item 'reason' [E0541]
   |                                          ^^^^^^ expected one of `feature`, `since`
error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:11:29
   |
   |
LL |     #[stable(feature = "a", since)] //~ ERROR incorrect meta item [E0539]

error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:14:14
   |
   |
LL |     #[stable(feature, since = "a")] //~ ERROR incorrect meta item [E0539]

error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:17:29
   |
   |
LL |     #[stable(feature = "a", since(b))] //~ ERROR incorrect meta item [E0539]

error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:20:14
   |
   |
LL |     #[stable(feature(b), since = "a")] //~ ERROR incorrect meta item [E0539]

error[E0546]: missing 'feature'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:25:5
   |
   |
LL |     #[unstable(issue = "none")] //~ ERROR missing 'feature' [E0546]

error[E0547]: missing 'issue'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:28:5
   |
   |
LL |     #[unstable(feature = "b")] //~ ERROR missing 'issue' [E0547]

error[E0546]: missing 'feature'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:31:5
   |
   |
LL |     #[stable(since = "a")] //~ ERROR missing 'feature' [E0546]

error[E0542]: missing 'since'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:36:5
   |
   |
LL |     #[stable(feature = "a")] //~ ERROR missing 'since' [E0542]

error[E0542]: missing 'since'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:40:5
   |
   |
LL |     #[deprecated(note = "a")] //~ ERROR missing 'since' [E0542]

error[E0543]: missing 'note'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:44:5
   |
   |
LL |     #[deprecated(since = "a")] //~ ERROR missing 'note' [E0543]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:49:1
   |
   |
LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:53:1
   |
   |
LL | #[unstable(feature = "b", issue = "none")] //~ ERROR multiple stability levels [E0544]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:57:1
   |
   |
LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:64:1
   |
   |
LL | #[rustc_const_unstable(feature = "d", issue = "none")] //~ ERROR multiple stability levels

error: invalid stability version found
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:60:1
   |
   |
LL | #[stable(feature = "a", since = "b")] //~ ERROR invalid stability version found
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid stability version
LL | pub const fn multiple4() { }
   | ---------------------------- the stability attribute annotates this item

error: invalid deprecation version found
error: invalid deprecation version found
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:67:1
   |
LL | #[stable(feature = "a", since = "1.0.0")] //~ ERROR invalid deprecation version found
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid deprecation version
LL | fn invalid_deprecation_version() {}
   | ----------------------------------- the stability attribute annotates this item


error[E0549]: deprecated attribute must be paired with either stable or unstable attribute
   |
   |
LL | #[deprecated(since = "a", note = "text")]

error[E0711]: feature `a` is declared stable since 1.0.0, but was previously declared stable since b
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:67:1
   |
   |
LL | #[stable(feature = "a", since = "1.0.0")] //~ ERROR invalid deprecation version found

error: aborting due to 20 previous errors

Some errors have detailed explanations: E0539, E0541, E0542, E0543, E0544, E0546, E0547, E0549, E0711.
---
To only update this specific test, also pass `--test-args variance/variance-associated-consts.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-associated-consts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-consts" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-consts/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [o]
   |
   |
LL | struct Foo<T: Trait> { //~ ERROR [o]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-associated-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-associated-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-associated-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-, +]
   |
   |
LL | struct Foo<'a, T : Trait<'a>> { //~ ERROR [-, +]


error[E0208]: [o, o]
   |
   |
LL | struct Bar<'a, T : Trait<'a>> { //~ ERROR [o, o]

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-object-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-object-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [o]
   |
   |
LL | struct Foo<'a> { //~ ERROR [o]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-regions-direct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-direct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-direct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-direct/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-, -, -]
   |
   |
LL | struct Test2<'a, 'b, 'c> { //~ ERROR [-, -, -]


error[E0208]: [+, +, +]
   |
   |
LL | struct Test3<'a, 'b, 'c> { //~ ERROR [+, +, +]


error[E0208]: [-, o]
   |
   |
LL | struct Test4<'a, 'b:'a> { //~ ERROR [-, o]


error[E0208]: [+, o]
   |
   |
LL | struct Test5<'a, 'b:'a> { //~ ERROR [+, o]


error[E0208]: [-, o]
   |
   |
LL | struct Test6<'a, 'b:'a> { //~ ERROR [-, o]

error[E0208]: [*]
  --> /checkout/src/test/ui/variance/variance-regions-direct.rs:52:1
   |
   |
LL | struct Test7<'a> { //~ ERROR [*]


error[E0208]: [+, -, o]
   |
   |
LL | enum Test8<'a, 'b, 'c:'b> { //~ ERROR [+, -, o]

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-regions-indirect.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-regions-indirect.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-indirect" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-regions-indirect/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [+, -, o, *]
   |
   |
LL | enum Base<'a, 'b, 'c:'b, 'd> { //~ ERROR [+, -, o, *]


error[E0208]: [*, o, -, +]
   |
   |
LL | struct Derived1<'w, 'x:'y, 'y, 'z> { //~ ERROR [*, o, -, +]


error[E0208]: [o, o, *]
   |
   |
LL | struct Derived2<'a, 'b:'a, 'c> { //~ ERROR [o, o, *]


error[E0208]: [o, -, *]
   |
   |
LL | struct Derived3<'a:'b, 'b, 'c> { //~ ERROR [o, -, *]


error[E0208]: [+, -, o]
   |
   |
LL | struct Derived4<'a, 'b, 'c:'b> { //~ ERROR [+, -, o]

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-trait-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-trait-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [+, +]
   |
   |
LL | struct TestStruct<U,T:Setter<U>> { //~ ERROR [+, +]


error[E0208]: [*, +]
   |
   |
LL | enum TestEnum<U,T:Setter<U>> { //~ ERROR [*, +]


error[E0208]: [*, +]
   |
   |
LL | struct TestContraStruct<U,T:Setter<U>> { //~ ERROR [*, +]


error[E0208]: [*, +]
   |
   |
LL | struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR [*, +]

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-trait-object-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-trait-object-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-object-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-trait-object-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-]
   |
   |
LL | struct TOption<'a> { //~ ERROR [-]

error: aborting due to previous error

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-types-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-types-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [+, +]
   |
   |
LL | struct TestImm<A, B> { //~ ERROR [+, +]


error[E0208]: [+, o]
   |
   |
LL | struct TestMut<A, B:'static> { //~ ERROR [+, o]


error[E0208]: [+, o]
   |
   |
LL | struct TestIndirect<A:'static, B:'static> { //~ ERROR [+, o]


error[E0208]: [o, o]
   |
   |
LL | struct TestIndirect2<A:'static, B:'static> { //~ ERROR [o, o]


error[E0208]: [o, o]
   |
   |
LL | struct TestObject<A, R> { //~ ERROR [o, o]

error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0208`.
---
To only update this specific test, also pass `--test-args variance/variance-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [-, o, o]
   |
   |
LL | struct InvariantMut<'a,A:'a,B:'a> { //~ ERROR [-, o, o]

error[E0208]: [o]
  --> /checkout/src/test/ui/variance/variance-types.rs:15:1
   |
   |
LL | struct InvariantCell<A> { //~ ERROR [o]

error[E0208]: [o]
  --> /checkout/src/test/ui/variance/variance-types.rs:20:1
   |
   |
LL | struct InvariantIndirect<A> { //~ ERROR [o]

error[E0208]: [+]
  --> /checkout/src/test/ui/variance/variance-types.rs:25:1
   |
   |
LL | struct Covariant<A> { //~ ERROR [+]

error[E0208]: [-]
  --> /checkout/src/test/ui/variance/variance-types.rs:30:1
   |
   |
LL | struct Contravariant<A> { //~ ERROR [-]


error[E0208]: [+, -, o]
   |
   |
LL | enum Enum<A,B,C> { //~ ERROR [+, -, o]

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0208`.
