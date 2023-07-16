
error: linking with `g++` failed with code 1
note: g++ arguments: -Lc:\bot\slave\auto-win-32-nopt-t\build\obj\i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin -m32 -o i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra-a7c050cfd46b2c9a-0.8-pre.dll i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o -Lc:\bot\slave\auto-win-32-nopt-t\build\obj\i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin -lstd-6c65cf4b443341b1-0.8-pre -Lc:\bot\slave\auto-win-32-nopt-t\build\obj\.rust -Lc:\bot\slave\auto-win-32-nopt-t\build\obj -shared -lmorestack -lrustrt
note: i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8d83c): undefined reference to `GetCurrentProcess'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8d963): undefined reference to `DuplicateHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8db74): undefined reference to `DuplicateHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8dd85): undefined reference to `DuplicateHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8eb67): undefined reference to `CloseHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8eb79): undefined reference to `CloseHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8eb8b): undefined reference to `CloseHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8ec50): undefined reference to `CloseHandle'
i686-pc-mingw32\stage1\bin\rustc\i686-pc-mingw32\bin\extra.o:fake:(.text+0x8f031): undefined reference to `CreateProcessA'
collect2: ld returned 1 exit status
