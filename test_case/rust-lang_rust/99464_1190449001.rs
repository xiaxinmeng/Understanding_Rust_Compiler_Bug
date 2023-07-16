
---- [run-make] src/test/run-make/thumb-none-qemu stdout ----

error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
bash script.sh
AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar
BASE_COMMIT=
BOOTSTRAP_CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
BOOTSTRAP_PARENT_ID=2744
BOOTSTRAP_PYTHON=/usr/bin/python3
CARGO_HOME=/cargo
CC_aarch64_unknown_none=aarch64-none-elf-gcc
CC_aarch64_unknown_none_softfloat=aarch64-none-elf-gcc
CC_armv7a_none_eabi=arm-none-eabi-gcc
CC_armv7a_none_eabihf=arm-none-eabi-gcc
CC_mips64_unknown_linux_muslabi64=mips64-linux-gnuabi64-gcc
CC_mips64el_unknown_linux_muslabi64=mips64el-linux-gnuabi64-gcc
CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
CC_riscv32i_unknown_none_elf=false
CC_riscv32imac_unknown_none_elf=false
CC_riscv32imc_unknown_none_elf=false
CC_riscv64gc_unknown_none_elf=false
CC_riscv64imac_unknown_none_elf=false
CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc
CC_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc
CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
CFLAGS_aarch64_unknown_none=-mstrict-align
CFLAGS_aarch64_unknown_none_softfloat=-mstrict-align
CFLAGS_arm_unknown_linux_musleabi=-march=armv6 -marm
CFLAGS_arm_unknown_linux_musleabihf=-march=armv6 -marm -mfpu=vfp
CFLAGS_armv5te_unknown_linux_musleabi=-march=armv5te -marm -mfloat-abi=soft
CFLAGS_armv7_unknown_linux_musleabihf=-march=armv7-a
CFLAGS_armv7a_none_eabi=-march=armv7-a
CFLAGS_armv7a_none_eabihf=-march=armv7-a+vfpv3
CI_JOB_NAME=
COMPILETEST_NEEDS_ALL_LLVM_COMPONENTS=1
CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
DOC_RUST_LANG_ORG_CHANNEL=https://doc.rust-lang.org/nightly
HERE=/checkout/src/test/run-make/thumb-none-qemu
HOME=/home/user
HOSTNAME=d9fb11f2672b
HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
LC_CTYPE=C.UTF-8
LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
LLVM_BIN_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin
LLVM_COMPONENTS=aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray
LLVM_FILECHECK=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck
MAIL=/var/mail/user
MAKEFLAGS=
MAKELEVEL=1
MFLAGS=
MIRRORS_BASE=https://ci-mirrors.rust-lang.org/rustc
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
PWD=/checkout/src/test/run-make/thumb-none-qemu
PYTHON=/usr/bin/python3
RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
RUSTC_FORCE_RUSTC_VERSION=compiletest
RUSTDOC=LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib -Clinker='arm-none-eabi-gcc'
RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7hf=/musl-armv7hf       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --musl-root-mips64=/musl-mips64       --musl-root-mips64el=/musl-mips64el       --disable-docs --set build.print-step-timings --enable-verbose-tests --set build.metrics --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --dist-compression-formats=xz --disable-dist-src --release-channel=nightly --enable-debug-assertions --enable-overflow-checks --enable-llvm-assertions --set rust.verify-llvm-ir --enable-missing-tools
RUST_RELEASE_CHANNEL=nightly
S=/checkout
SCCACHE_DIR=/sccache
SCRIPT=python3 ../x.py --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python3 ../x.py dist --host= --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-none,aarch64-unknown-none-softfloat,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi
SHLVL=1
SRC=/checkout
STAGING_DIR=/tmp
TARGET=thumbv6m-none-eabi
TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-none,aarch64-unknown-none-softfloat,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi
TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
_=/usr/bin/env
__COMPAT_LAYER=RunAsInvoker
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
x = 42
------------------------------------------
--- stderr -------------------------------
+ CRATE=example
+ env
+ sort
+ mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
+ pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
+ rm -rf example
+ cp -a /checkout/src/test/run-make/thumb-none-qemu/example .
+ pushd example
+ env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
+ grep 'x = 42'
 Downloading crates ...
  Downloaded bare-metal v0.2.5
  Downloaded stable_deref_trait v1.1.1
  Downloaded r0 v0.2.2
  Downloaded rustc_version v0.2.3
  Downloaded generic-array v0.12.3
  Downloaded typenum v1.11.2
  Downloaded syn v1.0.14
  Downloaded generic-array v0.13.2
  Downloaded volatile-register v0.2.0
  Downloaded panic-halt v0.2.0
  Downloaded cortex-m-rt-macros v0.1.7
  Downloaded quote v1.0.2
  Downloaded proc-macro2 v1.0.8
  Downloaded cortex-m-rt v0.6.11
  Downloaded cortex-m-semihosting v0.3.5
  Downloaded aligned v0.3.2
  Downloaded cortex-m v0.6.2
  Downloaded as-slice v0.1.2
  Downloaded vcell v0.1.2
    Blocking waiting for file lock on package cache
   Compiling typenum v1.11.2
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.8
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.14
   Compiling stable_deref_trait v1.1.1
   Compiling cortex-m v0.6.2
   Compiling vcell v0.1.2
   Compiling cortex-m-rt v0.6.11
   Compiling cortex-m-semihosting v0.3.5
   Compiling r0 v0.2.2
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.2
   Compiling generic-array v0.12.3
   Compiling generic-array v0.13.2
   Compiling as-slice v0.1.2
   Compiling aligned v0.3.2
   Compiling cortex-m-rt-macros v0.1.7
   Compiling example v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example)
    Finished dev [unoptimized + debuginfo] target(s) in 11.32s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv6m-none-eabi/debug/example`
+ env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi --release
+ grep 'x = 42'
   Compiling typenum v1.11.2
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.8
   Compiling unicode-xid v0.2.0
   Compiling stable_deref_trait v1.1.1
   Compiling cortex-m v0.6.2
   Compiling syn v1.0.14
   Compiling vcell v0.1.2
   Compiling cortex-m-rt v0.6.11
   Compiling cortex-m-semihosting v0.3.5
   Compiling r0 v0.2.2
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.2
   Compiling generic-array v0.12.3
   Compiling generic-array v0.13.2
   Compiling as-slice v0.1.2
   Compiling aligned v0.3.2
   Compiling cortex-m-rt-macros v0.1.7
   Compiling example v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example)
error: linking with `arm-none-eabi-ld` failed: exit status: 1
  |
  = note: "arm-none-eabi-ld" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/rustcZCjqaf/symbols.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/example-66c103cf6d0cb1da.example.744b9132-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/example-66c103cf6d0cb1da.example.744b9132-cgu.1.rcgu.o" "--as-needed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/build/cortex-m-2599a902437ff745/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/build/cortex-m-rt-e987ff34a4df0173/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/build/cortex-m-semihosting-8bdc40115de87528/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--start-group" "-Bstatic" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libpanic_halt-1d3bee26a90e38d4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libcortex_m_semihosting-cf824c2529d48183.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libcortex_m_rt-59ab5923988296e7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libr0-2e0b47341c9cd99d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libcortex_m-aeddcfcf74909f3a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libvolatile_register-9a5eef5addc3d6c1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libvcell-605953c6f4b0edc4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libbare_metal-9ce21529b0e5d4d9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libaligned-30ae367887383836.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libas_slice-600f68f8acac94b2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libstable_deref_trait-9c2f2a8d91a61071.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libgeneric_array-d0fea296dba91047.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libgeneric_array-3a4e48e3458692f9.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/libtypenum-f9f1bcbefa66ff62.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib/librustc_std_workspace_core-d2691d84fc7f3622.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib/libcore-83610fbcea5035fd.rlib" "--end-group" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib/libcompiler_builtins-55f46071a6c3dd6f.rlib" "-Bdynamic" "--eh-frame-hdr" "-znoexecstack" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example/target/thumbv6m-none-eabi/release/deps/example-66c103cf6d0cb1da" "--gc-sections" "-O1" "-Tlink.x"
  = note: arm-none-eabi-ld: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib/libcompiler_builtins-55f46071a6c3dd6f.rlib(compiler_builtins-55f46071a6c3dd6f.compiler_builtins.0b035247-cgu.116.rcgu.o): in function `compiler_builtins::mem::memcpy':
          compiler_builtins.0b035247-cgu.116:(.text._ZN17compiler_builtins3mem6memcpy17hc025bec50004822eE+0x76): undefined reference to `__atomic_load_4'
          
  = help: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
  = note: use the `-l` flag to specify native libraries to link
  = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)

error: could not compile `example` due to previous error
make: *** [Makefile:27: all] Error 1
------------------------------------------



failures:
    [run-make] src/test/run-make/thumb-none-qemu
