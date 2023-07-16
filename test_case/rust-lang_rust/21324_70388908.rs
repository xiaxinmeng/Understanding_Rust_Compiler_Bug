
Earls-MacBook-Pro:rust earljstsauver$ ag TEST_BENCH -C 3
mk/tests.mk
38-  TESTARGS += --ignored
39-endif
40-
41:TEST_BENCH =
42-
43-# Arguments to the cfail/rfail/rpass/bench tests
44-ifdef CFG_VALGRIND
45-  CTEST_RUNTOOL = --runtool "$(CFG_VALGRIND)"
46:  TEST_BENCH =
47-endif
48-
49-ifdef PLEASE_BENCH
50:  TEST_BENCH = --bench
51-endif
52-
53-# Arguments to the perf tests
Earls-MacBook-Pro:rust earljstsauver$ ag PLEASE_BENCH -C 3
Makefile.in
69-#
70-#   * `TESTNAME=...` - Specify the name of tests to run
71-#   * `CHECK_IGNORED=1` - Run normally-ignored tests
72:#   * `PLEASE_BENCH=1` - Run crate benchmarks (enable `--bench` flag)
73-#
74-#   * `CFG_ENABLE_VALGRIND=1` - Run tests under valgrind
75-#   * `VALGRIND_COMPILE=1` - Run the compiler itself under valgrind

mk/tests.mk
46-  TEST_BENCH =
47-endif
48-
49:ifdef PLEASE_BENCH
50-  TEST_BENCH = --bench
51-endif
52-
