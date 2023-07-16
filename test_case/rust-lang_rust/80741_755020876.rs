plain
   Compiling object v0.22.0
   Compiling hashbrown v0.9.0
   Compiling miniz_oxide v0.4.0
   Compiling addr2line v0.14.0
error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
     |
     |
2182 |             Err(_) if path.is_dir() => return Ok(()),
     |                            ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``
     |
     = note: `-D deprecated-in-future` implied by `-D warnings`

error: use of associated function `path::Path::is_dir` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace directories at any time
     |
     |
2193 |             Err(_) if path.is_dir() => Ok(()),
     |                            ^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::read_dir` or `std::fs::metadata``

error: use of associated function `path::Path::is_file` that will be deprecated in future version 1.51.0: other processes may remove, rename, or replace files at any time
 --> library/std/src/sys_common/fs.rs:8:14
  |
8 |     if !from.is_file() {
  |              ^^^^^^^ help: replace the use of the deprecated associated function: `use `std::fs::File::open` or `std::fs::metadata``
error: aborting due to 3 previous errors

error: could not compile `std`

