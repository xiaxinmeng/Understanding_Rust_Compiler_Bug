rust
iter::repeat_with(|| MaybeUninit::<T>::uninit()) // you can use `repeat` in case `T: Copy`
    .take(20)
    .collect::<Wrapper<[_]>>() // Wrapper is Box/Rc/Arc
