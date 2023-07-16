
    bb0: {
        var0 = arg0;                     // we don't need this copy
        tmp0 = var0;                     // this one too (but need source propagation)
        return = whatever(tmp0, const 0u32) -> bb1; // scope 3 at <anon>:2:5: 2:19
    }
