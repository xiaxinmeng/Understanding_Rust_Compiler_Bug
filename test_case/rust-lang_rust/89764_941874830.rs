rust
bb1: {
    _2 = discriminant(_1);  // typeof(_1) is an uninhabited enum
    switchInt(move _2) -> [0_isize: bb3, 1_isize: bb4, otherwise: bb2]; 
}
