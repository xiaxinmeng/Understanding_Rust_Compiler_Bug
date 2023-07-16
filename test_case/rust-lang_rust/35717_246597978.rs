
$ rustc foo.rs --emit=llvm-ir,llvm-bc,obj,asm,link --crate-type=rlib -o bar
...
$ ls -l
-rw-r--r-- 1 kyrias kyrias  812 Sep 13 09:25 bar.bc
-rw-r--r-- 1 kyrias kyrias  284 Sep 13 09:25 bar.ll
-rw-r--r-- 1 kyrias kyrias  856 Sep 13 09:25 bar.o
-rw-r--r-- 1 kyrias kyrias  368 Sep 13 09:25 bar.s
-rw-r--r-- 1 kyrias kyrias   13 Sep 13 09:22 foo.rs
-rw-r--r-- 1 kyrias kyrias 3302 Sep 13 09:25 libfoo.rlib
