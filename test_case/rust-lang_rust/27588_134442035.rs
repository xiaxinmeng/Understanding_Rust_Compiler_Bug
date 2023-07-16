
../src/libstd/io/error.rs:160:5: 160:18 error: non-deprecated unstable items need to point to an issue with `issue = "NNN"`
../src/libstd/io/error.rs:160     UnexpectedEOF,
                                  ^~~~~~~~~~~~~
../src/libstd/io/mod.rs:596:5: 611:6 error: non-deprecated unstable items need to point to an issue with `issue = "NNN"`
../src/libstd/io/mod.rs:596     fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
../src/libstd/io/mod.rs:597         while !buf.is_empty() {
../src/libstd/io/mod.rs:598             match self.read(buf) {
../src/libstd/io/mod.rs:599                 Ok(0) => break,
../src/libstd/io/mod.rs:600                 Ok(n) => { let tmp = buf; buf = &mut tmp[n..]; }
../src/libstd/io/mod.rs:601                 Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                            ...
error: aborting due to 2 previous errors
