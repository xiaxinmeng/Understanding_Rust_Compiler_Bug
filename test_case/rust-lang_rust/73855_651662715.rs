
 mateusz@arch  /tmp  gcc hello.c -o hello && readelf -S hello | grep -A1 -i hash
  [ 4] .gnu.hash         GNU_HASH         0000000000000308  00000308
       000000000000001c  0000000000000000   A       5     0     8
 mateusz@arch  /tmp  clang hello.c -o hello && readelf -S hello | grep -A1 -i hash
  [ 3] .hash             HASH             00000000000002e8  000002e8
       0000000000000030  0000000000000004   A       5     0     8
  [ 4] .gnu.hash         GNU_HASH         0000000000000318  00000318
       000000000000001c  0000000000000000   A       5     0     8
