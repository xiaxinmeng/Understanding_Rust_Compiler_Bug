console
>  cargo rustc -- -C link-arg=-nostartfiles
   Compiling app v0.1.0 (file:///C:/Users/steve/tmp/app)
error: linking with `link.exe` failed: exit code: 1561
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio\\2017\\Community\\VC\\Tools\\MSVC\\14.15.26726\\bin\\HostX64\\x64\\link.exe" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\steve\\tmp\\app\\target\\debug\\deps\\app-3187309a36d3b090.1aq5w1s18bk2pb4a.rcgu.o" "/OUT:C:\\Users\\steve\\tmp\\app\\target\\debug\\deps\\app-3187309a36d3b090.exe" "/OPT:REF,NOICF" "/DEBUG" "/NATVIS:C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\intrinsic.natvis" "/NATVIS:C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\liballoc.natvis" "/NATVIS:C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\etc\\libcore.natvis" "/LIBPATH:C:\\Users\\steve\\tmp\\app\\target\\debug\\deps" "/LIBPATH:C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-68de561760a7c421.rlib" "C:\\Users\\steve\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-a0d3327a110151c3.rlib" "-nostartfiles"
  = note: LINK : warning LNK4044: unrecognized option '/nostartfiles'; ignored
          LINK : fatal error LNK1561: entry point must be defined


error: aborting due to previous error

error: Could not compile `app`.
