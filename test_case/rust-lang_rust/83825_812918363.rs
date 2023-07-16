
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /Users/user/rust/compiler/rustc_middle/src/ty/sty.rs:968:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0

query stack during panic:
thread 'rustc' panicked at 'substs of instance DefId(0:4 ~ issue_64494[317d]::Foo::VAL) not normalized for codegen: [^0]', compiler/rustc_middle/src/ty/instance.rs:285:9
