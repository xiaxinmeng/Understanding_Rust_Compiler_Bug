diff
2,3d1
<   "abi-return-struct-as-int": true,
<   "allows-weak-linkage": false,
5d2
<   "code-model": "large",
7c4
<   "disable-redzone": true,
---
>   "default-hidden-visibility": true,
11,12d7
<   "is-builtin": true,
<   "is-like-msvc": true,
16,19c11
<   "linker-is-gnu": false,
<   "lld-flavor": "link",
<   "llvm-target": "aarch64-unknown-windows",
<   "max-atomic-width": 64,
---
>   "llvm-target": "aarch64-pc-windows-msvc",
24c16
<       "/NOLOGO",
---
>       "/subsystem:EFI_Application",
26,32d17
<       "/subsystem:efi_application",
<       "/machine:arm64"
<     ],
<     "msvc": [
<       "/NOLOGO",
<       "/entry:efi_main",
<       "/subsystem:efi_application",
36,40c21,23
<   "singlethread": true,
<   "split-debuginfo": "packed",
<   "stack-probes": {
<     "kind": "call"
<   },
---
>   "stack_probes": true,
>   "target-c-int-width": "32",
>   "target-endian": "little",
42c25
< }
\ No newline at end of file
---
> }
