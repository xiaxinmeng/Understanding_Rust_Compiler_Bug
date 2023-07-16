 c
#include<stdio.h>

int main () {
  int arr[1073741824] = {};
  arr[0x200000] = 0xdeadbeef;
  printf("0x%x\n", arr[0x200000]);
}
