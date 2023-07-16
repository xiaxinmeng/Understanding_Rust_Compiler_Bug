plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1971aa97
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:05:00]  ---> 3b1a45e1fbcd
[00:05:00] Step 33/38 : ENV HOSTS x86_64-unknown-linux-gnu
[00:05:00]  ---> Using cache
[00:05:00]  ---> c6ecaea432b7
[00:05:00] Step 34/38 : ENV RUST_CONFIGURE_ARGS --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang       --set target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       --set target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib       --set llvm.thin-lto=true
[00:05:00]  ---> dd101889e594
[00:05:00] Step 35/38 : ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:05:00]  ---> Using cache
[00:05:00]  ---> b5994262dff6
---
travis_time:start:0595a136
configure: processing command line
[00:05:00] configure: 
[00:05:00] configure: target.x86_64-unknown-linux-gnu.linker := clang
[00:05:00] configure: target.x86_64-unknown-linux-gnu.ar := /rustroot/bin/llvm-ar
[00:05:00] configure: target.x86_64-unknown-linux-gnu.ranlib := /rustroot/bin/llvm-ranlib
[00:05:00] configure: llvm.thin-lto        := True
[00:05:00] configure: build.submodules     := False
[00:05:00] configure: build.compiler-docs  := True
[00:05:00] configure: build.profiler       := True
[00:05:00] configure: build.locked-deps    := True
---

[00:23:08] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[00:23:08] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[00:23:09] -- The CXX compiler identification is Clang 6.0.0
[00:23:09] -- The ASM compiler identification is Clang
[00:23:09] -- Found assembler: /usr/local/bin/sccache
[00:23:09] -- Check for working C compiler: /usr/local/bin/sccache
---
[00:23:23] Call Stack (most recent call first):
[00:23:23]   CMakeLists.txt:616 (include)
[00:23:23] 
[00:23:23] 
[00:23:23] -- Performing Test CXX_SUPPORTS_CUSTOM_LINKER
[00:23:24] -- Performing Test CXX_SUPPORTS_CUSTOM_LINKER - Success
[00:23:24] -- Performing Test C_SUPPORTS_FPIC - Success
[00:23:24] -- Performing Test CXX_SUPPORTS_FPIC
[00:23:24] -- Performing Test CXX_SUPPORTS_FPIC - Success
[00:23:24] -- Building with -fPIC
---

[02:05:03] travis_fold:start:llvm
travis_time:start:llvm
Building Emscripten LLVM for x86_64-unknown-linux-gnu
[02:05:03] running: "cmake" "/checkout/src/llvm-emscripten" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=JSBackend" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm-emscripten" "-DCMAKE_BUILD_TYPE=Release"
[02:05:03] -- The CXX compiler identification is Clang 6.0.0
[02:05:03] -- The ASM compiler identification is Clang
[02:05:03] -- Found assembler: /usr/local/bin/sccache
[02:05:03] -- Check for working C compiler: /usr/local/bin/sccache
---

[02:54:04] travis_fold:start:lld
travis_time:start:lld
Building LLD for x86_64-unknown-linux-gnu
[02:54:04] running: "cmake" "/checkout/src/tools/lld" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DLLVM_CONFIG_PATH=/checkout/obj/build/bootstrap/debug/llvm-config-wrapper" "-DLLVM_INCLUDE_TESTS=OFF" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/lld" "-DCMAKE_BUILD_TYPE=Release"
[02:54:04] -- The CXX compiler identification is Clang 6.0.0
[02:54:04] -- Check for working C compiler: /usr/local/bin/sccache
[02:54:05] -- Check for working C compiler: /usr/local/bin/sccache -- works
[02:54:05] -- Detecting C compiler ABI info
---
[02:54:19] [ 99%] Built target lldELF
[02:54:19] Scanning dependencies of target lld
[02:54:19] [100%] Building CXX object tools/lld/CMakeFiles/lld.dir/lld.cpp.o
[02:54:19] [100%] Linking CXX executable ../../bin/lld
[02:54:19] /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/lib/libLLVMSupport.a: error adding symbols: Archive has no index; run ranlib to add one
[02:54:19] clang-6.0: error: linker command failed with exit code 1 (use -v to see invocation)
[02:54:19] gmake[2]: *** [bin/lld] Error 1
[02:54:19] gmake[1]: *** [tools/lld/CMakeFiles/lld.dir/all] Error 2
[02:54:19] gmake: *** [all] Error 2
[02:54:19] command did not execute successfully, got: exit code: 2
[02:54:19] 
[02:54:19] 
[02:54:19] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.31/src/lib.rs:643:5
[02:54:19]  finished in 14.828
[02:54:19] travis_fold:end:lld

