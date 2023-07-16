plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: local binding shadows glob re-export
  --> /checkout/library/core/src/../../stdarch/crates/core_arch/src/aarch64/mod.rs:12:1
   |
12 | mod neon;
   | ^^^^^^^^^ but the local binding here shadows the name `neon` in the type namespace
24 | pub use super::arm_shared::*;
24 | pub use super::arm_shared::*;
   |         -------------------- the name `neon` in the type namespace is introduced by the glob reexport here
   |
   = note: `-D local-binding-shadows-glob-reexport` implied by `-D warnings`
error: could not compile `core` (lib) due to previous error
fatal error: failed to build sysroot: sysroot build failed
Build completed unsuccessfully in 0:00:19
