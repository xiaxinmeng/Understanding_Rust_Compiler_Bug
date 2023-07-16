

[33mcommit a3f990dc08437ecf63f5e15e8ec6acb9cbedbc14[m
Merge: 2f581cf9d6 6faba5bf8d
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 1 14:28:11 2017 +0000

    Auto merge of #45472 - michaelwoerister:incr-comp-caching-base, r=nikomatsakis
    
    incr.comp.: Implement compiler diagnostic persistence.
    
    This PR implements storing and loading diagnostics that the compiler generates and thus allows for emitting warnings during incremental compilation without actually re-evaluating the thing the warning originally came from. It also lays some groundwork for storing and loading type information and MIR in the incr. comp. cache.
    
    ~~It is still work in progress:~~
    - ~~There's still some documentation to be added.~~
    - ~~The way anonymous queries are handled might lead to duplicated emissions of warnings. Not sure if there is a better way or how frequent such duplication would be in practice.~~
    
    Diagnostic message duplication is addressed separately in #45519.
    
    r? @nikomatsakis

[33mcommit 2f581cf9d692781847bede5d966b098a5d09b5e4[m
Merge: 740286657a 1a7fb7dc78
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 1 09:40:15 2017 +0000

    Auto merge of #45435 - eddyb:binop-subtype-lhs, r=nikomatsakis
    
    rustc_typeck: use subtyping on the LHS of binops.
    
    Fixes #45425.
    
    r? @nikomatsakis

[33mcommit 740286657a97770eca193062fd5e127c08c0808c[m
Merge: 31bbe57c79 028455082e
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 1 07:04:17 2017 +0000

    Auto merge of #45674 - kennytm:rollup, r=kennytm
    
    Rollup of 14 pull requests
    
    - Successful merges: #45450, #45579, #45602, #45619, #45624, #45644, #45646, #45648, #45649, #45650, #45652, #45660, #45664, #45671
    - Failed merges:

[33mcommit 31bbe57c79112e91d2d8783032231c7e1d22855b[m
Merge: fc3e12a03c fbf6885fd3
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 1 04:32:15 2017 +0000

    Auto merge of #45267 - oconnor663:rwlock_send, r=alexcrichton
    
    remove the `T: Sync` requirement for `RwLock<T>: Send`
    
    That requirement makes sense for containers like `Arc` that don't
    uniquely own their contents, but `RwLock` is not one of those.
    
    This restriction was added in https://github.com/rust-lang/rust/commit/380d23b5d4b9fb8f5f0ebf178590f61528b2483e, but it's not clear why. @hniksic
    and I [were discussing this on reddit](https://www.reddit.com/r/rust/comments/763o7r/blog_posts_introducing_lockfree_rust_comparing/dobcvbm/). I might be totally wrong about this change being sound, but I'm super curious to find out :)

[33mcommit fc3e12a03c8d5298ddfac6fb8c14c1b918eb55a8[m
Merge: f3b900cc3b 6fa521c491
Author: bors <bors@rust-lang.org>
Date:   Wed Nov 1 01:45:58 2017 +0000

    Auto merge of #45187 - GuillaumeGomez:doc-ui-improvement, r=QuietMisdreavus
    
    Improve sidebar rendering and add methods list
    
    I suppose it can be reviewed as is, but this is just the first step of a more global plan.
    
    cc @rust-lang/docs @nical
    
    And a screenshot of course:
    
    <img width="1440" alt="screen shot 2017-10-10 at 23 38 45" src="https://user-images.githubusercontent.com/3050060/31412170-657beaf6-ae14-11e7-9f01-1e562a034595.png">

[33mcommit f3b900cc3b122c7e9eb78ca28bec18df68791b08[m
Merge: 8b22e70b2d 4c853adce9
Author: bors <bors@rust-lang.org>
Date:   Tue Oct 31 23:06:37 2017 +0000

    Auto merge of #44764 - nvzqz:master, r=alexcrichton
    
    Implement TryFrom<&[T]> for &[T; N]
    
    There are many cases where a buffer with a static compile-time size is preferred over a slice with a dynamic size. This allows for performing a checked conversion from `&[T]` to `&[T; N]`. This may also lead to compile-time optimizations involving `[T; N]` such as loop unrolling.
    
    This is my first PR to Rust, so I'm not sure if discussion of this change should happen here or does it need its own RFC? I figured these changes would be a subset of #33417.

[33mcommit 8b22e70b2de5152db3b0c53cfa16eb96b0b9e40e[m
Merge: 6713736275 b1fd5a7618
Author: bors <bors@rust-lang.org>
Date:   Tue Oct 31 14:56:06 2017 +0000

    Auto merge of #45655 - alexcrichton:mips-less-cgus, r=michaelwoerister
    
    rustbuild: Don't build with ThinLTO on MIPS
    
    Discovered in #45529 it looks like cross-module TLS imports aren't quite working
    today, especially with `hidden` visibility which mostly comes up with multiple
    codegen units. As a result this completely disables compiling with ThinLTO and
    multiple codegen units on MIPS when bootstrapping.
    
    cc #45654, the tracking issue for this

