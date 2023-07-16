plain
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
    Checking chalk-engine v0.75.0
error: expected type, found `"+strict-align,+neon,+fp-armv8"`
 --> compiler/rustc_target/src/spec/aarch64_unknown_hermit.rs:6:20
  |
6 |     base.features: "+strict-align,+neon,+fp-armv8".to_string();
  |                  - ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected type
  |                  |
  |                  tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

    Checking gsgdt v0.1.2
    Checking rls-span v0.5.3
    Checking rls-data v0.19.1
