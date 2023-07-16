
In the "reverse-complement" loop, if there is an even number of element,
we forget to complement the element in the middle.  For example, if the
input is "ggg", the result before the fix is "CgC" instead of "CCC".

This is because of this bug that the official shootout says that the rust
version is in "Bad Output".  This commit should fix this error.
