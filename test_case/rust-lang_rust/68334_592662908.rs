
Assembling stage2 compiler (x86_64-unknown-linux-gnu)
Building stage2 std artifacts (x86_64-unknown-linux-gnu -> aarch64-unknown-none)
   Compiling cc v1.0.50
   Compiling core v0.0.0 (/checkout/src/libcore)
   Compiling libc v0.2.66
   Compiling autocfg v0.1.7
   Compiling std v0.0.0 (/checkout/src/libstd)
[RUSTC-TIMING] build_script_build test:false 0.323
[RUSTC-TIMING] build_script_build test:false 0.522
[RUSTC-TIMING] autocfg test:false 1.023
   Compiling hashbrown v0.6.2
[RUSTC-TIMING] build_script_build test:false 0.200
[RUSTC-TIMING] cc test:false 4.720
   Compiling compiler_builtins v0.1.25 (/checkout/src/compiler-builtins)
   Compiling backtrace-sys v0.1.32
   Compiling unwind v0.0.0 (/checkout/src/libunwind)
[RUSTC-TIMING] build_script_build test:false 0.648
[RUSTC-TIMING] build_script_build test:false 0.696
[RUSTC-TIMING] build_script_build test:false 1.357
error: failed to run custom build command for `backtrace-sys v0.1.32`

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/backtrace-sys-a122afda2a01190f/build-script-build` (exit code: 1)
--- stdout
cargo:rustc-cfg=rdos
TARGET = Some("aarch64-unknown-none")
OPT_LEVEL = Some("3")
HOST = Some("x86_64-unknown-linux-gnu")
CC_aarch64-unknown-none = Some("sccache aarch64-none-elf-gcc")
CFLAGS_aarch64-unknown-none = Some("-ffunction-sections -fdata-sections -fPIC -mstrict-align")
CRATE_CC_NO_DEFAULTS = None
CARGO_CFG_TARGET_FEATURE = Some("fp,neon")
running: "sccache" "aarch64-none-elf-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mstrict-align" "-I" "src/libbacktrace" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-none/release/build/backtrace-sys-022de964ab3bf907/out" "-fvisibility=hidden" "-DBACKTRACE_ELF_SIZE=64" "-DBACKTRACE_SUPPORTED=1" "-DBACKTRACE_USES_MALLOC=1" "-DBACKTRACE_SUPPORTS_THREADS=0" "-DBACKTRACE_SUPPORTS_DATA=0" "-DHAVE_DL_ITERATE_PHDR=1" "-D_GNU_SOURCE=1" "-D_LARGE_FILES=1" "-Dbacktrace_full=__rdos_backtrace_full" "-Dbacktrace_dwarf_add=__rdos_backtrace_dwarf_add" "-Dbacktrace_initialize=__rdos_backtrace_initialize" "-Dbacktrace_pcinfo=__rdos_backtrace_pcinfo" "-Dbacktrace_syminfo=__rdos_backtrace_syminfo" "-Dbacktrace_get_view=__rdos_backtrace_get_view" "-Dbacktrace_release_view=__rdos_backtrace_release_view" "-Dbacktrace_alloc=__rdos_backtrace_alloc" "-Dbacktrace_free=__rdos_backtrace_free" "-Dbacktrace_vector_finish=__rdos_backtrace_vector_finish" "-Dbacktrace_vector_grow=__rdos_backtrace_vector_grow" "-Dbacktrace_vector_release=__rdos_backtrace_vector_release" "-Dbacktrace_close=__rdos_backtrace_close" "-Dbacktrace_open=__rdos_backtrace_open" "-Dbacktrace_print=__rdos_backtrace_print" "-Dbacktrace_simple=__rdos_backtrace_simple" "-Dbacktrace_qsort=__rdos_backtrace_qsort" "-Dbacktrace_create_state=__rdos_backtrace_create_state" "-Dbacktrace_uncompress_zdebug=__rdos_backtrace_uncompress_zdebug" "-Dmacho_get_view=__rdos_macho_get_view" "-Dmacho_symbol_type_relevant=__rdos_macho_symbol_type_relevant" "-Dmacho_get_commands=__rdos_macho_get_commands" "-Dmacho_try_dsym=__rdos_macho_try_dsym" "-Dmacho_try_dwarf=__rdos_macho_try_dwarf" "-Dmacho_get_addr_range=__rdos_macho_get_addr_range" "-Dmacho_get_uuid=__rdos_macho_get_uuid" "-Dmacho_add=__rdos_macho_add" "-Dmacho_add_symtab=__rdos_macho_add_symtab" "-Dmacho_file_to_host_u64=__rdos_macho_file_to_host_u64" "-Dmacho_file_to_host_u32=__rdos_macho_file_to_host_u32" "-Dmacho_file_to_host_u16=__rdos_macho_file_to_host_u16" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-none/release/build/backtrace-sys-022de964ab3bf907/out/src/libbacktrace/alloc.o" "-c" "src/libbacktrace/alloc.c"
exit code: 0

###### Many more omitted here

running: "sccache" "aarch64-none-elf-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mstrict-align" "-I" "src/libbacktrace" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-none/release/build/backtrace-sys-022de964ab3bf907/out" "-fvisibility=hidden" "-DBACKTRACE_ELF_SIZE=64" "-DBACKTRACE_SUPPORTED=1" "-DBACKTRACE_USES_MALLOC=1" "-DBACKTRACE_SUPPORTS_THREADS=0" "-DBACKTRACE_SUPPORTS_DATA=0" "-DHAVE_DL_ITERATE_PHDR=1" "-D_GNU_SOURCE=1" "-D_LARGE_FILES=1" "-Dbacktrace_full=__rdos_backtrace_full" "-Dbacktrace_dwarf_add=__rdos_backtrace_dwarf_add" "-Dbacktrace_initialize=__rdos_backtrace_initialize" "-Dbacktrace_pcinfo=__rdos_backtrace_pcinfo" "-Dbacktrace_syminfo=__rdos_backtrace_syminfo" "-Dbacktrace_get_view=__rdos_backtrace_get_view" "-Dbacktrace_release_view=__rdos_backtrace_release_view" "-Dbacktrace_alloc=__rdos_backtrace_alloc" "-Dbacktrace_free=__rdos_backtrace_free" "-Dbacktrace_vector_finish=__rdos_backtrace_vector_finish" "-Dbacktrace_vector_grow=__rdos_backtrace_vector_grow" "-Dbacktrace_vector_release=__rdos_backtrace_vector_release" "-Dbacktrace_close=__rdos_backtrace_close" "-Dbacktrace_open=__rdos_backtrace_open" "-Dbacktrace_print=__rdos_backtrace_print" "-Dbacktrace_simple=__rdos_backtrace_simple" "-Dbacktrace_qsort=__rdos_backtrace_qsort" "-Dbacktrace_create_state=__rdos_backtrace_create_state" "-Dbacktrace_uncompress_zdebug=__rdos_backtrace_uncompress_zdebug" "-Dmacho_get_view=__rdos_macho_get_view" "-Dmacho_symbol_type_relevant=__rdos_macho_symbol_type_relevant" "-Dmacho_get_commands=__rdos_macho_get_commands" "-Dmacho_try_dsym=__rdos_macho_try_dsym" "-Dmacho_try_dwarf=__rdos_macho_try_dwarf" "-Dmacho_get_addr_range=__rdos_macho_get_addr_range" "-Dmacho_get_uuid=__rdos_macho_get_uuid" "-Dmacho_add=__rdos_macho_add" "-Dmacho_add_symtab=__rdos_macho_add_symtab" "-Dmacho_file_to_host_u64=__rdos_macho_file_to_host_u64" "-Dmacho_file_to_host_u32=__rdos_macho_file_to_host_u32" "-Dmacho_file_to_host_u16=__rdos_macho_file_to_host_u16" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-none/release/build/backtrace-sys-022de964ab3bf907/out/src/libbacktrace/elf.o" "-c" "src/libbacktrace/elf.c"
cargo:warning=src/libbacktrace/elf.c:43:10: fatal error: link.h: No such file or directory
cargo:warning=   43 | #include <link.h>
cargo:warning=      |          ^~~~~~~~
cargo:warning=compilation terminated.
exit code: 1

--- stderr


error occurred: Command "sccache" "aarch64-none-elf-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-ffunction-sections" "-fdata-sections" "-fPIC" "-mstrict-align" "-I" "src/libbacktrace" "-I" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-none/release/build/backtrace-sys-022de964ab3bf907/out" "-fvisibility=hidden" "-DBACKTRACE_ELF_SIZE=64" "-DBACKTRACE_SUPPORTED=1" "-DBACKTRACE_USES_MALLOC=1" "-DBACKTRACE_SUPPORTS_THREADS=0" "-DBACKTRACE_SUPPORTS_DATA=0" "-DHAVE_DL_ITERATE_PHDR=1" "-D_GNU_SOURCE=1" "-D_LARGE_FILES=1" "-Dbacktrace_full=__rdos_backtrace_full" "-Dbacktrace_dwarf_add=__rdos_backtrace_dwarf_add" "-Dbacktrace_initialize=__rdos_backtrace_initialize" "-Dbacktrace_pcinfo=__rdos_backtrace_pcinfo" "-Dbacktrace_syminfo=__rdos_backtrace_syminfo" "-Dbacktrace_get_view=__rdos_backtrace_get_view" "-Dbacktrace_release_view=__rdos_backtrace_release_view" "-Dbacktrace_alloc=__rdos_backtrace_alloc" "-Dbacktrace_free=__rdos_backtrace_free" "-Dbacktrace_vector_finish=__rdos_backtrace_vector_finish" "-Dbacktrace_vector_grow=__rdos_backtrace_vector_grow" "-Dbacktrace_vector_release=__rdos_backtrace_vector_release" "-Dbacktrace_close=__rdos_backtrace_close" "-Dbacktrace_open=__rdos_backtrace_open" "-Dbacktrace_print=__rdos_backtrace_print" "-Dbacktrace_simple=__rdos_backtrace_simple" "-Dbacktrace_qsort=__rdos_backtrace_qsort" "-Dbacktrace_create_state=__rdos_backtrace_create_state" "-Dbacktrace_uncompress_zdebug=__rdos_backtrace_uncompress_zdebug" "-Dmacho_get_view=__rdos_macho_get_view" "-Dmacho_symbol_type_relevant=__rdos_macho_symbol_type_relevant" "-Dmacho_get_commands=__rdos_macho_get_commands" "-Dmacho_try_dsym=__rdos_macho_try_dsym" "-Dmacho_try_dwarf=__rdos_macho_try_dwarf" "-Dmacho_get_addr_range=__rdos_macho_get_addr_range" "-Dmacho_get_uuid=__rdos_macho_get_uuid" "-Dmacho_add=__rdos_macho_add" "-Dmacho_add_symtab=__rdos_macho_add_symtab" "-Dmacho_file_to_host_u64=__rdos_macho_file_to_host_u64" "-Dmacho_file_to_host_u32=__rdos_macho_file_to_host_u32" "-Dmacho_file_to_host_u16=__rdos_macho_file_to_host_u16" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/aarch64-unknown-none/release/build/backtrace-sys-022de964ab3bf907/out/src/libbacktrace/elf.o" "-c" "src/libbacktrace/elf.c" with args "aarch64-none-elf-gcc" did not execute successfully (status code exit code: 1).



warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 19.568
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "aarch64-unknown-none" "-Zbinary-dep-depinfo" "-j" "12" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target aarch64-unknown-none
