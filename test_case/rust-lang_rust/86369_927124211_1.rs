none
error[E0621]: explicit lifetime required in the type of `cursor`
  --> src/lib.rs:19:19
   |
10 |     fn load(cursor: &mut Cursor<&'file [u8]>) -> io::Result<Self> {
   |                     ------------------------ help: add explicit lifetime `'file` to the type of `cursor`: `&'file mut std::io::Cursor<&'file [u8]>`
...
19 |         Ok(Self { data })
   |                   ^^^^ lifetime `'file` required
