
map: /abc/def -> /xyz

Absolute paths containing the prefix:
/abc/def/file1.rs -> /xyz/file1.rs
/abc/def/build/../file1.rs -> /xyz/file1.rs
/abc/def/./file1.rs -> /xyz/file1.rs
/abc/./def/file1.rs -> /xyz/file1.rs  // would not match with gcc
/abc/def/mod1/file1.rs -> /xyz/mod1/file1.rs 
/abc/def/mod1/./file1.rs -> /xyz/mod1/file1.rs 

Absolute paths not containing the prefix:
/std/file1.rs -> /std/file1.rs // no change
/std/./file1.rs -> /std/file1.rs // normalization
/std/build/../file1.rs -> /std/file1.rs // normalization

Relative paths containing the prefix:
(DW_AT_comp_dir=/abc/def/build, path=../file.rs) => /xyz/file1.rs
(DW_AT_comp_dir=/abc, path=./def/file.rs) => /xyz/file1.rs  // would not match with gcc
(DW_AT_comp_dir=/std, path=./mod1/file.rs) => /std/mod1/file1.rs

Mapping happens at the directory name level, no partial names allowed:
/abc/def-2/file1.rs -> /abc/def-2/file1.rs
/abc/def.rs -> /abc/def.rs
