C
int foo(restrict int *x, restrict int *y) {
  (x == y) ? 1 : 0
}
