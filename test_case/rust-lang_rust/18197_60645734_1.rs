rust
/// # #![allow(unused_must_use)]
/// use std::io::{BufferedWriter, File};
///
/// let file = File::open(&Path::new("message.txt"));
/// let mut writer = BufferedWriter::new(file);
///
/// writer.write_str("hello, world");
/// writer.flush();
/// 