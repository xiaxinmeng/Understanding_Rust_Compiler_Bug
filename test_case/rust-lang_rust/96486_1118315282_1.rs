json
{
  "abi": "eabi",
  "arch": "arm",
  "cpu": "arm946e-s",
  "crt-static-respected": true,
  "data-layout": "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64",
  "executables": true,
  "features": "+soft-float,+strict-align",
  "linker": "arm-none-eabi-gcc",
  "linker-flavor": "gcc",
  "llvm-target": "armv5te-none-eabi",
  "panic-strategy": "abort",
  "max-atomic-width": 0,
  "relocation-model": "static",
  "target-endian": "little",
  "target-pointer-width": "32",
  "target-c-int-width": "32",
  "disable-redzone": true,
  "linker-is-gnu": true,
  "atomic-cas": false,
  "has-rpath": false,
  "vendor": "nintendo",
  "env": "newlib",
  "dynamic-linking": false,
  "pre-link-args": {
    "gcc": [
      "-T", "../symbols/generated_NA.ld",
      "-T", "../symbols/custom_NA.ld",
      "-T", "../linker.ld"
    ]
  },
  "post-link-args": {
    "gcc": [
      "-lgcc", "-g", "-marm", "-mno-thumb-interwork",
      "-Xlinker", "-no-enum-size-warning", "-nostdlib"
    ]
  }
}
