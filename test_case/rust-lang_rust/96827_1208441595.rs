rust
// Unix-like linker with GNU extensions (both naked and compiler-wrapped forms)
// Besides similar "default" Linux/BSD linkers this also include Windows/GNU, which is somewhat different.
Gnu { cc: bool },
// Basic Unix-like linker, possibly with non-GNU extensions aka "any other Unix" (both naked and compiler-wrapped forms)
// Apple targets, Wasm, Solaris/illumos, l4re, msp430 - very different targets.
// Why this exists separately from GNU - the `linker_is_gnu` target property (which I will include into linker flavor in this post)
// is annoying to infer using other target properties.
Unix { cc: bool },
// MSVC linker has a unique interface
Msvc,
// Other linker-like tools with unique interfaces for exotic targets
EmCc,
Bpf,
Ptx,
