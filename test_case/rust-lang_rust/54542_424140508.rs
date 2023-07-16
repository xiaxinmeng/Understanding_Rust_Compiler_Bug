rust
let arr: [MaybeUninit<Vec<u8>>; 5] = [MaybeUninit::uninitialized(); 5];
for v in &mut arr {
    v.set(vec![]);
}
let arr: [Vec<u8>; 5] = mem::transmute(arr);
