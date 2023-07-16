plain

27    |
28    = note: doesn't satisfy `_: Iterator`
29    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/mismatched_types/issue-36053-2/issue-36053-2.long-type-1255529590694674092.txt'
31    = note: the following trait bounds were not satisfied:
32            `<[closure@$DIR/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
33            which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@$DIR/issue-36053-2.rs:7:39: 7:48]>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/issue-36053-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args mismatched_types/issue-36053-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/mismatched_types/issue-36053-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mismatched_types/issue-36053-2/auxiliary"
stdout: none
error[E0631]: type mismatch in closure arguments
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:32
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                |
   |                                expected due to this
   |
   |
   = note: expected closure signature `for<'a> fn(&'a &str) -> _`
              found closure signature `for<'a> fn(&'a str) -> _`
note: required by a bound in `filter`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:922:5
   |
   |
LL |     once::<&str>("str").fuse().filter(|a: &&str| true).count();


error[E0599]: the method `count` exists for struct `Filter<Fuse<Once<&str>>, [closure@issue-36053-2.rs:7:39]>`, but its trait bounds were not satisfied
  --> fake-test-src-base/mismatched_types/issue-36053-2.rs:7:55
   |
LL |     once::<&str>("str").fuse().filter(|a: &str| true).count();
   |                                       |
   |                                       |
   |                                       doesn't satisfy `<_ as FnOnce<(&&str,)>>::Output = bool`
   |                                       doesn't satisfy `_: FnMut<(&&str,)>`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/filter.rs:18:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48] as FnOnce<(&&str,)>>::Output = bool`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `[closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]: FnMut<(&&str,)>`
           which is required by `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           `Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
           which is required by `&mut Filter<Fuse<std::iter::Once<&str>>, [closure@fake-test-src-base/mismatched_types/issue-36053-2.rs:7:39: 7:48]>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0599, E0631.
For more information about an error, try `rustc --explain E0599`.
---

42    |
43    = note: doesn't satisfy `_: Iterator`
44    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/typeck/issue-31173/issue-31173.long-type-16864929411157086952.txt'
46    = note: the following trait bounds were not satisfied:
47            `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:7:21: 7:25]> as Iterator>::Item = &_`
48            which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@$DIR/issue-31173.rs:7:21: 7:25]>>: Iterator`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173/issue-31173.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-31173.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/typeck/issue-31173.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-31173/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0271]: expected `TakeWhile<&mut IntoIter<u8>, [closure@issue-31173.rs:7:21]>` to be an iterator that yields `&_`, but it yields `u8`
  --> fake-test-src-base/typeck/issue-31173.rs:11:10
   |
LL |         .cloned() //~ ERROR to be an iterator that yields `&_`, but it yields `u8`
   |          ^^^^^^ expected `&_`, found `u8`
   = note: expected reference `&_`
                   found type `u8`
note: the method call chain might not have had the expected associated types
  --> fake-test-src-base/typeck/issue-31173.rs:3:20
  --> fake-test-src-base/typeck/issue-31173.rs:3:20
   |
LL |   pub fn get_tok(it: &mut IntoIter<u8>) {
   |                      ^^^^^^^^^^^^^^^^^ `Iterator::Item` is `u8` here
...
LL |           .take_while(|&x| {
   |  __________-
LL | |             found_e = true;
LL | |             false
LL | |         })
   | |__________- `Iterator::Item` remains `u8` here
note: required by a bound in `cloned`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/traits/iterator.rs:3358:5

error[E0599]: the method `collect` exists for struct `Cloned<TakeWhile<&mut IntoIter<u8>, [closure@issue-31173.rs:7:21]>>`, but its trait bounds were not satisfied
  --> fake-test-src-base/typeck/issue-31173.rs:12:10
   |
LL |       let temp: Vec<u8> = it
   |  _________________________-
LL | |         .take_while(|&x| {
LL | |             found_e = true;
LL | |             false
LL | |         })
LL | |         .cloned() //~ ERROR to be an iterator that yields `&_`, but it yields `u8`
LL | |         .collect(); //~ ERROR the method
   | |         -^^^^^^^ method cannot be called due to unsatisfied trait bounds
   | 
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/take_while.rs:15:1
   |
   |
   = note: doesn't satisfy `<_ as Iterator>::Item = &_`
  --> /rustc/FAKE_PREFIX/library/core/src/iter/adapters/cloned.rs:17:1
   = note: doesn't satisfy `_: Iterator`
   |
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]> as Iterator>::Item = &_`
           which is required by `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
           `Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
           which is required by `&mut Cloned<TakeWhile<&mut std::vec::IntoIter<u8>, [closure@fake-test-src-base/typeck/issue-31173.rs:7:21: 7:25]>>: Iterator`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0271, E0599.
For more information about an error, try `rustc --explain E0271`.
