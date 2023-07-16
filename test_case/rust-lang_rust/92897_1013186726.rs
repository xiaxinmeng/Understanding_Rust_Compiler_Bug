
riscv64-unknown-elf-readelf.exe -a .\compiler_builtins-6af234694b8e479b.compiler_builtins.3786cd3d-cgu.96.rcgu.o | rg __atomic
0000009c  00017913 R_RISCV_CALL_PLT  00000000   __atomic_load_4 + 0
000000b4  00017913 R_RISCV_CALL_PLT  00000000   __atomic_load_4 + 0
00000158  00017913 R_RISCV_CALL_PLT  00000000   __atomic_load_4 + 0
   377: 00000000     0 NOTYPE  GLOBAL DEFAULT  UND __atomic_load_4
