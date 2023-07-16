
$ rustc -Z unstable-options --target x86_64-apple-darwin --print target-spec-json
{
  "abi-return-struct-as-int": true,
  "arch": "x86_64",
  "archive-format": "bsd",
  "cpu": "core2",
  "data-layout": "e-m:o-i64:64-f80:128-n8:16:32:64-S128",
  "dll-suffix": ".dylib",
  "dynamic-linking": true,
  "eliminate-frame-pointer": false,
  "emit-debug-gdb-scripts": false,
  "env": "",
  "exe-allocation-crate": "alloc_jemalloc",
  "executables": true,
  "function-sections": false,
  "has-elf-tls": true,
  "has-rpath": true,
  "is-builtin": true,
  "is-like-osx": true,
  "linker-flavor": "gcc",
  "llvm-target": "x86_64-apple-darwin",
  "max-atomic-width": 128,
  "os": "macos",
  "pre-link-args": {
    "gcc": [
      "-m64"
    ]
  },
  "stack-probes": true,
  "target-c-int-width": "32",
  "target-endian": "little",
  "target-family": "unix",
  "target-pointer-width": "64",
  "vendor": "apple"
}
