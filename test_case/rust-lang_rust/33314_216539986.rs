
mkdir -p x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/
cp: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/compiler-rt.lib
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/libcore
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/liblibc
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/librand
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/liballoc_system
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/liballoc
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/librustc_unicode
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/libcollections
touch x86_64-pc-windows-msvc/rt/backtrace.lib
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/libstd
rustc: x86_64-pc-windows-msvc/stage0/lib/rustlib/x86_64-pc-windows-msvc/lib/libarena
compile: x86_64-pc-windows-msvc/rt/miniz.o
/c/bot/slave/auto-win-msvc-64-opt-mir/build/mk/rt.mk:90: recipe for target 'x86_64-pc-windows-msvc/rt/miniz.o' failed
>> rustjob: found 1 remaining processes
>> rustjob: found remaining: 3520 - "\\Device\\HarddiskVolume1\\Program Files (x86)\\Microsoft Visual Studio 12.0\\VC\\bin\\mspdbsrv.exe"
>> rustjob:     oops, this is mspdbsrv
make: C:\Program: Command not found
make: *** [x86_64-pc-windows-msvc/rt/miniz.o] Error 127
program finished with exit code 2
elapsedTime=74.519000
