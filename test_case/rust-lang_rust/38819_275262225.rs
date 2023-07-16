
Building stage0 tool tidy (x86_64-unknown-linux-gnu)

   Compiling tidy v0.1.0 (file:///checkout/src/tools/tidy)

    Finished release [optimized] target(s) in 5.27 secs

tidy check (x86_64-unknown-linux-gnu)

duplicate error code: 579

/checkout/src/librustc/diagnostics.rs:1697: E0579: r##"

/checkout/src/librustc_passes/diagnostics.rs:227: E0579: r##"

thread 'main' panicked at 'some tidy checks failed', /checkout/src/tools/tidy/src/main.rs:55

note: Run with `RUST_BACKTRACE=1` for a backtrace.
