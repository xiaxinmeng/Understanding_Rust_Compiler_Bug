
fn ouch() -> () {
    let mut _0: ();
    let mut _1: !;
    let mut _2: std::fmt::Arguments<'_>;
    let mut _3: &[&str];
    let mut _4: &[&str; 1];
    scope 1 (inlined Arguments::<'_>::new_const) {
        debug pieces => _3;
        let mut _5: std::option::Option<&[core::fmt::rt::Placeholder]>;
        let mut _6: &[core::fmt::rt::Argument<'_>];
        let mut _7: &[core::fmt::rt::Argument<'_>; 0];
    }

    bb0: {
        _4 = const _;
        _3 = _4 as &[&str] (Pointer(Unsize));
        _5 = Option::<&[core::fmt::rt::Placeholder]>::None;
        _7 = const _;
        _6 = _7 as &[core::fmt::rt::Argument<'_>] (Pointer(Unsize));
        _2 = Arguments::<'_> { pieces: _3, fmt: move _5, args: move _6 };
        _1 = panic_fmt(move _2);
    }
}
