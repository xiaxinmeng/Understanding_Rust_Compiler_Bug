plain

---- [ui] tests/ui/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn.rs stdout ----
diff of stderr:

14    |       ^ expected `&dyn Fn(&dyn Fn(&dyn Fn(&...)))`, found `&dyn Fn(u32)`
15    |
16    = note: expected reference `&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&...)))))))))))`
-            the full type name has been written to '$TEST_BUILD_DIR/higher-rank-trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.long-type-hash.txt'
+            the full type name has been written to '$TEST_BUILD_DIR/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.long-type-hash.txt'
18               found reference `&dyn Fn(u32)`
20 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.stderr
To only update this specific test, also pass `--test-args higher-ranked/trait-bounds/hang-on-deeply-nested-dyn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn/auxiliary"
stdout: none
error[E0308]: mismatched types
  --> fake-test-src-base/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn.rs:12:5
   |
LL |   ) -> &dyn Fn(
LL |   ) -> &dyn Fn(
   |  ______-
LL | |     &dyn Fn(
LL | |         &dyn Fn(
LL | |             &dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(u32))))))))),
LL | |         ),
LL | |     ),
LL | | ) {
   | |_- expected `&dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn for<'a> Fn(&'a (dyn Fn(u32) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a)) + 'a))` because of return type
LL |       f
   |       ^ expected `&dyn Fn(&dyn Fn(&dyn Fn(&...)))`, found `&dyn Fn(u32)`
   |
   = note: expected reference `&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&...)))))))))))`
           the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/hang-on-deeply-nested-dyn/hang-on-deeply-nested-dyn.long-type-13549645763127033158.txt'
              found reference `&dyn Fn(u32)`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/higher-ranked/trait-bounds/issue-30786.rs stdout ----
diff of stderr:

31 LL |     let count = filter.countx();
33    |
33    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/higher-rank-trait-bounds/issue-30786/issue-30786.long-type-hash.txt'
+    = note: the full type name has been written to '$TEST_BUILD_DIR/higher-ranked/trait-bounds/issue-30786/issue-30786.long-type-hash.txt'
35 note: the following trait bounds were not satisfied:
36       `&'a mut &Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:131:30: 131:37]>: Stream`
37       `&'a mut &mut Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@$DIR/issue-30786.rs:131:30: 131:37]>: Stream`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/issue-30786/issue-30786.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args higher-ranked/trait-bounds/issue-30786.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/higher-ranked/trait-bounds/issue-30786.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/issue-30786" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/issue-30786/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: the method `filterx` exists for struct `Map<Repeat, [closure@issue-30786.rs:119:27]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:120:22
   |
LL | pub struct Map<S, F> {
   | |
   | |
   | method `filterx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let filter = map.filterx(|x: &_| true);
   |                      ^^^^^^^ method cannot be called on `Map<Repeat, [closure@issue-30786.rs:119:27]>` due to unsatisfied trait bounds
note: the following trait bounds were not satisfied:
note: the following trait bounds were not satisfied:
      `&'a mut &Map<Repeat, [closure@fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:119:27: 119:34]>: Stream`
      `&'a mut &mut Map<Repeat, [closure@fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:119:27: 119:34]>: Stream`
      `&'a mut Map<Repeat, [closure@fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:119:27: 119:34]>: Stream`
  --> fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:98:50
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here

error[E0599]: the method `countx` exists for struct `Filter<Map<Repeat, fn(&u64) -> &u64 {identity::<u64>}>, [closure@issue-30786.rs:131:30]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:132:24
   |
LL | pub struct Filter<S, F> {
   | |
   | |
   | method `countx` not found for this struct
   | doesn't satisfy `_: StreamExt`
...
LL |     let count = filter.countx();
   |
   |
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-ranked/trait-bounds/issue-30786/issue-30786.long-type-16423705919332610464.txt'
note: the following trait bounds were not satisfied:
      `&'a mut &Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:131:30: 131:37]>: Stream`
      `&'a mut &mut Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:131:30: 131:37]>: Stream`
      `&'a mut Filter<Map<Repeat, for<'a> fn(&'a u64) -> &'a u64 {identity::<u64>}>, [closure@fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:131:30: 131:37]>: Stream`
  --> fake-test-src-base/higher-ranked/trait-bounds/issue-30786.rs:98:50
   |
LL | impl<T> StreamExt for T where for<'a> &'a mut T: Stream {}
   |         ---------     -                          ^^^^^^ unsatisfied trait bound introduced here
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
