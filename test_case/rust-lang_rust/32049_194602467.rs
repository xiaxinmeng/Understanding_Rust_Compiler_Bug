
7:10 PM <eddyb> larsberg: ugh, isAllocaPromotable is non-trivial and basically depends on all the uses of allocas
7:10 PM  → @brson (opped) joined  
7:11 PM <eddyb> it has to be SROA::rewritePartition, which has its own promotable checks
7:14 PM <eddyb> larsberg: can you get a LLVM debug build?
7:14 PM <eddyb> larsberg: I severely understimated the size of this code
7:14 PM <eddyb> a debugger would let you "call AI->dump()" and at least get to see which alloca it is
7:15 PM  ↔ japaric (was japaric_) nipped out  
7:16 PM <@larsberg> eddyb: Sure, though I might have to do it in a couple of days. I am frantically trying to hack around this so that I can get a one-off ARM32 build of Servo together for a team to do some energy profiling on a board for a paper they're trying to get out in the next few days :-)
7:16 PM <eddyb> :(
7:16 PM <eddyb> larsberg: this is the worst bug
