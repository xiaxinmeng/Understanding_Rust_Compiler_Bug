
$ cargo test
   Compiling <crate> v0.0.1 (file:///path/to/crate)
src/lib.rs:159:5: 166:6 src/lib.rs:159:5: 166:6 (BROWN BEGIN)warning: (BROWN END)(YELLOW BEGIN)method is never used: `function`, #[warn(dead_code)] on by default(YELLOW END)
warning: method is never used: `function`, #[warn(dead_code)] on by default
src/lib.rs:159     fn function(&self) -> <result> {
src/lib.rs:160         <code>
src/lib.rs:src/lib.rs161: 159        if <code> { 
    fn function(&self) -> <result> {src/lib.rs
:src/lib.rs162: 160            <code> 
        <code>src/lib.rs:163         } else {
src/lib.rs:164             <code>

  src/lib.rs : 161           <code> { 
...
src/lib.rs:162             <code>
src/lib.rs:163         } else {
src/lib.rs:164             <code>
               ...
