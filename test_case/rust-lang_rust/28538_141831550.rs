
unsafe impl<E, A: Unsize<[E]>> FixedSizeArray for A {
    type T = A;

    #[inline]
    fn as_slice(&self) -> &[Self::T] {
        self
    }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::T] {
        self
    }
}
