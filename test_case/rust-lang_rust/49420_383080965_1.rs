
$ cat PASSES
-inline -sroa -simplifycfg -instcombine -loop-rotate -loop-idiom
$ rustc --emit=llvm-ir issue-49420.rs -o issue-49420.unpatched.ll -O -Cno-prepopulate-passes
$ /usr/local/Cellar/llvm/6.0.0/bin/opt issue-49420.unpatched.ll -S `cat PASSES` | grep memset
  call void @llvm.memset.p0i8.i64(i8* %result9, i8 42, i64 100000, i32 1, i1 false)
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i32, i1) #1
