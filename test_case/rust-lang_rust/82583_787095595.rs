bash
$ cargo bisect-rustc --test-dir=. --start=20xx-01-01
...
ERROR: the start of the range (nightly-20xx-01-01) must not reproduce the regression
