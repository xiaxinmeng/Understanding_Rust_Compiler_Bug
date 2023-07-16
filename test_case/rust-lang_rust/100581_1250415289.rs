plain
test [run-make] src/test/run-make/emit-shared-files ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] src/test/run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] src/test/run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] src/test/run-make/thumb-none-qemu stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
bash script.sh
AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar
AWS_ACCESS_KEY_ID=AKIA46X5W6CZI5DHEBFL
AWS_SECRET_ACCESS_KEY=***
BASE_COMMIT=a37499ae66ec5fc52a93d71493b78fb141c32f6b
BOOTSTRAP_CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
BOOTSTRAP_PARENT_ID=2608
BOOTSTRAP_PYTHON=/usr/bin/python3
CARGO_HOME=/cargo
CC_aarch64_unknown_none_softfloat=aarch64-none-elf-gcc
CC_armv7a_none_eabi=arm-none-eabi-gcc
CC_armv7a_none_eabihf=arm-none-eabi-gcc
CC_mips64_unknown_linux_muslabi64=mips64-linux-gnuabi64-gcc
---
CI_JOB_NAME=dist-various-1
COMPILETEST_NEEDS_ALL_LLVM_COMPONENTS=1
CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
DEPLOY=1
DOC_RUST_LANG_ORG_CHANNEL=https://doc.rust-lang.org/nightly
GITHUB_REF=refs/heads/auto
HERE=/checkout/src/test/run-make/thumb-none-qemu
HOME=/home/user
HOSTNAME=e91b0ac9d25d
HOSTNAME=e91b0ac9d25d
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
MIRRORS_BASE=https://ci-mirrors.rust-lang.org/rustc
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
***
PYTHON=/usr/bin/python3
PYTHON=/usr/bin/python3
RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
RUSTC_FORCE_RUSTC_VERSION=compiletest
RUSTDOC=LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib -Clinker='arm-none-eabi-gcc'
RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7hf=/musl-armv7hf       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --musl-root-mips64=/musl-mips64       --musl-root-mips64el=/musl-mips64el       --disable-docs --set build.print-step-timings --enable-verbose-tests --set build.metrics --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --dist-compression-formats=xz --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --debuginfo-level-std=1 --enable-missing-tools
RUST_DEMANGLER=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler
S=/checkout
SCCACHE_BUCKET=rust-lang-ci-sccache2
SCRIPT=python3 ../x.py --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python3 ../x.py dist --host= --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-none,aarch64-unknown-none-softfloat,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi
SHLVL=1
---
TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
TOOLSTATE_REPO_ACCESS_TOKEN=***
WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
_=/usr/bin/env
__COMPAT_LAYER=RunAsInvoker
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
--- stderr -------------------------------
--- stderr -------------------------------
+ CRATE=example
+ env
+ mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
+ pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
+ rm -rf example
+ rm -rf example
+ cp -a /checkout/src/test/run-make/thumb-none-qemu/example .
+ pushd example
+ env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
+ grep 'x = 42'
---
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.14
   Compiling cortex-m v0.6.2
   Compiling stable_deref_trait v1.1.1
   Compiling vcell v0.1.2
   Compiling cortex-m-rt v0.6.11
   Compiling cortex-m-semihosting v0.3.5
   Compiling r0 v0.2.2
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.2
   Compiling generic-array v0.12.3
   Compiling generic-array v0.12.3
   Compiling generic-array v0.13.2
   Compiling as-slice v0.1.2
   Compiling aligned v0.3.2
   Compiling cortex-m-rt-macros v0.1.7
   Compiling example v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example)
error: custom attribute panicked
   |
10 | #[entry]
   | ^^^^^^^^
   |
   |
   = help: message: observed race condition in proc_macro2::nightly_works
warning: unused import: `core::fmt::Write`
 --> src/main.rs:3:5
  |
3 | use core::fmt::Write;
---
  |
4 | use cortex_m::asm;
  |     ^^^^^^^^^^^^^

warning: unused import: `cortex_m_semihosting as semihosting`
  |
  |
6 | use cortex_m_semihosting as semihosting;


warning: `example` (bin "example") generated 3 warnings
error: could not compile `example` due to previous error; 3 warnings emitted
make: *** [Makefile:27: all] Error 1



failures:
