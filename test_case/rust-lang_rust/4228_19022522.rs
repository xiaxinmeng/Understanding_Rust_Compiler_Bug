 rust

impl<T> Vec4<T> {
    #[inline(always)]
    fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4 { x: x, y: y, z: z, w: w }
    }

    #[inline(always)]
    fn len() -> uint { 4 }

    //...
}

impl<T:Copy + Num> Vec4<T> {
    #[inline(always)]
    fn identity() -> Vec4<T> {
        Vec4::new(One::one::<T>(), One::one::<T>(), One::one::<T>(), One::one::<T>())
    }

    #[inline(always)]
    fn zero() -> Vec4<T> {
        Vec4::new(Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>(), Zero::zero::<T>())
    }

    //...
}
