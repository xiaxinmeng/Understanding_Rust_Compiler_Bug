plain
    Checking rustc-demangle v0.1.21
error: associated function has missing const stability attribute
    --> library/alloc/src/vec/mod.rs:1911:5
     |
1911 | /     pub const fn len(&self) -> usize {
1912 | |         self.len
     | |_____^

error: associated function has missing const stability attribute
    --> library/alloc/src/vec/mod.rs:1927:5
    --> library/alloc/src/vec/mod.rs:1927:5
     |
1927 | /     pub const fn is_empty(&self) -> bool {
1928 | |         self.len() == 0
     | |_____^

error: could not compile `alloc` due to 2 previous errors
Build completed unsuccessfully in 0:01:23
