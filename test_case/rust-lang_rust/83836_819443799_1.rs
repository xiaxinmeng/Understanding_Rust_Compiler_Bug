
error: internal compiler error: compiler/rustc_metadata/src/rmeta/decoder.rs:1233:18: cannot get associated-item of `DefKey { parent: Some(DefIndex(38829)), disambiguated_data: DisambiguatedDefPathData { data: ValueNs("iter"), disambiguator: 0 } }`

thread 'rustc' panicked at 'Box<Any>', /rustc/132b4e5d167b7e622fcc11fa2b67b931105b4de1/library/std/src/panic.rs:59:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (132b4e5d1 2021-04-13) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

query stack during panic:
#0 [associated_item] computing associated item data for `std::iter::Map::iter`
#1 [associated_items] collecting associated items of std::iter::Map
end of query stack
error: aborting due to previous error
