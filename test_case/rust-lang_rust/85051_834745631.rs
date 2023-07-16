plain
    Checking rand_chacha v0.3.0
    Checking rand v0.8.3
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking miri v0.1.0 (/checkout/src/tools/miri)
error[E0658]: or-patterns syntax is experimental
    |
    |
751 |                 MiriMemoryKind::Rust | MiriMemoryKind::C | MiriMemoryKind::WinHeap,
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
    |
760 | /                 MiriMemoryKind::Global
761 | |                 | MiriMemoryKind::Machine
762 | |                 | MiriMemoryKind::Env
762 | |                 | MiriMemoryKind::Env
763 | |                 | MiriMemoryKind::ExternStatic
764 | |                 | MiriMemoryKind::Tls,
    | |_____________________________________^
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable

error[E0658]: or-patterns syntax is experimental
   |
   |
99 | ...ported(UnsupportedOpInfo::ReadBytesAsPointer | UnsupportedOpInfo::ThreadLocalStatic(_) | UnsupportedOpInfo::ReadExternStatic(_)) =>
   |
   = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
   = help: add `#![feature(or_patterns)]` to the crate attributes to enable


error[E0658]: or-patterns syntax is experimental
    |
    |
483 |             MemoryKind::Machine(MiriMemoryKind::Global | MiriMemoryKind::ExternStatic | MiriMemoryKind::Tls | MiriMemoryKind::Env) =>
    |
    = note: see issue #54883 <https://github.com/rust-lang/rust/issues/54883> for more information
    = help: add `#![feature(or_patterns)]` to the crate attributes to enable

