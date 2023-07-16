plain
    --> library/test/src/tests.rs:86:1
     |
85   |   #[test]
     |   ------- in this procedural macro expansion
86   | / pub fn do_not_run_ignored_tests() {
87   | |     fn f() {
89   | |     }
...    |
...    |
104  | |     assert_ne!(result, TrOk);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:108:1
     |
107  |   #[test]
107  |   #[test]
     |   ------- in this procedural macro expansion
108  | / pub fn ignored_tests_result_in_ignored() {
109  | |     fn f() {}
110  | |     let desc = TestDescAndFn {
111  | |         desc: TestDesc {
124  | |     assert_eq!(result, TrIgnored);
125  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:130:1
     |
128  |   #[test]
128  |   #[test]
     |   ------- in this procedural macro expansion
129  |   #[cfg(not(target_os = "emscripten"))]
130  | / fn test_should_panic() {
131  | |     fn f() {
133  | |     }
...    |
...    |
148  | |     assert_eq!(result, TrOk);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:154:1
     |
152  |   #[test]
152  |   #[test]
     |   ------- in this procedural macro expansion
153  |   #[cfg(not(target_os = "emscripten"))]
154  | / fn test_should_panic_good_message() {
155  | |     fn f() {
156  | |         panic!("an error message");
...    |
...    |
172  | |     assert_eq!(result, TrOk);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:178:1
     |
176  |   #[test]
176  |   #[test]
     |   ------- in this procedural macro expansion
177  |   #[cfg(not(target_os = "emscripten"))]
178  | / fn test_should_panic_bad_message() {
179  | |     use crate::tests::TrFailedMsg;
180  | |     fn f() {
181  | |         panic!("an error message");
...    |
201  | |     assert_eq!(result, TrFailedMsg(failed_msg.to_string()));
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:207:1
     |
205  |   #[test]
205  |   #[test]
     |   ------- in this procedural macro expansion
206  |   #[cfg(not(target_os = "emscripten"))]
207  | / fn test_should_panic_non_string_message_type() {
208  | |     use crate::tests::TrFailedMsg;
209  | |     use std::any::TypeId;
210  | |     fn f() {
...    |
234  | |     assert_eq!(result, TrFailedMsg(failed_msg));
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:240:1
     |
238  |   #[test]
238  |   #[test]
     |   ------- in this procedural macro expansion
239  |   #[cfg(not(target_os = "emscripten"))]
240  | / fn test_should_panic_but_succeeds() {
241  | |     let should_panic_variants = [ShouldPanic::Yes, ShouldPanic::YesWithMessage("error message")];
242  | |
243  | |     for &should_panic in should_panic_variants.iter() {
273  | |     }
274  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:299:1
     |
298  |   #[test]
298  |   #[test]
     |   ------- in this procedural macro expansion
299  | / fn test_should_not_report_time() {
300  | |     let exec_time = report_time_test_template(false);
301  | |     assert!(exec_time.is_none());
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:305:1
     |
304  |   #[test]
304  |   #[test]
     |   ------- in this procedural macro expansion
305  | / fn test_should_report_time() {
306  | |     let exec_time = report_time_test_template(true);
307  | |     assert!(exec_time.is_some());
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:336:1
     |
335  |   #[test]
335  |   #[test]
     |   ------- in this procedural macro expansion
336  | / fn test_error_on_exceed() {
337  | |     let types = [TestType::UnitTest, TestType::IntegrationTest, TestType::DocTest];
338  | |
339  | |     for test_type in types.iter() {
...    |
347  | |     assert_eq!(result, TestResult::TrOk);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:366:1
     |
365  |   #[test]
365  |   #[test]
     |   ------- in this procedural macro expansion
366  | / fn test_time_options_threshold() {
367  | |     let unit = TimeThreshold::new(Duration::from_millis(50), Duration::from_millis(100));
368  | |     let integration = TimeThreshold::new(Duration::from_millis(500), Duration::from_millis(1000));
369  | |     let doc = TimeThreshold::new(Duration::from_millis(5000), Duration::from_millis(10000));
397  | |     }
398  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:401:1
     |
400  |   #[test]
400  |   #[test]
     |   ------- in this procedural macro expansion
401  | / fn parse_ignored_flag() {
402  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--ignored".to_string()];
403  | |     let opts = parse_opts(&args).unwrap().unwrap();
404  | |     assert_eq!(opts.run_ignored, RunIgnored::Only);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:408:1
     |
407  |   #[test]
407  |   #[test]
     |   ------- in this procedural macro expansion
408  | / fn parse_show_output_flag() {
409  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--show-output".to_string()];
410  | |     let opts = parse_opts(&args).unwrap().unwrap();
411  | |     assert!(opts.options.display_output);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:415:1
     |
414  |   #[test]
414  |   #[test]
     |   ------- in this procedural macro expansion
415  | / fn parse_include_ignored_flag() {
416  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--include-ignored".to_string()];
417  | |     let opts = parse_opts(&args).unwrap().unwrap();
418  | |     assert_eq!(opts.run_ignored, RunIgnored::Yes);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:422:1
     |
421  |   #[test]
421  |   #[test]
     |   ------- in this procedural macro expansion
422  | / pub fn filter_for_ignored_option() {
423  | |     // When we run ignored tests the test filter should filter out all the
424  | |     // unignored tests and flip the ignore flag on the rest to false
...    |
...    |
435  | |     assert!(!filtered[0].desc.ignore);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:439:1
     |
438  |   #[test]
438  |   #[test]
     |   ------- in this procedural macro expansion
439  | / pub fn run_include_ignored_option() {
440  | |     // When we "--include-ignored" tests, the ignore flag should be set to false on
442  | |
...    |
...    |
452  | |     assert!(!filtered[1].desc.ignore);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:456:1
     |
455  |   #[test]
455  |   #[test]
     |   ------- in this procedural macro expansion
456  | / pub fn exclude_should_panic_option() {
457  | |     let mut opts = TestOpts::new();
458  | |     opts.run_tests = true;
459  | |     opts.exclude_should_panic = true;
...    |
477  | |     assert!(filtered.iter().all(|test| test.desc.should_panic == ShouldPanic::No));
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:481:1
     |
480  |   #[test]
480  |   #[test]
     |   ------- in this procedural macro expansion
481  | / pub fn exact_filter_match() {
482  | |     fn tests() -> Vec<TestDescAndFn> {
483  | |         ["base", "base::test", "base::test1", "base::test2"]
484  | |             .into_iter()
...    |
553  | |     assert_eq!(exact.len(), 2);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:590:1
     |
589  |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:617:1
     |
616  |   #[test]
616  |   #[test]
     |   ------- in this procedural macro expansion
617  | / pub fn shuffle_tests() {
618  | |     let mut opts = TestOpts::new();
619  | |     opts.shuffle = true;
620  | |
...    |
632  | |     assert!(left.iter().zip(right).any(|(a, b)| a.1.desc.name != b.1.desc.name));
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:636:1
     |
635  |   #[test]
635  |   #[test]
     |   ------- in this procedural macro expansion
636  | / pub fn shuffle_tests_with_seed() {
637  | |     let mut opts = TestOpts::new();
638  | |     opts.shuffle = true;
639  | |
...    |
650  | |     assert!(left.iter().zip(right).all(|(a, b)| a.1.desc.name == b.1.desc.name));
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:654:1
     |
653  |   #[test]
653  |   #[test]
     |   ------- in this procedural macro expansion
654  | / pub fn order_depends_on_more_than_seed() {
655  | |     let mut opts = TestOpts::new();
656  | |     opts.shuffle = true;
657  | |
...    |
678  | |     assert!(left.iter().zip(right).any(|(a, b)| a.0 != b.0));
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:682:1
     |
681  |   #[test]
681  |   #[test]
     |   ------- in this procedural macro expansion
682  | / pub fn test_metricmap_compare() {
683  | |     let mut m1 = MetricMap::new();
684  | |     let mut m2 = MetricMap::new();
685  | |     m1.insert_metric("in-both-noise", 1000.0, 200.0);
...    |
701  | |     m2.insert_metric("in-both-want-upwards-and-improved", 2000.0, -10.0);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:705:1
     |
704  |   #[test]
704  |   #[test]
     |   ------- in this procedural macro expansion
705  | / pub fn test_bench_once_no_iter() {
706  | |     fn f(_: &mut Bencher) {}
707  | |     bench::run_once(f);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:711:1
     |
710  |   #[test]
710  |   #[test]
     |   ------- in this procedural macro expansion
711  | / pub fn test_bench_once_iter() {
712  | |     fn f(b: &mut Bencher) {
713  | |         b.iter(|| {})
714  | |     }
715  | |     bench::run_once(f);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:719:1
     |
718  |   #[test]
718  |   #[test]
     |   ------- in this procedural macro expansion
719  | / pub fn test_bench_no_iter() {
720  | |     fn f(_: &mut Bencher) {}
721  | |
722  | |     let (tx, rx) = channel();
...    |
734  | |     rx.recv().unwrap();
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:738:1
     |
737  |   #[test]
737  |   #[test]
     |   ------- in this procedural macro expansion
738  | / pub fn test_bench_iter() {
739  | |     fn f(b: &mut Bencher) {
740  | |         b.iter(|| {})
...    |
...    |
755  | |     rx.recv().unwrap();
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/tests.rs:759:1
     |
758  |   #[test]
758  |   #[test]
     |   ------- in this procedural macro expansion
759  | / fn should_sort_failures_before_printing_them() {
760  | |     let test_a = TestDesc {
761  | |         name: StaticTestName("a"),
762  | |         ignore: false,
...    |
804  | |     assert!(apos < bpos);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:43:1
     |
42   |   #[test]
42   |   #[test]
     |   ------- in this procedural macro expansion
43   | / fn test_min_max_nan() {
44   | |     let xs = &[1.0, 2.0, f64::NAN, 3.0, 4.0];
45   | |     let summary = Summary::new(xs);
46   | |     assert_eq!(summary.min, 1.0);
47   | |     assert_eq!(summary.max, 4.0);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:51:1
     |
50   |   #[test]
50   |   #[test]
     |   ------- in this procedural macro expansion
51   | / fn test_norm2() {
52   | |     let val = &[958.0000000000, 924.0000000000];
53   | |     let summ = &Summary {
54   | |         sum: 1882.0000000000,
67   | |     check(val, summ);
68   | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:70:1
     |
69   |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:100:1
     |
99   |   #[test]
99   |   #[test]
     |   ------- in this procedural macro expansion
100  | / fn test_norm10medium() {
101  | |     let val = &[
102  | |         954.0000000000,
103  | |         1064.0000000000,
127  | |     check(val, summ);
128  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:130:1
     |
129  |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:160:1
     |
159  |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:205:1
     |
204  |   #[test]
204  |   #[test]
     |   ------- in this procedural macro expansion
205  | / fn test_exp10a() {
206  | |     let val = &[
207  | |         23.0000000000,
208  | |         11.0000000000,
232  | |     check(val, summ);
233  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:235:1
     |
234  |   #[test]
234  |   #[test]
     |   ------- in this procedural macro expansion
235  | / fn test_exp10b() {
236  | |     let val = &[
237  | |         24.0000000000,
238  | |         17.0000000000,
262  | |     check(val, summ);
263  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:265:1
     |
264  |   #[test]
264  |   #[test]
     |   ------- in this procedural macro expansion
265  | / fn test_exp10c() {
266  | |     let val = &[
267  | |         71.0000000000,
268  | |         2.0000000000,
292  | |     check(val, summ);
293  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:295:1
     |
294  |   #[test]
294  |   #[test]
     |   ------- in this procedural macro expansion
295  | / fn test_exp25() {
296  | |     let val = &[
297  | |         3.0000000000,
298  | |         24.0000000000,
337  | |     check(val, summ);
338  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:340:1
     |
339  |   #[test]
339  |   #[test]
     |   ------- in this procedural macro expansion
340  | / fn test_binom25() {
341  | |     let val = &[
342  | |         18.0000000000,
343  | |         17.0000000000,
382  | |     check(val, summ);
383  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:385:1
     |
384  |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:430:1
     |
429  |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:475:1
     |
474  |   #[test]
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:520:1
     |
519  |   #[test]
519  |   #[test]
     |   ------- in this procedural macro expansion
520  | / fn test_unif25() {
521  | |     let val = &[
522  | |         99.0000000000,
523  | |         55.0000000000,
562  | |     check(val, summ);
563  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:566:1
     |
565  |   #[test]
565  |   #[test]
     |   ------- in this procedural macro expansion
566  | / fn test_sum_f64s() {
567  | |     assert_eq!([0.5f64, 3.2321f64, 1.5678f64].sum(), 5.2999);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:570:1
     |
569  |   #[test]
569  |   #[test]
     |   ------- in this procedural macro expansion
570  | / fn test_sum_f64_between_ints_that_sum_to_0() {
571  | |     assert_eq!([1e30f64, 1.2f64, -1e30f64].sum(), 1.2);
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:575:1
     |
     |
574  |   #[bench]
     |   -------- in this procedural macro expansion
575  | / pub fn sum_three_items(b: &mut Bencher) {
576  | |     b.iter(|| {
577  | |         [1e20f64, 1.5f64, -1e20f64].sum();
579  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1459:5
    ::: /checkout/library/core/src/macros/mod.rs:1459:5
     |
1459 | /     pub macro bench($item:item) {
1461 | |     }
1461 | |     }
     | |_____- in this expansion of `#[bench]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:581:1
     |
     |
580  |   #[bench]
     |   -------- in this procedural macro expansion
581  | / pub fn sum_many_f64(b: &mut Bencher) {
582  | |     let nums = [-1e30f64, 1e60, 1e30, 1.0, -1e60];
583  | |     let v = (0..500).map(|i| nums[i % 5]).collect::<Vec<_>>();
...    |
587  | |     })
588  | | }
     | |_^ `tests::test::TestDesc` does not have this field
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1459:5
     |
1459 | /     pub macro bench($item:item) {
1461 | |     }
1461 | |     }
     | |_____- in this expansion of `#[bench]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/stats/tests.rs:591:1
     |
     |
590  |   #[bench]
     |   -------- in this procedural macro expansion
591  |   pub fn no_iter(_: &mut Bencher) {}
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `tests::test::TestDesc` does not have this field
    ::: /checkout/library/core/src/macros/mod.rs:1459:5
     |
     |
1459 | /     pub macro bench($item:item) {
1461 | |     }
1461 | |     }
     | |_____- in this expansion of `#[bench]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/searcher/tests.rs:5:1
     |
3    |   #[test]
3    |   #[test]
     |   ------- in this procedural macro expansion
4    |   #[ignore = "buildbots don't have ncurses installed and I can't mock everything I need"]
5    | / fn test_get_dbpath_for_term() {
6    | |     // woefully inadequate test coverage
7    | |     // note: current tests won't work with non-standard terminfo hierarchies (e.g., macOS's)
8    | |     use std::env;
...    |
18   | |     env::remove_var("TERMINFO_DIRS");
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:6:1
5    |   #[test]
     |   ------- in this procedural macro expansion
     |   ------- in this procedural macro expansion
6    | / fn test_basic_setabf() {
7    | |     let s = b"\\E[48;5;%p1%dm";
8    | |     assert_eq!(
9    | |         expand(s, &[Number(1)], &mut Variables::new()).unwrap(),
10   | |         "\\E[48;5;1m".bytes().collect::<Vec<_>>()
12   | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:15:1
14   |   #[test]
     |   ------- in this procedural macro expansion
15   | / fn test_multiple_int_constants() {
16   | |     assert_eq!(
16   | |     assert_eq!(
17   | |         expand(b"%{1}%{2}%d%d", &[], &mut Variables::new()).unwrap(),
18   | |         "21".bytes().collect::<Vec<_>>()
20   | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:23:1
22   |   #[test]
     |   ------- in this procedural macro expansion
     |   ------- in this procedural macro expansion
23   | / fn test_op_i() {
24   | |     let mut vars = Variables::new();
25   | |     assert_eq!(
26   | |         expand(b"%p1%d%p2%d%p3%d%i%p1%d%p2%d%p3%d", &[Number(1), Number(2), Number(3)], &mut vars),
32   | |     );
33   | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:36:1
35   |   #[test]
     |   ------- in this procedural macro expansion
36   | / fn test_param_stack_failure_conditions() {
36   | / fn test_param_stack_failure_conditions() {
37   | |     let mut varstruct = Variables::new();
38   | |     let vars = &mut varstruct;
...    |
69   | |     }
70   | | }
     | |_^ `tests::test::TestDesc` does not have this field
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:73:1
72   |   #[test]
     |   ------- in this procedural macro expansion
73   | / fn test_push_bad_param() {
73   | / fn test_push_bad_param() {
74   | |     assert!(expand(b"%pa", &[], &mut Variables::new()).is_err());
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:78:1
77   |   #[test]
     |   ------- in this procedural macro expansion
78   | / fn test_comparison_ops() {
78   | / fn test_comparison_ops() {
79   | |     let v = [('<', [1u8, 0u8, 0u8]), ('=', [0u8, 1u8, 0u8]), ('>', [0u8, 0u8, 1u8])];
80   | |     for &(op, bs) in v.iter() {
81   | |         let s = format!("%{{1}}%{{2}}%{}%d", op);
93   | |     }
94   | | }
     | |_^ `tests::test::TestDesc` does not have this field
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:97:1
96   |   #[test]
     |   ------- in this procedural macro expansion
97   | / fn test_conditionals() {
98   | |     let mut vars = Variables::new();
98   | |     let mut vars = Variables::new();
99   | |     let s = b"\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m";
100  | |     let res = expand(s, &[Number(1)], &mut vars);
...    |
108  | |     assert_eq!(res.unwrap(), "\\E[38;5;42m".bytes().collect::<Vec<_>>());
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parm/tests.rs:112:1
111  |   #[test]
     |   ------- in this procedural macro expansion
112  | / fn test_format() {
112  | / fn test_format() {
113  | |     let mut varstruct = Variables::new();
114  | |     let vars = &mut varstruct;
...    |
123  | |     );
124  | | }
     | |_^ `tests::test::TestDesc` does not have this field
---
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
error[E0560]: struct `tests::test::TestDesc` has no field named `allow_fail`
    --> library/test/src/term/terminfo/parser/compiled/tests.rs:4:1
     |
3    |   #[test]
3    |   #[test]
     |   ------- in this procedural macro expansion
4    | / fn test_veclens() {
5    | |     assert_eq!(boolfnames.len(), boolnames.len());
6    | |     assert_eq!(numfnames.len(), numnames.len());
7    | |     assert_eq!(stringfnames.len(), stringnames.len());
     | |_^ `tests::test::TestDesc` does not have this field
     |
    ::: /checkout/library/core/src/macros/mod.rs:1446:5
     |
     |
1446 | /     pub macro test($item:item) {
1447 | |         /* compiler built-in */
1448 | |     }
     | |_____- in this expansion of `#[test]`
     |
     = note: available fields are: `name`, `ignore`, `should_panic`, `compile_fail`, `no_run`, `test_type`
    Checking rand_core v0.5.1
    Checking rand_chacha v0.2.2
    Checking rand_xorshift v0.2.0
For more information about this error, try `rustc --explain E0560`.
