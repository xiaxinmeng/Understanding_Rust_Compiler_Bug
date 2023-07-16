
% cat ../issue-55608.rs
#![feature(conservative_impl_trait)]

fn server() -> impl FilterBase2 {
    segment2(|| { loop { } }).map2(|| "")
}

trait FilterBase2 {
    fn map2<F>(self, _fn: F) -> Map2<F> where Self: Sized { loop { } }
}

struct Map2<F> { _func: F }

impl<F> FilterBase2 for Map2<F> { }

fn segment2<F>(_fn: F) -> Map2<F> where F: Fn() -> Result<(), ()> {
    loop { }
}

fn main() { server(); }
% rustup default nightly-2018-03-21-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2018-03-21-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2018-03-21-x86_64-unknown-linux-gnu'

  nightly-2018-03-21-x86_64-unknown-linux-gnu unchanged - rustc 1.26.0-nightly (75af15ee6 2018-03-20)

% rustc ../issue-55608.rs
% rustup default nightly-2018-03-22-x86_64-unknown-linux-gnu
info: syncing channel updates for 'nightly-2018-03-22-x86_64-unknown-linux-gnu'
error: no release found for 'nightly-2018-03-22'
% rustup default nightly-2018-03-23-x86_64-unknown-linux-gnu
info: syncing channel updates for 'nightly-2018-03-23-x86_64-unknown-linux-gnu'
error: no release found for 'nightly-2018-03-23'
% rustup default nightly-2018-03-24-x86_64-unknown-linux-gnu
info: using existing install for 'nightly-2018-03-24-x86_64-unknown-linux-gnu'
info: default toolchain set to 'nightly-2018-03-24-x86_64-unknown-linux-gnu'

  nightly-2018-03-24-x86_64-unknown-linux-gnu unchanged - rustc 1.26.0-nightly (c08480fce 2018-03-23)

% rustc ../issue-55608.rs
error: internal compiler error: librustc/infer/error_reporting/mod.rs:184: impossible case reached

thread 'rustc' panicked at 'Box<Any>', librustc_errors/lib.rs:543:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (c08480fce 2018-03-23) running on x86_64-unknown-linux-gnu
%
