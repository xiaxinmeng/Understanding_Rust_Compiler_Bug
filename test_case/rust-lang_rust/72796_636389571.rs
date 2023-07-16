
error: internal compiler error: broken MIR in DefId(0:39 ~ rustc_demangle[8d08]::v0[0]::{{impl}}[1]::try_small_punycode_decode[0]) (end of phase Optimized): encountered `Assign` statement with incompatible types:
left-hand side has type: [char; 128]
right-hand side has type: [char; _]
  --> /home/r/.cargo/registry/src/github.com-1ecc6299db9ec823/rustc-demangle-0.1.16/src/v0.rs:98:23
   |
98 |         let mut out = ['\0'; SMALL_PUNYCODE_LEN];
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^
