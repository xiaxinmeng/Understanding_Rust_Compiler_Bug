
> x check compiler/rustc --stage 0
Building bootstrap
[ ... ]
Checking stage0 library artifacts (x86_64-unknown-linux-gnu)
[ ... ]
Checking stage0 library test/bench/example targets (x86_64-unknown-linux-gnu)
[ ... ]
Checking stage0 compiler artifacts (x86_64-unknown-linux-gnu)
[ ... ]
> x check compiler/rustc --stage 1
Building bootstrap
[ ... ]
Building stage0 library artifacts (x86_64-unknown-linux-gnu) 
[ ... ]
Building compiler artifacts (stage0 -> stage1)
[ ... ]
Assembling stage1 compiler
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
[ ... ]
Checking stage1 compiler artifacts (x86_64-unknown-linux-gnu)
[ ... ]
> x check compiler/rustc --stage 2
Building bootstrap
[ ... ]
Building stage0 library artifacts (x86_64-unknown-linux-gnu) 
[ ... ]
Building compiler artifacts (stage0 -> stage1)
[ ... ]
Assembling stage1 compiler
Building stage1 library artifacts (x86_64-unknown-linux-gnu) 
[ ... ]
Building compiler artifacts (stage1 -> stage2)
[ ... ]
Assembling stage2 compiler
Uplifting library (stage1 -> stage2)
Checking stage2 compiler artifacts (x86_64-unknown-linux-gnu)
[ ... ]
Build completed successfully in 0:00:02
> x check library --stage 0
Building bootstrap
[ ... ]
Checking stage0 library artifacts (x86_64-unknown-linux-gnu)
[ ... ]
Checking stage0 library test/bench/example targets (x86_64-unknown-linux-gnu)
[ ... ]
Build completed successfully in 0:00:01
> x check library --stage 1
Building bootstrap
[ ... ]
Building stage0 library artifacts (x86_64-unknown-linux-gnu) 
[ ... ]
Building compiler artifacts (stage0 -> stage1)
[ ... ]
Assembling stage1 compiler
Checking stage1 library artifacts (x86_64-unknown-linux-gnu)
[ ... ]
Checking stage1 library test/bench/example targets (x86_64-unknown-linux-gnu)
[ ... ]
