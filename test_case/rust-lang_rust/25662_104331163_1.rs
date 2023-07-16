

If the last swap isn't run, the invariant is violated and my library will likely panic at some future point.


However, this example doesn't violate memory safety and I can't find a way to do so without using `unsafe`.
