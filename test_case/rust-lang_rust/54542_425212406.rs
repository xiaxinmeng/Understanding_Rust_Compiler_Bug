rust
let mut arr: MaybeUninit<[Vec<u8>; 5]> = MaybeUninit::uninitialized();
for i in 0..5 {
    ptr::write(&mut arr.get_mut()[i], vec![]);
}
let arr: [Vec<u8>; 5] = arr.into_inner();
