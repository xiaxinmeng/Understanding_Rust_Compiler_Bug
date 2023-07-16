
const <impl at [src/main.rs:4:1: 4:11](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)>::FOO: Thing = {
    let mut _0: Thing;                   // return place in scope 0 at [src/main.rs:5:20: 5:25](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)

    bb0: {
        Deinit(_0);                      // scope 0 at [src/main.rs:5:28: 5:44](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
        ((_0 as Foo).0: bool) = const true; // scope 0 at [src/main.rs:5:28: 5:44](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
        discriminant(_0) = 0;            // scope 0 at [src/main.rs:5:28: 5:44](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
        return;                          // scope 0 at [src/main.rs:5:5: 5:45](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
    }
}

const <impl at [src/main.rs:4:1: 4:11](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)>::BAR: Thing = {
    let mut _0: Thing;                   // return place in scope 0 at [src/main.rs:6:20: 6:25](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)

    bb0: {
        _0 = Thing::Foo(const true) -> bb1; // scope 0 at [src/main.rs:6:28: 6:43](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
                                         // mir::Constant
                                         // + span: [src/main.rs:6:28: 6:37](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
                                         // + literal: Const { ty: fn(bool) -> Thing {Thing::Foo}, val: Value(<ZST>) }
    }

    bb1: {
        return;                          // scope 0 at [src/main.rs:6:5: 6:44](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021#)
    }
}
