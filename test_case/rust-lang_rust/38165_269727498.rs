
thread 'main' panicked at 'explicit panic', t.rs:2
Some details are omitted; run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
    t::test @ ./t.rs:2
    t::main::{{closure}} @ ./t.rs:6
    <core::result::Result<T, E>>::map @ std libcore/result.rs:465
    t::main @ ./t.rs:6
