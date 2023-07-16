
❯ cargo bisect-rustc --test-dir=foo --start=2020-01-01

installing nightly-2020-01-01
rustc for x86_64-unknown-linux-gnu: 102.98 KB / 57.99 MB  0.17 % 1019.28 KB/s 5cargo for x86_64-unknown-linux-gnu: 4.80 MB / 4.80 MB [===] 100.00 % 4.47 MB/s testing...
RESULT: nightly-2020-01-01, ===> Yes
uninstalling nightly-2020-01-01

ERROR: the start of the range (nightly-2020-01-01) must not reproduce the regression
