
{
  "arch": "powerpc",
  "crt-static-respected": true,
  "data-layout": "E-m:e-p:32:32-i64:64-n32",
  "dynamic-linking": false,
  "env": "",
  "executables": true,
  "has-thread-local": false,
  "is-builtin": false,
  "llvm-target": "powerpc-unknown-none-elf",
  "max-atomic-width": 32,
  "os": "none",
  "position-independent-executables": true,
  "linker-is-gnu": true,
  "linker": "ppc64le-linux-gnu-gcc",
  "pre-link-args": {
    "gcc": [
      "-m32",
      "-ffreestanding",
      "-static",
      "-nostdlib",
      "-fPIC",
      "-Ttext",
      "100000",
      "-mbig-endian"
    ]
  },
  "features": "+crt-static",
  "relro-level": "full",
  "target-endian": "big",
  "target-family": [
    "unix"
  ],
  "target-mcount": "_mcount",
  "target-pointer-width": "32",
  "default-linker-libraries": "no"
}
