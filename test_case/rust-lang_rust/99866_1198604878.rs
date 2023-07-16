rust
fn <impl at lol.rs:17:1: 17:19>::iter(_1: HalSetLayouts) -> DSL {
    debug self => _1;                    // in scope 0 at lol.rs:18:22: 18:26
    let mut _0: DSL;                     // return place in scope 0 at lol.rs:18:31: 18:34

    bb0: {
        _0 = move (_1.0: DSL);           // scope 0 at lol.rs:22:9: 22:27
        return;                          // scope 0 at lol.rs:23:6: 23:6
    }
}
