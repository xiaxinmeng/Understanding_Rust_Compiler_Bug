shell
~/Temp/rust$ git log
commit 32717603f61a566ff0b8293ef3177cb7c4f50fa9 (HEAD -> master, origin/master, origin/HEAD)
Merge: 1755c853028 42321b01e0a
Author: bors <bors@rust-lang.org>
Date:   Fri Oct 14 07:41:55 2022 +0000

    Auto merge of #102695 - compiler-errors:int-and-float-trivial-copy, r=lcnr
    
    Int and float inference variables are trivially copy
    
    Fixes #102645
