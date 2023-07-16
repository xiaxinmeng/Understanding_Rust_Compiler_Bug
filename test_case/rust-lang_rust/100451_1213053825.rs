plain
    Checking test v0.0.0 (/checkout/library/test)
error[E0308]: mismatched types
  --> library/test/src/tests.rs:70:48
   |
70 |             testfn: DynTestFn(Box::new(move || {})),
   |
   = note:   expected enum `Result<(), String>`
           found unit type `()`


error[E0308]: mismatched types
  --> library/test/src/tests.rs:82:48
   |
82 |             testfn: DynTestFn(Box::new(move || {})),
   |
   = note:   expected enum `Result<(), String>`
           found unit type `()`


error[E0271]: type mismatch resolving `<fn() {report_time_test_template::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
297 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {time_test_failure_template::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
332 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {testfn} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
596 |             testfn: DynTestFn(Box::new(testfn)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

    Checking rand_core v0.5.1
error[E0271]: type mismatch resolving `<fn() {tests::do_not_run_ignored_tests::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
102 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {tests::ignored_tests_result_in_ignored::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
123 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {tests::test_should_panic::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
148 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {tests::test_should_panic_good_message::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
173 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {tests::test_should_panic_bad_message::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
203 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {tests::test_should_panic_non_string_message_type::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
237 |         testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0271]: type mismatch resolving `<fn() {tests::test_should_panic_but_succeeds::f} as FnOnce<()>>::Output == Result<(), String>`
    |
    |
263 |             testfn: DynTestFn(Box::new(f)),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`
    = note: required for the cast to the object type `dyn FnOnce() -> Result<(), String> + Send`

error[E0308]: mismatched types
   --> library/test/src/tests.rs:483:44
    |
483 |         testfn: DynTestFn(Box::new(move || {})),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`


error[E0308]: mismatched types
   --> library/test/src/tests.rs:507:52
    |
507 |                 testfn: DynTestFn(Box::new(move || {})),
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`


error[E0271]: type mismatch resolving `for<'r> <for<'s> fn(&'s mut bench::Bencher) {tests::test_bench_once_no_iter::f} as FnOnce<(&'r mut bench::Bencher,)>>::Output == Result<(), String>`
    |
721 |     bench::run_once(f);
    |     ^^^^^^^^^^^^^^^ expected enum `Result`, found `()`
    |
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
note: required by a bound in `bench::run_once`
   --> library/test/src/bench.rs:238:31
    |
236 | pub fn run_once<F>(f: F)
237 | where
238 |     F: FnMut(&mut Bencher) -> Result<(), String>,
    |                               ^^^^^^^^^^^^^^^^^^ required by this bound in `bench::run_once`


error[E0271]: type mismatch resolving `for<'r> <for<'s> fn(&'s mut bench::Bencher) {tests::test_bench_once_iter::f} as FnOnce<(&'r mut bench::Bencher,)>>::Output == Result<(), String>`
    |
729 |     bench::run_once(f);
    |     ^^^^^^^^^^^^^^^ expected enum `Result`, found `()`
    |
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
note: required by a bound in `bench::run_once`
   --> library/test/src/bench.rs:238:31
    |
236 | pub fn run_once<F>(f: F)
237 | where
238 |     F: FnMut(&mut Bencher) -> Result<(), String>,
    |                               ^^^^^^^^^^^^^^^^^^ required by this bound in `bench::run_once`


error[E0271]: type mismatch resolving `for<'r> <for<'s> fn(&'s mut bench::Bencher) {tests::test_bench_no_iter::f} as FnOnce<(&'r mut bench::Bencher,)>>::Output == Result<(), String>`
    |
    |
748 |     crate::bench::benchmark(TestId(0), desc, tx, true, f);
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
note: required by a bound in `bench::benchmark`
note: required by a bound in `bench::benchmark`
   --> library/test/src/bench.rs:198:31
    |
191 | pub fn benchmark<F>(
...
198 |     F: FnMut(&mut Bencher) -> Result<(), String>,
    |                               ^^^^^^^^^^^^^^^^^^ required by this bound in `bench::benchmark`


error[E0271]: type mismatch resolving `for<'r> <for<'s> fn(&'s mut bench::Bencher) {tests::test_bench_iter::f} as FnOnce<(&'r mut bench::Bencher,)>>::Output == Result<(), String>`
    |
    |
770 |     crate::bench::benchmark(TestId(0), desc, tx, true, f);
    |
    = note:   expected enum `Result<(), String>`
            found unit type `()`
note: required by a bound in `bench::benchmark`
note: required by a bound in `bench::benchmark`
   --> library/test/src/bench.rs:198:31
    |
191 | pub fn benchmark<F>(
...
198 |     F: FnMut(&mut Bencher) -> Result<(), String>,
    |                               ^^^^^^^^^^^^^^^^^^ required by this bound in `bench::benchmark`

