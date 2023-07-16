
unsafe impl<I> TrustedLen for Skip<I> where I: ExactSizeIterator + TrustedLen {}
