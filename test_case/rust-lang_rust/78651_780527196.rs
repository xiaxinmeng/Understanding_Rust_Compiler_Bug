bash
$ cargo bisect-rustc --test-dir=. --start=2020-01-01 --regress=ice
...
ERROR: the end of the range (nightly-2021-02-17) does not reproduce the regression
