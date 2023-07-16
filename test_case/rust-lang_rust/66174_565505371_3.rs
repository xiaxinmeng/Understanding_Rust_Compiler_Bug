rust
    fn unsound<T> (p: &mut T) -> &mut MaybeUninit<T> { unsafe { transmute(p) } }

    fn exploit<T> (mut x: T) -> T
    {
        *unsound(&mut x) = MaybeUninit::uninitialized();
        x // returns uninitialized::<T>() !
    }
    