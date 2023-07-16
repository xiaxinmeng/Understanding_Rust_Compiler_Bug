 Rust
bb0:
    if condition then bb1 else bb3
bb1:
    tmp0 = Foo::new() uwto cleanup0
    replace x <- tmp0 uwto cleanup2
bb3:
    tmp1 = Foo::new() uwto cleanup3
    replace x <- tmp1 uwto cleanup3
bb5:
    drop var0
    return
cleanup3:
    drop tmp1 -> cleanup1
cleanup2:
    drop tmp0
cleanup1:
    drop x
cleanup0:
    resume
