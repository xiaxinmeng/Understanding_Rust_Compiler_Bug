
error: linking with `gcc` failed: exit code: 1
note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-static-libgcc" "-m64" "-L" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib" "-o" <lots of rlibs and irrelevant stuff>  "-L" "C:\Users\Nathan\Desktop\code\image-viewer\target\release" "-L" "C:\Users\Nathan\Desktop\code\image-viewer\target\release\deps" "-L" "C:\Users\Nathan\Dropbox\code\in-progress\image-viewer\target\release\build\time-15ec407cfcd2d643\out" "-L" "C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib" "-L" "C:\Users\Nathan\Desktop\code\image-viewer\.rust\bin\x86_64-pc-windows-gnu" "-L" "C:\Users\Nathan\Desktop\code\image-viewer\bin\x86_64-pc-windows-gnu" "-Wl,--whole-archive" "-Wl,-Bstatic" "-Wl,--no-whole-archive" "-Wl,-Bdynamic" "-lglfw3" "-lopengl32" "-lgdi32" "-lfreetype-6" "-lws2_32" "-luserenv" "-lcompiler-rt"
note: ld: cannot find -lglfw3
ld: cannot find -lfreetype-6
