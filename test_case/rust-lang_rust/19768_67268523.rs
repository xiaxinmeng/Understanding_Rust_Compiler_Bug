
$ RUST_LOG=rustc::metadata::loader rustc /home/chris/.cargo/git/checkouts/rust-openssl-4145800ab3594a0f/master/openssl-sys/src/lib.rs --crate-name openssl-sys --crate-type lib -g -C metadata=c31362507ac3b399 -C extra-filename=-c31362507ac3b399 --out-dir /home/chris/rust-http/target/deps --dep-info /home/chris/rust-http/target/.fingerprint/openssl-sys-c31362507ac3b399/dep-lib-openssl-sys -L /home/chris/rust-http/target/deps -L /home/chris/rust-http/target/deps -Awarnings -L /usr/lib -l ssl -l crypto
INFO:rustc::metadata::loader: lib candidate: /usr/lib/libstd-59beb4f7-0.11.0-pre.rlib
INFO:rustc::metadata::loader: lib candidate: /usr/lib/libstd-59beb4f7-0.11.0-pre.so
INFO:rustc::metadata::loader: lib candidate: /usr/lib/libstdc++.so
INFO:rustc::metadata::loader: lib candidate: /home/chris/opt/rust/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.rlib
INFO:rustc::metadata::loader: lib candidate: /home/chris/opt/rust/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.so
INFO:rustc::metadata::loader: dylib reading metadata from: /usr/lib/libstdc++.so.6.0.20
DEBUG:rustc::metadata::loader: get_metadata_section: name 
DEBUG:rustc::metadata::loader: get_metadata_section: name .note.gnu.build-id
DEBUG:rustc::metadata::loader: get_metadata_section: name .gnu.hash
DEBUG:rustc::metadata::loader: get_metadata_section: name .dynsym
DEBUG:rustc::metadata::loader: get_metadata_section: name .dynstr
DEBUG:rustc::metadata::loader: get_metadata_section: name .gnu.version
DEBUG:rustc::metadata::loader: get_metadata_section: name .gnu.version_d
DEBUG:rustc::metadata::loader: get_metadata_section: name .gnu.version_r
DEBUG:rustc::metadata::loader: get_metadata_section: name .rela.dyn
DEBUG:rustc::metadata::loader: get_metadata_section: name .rela.plt
DEBUG:rustc::metadata::loader: get_metadata_section: name .init
DEBUG:rustc::metadata::loader: get_metadata_section: name .plt
DEBUG:rustc::metadata::loader: get_metadata_section: name .text
DEBUG:rustc::metadata::loader: get_metadata_section: name .fini
DEBUG:rustc::metadata::loader: get_metadata_section: name .rodata
DEBUG:rustc::metadata::loader: get_metadata_section: name .eh_frame_hdr
DEBUG:rustc::metadata::loader: get_metadata_section: name .eh_frame
DEBUG:rustc::metadata::loader: get_metadata_section: name .gcc_except_table
DEBUG:rustc::metadata::loader: get_metadata_section: name .tbss
DEBUG:rustc::metadata::loader: get_metadata_section: name .init_array
DEBUG:rustc::metadata::loader: get_metadata_section: name .fini_array
DEBUG:rustc::metadata::loader: get_metadata_section: name .jcr
DEBUG:rustc::metadata::loader: get_metadata_section: name .data.rel.ro
DEBUG:rustc::metadata::loader: get_metadata_section: name .dynamic
DEBUG:rustc::metadata::loader: get_metadata_section: name .got
DEBUG:rustc::metadata::loader: get_metadata_section: name .got.plt
DEBUG:rustc::metadata::loader: get_metadata_section: name .data
DEBUG:rustc::metadata::loader: get_metadata_section: name .bss
DEBUG:rustc::metadata::loader: get_metadata_section: name .comment
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_aranges
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_info
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_abbrev
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_line
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_str
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_loc
DEBUG:rustc::metadata::loader: get_metadata_section: name .debug_ranges
DEBUG:rustc::metadata::loader: get_metadata_section: name .shstrtab
DEBUG:rustc::metadata::loader: get_metadata_section: name .symtab
DEBUG:rustc::metadata::loader: get_metadata_section: name .strtab
INFO:rustc::metadata::loader: reading libstdc++.so.6.0.20 => 0ms
INFO:rustc::metadata::loader: no metadata found
INFO:rustc::metadata::loader: rlib reading metadata from: /home/chris/opt/rust/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-4e7c5e5c.rlib
INFO:rustc::metadata::loader: reading libstd-4e7c5e5c.rlib => 0ms
INFO:rustc::metadata::loader: rlib reading metadata from: /usr/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-59beb4f7-0.11.0-pre.rlib
INFO:rustc::metadata::loader: reading libstd-59beb4f7-0.11.0-pre.rlib => 0ms
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'assertion failed: end <= self.len()', /home/chris/rust/src/libcore/slice.rs:119

stack backtrace:
   1:     0x7fe97baa8cb0 - rt::backtrace::imp::write::he833ac6ceeaffba7FUx
   2:     0x7fe97baac080 - failure::on_fail::h854fc12341fc775e3ly
   3:     0x7fe97b6e2070 - unwind::begin_unwind_inner::h4cba7a159498d42faNc
   4:     0x7fe97b6e1b70 - unwind::begin_unwind_fmt::h8a8e9d1a1bcccc61xKc
   5:     0x7fe97b6e1b30 - rust_begin_unwind
   6:     0x7fe97b734d60 - panicking::panic_fmt::h9907a8e31ba54b84gll
   7:     0x7fe97b732a60 - panicking::panic::h96bf5aa3a954ec4eGil
   8:     0x7fe979e031b0 - metadata::loader::Context<'a>::extract_one::h1cd45d747fca78ddYzq
   9:     0x7fe979dfb0f0 - metadata::loader::Context<'a>::find_library_crate::h40143a4c672c09bekqq
  10:     0x7fe979df6c60 - metadata::creader::PluginMetadataReader<'a>::read_plugin_metadata::hc7ab5dca73da937eygp
  11:     0x7fe979e2d650 - plugin::load::PluginLoader<'a>.Visitor<'v>::visit_view_item::h735ac4c27cdca5e2T9t
  12:     0x7fe979e2ca80 - plugin::load::load_plugins::h6d9f72909f1dd8a0J8t
  13:     0x7fe97beeaa30 - driver::phase_2_configure_and_expand::hd0af59d1072d8b22Sha
  14:     0x7fe97bede8d0 - driver::compile_input::h9bc853abf80ea006rba
  15:     0x7fe97c092910 - run_compiler::h8f684c83f08e07dfAYb
  16:     0x7fe97c089550 - thunk::F.Invoke<A, R>::invoke::h9148176715583558026
  17:     0x7fe97ba81060 - thunk::F.Invoke<A, R>::invoke::h3657185632151857599
  18:     0x7fe97b6e07e0 - task::Task::spawn_thunk::closure.5783
  19:     0x7fe97b7492b0 - rust_try_inner
  20:     0x7fe97b7492a0 - rust_try
  21:     0x7fe97b6e08f0 - unwind::try::hf911d270a8188879rCc
  22:     0x7fe97b6e0680 - task::Task::run::h6b0467fbfd3af2dcsNb
  23:     0x7fe97b6dfdf0 - thunk::F.Invoke<A, R>::invoke::h12189959489304490692
  24:     0x7fe97b6e1740 - thread::thread_start::hb135021f83af19ccM4b
  25:     0x7fe9763ee250 - start_thread
  26:     0x7fe97b3b6589 - clone
  27:                0x0 - <unknown>
