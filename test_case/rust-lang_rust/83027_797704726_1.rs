bash
$ objdump -d regress| grep as_i32x8
$ objdump -d regress-skylake| grep as_i32x8
$ objdump -d regress-native| grep as_i32x8
0000000000006960 <_ZN4core9core_arch3x868m256iExt8as_i32x817h6f0a02a3bdc3d3e7E>:
    6ade:       e8 7d fe ff ff          callq  6960 <_ZN4core9core_arch3x868m256iExt8as_i32x817h6f0a02a3bdc3d3e7E>
    6b0a:       e8 51 fe ff ff          callq  6960 <_ZN4core9core_arch3x868m256iExt8as_i32x817h6f0a02a3bdc3d3e7E>
