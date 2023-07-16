
commit 0f12bad718bb8b1fb2b1b7267a02fffaef6e1e3c (HEAD) <- *Note*: this one also tested "bad"
Merge: a333eed7fc0 994d83666de
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 13 00:42:19 2019 +0000

    Auto merge of #66156 - Mark-Simulacrum:stage0-step, r=pietroalbini
    
    Stage0 step
    
    r? @pietroalbini

commit 994d83666defc0cc6b0fde305d164fbf23433114
Author: Mark Rousskov <mark.simulacrum@gmail.com>
Date:   Tue Nov 12 16:37:22 2019 -0500

    Remove no longer needed mutability

commit f6832adadb84364ce0c81fa02910b3706f441abc
Author: Mark Rousskov <mark.simulacrum@gmail.com>
Date:   Wed Nov 6 15:17:02 2019 -0500

    Compiletest bump to stage0 bootstrap libtest

commit 8783766215c28a98eeb0487d865b8afa0ada797c
Author: Mark Rousskov <mark.simulacrum@gmail.com>
Date:   Tue Nov 5 11:16:46 2019 -0500

    Hopefully fix rustdoc build
    
    It's super unclear why this broke when we switched to beta but not
    previously -- but at least it's hopefully fixed now.

commit 997feacddd8f6e98003428265c665f7149c49a48 (refs/bisect/bad)
Author: Mark Rousskov <mark.simulacrum@gmail.com>
Date:   Wed Nov 6 08:09:55 2019 -0500

    Snap cfgs

commit f4edc81ac4944c2cd59e19fd90861a26dde94041 <- *Note*: does not compile!
Author: Mark Rousskov <mark.simulacrum@gmail.com>
Date:   Wed Nov 6 08:01:56 2019 -0500

    Bump version to 1.41

commit a333eed7fc0c903df9d6befcfb40af02148bf255 (refs/bisect/good-a333eed7fc0c903df9d6befcfb40af02148bf255)
Merge: 4f03f4a989d b4545a4ad62
Author: bors <bors@rust-lang.org>
Date:   Tue Nov 12 21:27:04 2019 +0000

    Auto merge of #60026 - Aaron1011:feature/miri-unwind, r=RalfJung,oli-obk
    
    Add hooks for Miri panic unwinding
    
    This commits adds in some additional hooks to allow Miri to properly
    handle panic unwinding. None of this should have any impact on CTFE mode
    
    This supports https://github.com/rust-lang/miri/pull/693

commit 4f03f4a989d1c8346c19dfb417a77c09b34408b8 (refs/bisect/good-4f03f4a989d1c8346c19dfb417a77c09b34408b8)
Merge: 5dda3ee9314 4bf0685cca1
Author: bors <bors@rust-lang.org>
Date:   Tue Nov 12 18:02:54 2019 +0000

    Auto merge of #65608 - matthewjasper:mir-eval-order, r=pnkfelix
    
    Fix MIR lowering evaluation order and soundness bug
    
    * Fixes a soundness issue with built-in index operations
    * Ensures correct evaluation order of assignment expressions where the RHS is a FRU or is a use of a local of reference type.
    * Removes an unnecessary symbol to string conversion
    
    closes #65909
    closes #65910
