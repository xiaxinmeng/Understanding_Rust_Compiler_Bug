plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.69
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/checkout/library/rustc-std-workspace-core)
error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
74 | #[cfg_attr(not(target_has_atomic_load_store = "8"), allow(dead_code))]
   |
   = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
   = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
   = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
   |
   |
87 | #[cfg_attr(not(target_has_atomic_load_store = "8"), allow(dead_code))]
   |
   = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
   = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
   = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
    |
110 | #[cfg_attr(not(target_has_atomic_load_store = "8"), allow(dead_code))]
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
137 |     #[cfg(target_has_atomic_load_store = "8")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
141 |     #[cfg(target_has_atomic_load_store = "16")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
145 |     #[cfg(target_has_atomic_load_store = "32")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
149 |     #[cfg(target_has_atomic_load_store = "64")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
153 |     #[cfg(target_has_atomic_load_store = "128")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
158 |     #[cfg(target_has_atomic_load_store = "8")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
162 |     #[cfg(target_has_atomic_load_store = "16")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
166 |     #[cfg(target_has_atomic_load_store = "32")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
170 |     #[cfg(target_has_atomic_load_store = "64")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
174 |     #[cfg(target_has_atomic_load_store = "128")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
179 |     #[cfg(target_has_atomic_load_store = "8")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
183 |     #[cfg(target_has_atomic_load_store = "16")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
187 |     #[cfg(target_has_atomic_load_store = "32")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
191 |     #[cfg(target_has_atomic_load_store = "64")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable

error[E0658]: `cfg(target_has_atomic_load_store)` is experimental and subject to change
    |
195 |     #[cfg(target_has_atomic_load_store = "128")]
    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = note: see issue #94039 <https://github.com/rust-lang/rust/issues/94039> for more information
    = help: add `#![feature(cfg_target_has_atomic_load_store)]` to the crate attributes to enable
For more information about this error, try `rustc --explain E0658`.
error: could not compile `compiler_builtins` due to 18 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
