plain
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0583]: file not found for module `aarch64_unknown_none_hermitkernel`
    --> compiler/rustc_target/src/spec/mod.rs:773:11
     |
771  | / macro_rules! supported_targets {
772  | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
773  | |         $(mod $module;)+
774  | |
...    |
800  | |     };
801  | | }
801  | | }
     | |_- in this expansion of `supported_targets!`
803  | / supported_targets! {
803  | / supported_targets! {
804  | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
805  | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
806  | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
1022 | |     ("mips64-openwrt-linux-musl", mips64_openwrt_linux_musl),
1023 | | }
     | |_- in this macro invocation
     |
     |
     = help: to create the module `aarch64_unknown_none_hermitkernel`, create file "compiler/rustc_target/src/spec/aarch64_unknown_none_hermitkernel.rs" or "compiler/rustc_target/src/spec/aarch64_unknown_none_hermitkernel/mod.rs"

error[E0425]: cannot find function `target` in module `aarch64_unknown_none_hermitkernel`
    --> compiler/rustc_target/src/spec/mod.rs:780:45
771  | / macro_rules! supported_targets {
771  | / macro_rules! supported_targets {
772  | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
773  | |         $(mod $module;)+
...    |
...    |
780  | |                 $( $($triple)|+ => $module::target(), )+
     | |                                             ^^^^^^ not found in `aarch64_unknown_none_hermitkernel`
800  | |     };
801  | | }
801  | | }
     | |_- in this expansion of `supported_targets!`
