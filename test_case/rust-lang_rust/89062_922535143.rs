plain
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0433]: failed to resolve: use of undeclared type `StackProbeType`
  --> compiler/rustc_target/src/spec/x86_64_unknown_none.rs:14:23
   |
14 |         stack_probes: StackProbeType::Call,
   |                       ^^^^^^^^^^^^^^ use of undeclared type `StackProbeType`

error[E0433]: failed to resolve: use of undeclared type `RelroLevel`
  --> compiler/rustc_target/src/spec/x86_64_unknown_none.rs:17:22
   |
17 |         relro_level: RelroLevel::Full,
   |                      ^^^^^^^^^^ use of undeclared type `RelroLevel`
error[E0433]: failed to resolve: use of undeclared type `RelocModel`
  --> compiler/rustc_target/src/spec/x86_64_unknown_none.rs:18:27
   |
18 |         relocation_model: RelocModel::Static,
