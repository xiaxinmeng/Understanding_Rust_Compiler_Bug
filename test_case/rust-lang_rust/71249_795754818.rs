
$ cargo check --future-incompat-report
error: the `--future-incompat-report` flag is unstable, pass `-Z unstable-options` to enable it
$ cargo check --future-incompat-report -Z unstable-options
error: Usage of `--future-incompat-report` requires `-Z future-incompat-report`flag.
$ cargo check --future-incompat-report -Z unstable-options -Z future-incompat-report // this works
