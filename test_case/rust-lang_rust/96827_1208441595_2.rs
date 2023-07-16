rust
Gnu { lld: bool, cc: bool },
Mingw { lld: bool, cc: bool }, // split from GNU
Darwin { lld: bool, cc: bool },
WasmLld { cc: bool },
Solaris { cc: bool }, // split from Unix
L4Re { cc: bool }, // split from Unix
Unix { cc: bool }, // "other Unix" still needs to exist
Msvc { lld: bool },
EmCc,
Bpf,
Ptx,
