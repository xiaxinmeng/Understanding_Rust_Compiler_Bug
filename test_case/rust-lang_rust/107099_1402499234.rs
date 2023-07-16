console
---- buck-out/v2/gen/fbcode/882bd00a4ffb1e1d/tracing/stats/__tracing_stats__/__srcs/src/lib.rs - StatsLayer (line 70) stdout ----
error[E0433]: failed to resolve: use of undeclared type `StatsLayer`
 --> buck-out/v2/gen/fbcode/882bd00a4ffb1e1d/tracing/stats/__tracing_stats__/__srcs/src/lib.rs:71:13
  |
3 | let stats = StatsLayer::new("myservice", 8, false)
  |             ^^^^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
2 | use tracing_stats::StatsLayer;
  |
