
[01:36:33] error[E0432]: unresolved import `test::ColorConfig`
[01:36:33]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.19/src/common.rs:20:5
[01:36:33]    |
[01:36:33] 20 | use test::ColorConfig;
[01:36:33]    |     ^^^^^^^^^^^^^^^^^ no `ColorConfig` in the root
[01:36:33] 
[01:36:33] error[E0433]: failed to resolve: could not find `OutputFormat` in `test`
[01:36:33]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.19/src/lib.rs:104:41
[01:36:33]     |
[01:36:33] 104 |         format: if config.quiet { test::OutputFormat::Terse } else { test::OutputFormat::Pretty },
[01:36:33]     |                                         ^^^^^^^^^^^^ could not find `OutputFormat` in `test`
[01:36:33] 
[01:36:33] error[E0433]: failed to resolve: could not find `OutputFormat` in `test`
[01:36:33]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.19/src/lib.rs:104:76
[01:36:33]     |
[01:36:33] 104 |         format: if config.quiet { test::OutputFormat::Terse } else { test::OutputFormat::Pretty },
[01:36:33]     |                                                                            ^^^^^^^^^^^^ could not find `OutputFormat` in `test`
[01:36:33] 
[01:36:33] error[E0425]: cannot find function `run_tests_console` in module `test`
[01:36:33]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.19/src/lib.rs:87:21
[01:36:33]    |
[01:36:33] 87 |     let res = test::run_tests_console(&opts, tests.into_iter().collect());
[01:36:33]    |                     ^^^^^^^^^^^^^^^^^ not found in `test`
[01:36:33] 
[01:36:33] error[E0425]: cannot find value `AutoColor` in module `test`
[01:36:33]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.19/src/lib.rs:112:22
[01:36:33]     |
[01:36:33] 112 |         color: test::AutoColor,
[01:36:33]     |                      ^^^^^^^^^ not found in `test`
[01:36:33] 
[01:36:33] error[E0412]: cannot find type `TestFn` in module `test`
[01:36:33]    --> /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.19/src/lib.rs:274:75
[01:36:33]     |
[01:36:33] 274 | pub fn make_test_closure(config: &Config, testpaths: &TestPaths) -> test::TestFn {
[01:36:33]     |                                                                           ^^^^^^ not found in `test`
