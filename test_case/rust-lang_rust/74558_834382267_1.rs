
$ clang -c foo.s
foo.s:2:18: error: unexpected token!
mov  edi, offset 2f
                 ^

foo.s:4:17: error: unexpected token!
   push  offset 2b
                ^
