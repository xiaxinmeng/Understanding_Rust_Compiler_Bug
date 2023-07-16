plain
    Checking fluent-syntax v0.11.0
   Compiling zerofrom v0.1.1
    Checking tracing-subscriber v0.3.3
    Checking thorin-dwp v0.3.0
error[E0119]: conflicting implementations of trait `fmt::writer::MakeWriter<'_>` for type `std::sync::Arc<_>`
   --> /cargo/registry/src/github.com-1ecc6299db9ec823/tracing-subscriber-0.3.3/src/fmt/writer.rs:686:1
    |
674 | impl<'a, F, W> MakeWriter<'a> for F
...
...
686 | impl<'a, W> MakeWriter<'a> for Arc<W>
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `std::sync::Arc<_>`
For more information about this error, try `rustc --explain E0119`.
error: could not compile `tracing-subscriber` due to previous error
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:03:06
