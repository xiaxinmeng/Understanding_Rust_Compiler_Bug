
  25: <rustc_span[a35dddb0941cd0b0]::source_map::SourceMap>::find_width_of_character_at_span
          at rust/compiler/rustc_span/src/source_map.rs:1025:14
  26: <rustc_span[a35dddb0941cd0b0]::source_map::SourceMap>::end_point
          at rust/compiler/rustc_span/src/source_map.rs:931:21
  27: <rustc_mir_build[ddb99fb8276dd8b7]::build::Builder>::schedule_drop
          at rust/compiler/rustc_mir_build/src/build/scope.rs:946:33
  28: <rustc_mir_build[ddb99fb8276dd8b7]::build::Builder>::as_temp_inner
          at rust/compiler/rustc_mir_build/src/build/expr/as_temp.rs:103:21
  29: <rustc_mir_build[ddb99fb8276dd8b7]::build::Builder>::as_temp::{closure#0}
          at rust/compiler/rustc_mir_build/src/build/expr/as_temp.rs:23:36
  30: stacker[7ec4398987cef261]::maybe_grow::<rustc_mir_build[ddb99fb8276dd8b7]::build::BlockAnd<rustc_middle[3c153db6115e4e4e]::mir::Local>, <rustc_mir_build[ddb99f
b8276dd8b7]::build::Builder>::as_temp::{closure#0}>
          at .cargo/registry/src/index.crates.io-6f17d22bba15001f/stacker-0.1.15/src/lib.rs:55:9
  31: rustc_data_structures[169f270e33b59d29]::stack::ensure_sufficient_stack::<rustc_mir_build[ddb99fb8276dd8b7]::build::BlockAnd<rustc_middle[3c153db6115e4e4e]::mi
r::Local>, <rustc_mir_build[ddb99fb8276dd8b7]::build::Builder>::as_temp::{closure#0}>
          at rust/compiler/rustc_data_structures/src/stack.rs:17:5
  32: <rustc_mir_build[ddb99fb8276dd8b7]::build::Builder>::as_temp
          at rust/compiler/rustc_mir_build/src/build/expr/as_temp.rs:23:9
