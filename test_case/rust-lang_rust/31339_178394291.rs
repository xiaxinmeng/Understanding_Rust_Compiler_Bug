
  * check-$(crate) - Test a crate, e.g. `check-std`
  * check-ref - Run the language reference tests
  * check-docs - Test the documentation examples
  * check-stage$(stage)-$(crate) - Test a crate in a specific stage
  * check-stage$(stage)-{rpass,rfail,cfail,rmake,...} - Run tests in src/test/
  * check-stage1-T-$(target)-H-$(host) - Run cross-compiled-tests

  ...

  // Modifying libstd? Use this command to run unit tests just on your change
  make check-stage1-std NO_REBUILD=1 NO_BENCH=1

  // Added a run-pass test? Use this to test running your test
  make check-stage1-rpass TESTNAME=my-shiny-new-test

  // Having trouble figuring out which test is failing? Turn off parallel tests
  make check-stage1-std RUST_TEST_THREADS=1
