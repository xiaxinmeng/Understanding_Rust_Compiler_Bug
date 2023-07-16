
<anon>:4:9: 4:14 warning: unused variable: `C`, #[warn(unused_variables)] on by default
<anon>:4         C @ 2 => { println!("{}", C); }
                 ^~~~~
Switch constants must all be same type as switch value!
  switch i32 %3, label %match_else [
    i8 1, label %match_case
  ]
LLVM ERROR: Broken function found, compilation aborted!
playpen: application terminated with error code 1
