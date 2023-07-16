
$ cd src/etc/platform-intrinsics/x86
$ python2 ../generator.py --format compiler-defs -i info.json   \
  sse.json sse2.json sse3.json ssse3.json sse41.json sse42.json \
  avx.json avx2.json fma.json                                   \
  > ../../../librustc_platform_intrinsics/x86.rs
