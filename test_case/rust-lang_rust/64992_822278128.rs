rust
const fn test() {
    let _x = NonNull::<[i32; 0]>::dangling() as NonNull<[i32]>;
}
