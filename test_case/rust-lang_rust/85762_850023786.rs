
$ ntldd /c/Users/mateusz/.rustup/toolchains/f0c2c0aa6ba5b8eceb6adf9bd81195452e550cc1/lib/rustlib/x86_64-pc-windows-gnu/bin/rust-lld.exe
        ADVAPI32.dll => C:\Windows\SYSTEM32\ADVAPI32.dll (0x0000018861b80000)
        libgcc_s_seh-1.dll => D:\msys64\mingw64\bin\libgcc_s_seh-1.dll (0x0000018861960000)
        KERNEL32.dll => C:\Windows\SYSTEM32\KERNEL32.dll (0x0000018868b00000)
        msvcrt.dll => C:\Windows\SYSTEM32\msvcrt.dll (0x0000018868b00000)
        ole32.dll => C:\Windows\SYSTEM32\ole32.dll (0x0000018868b00000)
        libwinpthread-1.dll => D:\msys64\mingw64\bin\libwinpthread-1.dll (0x0000018861980000)
        SHELL32.dll => C:\Windows\SYSTEM32\SHELL32.dll (0x0000018868f90000)

$ /c/Users/mateusz/.rustup/toolchains/f0c2c0aa6ba5b8eceb6adf9bd81195452e550cc1/lib/rustlib/x86_64-pc-windows-gnu/bin/rust-lld -v
lld is a generic driver.
Invoke ld.lld (Unix), ld64.lld (macOS), lld-link (Windows), wasm-ld (WebAssembly) instead
