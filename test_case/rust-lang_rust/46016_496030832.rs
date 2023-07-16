
$ cargo +nightly rustc -- -Zunpretty=hir-tree | head > /dev/null 
  Compiling project v0.1.0 (/home/joshua/Documents/Programming/rust/project)
thread 'main' panicked at 'failed printing to stdout: Broken pipe (os error 32)', src/libstd/io/stdio.rs:792:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
