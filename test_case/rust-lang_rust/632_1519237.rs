
Starting program: /Users/jruderman/code/rust/build/stage1/rustc -o stage1/fuzzer ../src/fuzzer/fuzzer.rc
../src/fuzzer/fuzzer.rs:106:24:106:113: warning: Unused variable crate
rt: ---
rt: 9736:main:main:                   upcall fail 'explicit failure', /Users/graydon/src/rust-tinderbox/srcdir-snap-stage3-i386-unknown-darwin/src/lib/option.rs:10

Breakpoint 1, rust_task::fail (this=0x1b008c0, nargs=4) at ../src/rt/rust_task.cpp:222
222         DLOG(sched, task, "task %s @0x%" PRIxPTR " failing", name, this);
(gdb) bt
#0  rust_task::fail (this=0x1b008c0, nargs=4) at ../src/rt/rust_task.cpp:222
#1  0x00880451 in option::get ()
#2  0x005503d5 in metadata::decoder::find_item ()
#3  0x00556228 in metadata::decoder::get_tag_variants ()
#4  0x00174d41 in middle::ty::tag_variants ()
#5  0x00014273 in middle::trans::type_of_tag ()
#6  0x000113f1 in middle::trans::type_of_inner ()
#7  0x00011edb in middle::trans::type_of_inner ()
#8  0x0000ea36 in middle::trans::type_of ()
#9  0x0001c75e in middle::trans::static_size_of_tag ()
#10 0x0001450a in middle::trans::type_of_tag ()
#11 0x000113f1 in middle::trans::type_of_inner ()
#12 0x0001232a in middle::trans::type_of_inner ()
#13 0x000114d5 in middle::trans::type_of_inner ()
#14 0x000117a1 in middle::trans::type_of_inner ()
#15 0x0001232a in middle::trans::type_of_inner ()
#16 0x0001232a in middle::trans::type_of_inner ()
#17 0x000114d5 in middle::trans::type_of_inner ()
#18 0x0000ea36 in middle::trans::type_of ()
#19 0x000d3d65 in middle::trans::alloc_ty ()
#20 0x000d3f9d in middle::trans::alloc_local ()
#21 0x005f6f7e in middle::trans::trans_block::foreach2422 ()
#22 0x000d21ba in middle::trans::block_locals ()
#23 0x000d42bc in middle::trans::trans_block ()
#24 0x0007790d in middle::trans::trans_for::inner ()
#25 0x005d73c1 in middle::trans::trans_for::thunk1702 ()
#26 0x00049bd4 in middle::trans::iter_sequence_inner::adaptor_fn ()
#27 0x005cf0be in middle::trans::iter_sequence_inner::thunk1291 ()
#28 0x00048c80 in middle::trans::iter_sequence_raw ()
#29 0x00049f10 in middle::trans::iter_sequence_inner ()
#30 0x0004ab8e in middle::trans::iter_sequence::iter_sequence_body ()
#31 0x0004b45b in middle::trans::iter_sequence ()
#32 0x00078532 in middle::trans::trans_for ()
#33 0x000ad1c5 in middle::trans::trans_expr_out ()
#34 0x000abe09 in middle::trans::trans_expr ()
#35 0x000d4bff in middle::trans::trans_block ()
#36 0x000dcb5c in middle::trans::trans_fn ()
#37 0x000ef8ee in middle::trans::trans_item ()
#38 0x000f15e1 in middle::trans::trans_mod ()
#39 0x000f0bcd in middle::trans::trans_item ()
#40 0x000f15e1 in middle::trans::trans_mod ()
#41 0x00105849 in middle::trans::trans_crate ()
