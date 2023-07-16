
we@we-pc MINGW64 ~
$ cat test.rs
fn main() {
    println!("Hello, world!");
    panic!();
}

we@we-pc MINGW64 ~
$ rustc --version
rustc 1.16.0-nightly (24055d0f2 2017-01-31)

we@we-pc MINGW64 ~
$ rustc -g test.rs

we@we-pc MINGW64 ~
$ RUST_BACKTRACE=1 ./test.exe
Hello, world!
thread 'main' panicked at 'explicit panic', test.rs:3
stack backtrace:
   0:           0x4288ba - std::sys::imp::backtrace::_write::he5498b02db7db530
   1:           0x432f57 - std::panicking::default_hook::{{closure}}::h67a5fa5600945646
   2:           0x432b13 - std::panicking::default_hook::hf8a9629fab063fa5
   3:           0x4334de - std::panicking::rust_panic_with_hook::hf599e44aaa8545b8
   4:           0x4015d2 - std::panicking::begin_panic::h9a6bae97ed4f65b2
                        at C:\bot\slave\nightly-dist-rustc-win-gnu-64\build\src\libstd/panicking.rs:517
   5:           0x401871 - test::main::ha6bc039bc682a0db
                        at C:\msys64\home\we/test.rs:3
   6:           0x433168 - std::panicking::try::do_call::h4c4de3903bc7c076
   7:           0x43d178 - _rust_maybe_catch_panic
   8:           0x433c83 - std::rt::lang_start::h177832d72b1c79c7
   9:           0x4018aa - main
  10:           0x4013b4 - _tmainCRTStartup
  11:           0x4014e7 - mainCRTStartup
  12:     0x7ffc73cf13d1 - unit_addrs_search

we@we-pc MINGW64 ~
$ which strip
/c/mingw-w64/x86_64-6.2.0-win32-seh-rt_v5-rev1/mingw64/bin/strip

we@we-pc MINGW64 ~
$ strip ./test.exe

we@we-pc MINGW64 ~
$ RUST_BACKTRACE=1 ./test.exe
Hello, world!
thread 'main' panicked at 'explicit panic', test.rs:3
stack backtrace:

we@we-pc MINGW64 ~
$ RUST_BACKTRACE=1 gdb ./test.exe
GNU gdb (GDB) 7.9
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-msys".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Traceback (most recent call last):
  File "<string>", line 3, in <module>
ImportError: No module named libstdcxx.v6.printers
/etc/gdbinit:6: Error in sourced command file:
Error while executing Python code.
Reading symbols from ./test.exe...(no debugging symbols found)...done.
(gdb) run
Starting program: /home/we/test.exe
[New Thread 6068.0x126c]
Hello, world!
thread 'main' panicked at 'explicit panic', test.rs:3
stack backtrace:
warning: Critical error detected c0000374

Program received signal SIGTRAP, Trace/breakpoint trap.
0x00007ffc74361b30 in ntdll!RtlpNtMakeTemporaryKey () from /c/Windows/SYSTEM32/ntdll.dll
(gdb) c
Continuing.
gdb: unknown target exception 0xc0000374 at 0x7ffc74361b70

Program received signal ?, Unknown signal.
0x00007ffc74361b70 in ntdll!RtlpNtMakeTemporaryKey () from /c/Windows/SYSTEM32/ntdll.dll
(gdb) c
Continuing.
[Inferior 1 (process 6068) exited with code 030000001564]
(gdb) q

