
   Compiling cargo v0.25.0 (file:///C:/projects/rust/src/tools/cargo)
error: linking with `gcc` failed: exit code: 1
  |
  = note: <snip>
  = note: C:\projects\rust\build\x86_64-pc-windows-gnu\stage2-tools\x86_64-pc-windows-gnu\release\deps\liblibgit2_sys-21cea83aba30ff6f.rlib(socket_stream.c.obj):socket_stream.c:(.text$socket_connect+0x187): undefined reference to `gai_strerrorA'
          collect2.exe: error: ld returned 1 exit status
