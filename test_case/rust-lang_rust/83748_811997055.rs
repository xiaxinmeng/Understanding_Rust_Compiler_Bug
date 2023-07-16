plain
.................................................................................................... 9400/11730
.................................................................................................... 9500/11730
.......................................................................i......i..................... 9600/11730
.................................................................................................... 9700/11730
.................iiiiiii..iiiiii.i.................................................................. 9800/11730
.................................................................................................... 10000/11730
.................................................................................................... 10100/11730
.................................................................................................... 10200/11730
.................................................................................................... 10300/11730
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.101 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.395 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
i.....................i.....................i.....................i................................. 2800/2848
................................................
failures:

---- src/iter/traits/iterator.rs - iter::traits::iterator::Iterator::dedup_by_key (line 1687) stdout ----
error[E0599]: the method `next` exists for struct `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>`, but its trait bounds were not satisfied
    |
    |
8   | let mut iter = vec.into_iter().dedup_by_key(|&i| i / 10);
    |                                             ----------- doesn't satisfy `_: Fn<(&{integer},)>`
9   | 
10  | assert_eq!(iter.next(), Some(10));
    |                 ^^^^ method cannot be called on `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/dedup.rs:124:1
    |
    |
124 | pub struct DedupByKey<I, F, T> {
    | ------------------------------ doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `[closure@src/iter/traits/iterator.rs:8:45: 8:56]: Fn<(&{integer},)>`
            which is required by `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>: Iterator`

error[E0599]: the method `next` exists for struct `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>`, but its trait bounds were not satisfied
    |
    |
8   | let mut iter = vec.into_iter().dedup_by_key(|&i| i / 10);
    |                                             ----------- doesn't satisfy `_: Fn<(&{integer},)>`
...
11  | assert_eq!(iter.next(), Some(20));
    |                 ^^^^ method cannot be called on `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/dedup.rs:124:1
    |
    |
124 | pub struct DedupByKey<I, F, T> {
    | ------------------------------ doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `[closure@src/iter/traits/iterator.rs:8:45: 8:56]: Fn<(&{integer},)>`
            which is required by `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>: Iterator`

error[E0599]: the method `next` exists for struct `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>`, but its trait bounds were not satisfied
    |
    |
8   | let mut iter = vec.into_iter().dedup_by_key(|&i| i / 10);
    |                                             ----------- doesn't satisfy `_: Fn<(&{integer},)>`
...
12  | assert_eq!(iter.next(), Some(30));
    |                 ^^^^ method cannot be called on `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/dedup.rs:124:1
    |
    |
124 | pub struct DedupByKey<I, F, T> {
    | ------------------------------ doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `[closure@src/iter/traits/iterator.rs:8:45: 8:56]: Fn<(&{integer},)>`
            which is required by `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>: Iterator`

error[E0599]: the method `next` exists for struct `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>`, but its trait bounds were not satisfied
    |
    |
8   | let mut iter = vec.into_iter().dedup_by_key(|&i| i / 10);
    |                                             ----------- doesn't satisfy `_: Fn<(&{integer},)>`
...
13  | assert_eq!(iter.next(), Some(20));
    |                 ^^^^ method cannot be called on `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/dedup.rs:124:1
    |
    |
124 | pub struct DedupByKey<I, F, T> {
    | ------------------------------ doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `[closure@src/iter/traits/iterator.rs:8:45: 8:56]: Fn<(&{integer},)>`
            which is required by `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>: Iterator`

error[E0599]: the method `next` exists for struct `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>`, but its trait bounds were not satisfied
    |
    |
8   | let mut iter = vec.into_iter().dedup_by_key(|&i| i / 10);
    |                                             ----------- doesn't satisfy `_: Fn<(&{integer},)>`
...
14  | assert_eq!(iter.next(), None);
    |                 ^^^^ method cannot be called on `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>` due to unsatisfied trait bounds
   ::: /checkout/library/core/src/iter/adapters/dedup.rs:124:1
    |
    |
124 | pub struct DedupByKey<I, F, T> {
    | ------------------------------ doesn't satisfy `_: Iterator`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `[closure@src/iter/traits/iterator.rs:8:45: 8:56]: Fn<(&{integer},)>`
            which is required by `DedupByKey<std::vec::IntoIter<{integer}>, [closure@src/iter/traits/iterator.rs:8:45: 8:56], {integer}>: Iterator`
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:20:06