803  | / supported_targets! {
803  | / supported_targets! {
804  | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
805  | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
806  | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
1022 | |     ("mips64-openwrt-linux-musl", mips64_openwrt_linux_musl),
1023 | | }
     | |_- in this macro invocation
     |
---
37   | use crate::spec::aarch64_apple_ios_sim::target;
     |
       and 176 other candidates

error[E0425]: cannot find function `target` in module `super::aarch64_unknown_none_hermitkernel`
    --> compiler/rustc_target/src/spec/mod.rs:796:61
771  | / macro_rules! supported_targets {
771  | / macro_rules! supported_targets {
772  | |     ( $(($( $triple:literal, )+ $module:ident ),)+ ) => {
773  | |         $(mod $module;)+
...    |
...    |
796  | |                     tests_impl::test_target(super::$module::target());
     | |                                                             ^^^^^^ not found in `super::aarch64_unknown_none_hermitkernel`
800  | |     };
801  | | }
801  | | }
     | |_- in this expansion of `supported_targets!`
803  | / supported_targets! {
803  | / supported_targets! {
804  | |     ("x86_64-unknown-linux-gnu", x86_64_unknown_linux_gnu),
805  | |     ("x86_64-unknown-linux-gnux32", x86_64_unknown_linux_gnux32),
806  | |     ("i686-unknown-linux-gnu", i686_unknown_linux_gnu),
1022 | |     ("mips64-openwrt-linux-musl", mips64_openwrt_linux_musl),
1023 | | }
     | |_- in this macro invocation
     |
---
             crate::spec::aarch64_unknown_linux_musl::target
             crate::spec::aarch64_unknown_netbsd::target
             crate::spec::aarch64_unknown_none::target
             crate::spec::aarch64_unknown_none_softfloat::target
             crate::spec::aarch64_unknown_openbsd::target
             crate::spec::aarch64_unknown_redox::target
             crate::spec::aarch64_unknown_uefi::target
             crate::spec::aarch64_uwp_windows_msvc::target
             crate::spec::aarch64_wrs_vxworks::target
             crate::spec::arm_linux_androideabi::target
             crate::spec::arm_unknown_linux_gnueabi::target
             crate::spec::arm_unknown_linux_gnueabihf::target
             crate::spec::arm_unknown_linux_musleabi::target
             crate::spec::arm_unknown_linux_musleabihf::target
             crate::spec::armebv7r_none_eabi::target
             crate::spec::armebv7r_none_eabihf::target
             crate::spec::armv4t_unknown_linux_gnueabi::target
             crate::spec::armv5te_unknown_linux_gnueabi::target
             crate::spec::armv5te_unknown_linux_musleabi::target
             crate::spec::armv5te_unknown_linux_uclibceabi::target
             crate::spec::armv6_unknown_freebsd::target
             crate::spec::armv6_unknown_netbsd_eabihf::target
             crate::spec::armv6k_nintendo_3ds::target
             crate::spec::armv7_apple_ios::target
             crate::spec::armv7_linux_androideabi::target
             crate::spec::armv7_unknown_freebsd::target
             crate::spec::armv7_unknown_linux_gnueabi::target
             crate::spec::armv7_unknown_linux_gnueabihf::target
             crate::spec::armv7_unknown_linux_musleabi::target
             crate::spec::armv7_unknown_linux_musleabihf::target
             crate::spec::armv7_unknown_linux_uclibceabi::target
             crate::spec::armv7_unknown_linux_uclibceabihf::target
             crate::spec::armv7_unknown_netbsd_eabihf::target
             crate::spec::armv7_wrs_vxworks_eabihf::target
             crate::spec::armv7a_kmc_solid_asp3_eabi::target
             crate::spec::armv7a_kmc_solid_asp3_eabihf::target
             crate::spec::armv7a_none_eabi::target
             crate::spec::armv7a_none_eabihf::target
             crate::spec::armv7r_none_eabi::target
             crate::spec::armv7r_none_eabihf::target
             crate::spec::armv7s_apple_ios::target
             crate::spec::asmjs_unknown_emscripten::target
             crate::spec::avr_gnu_base::target
             crate::spec::avr_unknown_gnu_atmega328::target
             crate::spec::bpfeb_unknown_none::target
             crate::spec::bpfel_unknown_none::target
             crate::spec::hexagon_unknown_linux_musl::target
             crate::spec::i386_apple_ios::target
             crate::spec::i586_pc_windows_msvc::target
             crate::spec::i586_unknown_linux_gnu::target
             crate::spec::i586_unknown_linux_musl::target
             crate::spec::i686_apple_darwin::target
             crate::spec::i686_linux_android::target
             crate::spec::i686_pc_windows_gnu::target
             crate::spec::i686_pc_windows_msvc::target
             crate::spec::i686_unknown_freebsd::target
             crate::spec::i686_unknown_haiku::target
             crate::spec::i686_unknown_linux_gnu::target
             crate::spec::i686_unknown_linux_musl::target
             crate::spec::i686_unknown_netbsd::target
             crate::spec::i686_unknown_openbsd::target
             crate::spec::i686_unknown_uefi::target
             crate::spec::i686_uwp_windows_gnu::target
             crate::spec::i686_uwp_windows_msvc::target
             crate::spec::i686_wrs_vxworks::target
             crate::spec::m68k_unknown_linux_gnu::target
             crate::spec::mips64_openwrt_linux_musl::target
             crate::spec::mips64_unknown_linux_gnuabi64::target
             crate::spec::mips64_unknown_linux_muslabi64::target
             crate::spec::mips64el_unknown_linux_gnuabi64::target
             crate::spec::mips64el_unknown_linux_muslabi64::target
             crate::spec::mips_unknown_linux_gnu::target
             crate::spec::mips_unknown_linux_musl::target
             crate::spec::mips_unknown_linux_uclibc::target
             crate::spec::mipsel_sony_psp::target
             crate::spec::mipsel_unknown_linux_gnu::target
             crate::spec::mipsel_unknown_linux_musl::target
             crate::spec::mipsel_unknown_linux_uclibc::target
             crate::spec::mipsel_unknown_none::target
             crate::spec::mipsisa32r6_unknown_linux_gnu::target
             crate::spec::mipsisa32r6el_unknown_linux_gnu::target
             crate::spec::mipsisa64r6_unknown_linux_gnuabi64::target
             crate::spec::mipsisa64r6el_unknown_linux_gnuabi64::target
             crate::spec::msp430_none_elf::target
             crate::spec::nvptx64_nvidia_cuda::target
             crate::spec::powerpc64_unknown_freebsd::target
             crate::spec::powerpc64_unknown_linux_gnu::target
             crate::spec::powerpc64_unknown_linux_musl::target
             crate::spec::powerpc64_wrs_vxworks::target
             crate::spec::powerpc64le_unknown_freebsd::target
             crate::spec::powerpc64le_unknown_linux_gnu::target
             crate::spec::powerpc64le_unknown_linux_musl::target
             crate::spec::powerpc_unknown_freebsd::target
             crate::spec::powerpc_unknown_linux_gnu::target
             crate::spec::powerpc_unknown_linux_gnuspe::target
             crate::spec::powerpc_unknown_linux_musl::target
             crate::spec::powerpc_unknown_netbsd::target
             crate::spec::powerpc_unknown_openbsd::target
             crate::spec::powerpc_wrs_vxworks::target
             crate::spec::powerpc_wrs_vxworks_spe::target
             crate::spec::riscv32gc_unknown_linux_gnu::target
             crate::spec::riscv32gc_unknown_linux_musl::target
             crate::spec::riscv32i_unknown_none_elf::target
             crate::spec::riscv32imac_unknown_none_elf::target
             crate::spec::riscv32imc_esp_espidf::target
             crate::spec::riscv32imc_unknown_none_elf::target
             crate::spec::riscv64gc_unknown_freebsd::target
             crate::spec::riscv64gc_unknown_linux_gnu::target
             crate::spec::riscv64gc_unknown_linux_musl::target
             crate::spec::riscv64gc_unknown_none_elf::target
             crate::spec::riscv64imac_unknown_none_elf::target
             crate::spec::s390x_unknown_linux_gnu::target
             crate::spec::s390x_unknown_linux_musl::target
             crate::spec::sparc64_unknown_linux_gnu::target
             crate::spec::sparc64_unknown_netbsd::target
             crate::spec::sparc64_unknown_openbsd::target
             crate::spec::sparc_unknown_linux_gnu::target
             crate::spec::sparcv9_sun_solaris::target
             crate::spec::thumbv4t_none_eabi::target
             crate::spec::thumbv6m_none_eabi::target
             crate::spec::thumbv7a_pc_windows_msvc::target
             crate::spec::thumbv7a_uwp_windows_msvc::target
             crate::spec::thumbv7em_none_eabi::target
             crate::spec::thumbv7em_none_eabihf::target
             crate::spec::thumbv7m_none_eabi::target
             crate::spec::thumbv7neon_linux_androideabi::target
             crate::spec::thumbv7neon_unknown_linux_gnueabihf::target
             crate::spec::thumbv7neon_unknown_linux_musleabihf::target
             crate::spec::thumbv8m_base_none_eabi::target
             crate::spec::thumbv8m_main_none_eabi::target
             crate::spec::thumbv8m_main_none_eabihf::target
             crate::spec::wasm32_unknown_emscripten::target
             crate::spec::wasm32_unknown_unknown::target
             crate::spec::wasm32_wasi::target
             crate::spec::wasm64_unknown_unknown::target
             crate::spec::x86_64_apple_darwin::target
             crate::spec::x86_64_apple_ios::target
             crate::spec::x86_64_apple_ios_macabi::target
             crate::spec::x86_64_apple_tvos::target
             crate::spec::x86_64_fortanix_unknown_sgx::target
             crate::spec::x86_64_fuchsia::target
             crate::spec::x86_64_linux_android::target
             crate::spec::x86_64_pc_solaris::target
             crate::spec::x86_64_pc_windows_gnu::target
             crate::spec::x86_64_pc_windows_msvc::target
             crate::spec::x86_64_sun_solaris::target
             crate::spec::x86_64_unknown_dragonfly::target
             crate::spec::x86_64_unknown_freebsd::target
             crate::spec::x86_64_unknown_haiku::target
             crate::spec::x86_64_unknown_hermit::target
             crate::spec::x86_64_unknown_illumos::target
             crate::spec::x86_64_unknown_l4re_uclibc::target
             crate::spec::x86_64_unknown_linux_gnu::target
             crate::spec::x86_64_unknown_linux_gnux32::target
             crate::spec::x86_64_unknown_linux_musl::target
             crate::spec::x86_64_unknown_netbsd::target
             crate::spec::x86_64_unknown_none::target
             crate::spec::x86_64_unknown_none_hermitkernel::target
             crate::spec::x86_64_unknown_none_linuxkernel::target
             crate::spec::x86_64_unknown_openbsd::target
             crate::spec::x86_64_unknown_redox::target
             crate::spec::x86_64_unknown_uefi::target
             crate::spec::x86_64_uwp_windows_gnu::target
             crate::spec::x86_64_uwp_windows_msvc::target
             crate::spec::x86_64_wrs_vxworks::target
    Checking chalk-engine v0.76.0
Some errors have detailed explanations: E0425, E0583.
For more information about an error, try `rustc --explain E0425`.
    Checking gsgdt v0.1.2
