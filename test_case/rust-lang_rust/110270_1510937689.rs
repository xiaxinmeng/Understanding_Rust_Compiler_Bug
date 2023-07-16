
$ rustup default nightly
$ rustc --version
rustc 1.71.0-nightly (d0f204e4d 2023-04-16)
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
$ cargo clean
$ cargo build -Zbuild-std --target x86_64-unknown-linux-gnu
$ readelf -rW target/x86_64-unknown-linux-gnu/debug/pic-test | head

Relocation section '.rela.dyn' at offset 0xf80 contains 6170 entries:
    Offset             Info             Type               Symbol's Value  Symbol's Name + Addend
00000000001ac910  0000000000000008 R_X86_64_RELATIVE                         3a4c0
00000000001ac918  0000000000000008 R_X86_64_RELATIVE                         26170
00000000001ac920  0000000000000008 R_X86_64_RELATIVE                         26130
00000000001ac928  0000000000000008 R_X86_64_RELATIVE                         159000
00000000001ac938  0000000000000008 R_X86_64_RELATIVE                         26240
00000000001ac950  0000000000000008 R_X86_64_RELATIVE                         261d0
00000000001ac958  0000000000000008 R_X86_64_RELATIVE                         262d0

