

#0  0x082d097f in glue_drop208 ()
#1  0x081d5dc1 in syntax::fold::noop_fold_local ()
#2  0x0830fdeb in syntax::fold::make_fold::thunk5295 ()
#3  0x0830cf72 in syntax::fold::noop_fold_decl::anon5136 ()
#4  0xf7f0e129 in vec::map () from stage1/libstd.so
#5  0x081d0821 in syntax::fold::noop_fold_decl ()
#6  0x0830f555 in syntax::fold::make_fold::thunk5285 ()
#7  0x081cf5e3 in syntax::fold::noop_fold_stmt ()
#8  0x0830f235 in syntax::fold::make_fold::thunk5282 ()
#9  0xf7f0e129 in vec::map () from stage1/libstd.so
#10 0x081cf311 in syntax::fold::noop_fold_block ()
#11 0x0830f13f in syntax::fold::make_fold::thunk5281 ()
#12 0x081d4efe in syntax::fold::noop_fold_fn ()
#13 0x0830fa74 in syntax::fold::make_fold::thunk5289 ()
#14 0x081cdd9d in syntax::fold::noop_fold_item_underscore ()
#15 0x0830efae in syntax::fold::make_fold::thunk5279 ()
#16 0x081cd81e in syntax::fold::noop_fold_item ()
#17 0x0830ef57 in syntax::fold::make_fold::thunk5278 ()
#18 0xf7f0e129 in vec::map () from stage1/libstd.so
#19 0x081d53fa in syntax::fold::noop_fold_mod ()
#20 0x0830fadd in syntax::fold::make_fold::thunk5290 ()
#21 0x081cc087 in syntax::fold::noop_fold_crate ()
#22 0x0830ebcf in syntax::fold::make_fold::thunk5274 ()
#23 0x0823b5a8 in syntax::ext::expand::expand_crate ()
#24 0x0833c769 in driver::rustc::compile_input::thunk7959 ()
#25 0x082a139d in driver::rustc::time ()
#26 0x082a2cd5 in driver::rustc::compile_input ()
#27 0x082bc567 in driver::rustc::main ()
#28 0x082c61b4 in _rust_main ()
#29 0x0834358c in _rust_main_wrap ()
#30 0xf7fa9b31 in task_start_wrapper (a=0xf6b9e010) at ./src/rt/rust_task.cpp:149
#31 0xdeadbeef in ?? ()
