
PS C:\Users\yuriks\projects\mkos> rustc +nightly-2022-10-07 -Z unstable-options --target ..\rust-bootloader\x86_64-bootloader.json --print target-spec-json
{
  "arch": "x86_64",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float",
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "x86_64-unknown-none-gnu",
  "panic-strategy": "abort",
  "pre-link-args": {
    "ld.lld": [
      "--script=linker.ld",
      "--gc-sections",
      "--some-invalid-argument"
    ]
  },
  "relocation-model": "static",
  "target-pointer-width": "64"
}
PS C:\Users\yuriks\projects\mkos> rustc +nightly-2022-10-08 -Z unstable-options --target ..\rust-bootloader\x86_64-bootloader.json --print target-spec-json
{
  "arch": "x86_64",
  "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
  "disable-redzone": true,
  "features": "-mmx,-sse,+soft-float",
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "x86_64-unknown-none-gnu",
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-pointer-width": "64"
}
