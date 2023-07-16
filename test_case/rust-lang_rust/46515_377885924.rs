C
int foo(restrict int *x, restrict int *y) {
  // The restrict will compile to noalias in LLVM. However, C does not let you
  // optimize the following unless both x and y are actually dereferenced.
  (x == y) ? 1 : 0
}
