rust
let _6;

'0:
match $scrutinee {
    VariantA(a) => { .. }
    VariantB(k#placeref _5) => {
        _6 = deref(_5);
        k#goto '2;
    }
    _ => k#goto '3;
}
k#goto '4;

'2:
match _6 {
    VariantX(x) => { .. }
    VariantY(y) => { .. }
    _ => k#goto '3;
}
k#goto '4;

'3:
match $scrutinee {
    c => { .. }
    _ => static_unreachable!(), // "non-behavior"
}
k#goto '4;

'4:
