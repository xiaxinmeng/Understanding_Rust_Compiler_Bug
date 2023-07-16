
01:22:08] ---- include_overrides_gitignore stdout ----
[01:22:08]  build 1: all is new
[01:22:08] running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v`
[01:22:08] build 2: nothing changed; file timestamps reset by build script
[01:22:08] running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build -v`
[01:22:08] thread 'include_overrides_gitignore' panicked at '
[01:22:08] Expected: execs
[01:22:08]     but: differences:
[01:22:08]   2 - |[FRESH] reduction [..]|
[01:22:08]     + |   Compiling reduction v0.5.0 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t20/reduction)|
[01:22:08]
[01:22:08]   3 - |[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]|
[01:22:08]     + |     Running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t20\reduction\target\debug\build\reduction-666be347c5989aef\
build-script-tango-build`|
[01:22:08]
[01:22:08]   4 -
[01:22:08]     + |     Running `rustc --crate-name reduction src\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 -C metadata=318e2b284633cee4 -C extra-filename=-318e2
b284633cee4 --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t20\reduction\target\debug\deps -L dependency=C:\projects\rust\build\x
86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t20\reduction\target\debug\deps`|
[01:22:08]
[01:22:08]   5 -
[01:22:08]     + |    Finished dev [unoptimized + debuginfo] target(s) in 0.18 secs|
