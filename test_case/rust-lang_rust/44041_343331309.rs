
SimplifyCFG is the correct pass for this kind of transformation. I
don't think it's entirely unreasonable to run it more often, but the
compile time cost needs to be taken in account.

I'm not necessarily sympathetic to the idea of adding another canonicalization
pass only for this purpose.

Side note:
IMHO, at this point SimplifyCFG is doing even more than it should
(including, e.g. sinking of variable from "almost empty" BBs, i.e. BBs
which have only a single instruction & terminator, if I recall
correctly). If anything, I'd rather split the sinking logic and other operations
to a different pass, rather than moving simplifying proven unconditional
branches elsewhere :)


Now that LLVM has an incremental dominator API it should be more
feasible to run SimplifyCFG more times without recomputing the
dominator all the time. I haven't checked whether there are other
expensive analyses that need to be preserved
