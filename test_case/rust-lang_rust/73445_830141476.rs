
error[E0773]: attempted to define built-in macro more than once
    --> /home/joshua/rust/library/core/src/macros/mod.rs:1201:5
     |
1201 | /     macro_rules! cfg {
1202 | |         ($($cfg:tt)*) => {
1203 | |             /* compiler built-in */
1204 | |         };
1205 | |     }
     | |_____^
     |
note: previously defined here
    --> /home/joshua/rust/library/core/src/macros/mod.rs:1201:5
     |
1201 | /     macro_rules! cfg {
1202 | |         ($($cfg:tt)*) => {
1203 | |             /* compiler built-in */
1204 | |         };
1205 | |     }
     | |_____^

error: Compilation failed, aborting rustdoc
