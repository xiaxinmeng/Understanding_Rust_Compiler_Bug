rust
let arr: [T; N] = arr.map(|elem: MaybeUninit<T>| unsafe { elem.assume_init() });
