plain
    Checking test v0.0.0 (/checkout/library/test)
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:88:1
     |
87   |   #[test]
     |   ------- in this procedural macro expansion
88   | / pub fn do_not_run_ignored_tests() {
89   | |     fn f() {
91   | |     }
...    |
...    |
107  | |     assert_ne!(result, TrOk);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:111:1
     |
110  |   #[test]
110  |   #[test]
     |   ------- in this procedural macro expansion
111  | / pub fn ignored_tests_result_in_ignored() {
112  | |     fn f() {}
113  | |     let desc = TestDescAndFn {
114  | |         desc: TestDesc {
128  | |     assert_eq!(result, TrIgnored);
129  | | }
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:134:1
     |
132  |   #[test]
132  |   #[test]
     |   ------- in this procedural macro expansion
133  |   #[cfg(not(target_os = "emscripten"))]
134  | / fn test_should_panic() {
135  | |     fn f() {
137  | |     }
...    |
...    |
153  | |     assert_eq!(result, TrOk);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:159:1
     |
157  |   #[test]
157  |   #[test]
     |   ------- in this procedural macro expansion
158  |   #[cfg(not(target_os = "emscripten"))]
159  | / fn test_should_panic_good_message() {
160  | |     fn f() {
161  | |         panic!("an error message");
...    |
...    |
178  | |     assert_eq!(result, TrOk);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:184:1
     |
182  |   #[test]
182  |   #[test]
     |   ------- in this procedural macro expansion
183  |   #[cfg(not(target_os = "emscripten"))]
184  | / fn test_should_panic_bad_message() {
185  | |     use crate::tests::TrFailedMsg;
186  | |     fn f() {
187  | |         panic!("an error message");
...    |
208  | |     assert_eq!(result, TrFailedMsg(failed_msg.to_string()));
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:214:1
     |
212  |   #[test]
212  |   #[test]
     |   ------- in this procedural macro expansion
213  |   #[cfg(not(target_os = "emscripten"))]
214  | / fn test_should_panic_non_string_message_type() {
215  | |     use crate::tests::TrFailedMsg;
216  | |     use std::any::TypeId;
217  | |     fn f() {
...    |
242  | |     assert_eq!(result, TrFailedMsg(failed_msg));
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:248:1
     |
246  |   #[test]
246  |   #[test]
     |   ------- in this procedural macro expansion
247  |   #[cfg(not(target_os = "emscripten"))]
248  | / fn test_should_panic_but_succeeds() {
249  | |     let should_panic_variants = [ShouldPanic::Yes, ShouldPanic::YesWithMessage("error message")];
250  | |
251  | |     for &should_panic in should_panic_variants.iter() {
282  | |     }
283  | | }
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:309:1
     |
308  |   #[test]
308  |   #[test]
     |   ------- in this procedural macro expansion
309  | / fn test_should_not_report_time() {
310  | |     let exec_time = report_time_test_template(false);
311  | |     assert!(exec_time.is_none());
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:315:1
     |
314  |   #[test]
314  |   #[test]
     |   ------- in this procedural macro expansion
315  | / fn test_should_report_time() {
316  | |     let exec_time = report_time_test_template(true);
317  | |     assert!(exec_time.is_some());
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:347:1
     |
346  |   #[test]
346  |   #[test]
     |   ------- in this procedural macro expansion
347  | / fn test_error_on_exceed() {
348  | |     let types = [TestType::UnitTest, TestType::IntegrationTest, TestType::DocTest];
349  | |
350  | |     for test_type in types.iter() {
...    |
358  | |     assert_eq!(result, TestResult::TrOk);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:378:1
     |
377  |   #[test]
377  |   #[test]
     |   ------- in this procedural macro expansion
378  | / fn test_time_options_threshold() {
379  | |     let unit = TimeThreshold::new(Duration::from_millis(50), Duration::from_millis(100));
380  | |     let integration = TimeThreshold::new(Duration::from_millis(500), Duration::from_millis(1000));
381  | |     let doc = TimeThreshold::new(Duration::from_millis(5000), Duration::from_millis(10000));
409  | |     }
410  | | }
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:413:1
     |
412  |   #[test]
412  |   #[test]
     |   ------- in this procedural macro expansion
413  | / fn parse_ignored_flag() {
414  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--ignored".to_string()];
415  | |     let opts = parse_opts(&args).unwrap().unwrap();
416  | |     assert_eq!(opts.run_ignored, RunIgnored::Only);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:420:1
     |
419  |   #[test]
419  |   #[test]
     |   ------- in this procedural macro expansion
420  | / fn parse_show_output_flag() {
421  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--show-output".to_string()];
422  | |     let opts = parse_opts(&args).unwrap().unwrap();
423  | |     assert!(opts.options.display_output);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:427:1
     |
426  |   #[test]
426  |   #[test]
     |   ------- in this procedural macro expansion
427  | / fn parse_include_ignored_flag() {
428  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--include-ignored".to_string()];
429  | |     let opts = parse_opts(&args).unwrap().unwrap();
430  | |     assert_eq!(opts.run_ignored, RunIgnored::Yes);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:434:1
     |
433  |   #[test]
433  |   #[test]
     |   ------- in this procedural macro expansion
434  | / pub fn filter_for_ignored_option() {
435  | |     // When we run ignored tests the test filter should filter out all the
436  | |     // unignored tests and flip the ignore flag on the rest to false
...    |
...    |
447  | |     assert!(!filtered[0].desc.ignore);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:451:1
     |
450  |   #[test]
450  |   #[test]
     |   ------- in this procedural macro expansion
451  | / pub fn run_include_ignored_option() {
452  | |     // When we "--include-ignored" tests, the ignore flag should be set to false on
454  | |
...    |
...    |
464  | |     assert!(!filtered[1].desc.ignore);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:468:1
     |
467  |   #[test]
467  |   #[test]
     |   ------- in this procedural macro expansion
468  | / pub fn exclude_should_panic_option() {
469  | |     let mut opts = TestOpts::new();
470  | |     opts.run_tests = true;
471  | |     opts.exclude_should_panic = true;
...    |
490  | |     assert!(filtered.iter().all(|test| test.desc.should_panic == ShouldPanic::No));
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:494:1
     |
493  |   #[test]
493  |   #[test]
     |   ------- in this procedural macro expansion
494  | / pub fn exact_filter_match() {
495  | |     fn tests() -> Vec<TestDescAndFn> {
496  | |         vec!["base", "base::test", "base::test1", "base::test2"]
497  | |             .into_iter()
...    |
567  | |     assert_eq!(exact.len(), 2);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:605:1
     |
604  |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:632:1
     |
631  |   #[test]
631  |   #[test]
     |   ------- in this procedural macro expansion
632  | / pub fn shuffle_tests() {
633  | |     let mut opts = TestOpts::new();
634  | |     opts.shuffle = true;
635  | |
...    |
647  | |     assert!(left.iter().zip(right).any(|(a, b)| a.1.desc.name != b.1.desc.name));
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:651:1
     |
650  |   #[test]
650  |   #[test]
     |   ------- in this procedural macro expansion
651  | / pub fn shuffle_tests_with_seed() {
652  | |     let mut opts = TestOpts::new();
653  | |     opts.shuffle = true;
...    |
...    |
665  | |     assert!(left.iter().zip(right).all(|(a, b)| a.1.desc.name == b.1.desc.name));
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:669:1
     |
668  |   #[test]
668  |   #[test]
     |   ------- in this procedural macro expansion
669  | / pub fn order_depends_on_more_than_seed() {
670  | |     let mut opts = TestOpts::new();
671  | |     opts.shuffle = true;
...    |
...    |
693  | |     assert!(left.iter().zip(right).any(|(a, b)| a.0 != b.0));
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:697:1
     |
696  |   #[test]
696  |   #[test]
     |   ------- in this procedural macro expansion
697  | / pub fn test_metricmap_compare() {
698  | |     let mut m1 = MetricMap::new();
699  | |     let mut m2 = MetricMap::new();
700  | |     m1.insert_metric("in-both-noise", 1000.0, 200.0);
...    |
716  | |     m2.insert_metric("in-both-want-upwards-and-improved", 2000.0, -10.0);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:720:1
     |
719  |   #[test]
719  |   #[test]
     |   ------- in this procedural macro expansion
720  | / pub fn test_bench_once_no_iter() {
721  | |     fn f(_: &mut Bencher) {}
722  | |     bench::run_once(f);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:726:1
     |
725  |   #[test]
725  |   #[test]
     |   ------- in this procedural macro expansion
726  | / pub fn test_bench_once_iter() {
727  | |     fn f(b: &mut Bencher) {
728  | |         b.iter(|| {})
729  | |     }
730  | |     bench::run_once(f);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:734:1
     |
733  |   #[test]
733  |   #[test]
     |   ------- in this procedural macro expansion
734  | / pub fn test_bench_no_iter() {
735  | |     fn f(_: &mut Bencher) {}
736  | |
737  | |     let (tx, rx) = channel();
...    |
750  | |     rx.recv().unwrap();
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:754:1
     |
753  |   #[test]
753  |   #[test]
     |   ------- in this procedural macro expansion
754  | / pub fn test_bench_iter() {
755  | |     fn f(b: &mut Bencher) {
756  | |         b.iter(|| {})
...    |
...    |
772  | |     rx.recv().unwrap();
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:776:1
     |
775  |   #[test]
775  |   #[test]
     |   ------- in this procedural macro expansion
776  | / fn should_sort_failures_before_printing_them() {
777  | |     let test_a = TestDesc {
778  | |         name: StaticTestName("a"),
779  | |         ignore: false,
...    |
824  | |     assert!(apos < bpos);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:70:1
     |
69   |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:130:1
     |
129  |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:160:1
     |
159  |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:385:1
     |
384  |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:430:1
     |
429  |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:475:1
     |
474  |   #[test]
---
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:566:1
     |
565  |   #[test]
565  |   #[test]
     |   ------- in this procedural macro expansion
566  | / fn test_sum_f64s() {
567  | |     assert_eq!([0.5f64, 3.2321f64, 1.5678f64].sum(), 5.2999);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:570:1
     |
569  |   #[test]
569  |   #[test]
     |   ------- in this procedural macro expansion
570  | / fn test_sum_f64_between_ints_that_sum_to_0() {
571  | |     assert_eq!([1e30f64, 1.2f64, -1e30f64].sum(), 1.2);
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:575:1
     |
     |
574  |   #[bench]
     |   -------- in this procedural macro expansion
575  | / pub fn sum_three_items(b: &mut Bencher) {
576  | |     b.iter(|| {
577  | |         [1e20f64, 1.5f64, -1e20f64].sum();
579  | | }
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1461:5
    ::: /checkout/library/core/src/macros/mod.rs:1461:5
     |
1461 | /     pub macro bench($item:item) {
1463 | |     }
1463 | |     }
     | |_____- in this expansion of `#[bench]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1461:5
     |
1461 | /     pub macro bench($item:item) {
1463 | |     }
1463 | |     }
     | |_____- in this expansion of `#[bench]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:591:1
     |
     |
590  |   #[bench]
     |   -------- in this procedural macro expansion
591  |   pub fn no_iter(_: &mut Bencher) {}
     |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `ignore_message`
    ::: /checkout/library/core/src/macros/mod.rs:1461:5
     |
     |
1461 | /     pub macro bench($item:item) {
1463 | |     }
1463 | |     }
     | |_____- in this expansion of `#[bench]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/term/terminfo/parm/tests.rs:15:1
14   |   #[test]
     |   ------- in this procedural macro expansion
15   | / fn test_multiple_int_constants() {
16   | |     assert_eq!(
16   | |     assert_eq!(
17   | |         expand(b"%{1}%{2}%d%d", &[], &mut Variables::new()).unwrap(),
18   | |         "21".bytes().collect::<Vec<_>>()
20   | | }
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/term/terminfo/parm/tests.rs:73:1
72   |   #[test]
     |   ------- in this procedural macro expansion
73   | / fn test_push_bad_param() {
73   | / fn test_push_bad_param() {
74   | |     assert!(expand(b"%pa", &[], &mut Variables::new()).is_err());
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
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
     | |_^ missing `ignore_message`
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/term/terminfo/parser/compiled/tests.rs:4:1
     |
3    |   #[test]
3    |   #[test]
     |   ------- in this procedural macro expansion
4    | / fn test_veclens() {
5    | |     assert_eq!(boolfnames.len(), boolnames.len());
6    | |     assert_eq!(numfnames.len(), numnames.len());
7    | |     assert_eq!(stringfnames.len(), stringnames.len());
     | |_^ missing `ignore_message`
     |
    ::: /checkout/library/core/src/macros/mod.rs:1448:5
     |
     |
1448 | /     pub macro test($item:item) {
1450 | |     }
1450 | |     }
     | |_____- in this expansion of `#[test]`
    Checking rand_core v0.5.1
error[E0063]: missing field `ignore_message` in initializer of `types::TestDesc`
  --> library/test/src/tests.rs:61:19
   |
---

error[E0063]: missing field `ignore_message` in initializer of `types::TestDesc`
   --> library/test/src/tests.rs:777:18
    |
777 |     let test_a = TestDesc {
    |                  ^^^^^^^^ missing `ignore_message`
error[E0063]: missing field `ignore_message` in initializer of `types::TestDesc`
   --> library/test/src/tests.rs:787:18
    |
    |
787 |     let test_b = TestDesc {
    |                  ^^^^^^^^ missing `ignore_message`
For more information about this error, try `rustc --explain E0063`.
error: could not compile `test` due to 77 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
