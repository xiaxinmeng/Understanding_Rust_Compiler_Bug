rust

#[repr(u8)]
#[derive(Debug)]
enum Test {
    A = 1,
    B = 2,
}

impl Drop for Test {
    fn drop(&mut self) {}
}

let v = vec![Test::A, Test::B];
let v2 = v.map_in_place(|x| match x {
    Test::A => 0u8,
    Test::B => panic!(),
});
