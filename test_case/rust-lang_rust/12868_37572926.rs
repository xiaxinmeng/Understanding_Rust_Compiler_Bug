
// lldb thinks it's returning a function, so call it again!
(lldb) print factorial(4)(3)
zsh: segmentation fault  lldb ./inspect

// cast the return value to an integer.
(lldb) print (long) factorial(4)
Instruction returns a non-scalar type!
  %9 = call i64 (i64) inttoptr (i64 4294972928 to i64 (i64) (i64)*)(i64 4), !lldb.call.realName !9
PtrToInt source must be pointer
  %10 = ptrtoint i64 (i64) %9 to i64
Broken module found, compilation aborted!
zsh: abort      lldb ./inspect

