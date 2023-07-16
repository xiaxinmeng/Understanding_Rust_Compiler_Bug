plain
    Finished release [optimized] target(s) in 2m 17s
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
error: failed to run `rustc` to learn about target-specific information
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_os,"watchos")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","nvptx64","le32","xtensa")' '--check-cfg=values(dont_compile_me)' '--check-cfg=values(rustix_use_libc)' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 101)
  --- stderr
  thread 'rustc' panicked at 'start of span is out of bounds', /checkout/compiler/rustc_span/src/span_encoding.rs:100:25
     0:     0x7f835b4125f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h62d61b912b6c1e0a
     0:     0x7f835b4125f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h62d61b912b6c1e0a
     1:     0x7f835b480e78 - core::fmt::write::h0cd9f5419c66d611
     2:     0x7f835b403621 - std::io::Write::write_fmt::hdf8da66e4a05be95
     3:     0x7f835b4123b5 - std::sys_common::backtrace::print::h11723a34e1ab4b18
     4:     0x7f835b415767 - std::panicking::default_hook::{{closure}}::h5d3bed7a6b7b491a
     5:     0x7f835b4154c5 - std::panicking::default_hook::h4fdd417709624f4e
     6:     0x7f835be078e2 - rustc_driver[f2cff6577f7e0d3e]::DEFAULT_HOOK::{closure#0}::{closure#0}
     7:     0x7f835b416073 - std::panicking::rust_panic_with_hook::h42b5aa8dbb667dc4
     8:     0x7f835b415d61 - std::panicking::begin_panic_handler::{{closure}}::h84cf78d9ad8c5dc1
     9:     0x7f835b412b9c - std::sys_common::backtrace::__rust_end_short_backtrace::hbeb7f776584da27b
    10:     0x7f835b415a72 - rust_begin_unwind
    11:     0x7f835b3c7023 - core::panicking::panic_fmt::ha29375e42f9e82fb
    12:     0x7f835e4b072a - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[d6260eeb2cb43e8c]::SessionGlobals>>::with::<<rustc_span[d6260eeb2cb43e8c]::span_encoding::Span>::new::{closure#0}, ()>
Build completed unsuccessfully in 0:03:02
    13:     0x7f835e4f7493 - <rustc_parse[482c168b80988ad0]::lexer::StringReader>::next_token
    14:     0x7f835e50d65d - <rustc_parse[482c168b80988ad0]::lexer::tokentrees::TokenTreesReader>::parse_token_trees
    15:     0x7f835e50d55b - <rustc_parse[482c168b80988ad0]::lexer::tokentrees::TokenTreesReader>::parse_all_token_trees
    16:     0x7f835e4f69b9 - rustc_parse[482c168b80988ad0]::lexer::parse_token_trees
    17:     0x7f835e4edb18 - rustc_parse[482c168b80988ad0]::maybe_file_to_stream
    18:     0x7f835e4ed6e3 - rustc_parse[482c168b80988ad0]::maybe_source_file_to_parser
    19:     0x7f835e4ed0df - rustc_parse[482c168b80988ad0]::new_parser_from_source_str
    20:     0x7f835e4ecf2c - rustc_parse[482c168b80988ad0]::parse_crate_attrs_from_source_str
    21:     0x7f835be03903 - rustc_driver[f2cff6577f7e0d3e]::print_crate_info
    22:     0x7f835be08f28 - rustc_span[d6260eeb2cb43e8c]::with_source_map::<core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>, rustc_interface[b289e5a63908c72c]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>, rustc_driver[f2cff6577f7e0d3e]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
    23:     0x7f835be69008 - <scoped_tls[b64ea83672690cf8]::ScopedKey<rustc_span[d6260eeb2cb43e8c]::SessionGlobals>>::set::<rustc_interface[b289e5a63908c72c]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>, rustc_driver[f2cff6577f7e0d3e]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>
    24:     0x7f835be264f0 - std[4681af465d7cd70e]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b289e5a63908c72c]::util::run_in_thread_pool_with_globals<rustc_interface[b289e5a63908c72c]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>, rustc_driver[f2cff6577f7e0d3e]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>
    25:     0x7f835be6a446 - std[4681af465d7cd70e]::panic::catch_unwind::<core[867cfca19013d5a]::panic::unwind_safe::AssertUnwindSafe<<std[4681af465d7cd70e]::thread::Builder>::spawn_unchecked_<rustc_interface[b289e5a63908c72c]::util::run_in_thread_pool_with_globals<rustc_interface[b289e5a63908c72c]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>, rustc_driver[f2cff6577f7e0d3e]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>
    26:     0x7f835be18fa9 - <<std[4681af465d7cd70e]::thread::Builder>::spawn_unchecked_<rustc_interface[b289e5a63908c72c]::util::run_in_thread_pool_with_globals<rustc_interface[b289e5a63908c72c]::interface::run_compiler<core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>, rustc_driver[f2cff6577f7e0d3e]::run_compiler::{closure#1}>::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[867cfca19013d5a]::result::Result<(), rustc_errors[84e1187ac3583d13]::ErrorGuaranteed>>::{closure#1} as core[867cfca19013d5a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
    27:     0x7f835b42389e - std::sys::unix::thread::Thread::new::thread_start::h977952f5dce19fec
    28:     0x7f835b1b6b43 - <unknown>
    29:     0x7f835b248a00 - <unknown>
    30:                0x0 - <unknown>
  error: internal compiler error: unexpected panic

  note: the compiler unexpectedly panicked. this is a bug.


  note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

  note: rustc 1.68.0-nightly (7ed338cc1 2022-12-28) running on x86_64-unknown-linux-gnu

  note: compiler flags: -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro -Z force-unstable-if-unmarked
  query stack during panic:
  end of query stack
