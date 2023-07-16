
---- [ui] tests/rustdoc-ui/ice-bug-report-url.rs stdout ----
diff of stderr:

5          |          ^ expected one of `->`, `where`, or `{`
6
7       thread panicked at 'aborting due to `-Z treat-err-as-bug`'
-       stack backtrace:
-       error: the compiler unexpectedly panicked. this is a bug.
+       stack backtrace:                             error: the compiler unexpectedly panicked. this is a bug.
10
11      note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-rustdoc&template=ice.md
12


The actual stderr differed from the expected stderr.
