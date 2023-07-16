
Thread 1 "rustup-init" received signal SIGILL, Illegal instruction.
0x00007ffff781bc11 in sha2::sha512::x86::sha512_compress_x86_64_avx2 ()
(gdb) bt
#0  0x00007ffff781bc11 in sha2::sha512::x86::sha512_compress_x86_64_avx2 ()
#1  0x00007ffff78179d3 in sha2::sha512::Engine512::update ()
#2  0x00007ffff77b9a04 in sequoia_openpgp::parse::hashed_reader::<impl sequoia_openpgp::parse::Cookie>::hash_update ()
#3  0x00007ffff76c2082 in rustup::dist::signatures::verify_signature ()
#4  0x00007ffff76bf7f8 in rustup::dist::download::DownloadCfg::check_signature ()
#5  0x00007ffff766e8f4 in rustup::dist::download::DownloadCfg::download_and_check ()
#6  0x00007ffff766d088 in rustup::dist::dist::dl_v2_manifest ()
#7  0x00007ffff76689e0 in rustup::dist::dist::try_update_from_dist_ ()
#8  0x00007ffff7665aaa in rustup::install::InstallMethod::install ()
#9  0x00007ffff7663237 in rustup::toolchain::DistributableToolchain::install_from_dist ()
#10 0x00007ffff774d0aa in rustup::cli::self_update::install ()
#11 0x00007ffff7746a46 in rustup::cli::setup_mode::main ()
#12 0x00007ffff73cd071 in rustup_init::main ()
#13 0x00007ffff73cb223 in std::sys_common::backtrace::__rust_begin_short_backtrace ()
#14 0x00007ffff73cd940 in main ()
