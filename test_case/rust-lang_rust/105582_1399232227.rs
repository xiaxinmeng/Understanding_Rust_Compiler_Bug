plain
test [run-make] tests/run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] tests/run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] tests/run-make/rustdoc-verify-output-files ... ok
test [run-make] tests/run-make/thumb-none-cortex-m ... ok
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] tests/run-make/thumb-none-qemu stdout ----
error: make failed
status: exit status: 2
status: exit status: 2
command: cd "/checkout/tests/run-make/thumb-none-qemu" && AR="arm-none-eabi-ar" CC="arm-none-eabi-gcc -ffunction-sections -fdata-sections -mthumb -march=armv6s-m" CXX="arm-none-eabi-g++ -ffunction-sections -fdata-sections -mthumb -march=armv6s-m" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzercli fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts objcopy object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsdriver windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xray" LLVM_FILECHECK="/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTC_LINKER="arm-none-eabi-gcc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-thumbv6m-none-eabi" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="thumbv6m-none-eabi" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu" "make"
bash script.sh
AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar
AWS_ACCESS_KEY_ID=AKIA46X5W6CZI5DHEBFL
AWS_SECRET_ACCESS_KEY=***
AWS_SECRET_ACCESS_KEY=***
BASE_COMMIT=b22aa57fd54c240131c9a31b78213de9f3bee64b
BOOTSTRAP_CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
BOOTSTRAP_PARENT_ID=2604
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
HERE=/checkout/tests/run-make/thumb-none-qemu
HOME=/home/user
HOSTNAME=df811de18de2
HOSTNAME=df811de18de2
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
SCRIPT=python3 ../x.py --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf tests/run-make &&       python3 ../x.py dist --host= --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-none,aarch64-unknown-none-softfloat,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi
SHLVL=1
---
TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate
TOOLSTATE_REPO_ACCESS_TOKEN=***
WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
_=/usr/bin/env
__COMPAT_LAYER=RunAsInvoker
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/tests/run-make/thumb-none-qemu
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/tests/run-make/thumb-none-qemu
------------------------------------------
--- stderr -------------------------------
--- stderr -------------------------------
+ CRATE=example
+ env
+ mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
+ pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
+ rm -rf example
+ rm -rf example
+ cp -a /checkout/tests/run-make/thumb-none-qemu/example .
+ pushd example
+ env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
+ grep 'x = 42'
---
   Compiling proc-macro2 v1.0.8
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.14
   Compiling stable_deref_trait v1.1.1
   Compiling cortex-m v0.6.2
   Compiling vcell v0.1.2
   Compiling cortex-m-semihosting v0.3.5
   Compiling cortex-m-rt v0.6.11
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
    Finished dev [unoptimized + debuginfo] target(s) in 11.04s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv6m-none-eabi/debug/example`
+ env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi --release
+ grep 'x = 42'
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.8
   Compiling unicode-xid v0.2.0
   Compiling stable_deref_trait v1.1.1
   Compiling stable_deref_trait v1.1.1
   Compiling cortex-m v0.6.2
   Compiling syn v1.0.14
   Compiling vcell v0.1.2
   Compiling cortex-m-semihosting v0.3.5
   Compiling cortex-m-rt v0.6.11
   Compiling r0 v0.2.2
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling rustc_version v0.2.3
   Compiling bare-metal v0.2.5
   Compiling quote v1.0.2
   Compiling quote v1.0.2
error: internal compiler error: /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/compiler/rustc_middle/src/ty/layout.rs:793:21: failed to get layout for `PhantomData<<Ul as type_operators::Min<Ur>>::Output>`: unable to determine layout for `PhantomData<<Ul as type_operators::Min<Ur>>::Output>` because `PhantomData<<Ul as type_operators::Min<Ur>>::Output>` cannot be normalized,
                                despite it being a field (#0) of an existing layout: TyAndLayout {
                                    ty: int::PInt<<Ul as type_operators::Min<Ur>>::Output>,
                                    layout: Layout {
                                        size: Size(0 bytes),
                                        align: AbiAndPrefAlign {
                                            abi: Align(1 bytes),
                                            pref: Align(4 bytes),
                                        abi: Aggregate {
                                            sized: true,
                                        },
                                        fields: Arbitrary {
---
                                }

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/compiler/rustc_errors/src/lib.rs:1609:9
stack backtrace:
   0:     0x7fd06efce6da - std::backtrace_rs::backtrace::libunwind::trace::h1903a7ad93ae780d
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7fd06efce6da - std::backtrace_rs::backtrace::trace_unsynchronized::h894f65b4c744907e
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd06efce6da - std::sys_common::backtrace::_print_fmt::hb25ec065d7fa7556
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fd06efce6da - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hed26badc46b4a3cb
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fd06f02df6e - core::fmt::write::h1a087d8b9c200165
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/core/src/fmt/mod.rs:1213:17
   5:     0x7fd06efc05f5 - std::io::Write::write_fmt::h39959f5f97eb2e9f
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/io/mod.rs:1682:15
   6:     0x7fd06efce4a5 - std::sys_common::backtrace::_print::h1b73130a2aa5cb12
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fd06efce4a5 - std::sys_common::backtrace::print::hbc4eca6078cd8177
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fd06efd126f - std::panicking::default_hook::{{closure}}::h93dafcd3b719862c
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/panicking.rs:267:22
   9:     0x7fd06efd0fab - std::panicking::default_hook::h2158dd3cde2eeb88
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/panicking.rs:286:9
  10:     0x7fd06fd57015 - rustc_driver[98bc03253640795a]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7fd06efd1a4a - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h7c5b7111fb04e456
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/alloc/src/boxed.rs:2002:9
  12:     0x7fd06efd1a4a - std::panicking::rust_panic_with_hook::he00c91de42c7a78a
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/panicking.rs:692:13
  13:     0x7fd07575bb03 - std[405deea69a07d187]::panicking::begin_panic::<rustc_errors[e3507042a327c4ef]::ExplicitBug>::{closure#0}
  14:     0x7fd075755036 - std[405deea69a07d187]::sys_common::backtrace::__rust_end_short_backtrace::<std[405deea69a07d187]::panicking::begin_panic<rustc_errors[e3507042a327c4ef]::ExplicitBug>::{closure#0}, !>
  15:     0x7fd06fbc66b6 - std[405deea69a07d187]::panicking::begin_panic::<rustc_errors[e3507042a327c4ef]::ExplicitBug>
  16:     0x7fd0757c2e86 - std[405deea69a07d187]::panic::panic_any::<rustc_errors[e3507042a327c4ef]::ExplicitBug>
  17:     0x7fd0757beeda - <rustc_errors[e3507042a327c4ef]::HandlerInner>::bug::<&alloc[cc5153341afd5aba]::string::String>
  18:     0x7fd0757bcb40 - <rustc_errors[e3507042a327c4ef]::Handler>::bug::<&alloc[cc5153341afd5aba]::string::String>
  19:     0x7fd0757e8905 - rustc_middle[a6a80859aff2bcce]::util::bug::opt_span_bug_fmt::<rustc_span[b68c542f94839c1c]::span_encoding::Span>::{closure#0}
  20:     0x7fd0757e844c - rustc_middle[a6a80859aff2bcce]::ty::context::tls::with_opt::<rustc_middle[a6a80859aff2bcce]::util::bug::opt_span_bug_fmt<rustc_span[b68c542f94839c1c]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  21:     0x7fd0757e83fe - rustc_middle[a6a80859aff2bcce]::ty::context::tls::with_context_opt::<rustc_middle[a6a80859aff2bcce]::ty::context::tls::with_opt<rustc_middle[a6a80859aff2bcce]::util::bug::opt_span_bug_fmt<rustc_span[b68c542f94839c1c]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  22:     0x7fd0757e8849 - rustc_middle[a6a80859aff2bcce]::util::bug::opt_span_bug_fmt::<rustc_span[b68c542f94839c1c]::span_encoding::Span>
  23:     0x7fd06fbca8f5 - rustc_middle[a6a80859aff2bcce]::util::bug::bug_fmt
  24:     0x7fd0743a3ed7 - <rustc_middle[a6a80859aff2bcce]::ty::Ty as rustc_target[b7a572da82968ed5]::abi::TyAbiInterface<rustc_middle[a6a80859aff2bcce]::ty::layout::LayoutCx<rustc_middle[a6a80859aff2bcce]::ty::context::TyCtxt>>>::ty_and_layout_field
  25:     0x7fd0743db6b3 - rustc_const_eval[fe393f0423fddf5c]::util::might_permit_raw_init::might_permit_raw_init_lax
  26:     0x7fd0743da854 - rustc_const_eval[fe393f0423fddf5c]::util::might_permit_raw_init::might_permit_raw_init
  27:     0x7fd074803ea4 - rustc_query_system[5315918f4588ec09]::query::plumbing::try_execute_query::<rustc_query_impl[9363500e31190401]::queries::permits_uninit_init, rustc_query_impl[9363500e31190401]::plumbing::QueryCtxt>
  28:     0x7fd074552c4d - <rustc_query_impl[9363500e31190401]::Queries as rustc_middle[a6a80859aff2bcce]::ty::query::QueryEngine>::permits_uninit_init
  29:     0x7fd07387e5ee - rustc_mir_transform[9f99d78956baeaa3]::instcombine::intrinsic_assert_panics::mem_uninitialized_valid_predicate
  30:     0x7fd07387dd28 - <rustc_mir_transform[9f99d78956baeaa3]::instcombine::InstCombine as rustc_middle[a6a80859aff2bcce]::mir::MirPass>::run_pass
  31:     0x7fd0738a7be2 - rustc_mir_transform[9f99d78956baeaa3]::pass_manager::run_passes_inner
  32:     0x7fd073888b47 - rustc_mir_transform[9f99d78956baeaa3]::optimized_mir
  33:     0x7fd074798443 - rustc_query_system[5315918f4588ec09]::query::plumbing::try_execute_query::<rustc_query_impl[9363500e31190401]::queries::optimized_mir, rustc_query_impl[9363500e31190401]::plumbing::QueryCtxt>
  34:     0x7fd074546cd3 - <rustc_query_impl[9363500e31190401]::Queries as rustc_middle[a6a80859aff2bcce]::ty::query::QueryEngine>::optimized_mir
  35:     0x7fd074cdb0c5 - <rustc_metadata[9123dfc30cf72a82]::rmeta::encoder::EncodeContext>::encode_crate_root
  36:     0x7fd074cf0d6b - rustc_metadata[9123dfc30cf72a82]::rmeta::encoder::encode_metadata_impl
  37:     0x7fd074e0979e - rustc_data_structures[e02515ef02d39290]::sync::join::<rustc_metadata[9123dfc30cf72a82]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[9123dfc30cf72a82]::rmeta::encoder::encode_metadata::{closure#1}, (), ()>
  38:     0x7fd074cf0263 - rustc_metadata[9123dfc30cf72a82]::rmeta::encoder::encode_metadata
  39:     0x7fd074e1fa84 - rustc_metadata[9123dfc30cf72a82]::fs::encode_and_write_metadata
  40:     0x7fd06fe1744b - rustc_interface[4f97c323162a7973]::passes::start_codegen
  41:     0x7fd06fe1d923 - <rustc_interface[4f97c323162a7973]::queries::Queries>::ongoing_codegen
  42:     0x7fd06fd2be3b - rustc_span[b68c542f94839c1c]::with_source_map::<core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>, rustc_interface[4f97c323162a7973]::interface::run_compiler<core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>, rustc_driver[98bc03253640795a]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  43:     0x7fd06fd20895 - <scoped_tls[617d7561dd648bcb]::ScopedKey<rustc_span[b68c542f94839c1c]::SessionGlobals>>::set::<rustc_interface[4f97c323162a7973]::interface::run_compiler<core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>, rustc_driver[98bc03253640795a]::run_compiler::{closure#1}>::{closure#0}, core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>>
  44:     0x7fd06fcd98d0 - std[405deea69a07d187]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4f97c323162a7973]::util::run_in_thread_pool_with_globals<rustc_interface[4f97c323162a7973]::interface::run_compiler<core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>, rustc_driver[98bc03253640795a]::run_compiler::{closure#1}>::{closure#0}, core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>>
  45:     0x7fd06fcbd7ed - <<std[405deea69a07d187]::thread::Builder>::spawn_unchecked_<rustc_interface[4f97c323162a7973]::util::run_in_thread_pool_with_globals<rustc_interface[4f97c323162a7973]::interface::run_compiler<core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>, rustc_driver[98bc03253640795a]::run_compiler::{closure#1}>::{closure#0}, core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c89b0a3a2ff3d757]::result::Result<(), rustc_errors[e3507042a327c4ef]::ErrorGuaranteed>>::{closure#1} as core[c89b0a3a2ff3d757]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7fd06efdb8b3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hda6b206aaa3d2e7f
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/alloc/src/boxed.rs:1988:9
  47:     0x7fd06efdb8b3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hccc171e0d4d6ee10
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/alloc/src/boxed.rs:1988:9
  48:     0x7fd06efdb8b3 - std::sys::unix::thread::Thread::new::thread_start::h7364003f24963e0e
                               at /rustc/ed9d19a24db785dd6b0b49eead9328842892e10e/library/std/src/sys/unix/thread.rs:108:17
  49:     0x7fd06ed0b609 - start_thread
  50:     0x7fd06ee4b133 - clone
  51:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.68.0-nightly (ed9d19a24 2023-01-21) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C linker=arm-none-eabi-ld -C link-arg=-Tlink.x
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [permits_uninit_init] checking to see if `int::PInt<<Ul as type_operators::Min<Ur>>::Output>` permits being left uninit
#1 [optimized_mir] optimizing MIR for `int::<impl at /cargo/registry/src/github.com-1ecc6299db9ec823/typenum-1.11.2/src/int.rs:804:1: 804:40>::min`
error: could not compile `typenum`
warning: build failed, waiting for other jobs to finish...
make: *** [Makefile:27: all] Error 1
------------------------------------------
