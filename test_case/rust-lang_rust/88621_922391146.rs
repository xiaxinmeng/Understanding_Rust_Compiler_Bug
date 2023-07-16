rust
#[repr(u8)]
enum E {
    V1 = 1,
    V2(),
    V3{}
}

let x = E::V2() as u8;
