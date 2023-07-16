
#0  core::cell::{{impl}}::set<usize> (value=6215344901283465300, self=<optimized out>) at /home/mark/Edit/rust/src/libcore/cell.rs:239
#1  alloc::rc::RcBoxPtr::dec_strong<alloc::rc::Rc<syntax::tokenstream::Delimited>,syntax::tokenstream::Delimited> (self=0x7fd537eedd98) at /home/mark/Edit/rust/src/liballoc/rc.rs:1115
#2  alloc::rc::{{impl}}::drop<syntax::tokenstream::Delimited> (self=0x7fd537eedd98) at /home/mark/Edit/rust/src/liballoc/rc.rs:648
#3  0x00007fd53a4156ad in rustc_data_structures::array_vec::{{impl}}::push<[syntax::ext::tt::transcribe::TtFrame; 1]> (self=0x7fd537eedd80, el=...) at /home/mark/Edit/rust/src/librustc_data_structures/array_vec.rs:70
#4  0x00007fd53a42708e in rustc_data_structures::array_vec::{{impl}}::extend<[syntax::ext::tt::transcribe::TtFrame; 1],core::iter::sources::Once<syntax::ext::tt::transcribe::TtFrame>> (self=<optimized out>, iter=...)
    at /home/mark/Edit/rust/src/librustc_data_structures/array_vec.rs:132
#5  rustc_data_structures::accumulate_vec::{{impl}}::from_iter<[syntax::ext::tt::transcribe::TtFrame; 1],core::iter::sources::Once<syntax::ext::tt::transcribe::TtFrame>> (iter=...)
    at /home/mark/Edit/rust/src/librustc_data_structures/accumulate_vec.rs:112
#6  core::iter::iterator::Iterator::collect<core::iter::sources::Once<syntax::ext::tt::transcribe::TtFrame>,rustc_data_structures::accumulate_vec::AccumulateVec<[syntax::ext::tt::transcribe::TtFrame; 1]>> (self=...)
    at /home/mark/Edit/rust/src/libcore/iter/iterator.rs:1196
#7  rustc_data_structures::accumulate_vec::{{impl}}::one<[syntax::ext::tt::transcribe::TtFrame; 1]> (el=...) at /home/mark/Edit/rust/src/librustc_data_structures/accumulate_vec.rs:48
#8  rustc_data_structures::small_vec::{{impl}}::one<[syntax::ext::tt::transcribe::TtFrame; 1]> (el=...) at /home/mark/Edit/rust/src/librustc_data_structures/small_vec.rs:51
#9  syntax::ext::tt::transcribe::new_tt_reader_with_doc_flag (sp_diag=0x7fd537f0a108, interp=..., src=..., desugar_doc_comments=<error reading variable: access outside bounds of object referenced via synthetic pointer>)
    at /home/mark/Edit/rust/src/libsyntax/ext/tt/transcribe.rs:78
#10 0x00007fd53a426bb2 in syntax::ext::tt::transcribe::new_tt_reader (sp_diag=0x0, interp=..., src=...) at /home/mark/Edit/rust/src/libsyntax/ext/tt/transcribe.rs:62
#11 0x00007fd53a285981 in syntax::parse::tts_to_parser (sess=0x7fd537f0a108, tts=...) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:217
#12 0x00007fd53a28543a in syntax::parse::filemap_to_parser (sess=0x7fd537f0a108, filemap=...) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:166
#13 0x00007fd53a28537b in syntax::parse::new_sub_parser_from_file (sess=0x7fd537f0a108, path=<optimized out>, owns_directory=false, module_name=..., sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:157
#14 0x00007fd53a3209d3 in syntax::parse::parser::{{impl}}::eval_src_mod_from_path (self=<optimized out>, path=..., owns_directory=<optimized out>, name=..., id_sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5492
#15 syntax::parse::parser::{{impl}}::eval_src_mod (self=0x7fd537efa610, id=..., outer_attrs=..., id_sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5467
#16 0x00007fd53a32742c in syntax::parse::parser::{{impl}}::parse_item_mod (outer_attrs=..., self=<optimized out>) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5339
#17 syntax::parse::parser::{{impl}}::parse_item_ (self=<optimized out>, attrs=..., macros_allowed=<optimized out>, attributes_allowed=<optimized out>) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5919
#18 0x00007fd53a330a06 in syntax::parse::parser::{{impl}}::parse_item (self=0x7fd537efa610) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:6097
#19 0x00007fd53a31ed3b in syntax::parse::parser::{{impl}}::parse_mod_items (self=<optimized out>, term=0x7fd53a443238 <ref24582>, inner_lo=BytePos = {...}) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5286
#20 0x00007fd53a320bdc in syntax::parse::parser::{{impl}}::eval_src_mod_from_path (self=<optimized out>, path=..., owns_directory=<optimized out>, name=..., id_sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5495
#21 syntax::parse::parser::{{impl}}::eval_src_mod (self=0x7fd537f052b0, id=..., outer_attrs=..., id_sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5467
#22 0x00007fd53a32742c in syntax::parse::parser::{{impl}}::parse_item_mod (outer_attrs=..., self=<optimized out>) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5339
#23 syntax::parse::parser::{{impl}}::parse_item_ (self=<optimized out>, attrs=..., macros_allowed=<optimized out>, attributes_allowed=<optimized out>) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5919
#24 0x00007fd53a330a06 in syntax::parse::parser::{{impl}}::parse_item (self=0x7fd537f052b0) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:6097
#25 0x00007fd53a31ed3b in syntax::parse::parser::{{impl}}::parse_mod_items (self=<optimized out>, term=0x7fd53a443238 <ref24582>, inner_lo=BytePos = {...}) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5286
#26 0x00007fd53a33182b in syntax::parse::parser::{{impl}}::parse_crate_mod (self=0x7fd537f052b0) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:6185
#27 0x00007fd53a284c18 in syntax::parse::parse_crate_from_file (input=<optimized out>, sess=0x7fd537f0a108) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:85
#28 0x00007fd5417d43a2 in rustc_driver::driver::phase_1_parse_input::{{closure}} () at /home/mark/Edit/rust/src/librustc_driver/driver.rs:493
#29 rustc::util::common::time<core::result::Result<syntax::ast::Crate, rustc_errors::diagnostic_builder::DiagnosticBuilder>,closure> (what=<error reading variable: access outside bounds of object referenced via synthetic pointer>,
    do_it=<optimized out>, f=...) at /home/mark/Edit/rust/src/librustc/util/common.rs:38
#30 rustc_driver::driver::phase_1_parse_input (sess=0x7fd537f096e0, input=0x7fd537f0ab80) at /home/mark/Edit/rust/src/librustc_driver/driver.rs:490
#31 0x00007fd5417cf57b in rustc_driver::driver::compile_input (sess=0x0, cstore=0x7fd530009d40, input=0x7fd537f0ab80, outdir=0x7fd537f0ab68, output=0x7fd537f0ab48, addl_plugins=..., control=0x7fd537f09648)
    at /home/mark/Edit/rust/src/librustc_driver/driver.rs:95
#32 0x00007fd54188a42d in rustc_driver::run_compiler (args=..., callbacks=..., file_loader=..., emitter_dest=...) at /home/mark/Edit/rust/src/librustc_driver/lib.rs:222
#33 0x00007fd54187b252 in rustc_driver::main::{{closure}} () at /home/mark/Edit/rust/src/librustc_driver/lib.rs:1138
#34 rustc_driver::run::{{closure}}<closure> () at /home/mark/Edit/rust/src/librustc_driver/lib.rs:138
#35 rustc_driver::monitor::{{closure}}<closure> () at /home/mark/Edit/rust/src/librustc_driver/lib.rs:1072
#36 std::panic::{{impl}}::call_once<(),closure> (self=...) at /home/mark/Edit/rust/src/libstd/panic.rs:295
#37 std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()> (data=<optimized out>) at /home/mark/Edit/rust/src/libstd/panicking.rs:356
#38 0x00007fd5414eece8 in panic_unwind::__rust_maybe_catch_panic (f=0x7fd54187aed0 <std::panicking::try::do_call<std::panic::AssertUnwindSafe<closure>,()>>, data=0x7fd537eedd98 "", data_ptr=0x7fd537f0dc00, vtable_ptr=0x7fd537f0dc08)
    at /home/mark/Edit/rust/src/libpanic_unwind/lib.rs:97
#39 0x00007fd541885259 in std::panicking::try<(),std::panic::AssertUnwindSafe<closure>> (f=AssertUnwindSafe<closure> = {...}) at /home/mark/Edit/rust/src/libstd/panicking.rs:332
#40 std::panic::catch_unwind<std::panic::AssertUnwindSafe<closure>,()> (f=AssertUnwindSafe<closure> = {...}) at /home/mark/Edit/rust/src/libstd/panic.rs:351
#41 std::thread::{{impl}}::spawn::{{closure}}<closure,()> () at /home/mark/Edit/rust/src/libstd/thread/mod.rs:287
#42 alloc::boxed::{{impl}}::call_box<(),closure> (self=0x556231475090, args=<optimized out>) at /home/mark/Edit/rust/src/liballoc/boxed.rs:595
#43 0x00007fd5414b50f4 in alloc::boxed::{{impl}}::call_once<(),()> (self=...) at /home/mark/Edit/rust/src/liballoc/boxed.rs:605
#44 std::sys_common::thread::start_thread (main=0x5562314751a0) at /home/mark/Edit/rust/src/libstd/sys_common/thread.rs:21
#45 0x00007fd5414bcbb9 in std::sys::imp::thread::{{impl}}::new::thread_start (main=0x7fd537eedd98) at /home/mark/Edit/rust/src/libstd/sys/unix/thread.rs:84
#46 0x00007fd53944f70a in start_thread (arg=0x7fd537f0e700) at pthread_create.c:333
#47 0x00007fd54118282d in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
