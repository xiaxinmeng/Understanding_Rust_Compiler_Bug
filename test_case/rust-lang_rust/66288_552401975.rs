plain
2019-11-11T11:15:48.8154328Z [RUSTC-TIMING] rustc_errors test:false 19.246
2019-11-11T11:15:48.8207518Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-11T11:15:50.6440649Z [RUSTC-TIMING] fmt_macros test:false 1.815
2019-11-11T11:16:04.9985484Z [RUSTC-TIMING] rustc_target test:false 29.791
2019-11-11T11:16:33.0968264Z error[E0599]: no method named `record_instant_event` found for type `measureme::profiler::Profiler<measureme::mmap_serialization_sink::MmapSerializationSink>` in the current scope
2019-11-11T11:16:33.0969380Z    --> src/librustc/util/profiling.rs:248:31
2019-11-11T11:16:33.0970039Z     |
2019-11-11T11:16:33.0970995Z 248 |             profiler.profiler.record_instant_event(
2019-11-11T11:16:33.0972299Z     |                               ^^^^^^^^^^^^^^^^^^^^ method not found in `measureme::profiler::Profiler<measureme::mmap_serialization_sink::MmapSerializationSink>`
2019-11-11T11:16:33.4497012Z error: aborting due to previous error
2019-11-11T11:16:33.4497958Z 
2019-11-11T11:16:33.4498524Z For more information about this error, try `rustc --explain E0599`.
2019-11-11T11:16:33.6446680Z [RUSTC-TIMING] rustc test:false 28.631
---
2019-11-11T11:16:38.3853914Z   local time: Mon Nov 11 11:16:38 UTC 2019
2019-11-11T11:16:38.7216249Z   network time: Mon, 11 Nov 2019 11:16:38 GMT
2019-11-11T11:16:38.7222281Z == end clock drift check ==
2019-11-11T11:16:39.5163584Z 
2019-11-11T11:16:39.5276954Z ##[error]Bash exited with code '1'.
2019-11-11T11:16:39.5332534Z ##[section]Starting: Checkout
2019-11-11T11:16:39.5334510Z ==============================================================================
2019-11-11T11:16:39.5334605Z Task         : Get sources
2019-11-11T11:16:39.5334711Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
