
---- native_plugin_dependency_with_custom_ar_linker stdout ----
	running `C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe build --verbose`
thread 'native_plugin_dependency_with_custom_ar_linker' panicked at '
Expected: execs
    but: expected to find:
[COMPILING] foo v0.0.1 ([..])
[RUNNING] `rustc [..] -C ar=nonexistent-ar -C linker=nonexistent-linker [..]`
[ERROR] could not exec the linker [..]
did not find in output:
   Compiling foo v0.0.1 (file:///C:/projects/rust/build/x86_64-pc-windows-msvc/stage2-tools/x86_64-pc-windows-msvc/cit/t1/foo)
     Running `rustc --crate-name foo C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t1\foo\src\lib.rs --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C debuginfo=2 -C metadata=17b7a4ba95a0e141 --out-dir C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t1\bar\target\debug\deps -C ar=nonexistent-ar -C linker=nonexistent-linker -L dependency=C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t1\bar\target\debug\deps`
error: linker `nonexistent-linker` not found
  |
  = note: The system cannot find the file specified. (os error 2)
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that VS 2013 or VS 2015 was installed with the Visual C++ option
error: aborting due to previous error
