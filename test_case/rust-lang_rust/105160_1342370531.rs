
running 76 tests
test [run-make] src/test\run-make\coverage-reports ... FAILED
<...many successful and ignored test result lines omitted...>

failures:

---- [run-make] src/test\run-make\coverage-reports stdout ----

error: make failed
status: exit code: 2
command: "make"
--- stdout -------------------------------
make[1]: Entering directory '/d/a/rust/rust/src/test/run-make/coverage-reports'
make[1]: Leaving directory '/d/a/rust/rust/src/test/run-make/coverage-reports'
------------------------------------------
--- stderr -------------------------------
Makefile:83: clear_expected_if_blessed: No such file or directory
------------------------------------------

failures:
    [run-make] src/test\run-make\coverage-reports
