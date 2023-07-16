
% RUSTC_LOG=syntax::parse,syntax::source_map ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc  /tmp/off_index.rs
DEBUG 2019-07-11T12:58:22Z: syntax::parse::attr: parse_outer_attributes: self.token=Token { kind: OpenDelim(Paren), spa\
n: Span { lo: BytePos(8), hi: BytePos(9), ctxt: #0 } }
DEBUG 2019-07-11T12:58:22Z: syntax::parse::parser: parse_arg_general parse_pat (is_name_required:true)
DEBUG 2019-07-11T12:58:22Z: syntax::source_map: find_width_of_character_at_span: local_begin=`SourceFileAndBytePos { sf\
: SourceFile(/tmp/off_index.rs), pos: BytePos(10) }`, local_end=`SourceFileAndBytePos { sf: SourceFile(/tmp/off_index.r\
s), pos: BytePos(11) }`
DEBUG 2019-07-11T12:58:22Z: syntax::source_map: find_width_of_character_at_span: start_index=`10`, end_index=`11`
DEBUG 2019-07-11T12:58:22Z: syntax::source_map: find_width_of_character_at_span: source_len=`11`
thread 'rustc' panicked at 'byte index 10 is not a char boundary; it is inside 'ؼ' (bytes 9..11) of `fn main((ؼ`', src/\
libcore/str/mod.rs:2039:5
