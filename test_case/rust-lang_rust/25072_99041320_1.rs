
$>cargo build
   Compiling unicode-segmentation v0.1.0
   Compiling rustc-serialize v0.3.14
   Compiling regex v0.1.30
   Compiling libc v0.1.6
   Compiling rand v0.3.8 (https://github.com/rust-lang/rand.git#93544850)
   Compiling word_generator v0.0.1
             (file:///C:/Users/Sean/Documents/Rust%20Projects/word_generator)
error: linking with `gcc` failed: exit code: 1
note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-static-libgcc" "-m64" "-L" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib" "-o" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\word_generator.exe" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\word_generator.o" "-Wl,--gc-sections" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\deps\librustc_serialize-9ef26f158d5284e0.rlib" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\deps\libunicode_segmentation-57da89e19b25ee00.rlib" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\deps\libregex-3bea3061fd389532.rlib" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\deps\librand-3b6646ffd596e899.rlib" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\deps\liblibc-9b7976990ae0dbd4.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\libstd-4e7c5e5c.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\libcollections-4e7c5e5c.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\librustc_unicode-4e7c5e5c.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\librand-4e7c5e5c.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\liballoc-4e7c5e5c.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\liblibc-4e7c5e5c.rlib" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib\libcore-4e7c5e5c.rlib" "-L" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug" "-L" "C:\Users\Sean\Documents\Rust Projects\word_generator\target\debug\deps" "-L" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib" "-L" "C:\Users\Sean\Documents\Rust Projects\word_generator\.rust\bin\x86_64-pc-windows-gnu" "-L" "C:\Users\Sean\Documents\Rust Projects\word_generator\bin\x86_64-pc-windows-gnu" "-Wl,--whole-archive" "-Wl,-Bstatic" "-Wl,--no-                whole-archive" "-Wl,-Bdynamic" "-lws2_32" "-luserenv" "-lcompiler-rt"
note: ld: cannot find crt2.o: No such file or directory
ld: cannot find crtbegin.o: No such file or directory
ld: cannot find crtend.o: No such file or directory

error: aborting due to previous error
Could not compile `word_generator`.

To learn more, run the command again with --verbose.
