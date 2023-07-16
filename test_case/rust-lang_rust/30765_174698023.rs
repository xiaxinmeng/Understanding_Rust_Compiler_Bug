
thread '<main>' panicked at 'position 0 does not resolve to a source location', src/libsyntax/codemap.rs:1064

Breakpoint 1, 0x000055555577abf0 in rust_panic ()
(gdb) bt
#0  0x000055555577abf0 in rust_panic ()
#1  0x0000555555775f83 in sys_common::unwind::begin_unwind_inner::h5a8431dee636759bfgt ()
#2  0x0000555555776369 in sys_common::unwind::begin_unwind_fmt::hfa6b9a8c65cd0705lft ()
#3  0x00005555555bc85e in codemap::CodeMap::lookup_filemap_idx::h3b3fd7c2fa2bb1c3TFD ()
#4  0x00005555555bc643 in codemap::CodeMap::bytepos_to_file_charpos::h168ad2fe9d3430b6oCD ()
#5  0x000055555559679d in codemap::CodeMap::lookup_char_pos::h4159a17f5570f91cgeD ()
#6  0x0000555555595224 in codemap::CodeMap::span_to_lines::hb0a634d700a83c418uD ()
#7  0x000055555558fab2 in errors::emitter::EmitterWriter::emit_::h30e316131ba94c17B5b ()
#8  0x000055555558dfe6 in errors::emitter::EmitterWriter.Emitter::emit::hd5a2f310c1cd3936A1b ()
#9  0x000055555558e27b in errors::emitter::Emitter::emit_struct::h10020655746231161034 ()
#10 0x00005555557462a3 in ext::quote::parse_expr_panic::h17ebcf9dc12ef60fltb ()
#11 0x000055555556748b in foo::main () at foo.rs:29
#12 0x00005555557812e5 in sys_common::unwind::try::try_fn::h3804301371837416440 ()
#13 0x000055555577ebb9 in __rust_try ()
#14 0x0000555555780f1e in rt::lang_start::h620f739da2fd976doxy ()
#15 0x0000555555584fca in main ()