[02:54:19] travis_time:end:lld:start=1533929482661993315,finish=1533929497490328765,duration=14828335450
---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:334d144f
$ sudo tail -n 500 /var/log/syslog
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] kvm-clock: using sched offset of 1538679797 cycles
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Zone ranges:
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   Device   empty
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Movable zone start for each node
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Early memory node ranges
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Policy zone: Normal
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] console [ttyS0] enabled
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.000000] tsc: Detected 2600.000 MHz processor
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.612428] Calibrating delay loop (skipped) preset value.. 5200.00 BogoMIPS (lpj=10400000)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.616881] pid_max: default: 32768 minimum: 301
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.619123] ACPI: Core revision 20150930
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.627255] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.630696] Security Framework initialized
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.632771] Yama: becoming mindful.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.634923] AppArmor: AppArmor disabled by boot time parameter
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.639404] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.651612] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.659516] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.664249] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.668970] Initializing cgroup subsys io
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.671113] Initializing cgroup subsys memory
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.673842] Initializing cgroup subsys devices
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.675991] Initializing cgroup subsys freezer
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.678357] Initializing cgroup subsys net_cls
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.680919] Initializing cgroup subsys perf_event
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.683488] Initializing cgroup subsys net_prio
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.686051] Initializing cgroup subsys hugetlb
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.688232] Initializing cgroup subsys pids
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.690427] CPU: Physical Processor ID: 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.692354] CPU: Processor Core ID: 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.694090] mce: CPU supports 32 MCE banks
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.696717] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.699278] Last level dTLB entries: 4KB 512, 2MB 32, 4MB 32, 1GB 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.704744] Freeing SMP alternatives memory: 32K
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.715423] ftrace: allocating 32185 entries in 126 pages
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.765548] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.769162] smpboot: Max logical packages: 2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.771915] x2apic enabled
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.774345] Switched APIC routing to physical x2apic.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.780305] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.890268] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.60GHz (family: 0x6, model: 0x2d, stepping: 0x7)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.896774] Performance Events: unsupported p6 CPU model 45 no PMU driver, software events only.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.902831] x86: Booting SMP configuration:
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.905650] .... node  #0, CPUs:      #1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.908071] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.914051]  #2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.915155] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.920962]  #3
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.922082] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.928925] x86: Booted up 1 node, 4 CPUs
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.931281] smpboot: Total of 4 processors activated (20800.00 BogoMIPS)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.936719] devtmpfs: initialized
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.942215] evm: security.selinux
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.944145] evm: security.SMACK64
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.945828] evm: security.SMACK64EXEC
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.947478] evm: security.SMACK64TRANSMUTE
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.950262] evm: security.SMACK64MMAP
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.951978] evm: security.ima
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.953816] evm: security.capability
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.956069] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.962075] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.965885] pinctrl core: initialized pinctrl subsystem
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.968767] RTC time: 16:35:57, date: 08/10/18
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.972556] NET: Registered protocol family 16
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.986436] cpuidle: using governor ladder
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    0.998427] cpuidle: using governor menu
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.000765] PCCT header not found.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.002975] ACPI: bus type PCI registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.005353] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.009520] PCI: Using configuration type 1 for base access
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.024008] ACPI: Added _OSI(Module Device)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.026477] ACPI: Added _OSI(Processor Device)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.028816] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.031937] ACPI: Added _OSI(Processor Aggregator Device)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.037590] ACPI: Executed 2 blocks of module-level executable AML code
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.064475] ACPI: Interpreter enabled
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.066530] ACPI: (supports S0 S3 S4 S5)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.068675] ACPI: Using IOAPIC for interrupt routing
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.071395] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.105136] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.108992] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.113034] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.116987] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.123816] PCI host bridge to bus 0000:00
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.126088] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.129628] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.133003] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.137297] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.141169] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.144186] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.144632] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.169638] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.193562] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.197425] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.206876] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.214989] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.234700] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.243578] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.250780] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.271639] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.276987] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.282268] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.287166] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.292447] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.314985] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.318195] vgaarb: loaded
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.319973] SCSI subsystem initialized
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.322520] libata version 3.00 loaded.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.322556] ACPI: bus type USB registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.324917] usbcore: registered new interface driver usbfs
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.328381] usbcore: registered new interface driver hub
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.331292] usbcore: registered new device driver usb
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.334645] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.338688] dmi: Firmware registration failed.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.341801] PCI: Using ACPI for IRQ routing
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.344121] PCI: pci_cache_line_size set to 64 bytes
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.344222] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.344224] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.344385] NetLabel: Initializing
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.346170] NetLabel:  domain hash size = 128
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.348592] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.351250] NetLabel:  unlabeled traffic allowed by default
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.354529] amd_nb: Cannot enumerate AMD northbridges
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.357943] clocksource: Switched to clocksource kvm-clock
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.368199] pnp: PnP ACPI init
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370224] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370296] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370375] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370427] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370469] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370510] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370552] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.370759] pnp: PnP ACPI: found 7 devices
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.381307] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.385853] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.385856] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.385858] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.385859] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.385895] NET: Registered protocol family 2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.388521] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.393034] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.396813] TCP: Hash tables configured (established 131072 bind 65536)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.400556] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.403977] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.407624] NET: Registered protocol family 1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.410265] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.413512] PCI: CLS 0 bytes, default 64
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    1.413568] Unpacking initramfs...
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.445720] Freeing initrd memory: 21432K
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.448700] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.453173] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.460009] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.464820] hw unit of domain pp0-core 2^-0 Joules
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.467876] hw unit of domain package 2^-0 Joules
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.471013] hw unit of domain dram 2^-0 Joules
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.474626] Scanning for low memory corruption every 60 seconds
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.479800] audit: initializing netlink subsys (disabled)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.483029] audit: type=2000 audit(1533918959.842:1): initialized
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.487827] Initialise system trusted keyring
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.491366] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.495246] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.500167] zbud: loaded
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.502367] VFS: Disk quotas dquot_6.6.0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.505619] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.510670] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.515416] fuse init (API version 7.23)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.518395] Key type big_key registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.520739] Allocating IMA MOK and blacklist keyrings.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.529147] Key type asymmetric registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.531863] Asymmetric key parser 'x509' registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.535032] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.539747] io scheduler noop registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.543183] io scheduler deadline registered (default)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.547134] io scheduler cfq registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.550234] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.553598] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.558716] intel_idle: does not run on family 6 model 45
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.558837] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.563647] ACPI: Power Button [PWRF]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.566112] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.570815] ACPI: Sleep Button [SLPF]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.574558] GHES: HEST is not enabled!
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.579572] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.584773] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.597929] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.601785] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.616165] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.643087] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.670863] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.700627] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.728298] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.737000] Linux agpgart interface v0.103
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.745882] loop: module loaded
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.749522] libphy: Fixed MDIO Bus: probed
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.754878] tun: Universal TUN/TAP device driver, 1.6
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.758757] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.821829] PPP generic driver version 2.4.2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.825584] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.830452] ehci-pci: EHCI PCI platform driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.833648] ehci-platform: EHCI generic platform driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.837719] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.842892] ohci-pci: OHCI PCI platform driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.846693] ohci-platform: OHCI generic platform driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.850286] uhci_hcd: USB Universal Host Controller Interface driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.855215] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.861610] i8042: Warning: Keylock active
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.865799] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.869289] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.873464] mousedev: PS/2 mouse device common for all mice
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.877726] rtc_cmos 00:00: RTC can wake from S4
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.882293] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.888300] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.893955] i2c /dev entries driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.897069] device-mapper: uevent: version 1.0.3
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.900974] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.907709] ledtrig-cpu: registered to indicate activity on CPUs
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.914084] NET: Registered protocol family 10
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.919293] NET: Registered protocol family 17
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.923928] Key type dns_resolver registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.927458] microcode: CPU0 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.931409] microcode: CPU1 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.934835] microcode: CPU2 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.938132] microcode: CPU3 sig=0x206d7, pf=0x1, revision=0x1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.942193] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.949241] registered taskstats version 1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.952352] Loading compiled-in X.509 certificates
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.956908] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.963471] zswap: loaded using pool lzo/zbud
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.969765] Key type trusted registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.979047] Key type encrypted registered
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.982600] ima: No TPM chip found, activating TPM-bypass!
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.987174] evm: HMAC attrs: 0x1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.990590]   Magic number: 14:489:590
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    3.994793] rtc_cmos 00:00: setting system clock to 2018-08-10 16:36:00 UTC (1533918960)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.001839] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.005948] EDD information not available.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.009409] PM: Hibernation image not present or could not be loaded.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.010963] Freeing unused kernel memory: 1496K
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.014290] Write protecting the kernel read-only data: 14336k
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.021139] Freeing unused kernel memory: 1956K
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.025212] Freeing unused kernel memory: 92K
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.047127] systemd-udevd[120]: starting version 204
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.074596] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.122266] scsi host0: Virtio SCSI HBA
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.136887] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.147431] AVX version of gcm_enc/dec engaged.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.150735] AES CTR mode by8 optimization enabled
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.257717] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.257769] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.257771] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.258298] sd 0:0:1:0: [sda] Write Protect is off
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.258300] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.258833] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.262431]  sda: sda1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.264176] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.474224] tsc: Refined TSC clocksource calibration: 2599.999 MHz
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.485344] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x257a3b2ad7e, max_idle_ns: 440795282337 ns
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    4.939131] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    7.110473] floppy0: no floppy controllers found
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.281960] raid6: sse2x1   gen()  8770 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.349958] raid6: sse2x1   xor()  6541 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.417959] raid6: sse2x2   gen() 10734 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.485963] raid6: sse2x2   xor()  7137 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.553957] raid6: sse2x4   gen() 12426 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.621958] raid6: sse2x4   xor()  8846 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.624679] raid6: using algorithm sse2x4 gen() 12426 MB/s
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.627901] raid6: .... xor() 8846 MB/s, rmw enabled
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.631225] raid6: using ssse3x2 recovery algorithm
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.637422] xor: automatically using best checksumming function:
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.678003]    avx       : 26925.000 MB/sec
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.696201] Btrfs loaded
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.768245] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.772278] EXT4-fs (sda1): write access will be enabled during recovery
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.874716] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.884563] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.886870] EXT4-fs (sda1): recovery complete
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    8.895448] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    9.125220] random: init: uninitialized urandom read (12 bytes read, 23 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    9.237523] random: mountall: uninitialized urandom read (12 bytes read, 27 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    9.292001] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [    9.516294] random: cloud-init: uninitialized urandom read (32 bytes read, 34 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.156473] random: cloud-init: uninitialized urandom read (32 bytes read, 41 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.289118] systemd-udevd[702]: starting version 204
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.412085] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.473889] intel_rapl: no valid rapl domains found in package 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.524683] ppdev: user-space parallel port driver
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.657153] random: mktemp: uninitialized urandom read (6 bytes read, 52 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.714877] random: mktemp: uninitialized urandom read (6 bytes read, 53 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.790097] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   10.959811] random: cloud-init: uninitialized urandom read (32 bytes read, 53 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   11.233116] random: mktemp: uninitialized urandom read (12 bytes read, 56 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   11.318995] random: mktemp: uninitialized urandom read (6 bytes read, 57 bits of entropy available)
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   11.407440] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   11.454860] EXT4-fs (sda1): resized filesystem to 7864064
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   11.899299] init: failsafe main process (1093) killed by TERM signal
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Running set_multiqueue.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Set channels for eth0 to 4.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 10 16:36:08 travis-job-54f77352-9977-4480-b304-f32f0be65665 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Starting Google Accounts daemon.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for me.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account me.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for henrik.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account henrik.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for emma.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account emma.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for igor.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account igor.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account konstantinhaase.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for aj.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account aj.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for solarce.
Aug 10 16:36:09 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account solarce.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-clock-skew: INFO Synced system time with hardware clock.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for asari.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account asari.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for bogdana.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account bogdana.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for konstantin.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.323648] random: nonblocking pool is initialized
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account konstantin.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for carmen.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.389056] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.393627] Bridge firewalling registered
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.408135] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account carmen.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Creating a new user account for maria.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.450095] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Created user account maria.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.486590] floppy0: no floppy controllers found
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 google-accounts: INFO Removing user packer.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.529927] Initializing XFRM netlink socket
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.539237] Netfilter messages via NETLINK v0.30.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   13.543534] ctnetlink v0.93: registering with nfnetlink.
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 16:36:10 travis-job-54f77352-9977-4480-b304-f32f0be65665 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 cron[1709]: (CRON) INFO (pidfile fd = 3)
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 cron[1746]: (CRON) STARTUP (fork ok)
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 cron[1746]: (CRON) INFO (Running @reboot jobs)
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 acpid: starting up with netlink and the input layer
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 acpid: 1 rule loaded
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 acpid: waiting for events: event logging is off
Aug 10 16:36:12 travis-job-54f77352-9977-4480-b304-f32f0be65665 haveged: haveged starting up
Aug 10 16:36:13 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   16.143658] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1843]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: proto: precision = 0.143 usec
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen normally on 3 eth0 10.20.3.53 UDP 123
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: peers refreshed
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listening on routing socket on fd #21 for interface updates
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   21.338974] init: plymouth-upstart-bridge main process ended, respawning
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 startup-script: INFO Found startup-script in metadata.
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 startup-script: INFO startup-script: job 1 at Fri Aug 10 19:46:00 2018
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 startup-script: INFO startup-script: Return code 0.
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 startup-script: INFO startup-script: Return code 0.
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 startup-script: INFO Finished running startup scripts.
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: 
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: #############################################################
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: 1024 ba:38:fb:c3:b4:e6:49:35:b0:05:dd:e4:dc:40:57:ea  root@travis-job-54f77352-9977-4480-b304-f32f0be65665 (DSA)
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: 256 38:3b:6c:ee:f9:8e:9f:fd:0e:d4:c8:55:e0:c2:40:af  root@travis-job-54f77352-9977-4480-b304-f32f0be65665 (ECDSA)
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: 256 6e:87:66:84:29:07:5f:0f:a5:6c:15:66:04:ef:fd:e2  root@travis-job-54f77352-9977-4480-b304-f32f0be65665 (ED25519)
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: 2048 d8:34:0e:18:14:96:34:97:75:8f:f6:9d:73:18:2c:a6  root@travis-job-54f77352-9977-4480-b304-f32f0be65665 (RSA)
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 10 16:36:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 ec2: #############################################################
Aug 10 16:36:25 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpdate[2253]: the NTP socket is in use, exiting
Aug 10 16:37:17 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [   80.342736] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.496561] device veth7c086a0 entered promiscuous mode
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.609226] cgroup: docker-runc (4980) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.609230] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.696664] eth0: renamed from veth7817e3c
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.737566] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.739047] docker0: port 1(veth7c086a0) entered forwarding state
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.739068] docker0: port 1(veth7c086a0) entered forwarding state
Aug 10 16:42:18 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  381.739099] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 10 16:42:22 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen normally on 5 docker0 fe80::42:5aff:fe64:3950 UDP 123
Aug 10 16:42:22 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 10 16:42:22 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 10 16:42:22 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: peers refreshed
Aug 10 16:42:22 travis-job-54f77352-9977-4480-b304-f32f0be65665 ntpd[1844]: new interface(s) found: waking up resolver
Aug 10 16:42:33 travis-job-54f77352-9977-4480-b304-f32f0be65665 kernel: [  396.764093] docker0: port 1(veth7c086a0) entered forwarding state
Aug 10 17:17:01 travis-job-54f77352-9977-4480-b304-f32f0be65665 CRON[27963]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 18:17:01 travis-job-54f77352-9977-4480-b304-f32f0be65665 CRON[29158]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
Aug 10 19:17:01 travis-job-54f77352-9977-4480-b304-f32f0be65665 CRON[7545]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:1f040a48:start=1533929498752264147,finish=1533929498759324699,duration=7060552
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16293480
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00ad887f
travis_time:start:00ad887f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0da3d561
$ dmesg | grep -i kill
