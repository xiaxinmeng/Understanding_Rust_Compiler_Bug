
    bb0: {
        _1 = <TestStats as Default>::default() -> bb1; // scope 0 at [src/main.rs:24:21: 24:41](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
                                         // mir::Constant
                                         // + span: [src/main.rs:24:21: 24:39](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
                                         // + literal: Const { ty: fn() -> TestStats {<TestStats as Default>::default}, val: Value(<ZST>) }
    }

    bb1: {
        _3 = [generator@[src/main.rs:26:34: 26:41](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#) (#0)] { stats: move _1 }; // scope 1 at [src/main.rs:26:34: 30:6](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)
                                         // generator
                                         // + def_id: DefId(0:8 ~ playground[c2be]::main::{closure#0})
                                         // + substs: [
                                         //     (),
                                         //     i32,
                                         //     (),
                                         //     {i32, ()},
                                         //     (TestStats,),
                                         // ]
                                         // + movability: Movable
