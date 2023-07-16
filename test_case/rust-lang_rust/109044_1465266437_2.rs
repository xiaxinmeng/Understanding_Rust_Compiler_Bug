rs
$ cargo +stable test -- --report-time
error: The "report-time" flag is only accepted on the nightly compiler with -Z unstable-options

$ cargo +stable test -- --format json
error: The "json" format is only accepted on the nightly compiler
