plain
    Checking test v0.0.0 (/checkout/library/test)
error[E0063]: missing field `ignore_message` in initializer of `tests::test::TestDesc`
    --> library/test/src/tests.rs:90:1
     |
89   |   #[test]
     |   ------- in this procedural macro expansion
90   | / pub fn do_not_run_ignored_tests() {
91   | |     fn f() {
93   | |     }
...    |
...    |
110  | |     assert_ne!(result, TrOk);
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
    --> library/test/src/tests.rs:114:1
     |
113  |   #[test]
113  |   #[test]
     |   ------- in this procedural macro expansion
114  | / pub fn ignored_tests_result_in_ignored() {
115  | |     fn f() {}
116  | |     let desc = TestDescAndFn {
117  | |         desc: TestDesc {
132  | |     assert_eq!(result, TrIgnored);
133  | | }
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
    --> library/test/src/tests.rs:138:1
     |
136  |   #[test]
136  |   #[test]
     |   ------- in this procedural macro expansion
137  |   #[cfg(not(target_os = "emscripten"))]
138  | / fn test_should_panic() {
139  | |     fn f() {
141  | |     }
...    |
...    |
158  | |     assert_eq!(result, TrOk);
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
    --> library/test/src/tests.rs:164:1
     |
162  |   #[test]
162  |   #[test]
     |   ------- in this procedural macro expansion
163  |   #[cfg(not(target_os = "emscripten"))]
164  | / fn test_should_panic_good_message() {
165  | |     fn f() {
166  | |         panic!("an error message");
...    |
...    |
184  | |     assert_eq!(result, TrOk);
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
    --> library/test/src/tests.rs:190:1
     |
188  |   #[test]
188  |   #[test]
     |   ------- in this procedural macro expansion
189  |   #[cfg(not(target_os = "emscripten"))]
190  | / fn test_should_panic_bad_message() {
191  | |     use crate::tests::TrFailedMsg;
192  | |     fn f() {
193  | |         panic!("an error message");
...    |
215  | |     assert_eq!(result, TrFailedMsg(failed_msg.to_string()));
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
    --> library/test/src/tests.rs:221:1
     |
219  |   #[test]
219  |   #[test]
     |   ------- in this procedural macro expansion
220  |   #[cfg(not(target_os = "emscripten"))]
221  | / fn test_should_panic_non_string_message_type() {
222  | |     use crate::tests::TrFailedMsg;
223  | |     use std::any::TypeId;
224  | |     fn f() {
...    |
250  | |     assert_eq!(result, TrFailedMsg(failed_msg));
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
    --> library/test/src/tests.rs:256:1
     |
254  |   #[test]
254  |   #[test]
     |   ------- in this procedural macro expansion
255  |   #[cfg(not(target_os = "emscripten"))]
256  | / fn test_should_panic_but_succeeds() {
257  | |     let should_panic_variants = [ShouldPanic::Yes, ShouldPanic::YesWithMessage("error message")];
258  | |
259  | |     for &should_panic in should_panic_variants.iter() {
291  | |     }
292  | | }
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
    --> library/test/src/tests.rs:319:1
     |
318  |   #[test]
318  |   #[test]
     |   ------- in this procedural macro expansion
319  | / fn test_should_not_report_time() {
320  | |     let exec_time = report_time_test_template(false);
321  | |     assert!(exec_time.is_none());
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
    --> library/test/src/tests.rs:325:1
     |
324  |   #[test]
324  |   #[test]
     |   ------- in this procedural macro expansion
325  | / fn test_should_report_time() {
326  | |     let exec_time = report_time_test_template(true);
327  | |     assert!(exec_time.is_some());
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
    --> library/test/src/tests.rs:358:1
     |
357  |   #[test]
357  |   #[test]
     |   ------- in this procedural macro expansion
358  | / fn test_error_on_exceed() {
359  | |     let types = [TestType::UnitTest, TestType::IntegrationTest, TestType::DocTest];
360  | |
361  | |     for test_type in types.iter() {
...    |
369  | |     assert_eq!(result, TestResult::TrOk);
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
    --> library/test/src/tests.rs:390:1
     |
389  |   #[test]
389  |   #[test]
     |   ------- in this procedural macro expansion
390  | / fn test_time_options_threshold() {
391  | |     let unit = TimeThreshold::new(Duration::from_millis(50), Duration::from_millis(100));
392  | |     let integration = TimeThreshold::new(Duration::from_millis(500), Duration::from_millis(1000));
393  | |     let doc = TimeThreshold::new(Duration::from_millis(5000), Duration::from_millis(10000));
421  | |     }
422  | | }
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
    --> library/test/src/tests.rs:425:1
     |
424  |   #[test]
424  |   #[test]
     |   ------- in this procedural macro expansion
425  | / fn parse_ignored_flag() {
426  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--ignored".to_string()];
427  | |     let opts = parse_opts(&args).unwrap().unwrap();
428  | |     assert_eq!(opts.run_ignored, RunIgnored::Only);
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
    --> library/test/src/tests.rs:432:1
     |
431  |   #[test]
431  |   #[test]
     |   ------- in this procedural macro expansion
432  | / fn parse_show_output_flag() {
433  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--show-output".to_string()];
434  | |     let opts = parse_opts(&args).unwrap().unwrap();
435  | |     assert!(opts.options.display_output);
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
    --> library/test/src/tests.rs:439:1
     |
438  |   #[test]
438  |   #[test]
     |   ------- in this procedural macro expansion
439  | / fn parse_include_ignored_flag() {
440  | |     let args = vec!["progname".to_string(), "filter".to_string(), "--include-ignored".to_string()];
441  | |     let opts = parse_opts(&args).unwrap().unwrap();
442  | |     assert_eq!(opts.run_ignored, RunIgnored::Yes);
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
    --> library/test/src/tests.rs:446:1
     |
445  |   #[test]
445  |   #[test]
     |   ------- in this procedural macro expansion
446  | / pub fn filter_for_ignored_option() {
447  | |     // When we run ignored tests the test filter should filter out all the
448  | |     // unignored tests and flip the ignore flag on the rest to false
...    |
...    |
459  | |     assert!(!filtered[0].desc.ignore);
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
    --> library/test/src/tests.rs:463:1
     |
462  |   #[test]
462  |   #[test]
     |   ------- in this procedural macro expansion
463  | / pub fn run_include_ignored_option() {
464  | |     // When we "--include-ignored" tests, the ignore flag should be set to false on
466  | |
...    |
...    |
476  | |     assert!(!filtered[1].desc.ignore);
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
    --> library/test/src/tests.rs:480:1
     |
479  |   #[test]
479  |   #[test]
     |   ------- in this procedural macro expansion
480  | / pub fn exclude_should_panic_option() {
481  | |     let mut opts = TestOpts::new();
482  | |     opts.run_tests = true;
483  | |     opts.exclude_should_panic = true;
...    |
503  | |     assert!(filtered.iter().all(|test| test.desc.should_panic == ShouldPanic::No));
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
    --> library/test/src/tests.rs:507:1
     |
506  |   #[test]
506  |   #[test]
     |   ------- in this procedural macro expansion
507  | / pub fn exact_filter_match() {
508  | |     fn tests() -> Vec<TestDescAndFn> {
509  | |         vec!["base", "base::test", "base::test1", "base::test2"]
510  | |             .into_iter()
...    |
581  | |     assert_eq!(exact.len(), 2);
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
    --> library/test/src/tests.rs:620:1
     |
619  |   #[test]
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
    --> library/test/src/tests.rs:647:1
     |
646  |   #[test]
646  |   #[test]
     |   ------- in this procedural macro expansion
647  | / pub fn shuffle_tests() {
648  | |     let mut opts = TestOpts::new();
649  | |     opts.shuffle = true;
650  | |
...    |
662  | |     assert!(left.iter().zip(right).any(|(a, b)| a.1.desc.name != b.1.desc.name));
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
    --> library/test/src/tests.rs:666:1
     |
665  |   #[test]
665  |   #[test]
     |   ------- in this procedural macro expansion
666  | / pub fn shuffle_tests_with_seed() {
667  | |     let mut opts = TestOpts::new();
668  | |     opts.shuffle = true;
...    |
...    |
680  | |     assert!(left.iter().zip(right).all(|(a, b)| a.1.desc.name == b.1.desc.name));
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
    --> library/test/src/tests.rs:684:1
     |
683  |   #[test]
683  |   #[test]
     |   ------- in this procedural macro expansion
684  | / pub fn order_depends_on_more_than_seed() {
685  | |     let mut opts = TestOpts::new();
686  | |     opts.shuffle = true;
...    |
...    |
708  | |     assert!(left.iter().zip(right).any(|(a, b)| a.0 != b.0));
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
    --> library/test/src/tests.rs:712:1
     |
711  |   #[test]
711  |   #[test]
     |   ------- in this procedural macro expansion
712  | / pub fn test_metricmap_compare() {
713  | |     let mut m1 = MetricMap::new();
714  | |     let mut m2 = MetricMap::new();
715  | |     m1.insert_metric("in-both-noise", 1000.0, 200.0);
...    |
731  | |     m2.insert_metric("in-both-want-upwards-and-improved", 2000.0, -10.0);
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
    --> library/test/src/tests.rs:735:1
     |
734  |   #[test]
734  |   #[test]
     |   ------- in this procedural macro expansion
735  | / pub fn test_bench_once_no_iter() {
736  | |     fn f(_: &mut Bencher) {}
737  | |     bench::run_once(f);
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
    --> library/test/src/tests.rs:741:1
     |
740  |   #[test]
740  |   #[test]
     |   ------- in this procedural macro expansion
741  | / pub fn test_bench_once_iter() {
742  | |     fn f(b: &mut Bencher) {
743  | |         b.iter(|| {})
744  | |     }
745  | |     bench::run_once(f);
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
    --> library/test/src/tests.rs:749:1
     |
748  |   #[test]
748  |   #[test]
     |   ------- in this procedural macro expansion
749  | / pub fn test_bench_no_iter() {
750  | |     fn f(_: &mut Bencher) {}
751  | |
752  | |     let (tx, rx) = channel();
...    |
766  | |     rx.recv().unwrap();
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
    --> library/test/src/tests.rs:770:1
     |
769  |   #[test]
769  |   #[test]
     |   ------- in this procedural macro expansion
770  | / pub fn test_bench_iter() {
771  | |     fn f(b: &mut Bencher) {
772  | |         b.iter(|| {})
...    |
...    |
789  | |     rx.recv().unwrap();
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
    --> library/test/src/tests.rs:793:1
     |
792  |   #[test]
792  |   #[test]
     |   ------- in this procedural macro expansion
793  | / fn should_sort_failures_before_printing_them() {
794  | |     let test_a = TestDesc {
795  | |         name: StaticTestName("a"),
796  | |         ignore: false,
...    |
843  | |     assert!(apos < bpos);
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
For more information about this error, try `rustc --explain E0063`.
error: could not compile `test` due to 58 previous errors
warning: build failed, waiting for other jobs to finish...
