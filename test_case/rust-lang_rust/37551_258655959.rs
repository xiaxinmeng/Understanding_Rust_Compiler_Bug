
#0  alloc::rc::RcBoxPtr::dec_strong<alloc::rc::Rc<syntax::tokenstream::Delimited>,syntax::tokenstream::Delimited> (self=0x7fffee1bed98)
    at /home/mark/Edit/rust/src/liballoc/rc.rs:1115
#1  alloc::rc::{{impl}}::drop<syntax::tokenstream::Delimited> (self=0x7fffee1bed98) at /home/mark/Edit/rust/src/liballoc/rc.rs:648
#2  0x00007ffff06e66ad in rustc_data_structures::array_vec::{{impl}}::push<[syntax::ext::tt::transcribe::TtFrame; 1]> (self=0x7fffee1bed80, el=...)
    at /home/mark/Edit/rust/src/librustc_data_structures/array_vec.rs:70
#3  0x00007ffff06f808e in rustc_data_structures::array_vec::{{impl}}::extend<[syntax::ext::tt::transcribe::TtFrame; 1],core::iter::sources::Once<syntax::ext::tt::transcribe::TtFrame>> (self=<optimized out>, iter=...) at /home/mark/Edit/rust/src/librustc_data_structures/array_vec.rs:132
#4  rustc_data_structures::accumulate_vec::{{impl}}::from_iter<[syntax::ext::tt::transcribe::TtFrame; 1],core::iter::sources::Once<syntax::ext::tt::transcribe::TtFrame>> (
    iter=...) at /home/mark/Edit/rust/src/librustc_data_structures/accumulate_vec.rs:112
#5  core::iter::iterator::Iterator::collect<core::iter::sources::Once<syntax::ext::tt::transcribe::TtFrame>,rustc_data_structures::accumulate_vec::AccumulateVec<[syntax::ext::tt::transcribe::TtFrame; 1]>> (self=...) at /home/mark/Edit/rust/src/libcore/iter/iterator.rs:1196
#6  rustc_data_structures::accumulate_vec::{{impl}}::one<[syntax::ext::tt::transcribe::TtFrame; 1]> (el=...)
    at /home/mark/Edit/rust/src/librustc_data_structures/accumulate_vec.rs:48
#7  rustc_data_structures::small_vec::{{impl}}::one<[syntax::ext::tt::transcribe::TtFrame; 1]> (el=...) at /home/mark/Edit/rust/src/librustc_data_structures/small_vec.rs:51
#8  syntax::ext::tt::transcribe::new_tt_reader_with_doc_flag (sp_diag=0x7fffee1db108, interp=..., src=...,
    desugar_doc_comments=<error reading variable: access outside bounds of object referenced via synthetic pointer>)
    at /home/mark/Edit/rust/src/libsyntax/ext/tt/transcribe.rs:78
#9  0x00007ffff06f7bb2 in syntax::ext::tt::transcribe::new_tt_reader (sp_diag=0x0, interp=..., src=...) at /home/mark/Edit/rust/src/libsyntax/ext/tt/transcribe.rs:62
#10 0x00007ffff0556981 in syntax::parse::tts_to_parser (sess=0x7fffee1db108, tts=...) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:217
#11 0x00007ffff055643a in syntax::parse::filemap_to_parser (sess=0x7fffee1db108, filemap=...) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:166
#12 0x00007ffff055637b in syntax::parse::new_sub_parser_from_file (sess=0x7fffee1db108, path=<optimized out>, owns_directory=false, module_name=..., sp=...)
    at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:157
#13 0x00007ffff05f19d3 in syntax::parse::parser::{{impl}}::eval_src_mod_from_path (self=<optimized out>, path=..., owns_directory=<optimized out>, name=..., id_sp=...)
    at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5492
#14 syntax::parse::parser::{{impl}}::eval_src_mod (self=0x7fffee1cb610, id=..., outer_attrs=..., id_sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5467
#15 0x00007ffff05f842c in syntax::parse::parser::{{impl}}::parse_item_mod (outer_attrs=..., self=<optimized out>) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5339
#16 syntax::parse::parser::{{impl}}::parse_item_ (self=<optimized out>, attrs=..., macros_allowed=<optimized out>, attributes_allowed=<optimized out>)
    at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5919
#17 0x00007ffff0601a06 in syntax::parse::parser::{{impl}}::parse_item (self=0x7fffee1cb610) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:6097
#18 0x00007ffff05efd3b in syntax::parse::parser::{{impl}}::parse_mod_items (self=<optimized out>, term=0x7ffff0714238 <ref24582>, inner_lo=BytePos = {...})
    at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5286
#19 0x00007ffff05f1bdc in syntax::parse::parser::{{impl}}::eval_src_mod_from_path (self=<optimized out>, path=..., owns_directory=<optimized out>, name=..., id_sp=...)
    at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5495
#20 syntax::parse::parser::{{impl}}::eval_src_mod (self=0x7fffee1d62b0, id=..., outer_attrs=..., id_sp=...) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5467
#21 0x00007ffff05f842c in syntax::parse::parser::{{impl}}::parse_item_mod (outer_attrs=..., self=<optimized out>) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5339
#22 syntax::parse::parser::{{impl}}::parse_item_ (self=<optimized out>, attrs=..., macros_allowed=<optimized out>, attributes_allowed=<optimized out>)
    at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5919
#23 0x00007ffff0601a06 in syntax::parse::parser::{{impl}}::parse_item (self=0x7fffee1d62b0) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:6097
#24 0x00007ffff05efd3b in syntax::parse::parser::{{impl}}::parse_mod_items (self=<optimized out>, term=0x7ffff0714238 <ref24582>, inner_lo=BytePos = {...})
    at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:5286
#25 0x00007ffff060282b in syntax::parse::parser::{{impl}}::parse_crate_mod (self=0x7fffee1d62b0) at /home/mark/Edit/rust/src/libsyntax/parse/parser.rs:6185
#26 0x00007ffff0555c18 in syntax::parse::parse_crate_from_file (input=<optimized out>, sess=0x7fffee1db108) at /home/mark/Edit/rust/src/libsyntax/parse/mod.rs:85
