 rust
// Where OutputWriter is a struct and OutputStream is a trait it implements
let mut writer = OutputWriter::new(counter.bytecount);
let &mut writer_stream = &mut writer as &mut OutputStream;

do_something_with_writer_stream(writer_stream);

// Calling writer.bytes() moves the original writer out, so
// the compiler complains because there is still a mutable loan outstanding.
writer.bytes();
