plain
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
error[E0433]: failed to resolve: use of undeclared type `PanicStrategy`
  --> compiler/rustc_target/src/spec/thumbv4t_none_eabi.rs:50:29
   |
50 |             panic_strategy: PanicStrategy::Abort,
   |                             ^^^^^^^^^^^^^ use of undeclared type `PanicStrategy`
error[E0433]: failed to resolve: use of undeclared type `RelocModel`
  --> compiler/rustc_target/src/spec/thumbv4t_none_eabi.rs:51:31
   |
51 |             relocation_model: RelocModel::Static,
