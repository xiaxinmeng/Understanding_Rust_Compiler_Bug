
for<T: Sized> &mut T -> &mut MaybeUninit<T>						(unsafe) (should not require Sized)
for<T: Sized> &mut MaybeUninit<T> -> &mut [MaybeUninit<u8>]		(safe)   (should not require Sized)
