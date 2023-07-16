
C:\msys64\home\Peter\word_generator>cargo build
    Updating git repository `https://github.com/rust-lang/rand.git`
    Updating registry `https://github.com/rust-lang/crates.io-index`
 Downloading libc v0.1.6
 Downloading rustc-serialize v0.3.14
 Downloading unicode-segmentation v0.1.0
 Downloading regex v0.1.30
   Compiling unicode-segmentation v0.1.0
   Compiling rustc-serialize v0.3.14
   Compiling libc v0.1.6
   Compiling regex v0.1.30
   Compiling rand v0.3.8 (https://github.com/rust-lang/rand.git#93544850)
   Compiling word_generator v0.0.1 (file:///C:/msys64/home/Peter/word_generator)
error: linking with `gcc` failed: exit code: 1
note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-static-lib
gcc" "-m64" "-L" "C:\Rust64\bin\rustlib\x86_64-pc-windows-gnu\lib" "-o" "C:\msys64\home\Peter\word_g
enerator\target\debug\word_generator.exe" "C:\msys64\home\Peter\word_generator\target\debug\word_gen
erator.o" "-Wl,--gc-sections" "C:\msys64\home\Peter\word_generator\target\debug\deps\librustc_serial
ize-9ef26f158d5284e0.rlib" "C:\msys64\home\Peter\word_generator\target\debug\deps\libunicode_segment
ation-57da89e19b25ee00.rlib" "C:\msys64\home\Peter\word_generator\target\debug\deps\libregex-3bea306
1fd389532.rlib" "C:\msys64\home\Peter\word_generator\target\debug\deps\librand-3b6646ffd596e899.rlib
" "C:\msys64\home\Peter\word_generator\target\debug\deps\liblibc-9b7976990ae0dbd4.rlib" "C:\Rust64\b
in\rustlib\x86_64-pc-windows-gnu\lib\libstd-4e7c5e5c.rlib" "C:\Rust64\bin\rustlib\x86_64-pc-windows-
gnu\lib\libcollections-4e7c5e5c.rlib" "C:\Rust64\bin\rustlib\x86_64-pc-windows-gnu\lib\librustc_unic
ode-4e7c5e5c.rlib" "C:\Rust64\bin\rustlib\x86_64-pc-windows-gnu\lib\librand-4e7c5e5c.rlib" "C:\Rust6
4\bin\rustlib\x86_64-pc-windows-gnu\lib\liballoc-4e7c5e5c.rlib" "C:\Rust64\bin\rustlib\x86_64-pc-win
dows-gnu\lib\liblibc-4e7c5e5c.rlib" "C:\Rust64\bin\rustlib\x86_64-pc-windows-gnu\lib\libcore-4e7c5e5
c.rlib" "-L" "C:\msys64\home\Peter\word_generator\target\debug" "-L" "C:\msys64\home\Peter\word_gene
rator\target\debug\deps" "-L" "C:\Rust64\bin\rustlib\x86_64-pc-windows-gnu\lib" "-L" "C:\msys64\home
\Peter\word_generator\.rust\bin\x86_64-pc-windows-gnu" "-L" "C:\msys64\home\Peter\word_generator\bin
\x86_64-pc-windows-gnu" "-Wl,--whole-archive" "-Wl,-Bstatic" "-Wl,--no-whole-archive" "-Wl,-Bdynamic
" "-lws2_32" "-luserenv" "-lcompiler-rt"
note: ld: cannot find crt2.o: No such file or directory
ld: cannot find crtbegin.o: No such file or directory
ld: cannot find crtend.o: No such file or directory

error: aborting due to previous error
Could not compile `word_generator`.

To learn more, run the command again with --verbose.

C:\msys64\home\Peter\word_generator>rustc -vV
rustc 1.1.0-nightly (435622028 2015-05-04) (built 2015-05-05)
binary: rustc
commit-hash: 435622028f37085819843f4ac8938557501f0468
commit-date: 2015-05-04
build-date: 2015-05-05
host: x86_64-pc-windows-gnu
release: 1.1.0-nightly
