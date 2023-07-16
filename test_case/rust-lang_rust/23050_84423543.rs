
(item)➜  rusti git:(master) ✗ cargo run
   Compiling rusti v0.0.1 (file:///Users/lowks/Projects/src/rustlang/rusti)
     Running `target/debug/rusti`
rusti=> let xs: [i32; 5] = [1, 2, 3, 4, 5];
<anon>:13:5: 13:7 warning: unused variable: `xs`, #[warn(unused_variables)] on by default
<anon>:13 let xs: [i32; 5] = [1, 2, 3, 4, 5];
              ^~
LLVM ERROR: Program used external function '___morestack_addr' which could not be resolved!
An unknown error occurred

To learn more, run the command again with --verbose.
