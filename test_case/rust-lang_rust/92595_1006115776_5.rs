
    thread 'rustc' panicked at 'failed to lookup `SourceFile` in new context', compiler/rustc_query_impl/src/on_disk_cache.rs:514:22
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    error: internal compiler error: unexpected panic

    note: the compiler unexpectedly panicked. this is a bug.

    note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

    note: rustc 1.59.0-nightly (7d6f94817 2022-01-04) running on x86_64-unknown-linux-gnu

    note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

    note: some of the compiler flags provided by cargo are hidden

    query stack during panic:
    #0 [type_of] computing type of `stonefish::node::Node::evaluation`
    #1 [inferred_outlives_crate] computing the inferred outlives predicates for items in this crate
    end of query stack
    error: could not compile `stonefish_engine`
    