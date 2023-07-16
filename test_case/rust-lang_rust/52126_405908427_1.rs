
[33mcommit 2be4cc040211a85b17f21e813ff62351ae4de642[m
Merge: a3f990dc08 aae3e74e70
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 1 18:14:13 2017 +0000

    Auto merge of #45538 - nikomatsakis:nll-liveness, r=pnkfelix
    
    enable non-lexical lifetimes in the MIR borrow checker
    
    This PR, joint work with @spastorino, fills out the NLL infrastructure and integrates it with the borrow checker. **Don't get too excited:** it includes still a number of hacks (the subtyping code is particularly hacky). However, it *does* kinda' work. =)
    
    The final commit demonstrates this by including a test that -- with both the AST borrowck and MIR borrowck -- reports an error by default. But if you pass `-Znll`, you only get an error from the AST borrowck, demonstrating that the integration succeeds:
    
    