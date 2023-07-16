plain
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error: unused import: `PanicStrategy`
 --> compiler/rustc_target/src/spec/armv6k_nintendo_3ds.rs:1:43
  |
1 | use crate::spec::{LinkArgs, LinkerFlavor, PanicStrategy, RelocModel, Target, TargetOptions};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking gsgdt v0.1.2
    Checking tracing-serde v0.1.2
    Checking rls-span v0.5.3
error: could not compile `rustc_target` due to previous error
