
commit 58e8c793d0e43150a6452e971a32d7407a8a7401
Author: Tim Northover <tnorthover@apple.com>
Date:   Mon Sep 30 07:46:52 2019 +0000

    Revert "[SCEV] add no wrap flag for SCEVAddExpr."
    
    This reverts r366419 because the analysis performed is within the context of
    the loop and it's only valid to add wrapping flags to "global" expressions if
    they're always correct.
    
    llvm-svn: 373184
