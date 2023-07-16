
$ cargo test
   Compiling <crate> v0.0.1 (file:///path/to/crate)
src/lib.rs:154:5: 161:6 src/lib.rs:154:5: 161:6 (BROWN BEGIN)warning: (BROWN END)method is never used: `function`, #[warn(dead_code)] on by default
(BROWN BEGIN)warning: src/lib.rs:154     fn function(&self) -> <result> {
src/lib.rs:158         } else {
src/lib.rs:159             <code>
               ...(BROWN END)
method is never used: `function`, #[warn(dead_code)] on by default
src/lib.rs:154     fn function(&self) -> <result> {
src/lib.rs:155         <code>
src/lib.rs:156         if <code> {
src/lib.rs:157             <code>
src/lib.rs:158         } else {
src/lib.rs:159             <code>
               ...
