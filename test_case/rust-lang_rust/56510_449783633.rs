
micha@DESKTOP-IIQA1VP MINGW64 ~/IdeaProjects/rust (master)
$ ldd build/x86_64-pc-windows-msvc/stage1/bin/rustc.exe | grep -i ntdll
        ntdll.dll => /c/WINDOWS/SYSTEM32/ntdll.dll (0x7ffbede90000)

micha@DESKTOP-IIQA1VP MINGW64 ~/IdeaProjects/rust (master)
$ ldd build/x86_64-pc-windows-msvc/stage1/bin/rustc.exe | grep -i gdi

micha@DESKTOP-IIQA1VP MINGW64 ~/IdeaProjects/rust (master)
$
