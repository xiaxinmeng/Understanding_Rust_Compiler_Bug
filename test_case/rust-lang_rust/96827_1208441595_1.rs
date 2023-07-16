rust
// Unix-like linker with GNU extensions (both naked and compiler-wrapped forms)
// Besides similar "default" Linux/BSD linkers this also include Windows/GNU, which is somewhat different.
Gnu { lld: bool, cc: bool },
// Unix-like linker for Apple targets (both naked and compiler-wrapped forms)
// Why it was extracted from "umbrella" Unix - due to the corresponding LLD flavor.
Darwin { lld: bool, cc: bool },
// Unix-like linker for Wasm targets (both naked and compiler-wrapped forms)
// Why it was extracted from "umbrella" Unix - due to the corresponding LLD flavor.
// Non-LLD version does not exist, so the lld flag here is currently hardcoded and not boolean.
WasmLld { cc: bool },
// Basic Unix-like linker, possibly with non-GNU extensions aka "any other Unix" (both naked and compiler-wrapped forms)
// Solaris/illumos, l4re, msp430 - very different targets.
// LLD doesn't support any of these.
Unix { cc: bool },
// MSVC linker has a unique interface
// LLD supports it
Msvc { lld: bool },
// Other linker-like tools with unique interfaces for exotic targets
EmCc,
Bpf,
Ptx,
