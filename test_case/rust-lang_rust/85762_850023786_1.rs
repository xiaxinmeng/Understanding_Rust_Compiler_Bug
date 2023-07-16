
$ ntldd /c/Users/mateusz/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/bin/rust-lld.exe
        zlib1__.dll => not found
        ADVAPI32.dll => C:\Windows\SYSTEM32\ADVAPI32.dll (0x00000184c2800000)
        libgcc_s_seh-1.dll => D:\msys64\mingw64\bin\libgcc_s_seh-1.dll (0x00000184bb8d0000)
        KERNEL32.dll => C:\Windows\SYSTEM32\KERNEL32.dll (0x00000184c28b0000)
        msvcrt.dll => C:\Windows\SYSTEM32\msvcrt.dll (0x00000184c28b0000)
        ole32.dll => C:\Windows\SYSTEM32\ole32.dll (0x00000184c2800000)
        libwinpthread-1.dll => D:\msys64\mingw64\bin\libwinpthread-1.dll (0x00000184bb8f0000)
        SHELL32.dll => C:\Windows\SYSTEM32\SHELL32.dll (0x00000184c2d30000)

$ /c/Users/mateusz/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/bin/rust-lld
C:/Users/mateusz/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/bin/rust-lld.exe: error while loading shared libraries: libwinpthread-1.dll: cannot open shared object file: No such file or directory
