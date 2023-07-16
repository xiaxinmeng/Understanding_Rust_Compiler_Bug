plain
   --> library/std/src/io/util.rs:103:1
    |
103 | / impl Write for Empty {
104 | |     #[inline]
105 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
106 | |         Ok(buf.len())
123 | |     }
124 | | }
    | |_^


error: implementation has missing stability attribute
   --> library/std/src/io/util.rs:126:1
    |
126 | / impl Write for &Empty {
127 | |     #[inline]
128 | |     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
129 | |         Ok(buf.len())
146 | |     }
147 | | }
    | |_^

