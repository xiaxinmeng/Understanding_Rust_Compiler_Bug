
$ rustc main_start_1.rs -C opt-level=2 -o builds/main_start_1
 
$ objdump -x builds/main_start_1 | grep main
 builds/main_start_1:     file format mach-o-x86-64
 builds/main_start_1
 00000001000008b0 l       0e SECT   01 0000 [.text] __ZN12main_start_14main17hb1ec1b2167cb471eE
 00000001000008c0 g       0f SECT   01 0000 [.text] _main
 
$ objdump -D --start-address=0x1000008b0 builds/main_start_1  | awk '{print $0} $3~/retq?/{exit}'
 
 builds/main_start_1:     file format mach-o-x86-64
 
 
 Disassembly of section .text:
 
 00000001000008b0 <__ZN12main_start_14main17hb1ec1b2167cb471eE>:
    1000008b0:   55                      push   %rbp
    1000008b1:   48 89 e5                mov    %rsp,%rbp
    1000008b4:   5d                      pop    %rbp
    1000008b5:   c3                      retq
 
$ rustc -vV
 rustc 1.18.0-nightly (bbdaad0dc 2017-04-14)
 binary: rustc
 commit-hash: bbdaad0dc8dc64e036ccee817f90a91876b32a9d
 commit-date: 2017-04-14
 host: x86_64-apple-darwin
 release: 1.18.0-nightly
 LLVM version: 3.9
