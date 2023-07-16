plain
   Compiling libz-sys v1.1.3
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_utils/src/sugg.rs:773:63
    |
773 |         let fmpos = cx.sess().source_map().lookup_byte_offset(hi);
    |                                            ------------------ ^^ expected struct `BytePos`, found struct `Span`
    |                                            arguments to this function are incorrect
    |
note: associated function defined here
   --> /checkout/compiler/rustc_span/src/source_map.rs:965:12
   --> /checkout/compiler/rustc_span/src/source_map.rs:965:12
    |
965 |     pub fn lookup_byte_offset(&self, bpos: BytePos) -> SourceFileAndBytePos {

For more information about this error, try `rustc --explain E0308`.
error: could not compile `clippy_utils` due to previous error
warning: build failed, waiting for other jobs to finish...
