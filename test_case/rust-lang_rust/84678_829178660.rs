
> rg 'UnexpectedEof, ' library/
library/std/src/sys/unix/ext/fs.rs
112:            Err(io::Error::new_const(io::ErrorKind::UnexpectedEof, &"failed to fill whole buffer"))

library/std/src/sys/wasi/ext/fs.rs
88:            Err(io::Error::new_const(io::ErrorKind::UnexpectedEof, &"failed to fill whole buffer"))

library/std/src/io/mod.rs
432:        Err(Error::new_const(ErrorKind::UnexpectedEof, &"failed to fill whole buffer"))

library/std/src/io/impls.rs
266:            return Err(Error::new_const(ErrorKind::UnexpectedEof, &"failed to fill whole buffer"));
