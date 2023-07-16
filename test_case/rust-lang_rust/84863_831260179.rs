plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking ppv-lite86 v0.2.8
    Checking term v0.0.0 (/checkout/library/term)
    Checking proc_macro v0.0.0 (/checkout/library/proc_macro)
    Checking test v0.0.0 (/checkout/library/test)
error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/proc_macro/tests/test.rs:6:1
5    |   #[test]
     |   ------- in this macro invocation
6    | / fn test_line_column_ord() {
6    | / fn test_line_column_ord() {
7    | |     let line0_column0 = LineColumn { line: 0, column: 0 };
8    | |     let line0_column1 = LineColumn { line: 0, column: 1 };
9    | |     let line1_column0 = LineColumn { line: 1, column: 0 };
10   | |     assert!(line0_column0 < line0_column1);
11   | |     assert!(line0_column1 < line1_column0);
12   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/proc_macro/tests/test.rs:15:1
14   |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
15   | / fn test_punct_eq() {
16   | |     // Good enough if it typechecks, since proc_macro::Punct can't exist in a test.
17   | |     fn _check(punct: Punct) {
18   | |         let _ = punct == ':';
20   | | }
20   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0063`.
error: could not compile `proc_macro`
error: could not compile `proc_macro`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/searcher/tests.rs:5:1
3    |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
4    |   #[ignore = "buildbots don't have ncurses installed and I can't mock everything I need"]
5    | / fn test_get_dbpath_for_term() {
6    | |     // woefully inadequate test coverage
7    | |     // note: current tests won't work with non-standard terminfo hierarchies (e.g., macOS's)
8    | |     use std::env;
...    |
18   | |     env::remove_var("TERMINFO_DIRS");
19   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parser/compiled/tests.rs:4:1
3    |   #[test]
     |   ------- in this macro invocation
4    | / fn test_veclens() {
4    | / fn test_veclens() {
5    | |     assert_eq!(boolfnames.len(), boolnames.len());
6    | |     assert_eq!(numfnames.len(), numnames.len());
7    | |     assert_eq!(stringfnames.len(), stringnames.len());
8    | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:6:1
5    |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
6    | / fn test_basic_setabf() {
7    | |     let s = b"\\E[48;5;%p1%dm";
8    | |     assert_eq!(
9    | |         expand(s, &[Number(1)], &mut Variables::new()).unwrap(),
10   | |         "\\E[48;5;1m".bytes().collect::<Vec<_>>()
12   | | }
12   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:15:1
14   |   #[test]
     |   ------- in this macro invocation
15   | / fn test_multiple_int_constants() {
16   | |     assert_eq!(
16   | |     assert_eq!(
17   | |         expand(b"%{1}%{2}%d%d", &[], &mut Variables::new()).unwrap(),
18   | |         "21".bytes().collect::<Vec<_>>()
20   | | }
20   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:23:1
22   |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
23   | / fn test_op_i() {
24   | |     let mut vars = Variables::new();
25   | |     assert_eq!(
26   | |         expand(b"%p1%d%p2%d%p3%d%i%p1%d%p2%d%p3%d", &[Number(1), Number(2), Number(3)], &mut vars),
32   | |     );
33   | | }
33   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:36:1
35   |   #[test]
     |   ------- in this macro invocation
36   | / fn test_param_stack_failure_conditions() {
36   | / fn test_param_stack_failure_conditions() {
37   | |     let mut varstruct = Variables::new();
38   | |     let vars = &mut varstruct;
...    |
66   | |     }
67   | | }
67   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:70:1
69   |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
70   | / fn test_push_bad_param() {
71   | |     assert!(expand(b"%pa", &[], &mut Variables::new()).is_err());
72   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:75:1
74   |   #[test]
     |   ------- in this macro invocation
75   | / fn test_comparison_ops() {
75   | / fn test_comparison_ops() {
76   | |     let v = [('<', [1u8, 0u8, 0u8]), ('=', [0u8, 1u8, 0u8]), ('>', [0u8, 0u8, 1u8])];
77   | |     for &(op, bs) in v.iter() {
78   | |         let s = format!("%{{1}}%{{2}}%{}%d", op);
90   | |     }
91   | | }
91   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:94:1
93   |   #[test]
     |   ------- in this macro invocation
94   | / fn test_conditionals() {
95   | |     let mut vars = Variables::new();
95   | |     let mut vars = Variables::new();
96   | |     let s = b"\\E[%?%p1%{8}%<%t3%p1%d%e%p1%{16}%<%t9%p1%{8}%-%d%e38;5;%p1%d%;m";
97   | |     let res = expand(s, &[Number(1)], &mut vars);
...    |
105  | |     assert_eq!(res.unwrap(), "\\E[38;5;42m".bytes().collect::<Vec<_>>());
106  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `TestDesc`
    --> library/term/src/terminfo/parm/tests.rs:109:1
108  |   #[test]
     |   ------- in this macro invocation
109  | / fn test_format() {
109  | / fn test_format() {
110  | |     let mut varstruct = Variables::new();
111  | |     let vars = &mut varstruct;
...    |
137  | |     );
138  | | }
138  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:43:1
42   |   #[test]
     |   ------- in this macro invocation
43   | / fn test_min_max_nan() {
43   | / fn test_min_max_nan() {
44   | |     let xs = &[1.0, 2.0, f64::NAN, 3.0, 4.0];
45   | |     let summary = Summary::new(xs);
46   | |     assert_eq!(summary.min, 1.0);
47   | |     assert_eq!(summary.max, 4.0);
48   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:51:1
50   |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
51   | / fn test_norm2() {
52   | |     let val = &[958.0000000000, 924.0000000000];
53   | |     let summ = &Summary {
54   | |         sum: 1882.0000000000,
67   | |     check(val, summ);
68   | | }
68   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:70:1
69   |   #[test]
     |   ------- in this macro invocation
70   | / fn test_norm10narrow() {
71   | |     let val = &[
71   | |     let val = &[
72   | |         966.0000000000,
73   | |         985.0000000000,
...    |
97   | |     check(val, summ);
98   | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:100:1
99   |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
100  | / fn test_norm10medium() {
101  | |     let val = &[
102  | |         954.0000000000,
103  | |         1064.0000000000,
127  | |     check(val, summ);
128  | | }
128  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:130:1
129  |   #[test]
     |   ------- in this macro invocation
130  | / fn test_norm10wide() {
131  | |     let val = &[
131  | |     let val = &[
132  | |         505.0000000000,
133  | |         497.0000000000,
...    |
157  | |     check(val, summ);
158  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:160:1
159  |   #[test]
     |   ------- in this macro invocation
160  | / fn test_norm25verynarrow() {
161  | |     let val = &[
161  | |     let val = &[
162  | |         991.0000000000,
163  | |         1018.0000000000,
...    |
202  | |     check(val, summ);
203  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:205:1
204  |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
205  | / fn test_exp10a() {
206  | |     let val = &[
207  | |         23.0000000000,
208  | |         11.0000000000,
232  | |     check(val, summ);
233  | | }
233  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:235:1
234  |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
235  | / fn test_exp10b() {
236  | |     let val = &[
237  | |         24.0000000000,
238  | |         17.0000000000,
262  | |     check(val, summ);
263  | | }
263  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:265:1
264  |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
265  | / fn test_exp10c() {
266  | |     let val = &[
267  | |         71.0000000000,
268  | |         2.0000000000,
292  | |     check(val, summ);
293  | | }
293  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:295:1
294  |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
295  | / fn test_exp25() {
296  | |     let val = &[
297  | |         3.0000000000,
298  | |         24.0000000000,
337  | |     check(val, summ);
338  | | }
338  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:340:1
339  |   #[test]
     |   ------- in this macro invocation
     |   ------- in this macro invocation
340  | / fn test_binom25() {
341  | |     let val = &[
342  | |         18.0000000000,
343  | |         17.0000000000,
382  | |     check(val, summ);
383  | | }
383  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
     |
     |
1404 | /     pub macro test($item:item) {
1406 | |     }
1406 | |     }
     | |_____- in this expansion of `#[test]`

error[E0063]: missing fields `compile_fail` and `no_run` in initializer of `tests::test::TestDesc`
    --> library/test/src/stats/tests.rs:385:1
384  |   #[test]
     |   ------- in this macro invocation
385  | / fn test_pois25lambda30() {
386  | |     let val = &[
386  | |     let val = &[
387  | |         27.0000000000,
388  | |         33.0000000000,
...    |
427  | |     check(val, summ);
428  | | }
     | |_^ missing `compile_fail` and `no_run`
    ::: /checkout/library/core/src/macros/mod.rs:1404:5
