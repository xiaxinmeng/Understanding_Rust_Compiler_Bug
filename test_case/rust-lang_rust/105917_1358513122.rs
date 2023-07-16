plain
   Compiling addr2line v0.17.0
error[E0425]: cannot find value `byte` in this scope
    --> library/std/src/io/mod.rs:2466:42
     |
2466 |             read += self.first.read_line(byte)?;

error[E0425]: cannot find value `byte` in this scope
    --> library/std/src/io/mod.rs:2473:39
     |
     |
2473 |         read += self.second.read_line(byte)?;

For more information about this error, try `rustc --explain E0425`.
error: could not compile `std` due to 2 previous errors
Build completed unsuccessfully in 0:00:18
