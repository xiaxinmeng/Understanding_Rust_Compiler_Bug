 rust
    bb0: {
        /* snip */
        _2 = A::A(const 1u32,);          // scope 0 at <anon>:12:15: 12:19
        _1 = &_2;                        // scope 0 at <anon>:12:13: 12:25
        drop(_2) -> bb1;                 // scope 0 at <anon>:12:15: 12:19
    }

    bb1: {
        /* snip */
        _4 = ((*_1).0: u32);             // scope 1 at <anon>:13:5: 13:8
        /* ... */
