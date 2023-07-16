rust
let _1: &Scrutinee;
let mut _2: isize;
let mut _3: isize;
let a: &_;
let x: &_;
let y: &_;
let c: &Scrutinee;
let mut _8: &Inner;
let mut _9: &Inner;
let mut _10: &Inner;

// entry
bb0: {
    _3 = discriminant((*_1));
    switchInt(move _3) -> [0: bb3, 1: bb1, otherwise: bb2];
}

// VariantB(..)
bb1: {
    _8 = deref_copy (((*_1) as VariantB).0: &Inner);
    _2 = discriminant((*_8));
    switchInt(move _2) -> [0: bb4, 1: bb5, otherwise: bb2];
}

// c =>
bb2: {
    c = _1;
    ..
}

// VariantA(a) =>
bb3: {
    a = &(((*_1) as VariantA).0: _);
    ..
}

// VariantB(VariantX(x)) =>
bb4: {
    _9 = deref_copy (((*_1) as VariantB).0: &Inner);
    x = &(((*_9) as VariantX).0: i32);
    ..
}

// VariantB(VariantY(y)) =>
bb5: {
    _10 = deref_copy (((*_1) as VariantB).0: &Inner);
    y = &(((*_10) as VariantY).0: i32);
    ..
}
