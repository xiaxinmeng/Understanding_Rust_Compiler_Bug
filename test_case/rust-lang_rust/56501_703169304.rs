
↳cargo build
   Compiling clap v2.33.3
   Compiling syn v1.0.42
error: internal compiler error: src/librustc_traits/normalize_erasing_regions.rs:37:32: could not fully normalize `for<'r> fn(buffer::Cursor<'r>) -> bool {<<fn(lookahead::TokenMarker) -> token::If {token::If::<lookahead::TokenMarker>} as lookahead::Peek>::Token as token::Token>::peek}`

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:916:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0 (04488afe3 2020-08-24) running on armv7-unknown-linux-gnueabihf

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: aborting due to previous error

error: could not compile `syn`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
