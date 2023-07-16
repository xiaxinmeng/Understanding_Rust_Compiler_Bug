
void demo1(int *restrict x, int *restrict y) {
  // even if x and y are the same, since they are only used to read, we can treat them as noalias
  int i = *x;
  int i = *y;
}
void demo2(int *restrict x, int *restrict y) {
  // even if x and y are the same, since they are not accessed at the same index, we can treat them as noalias
  x[0] = 0;
  y[1] = 0;
}

void main() {
  int arr[2] = {0, 0};
  demo1(arr, arr);
  demo2(arr, arr);
}
