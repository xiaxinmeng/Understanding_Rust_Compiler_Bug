$ cargo install cargo-update
    Updating registry `https://github.com/rust-lang/crates.io-index`
  Installing cargo-update v1.1.1
   Compiling semver-parser v0.7.0
   Compiling json v0.11.8
   Compiling libc v0.2.26
   Compiling winapi-build v0.1.1
   Compiling winapi v0.2.8
   Compiling void v1.0.2
   Compiling lazy_static v0.2.8
   Compiling utf8-ranges v1.0.0
   Compiling unreachable v1.0.0
   Compiling vcpkg v0.2.2
   Compiling serde v1.0.10
   Compiling thread_local v0.3.4
   Compiling percent-encoding v1.0.0
   Compiling array_tool v0.4.0
   Compiling unicode-xid v0.0.4
   Compiling memchr v1.0.1
   Compiling synom v0.11.3
   Compiling gcc v0.3.51
   Compiling semver v0.6.0
   Compiling aho-corasick v0.6.3
   Compiling regex-syntax v0.4.1
   Compiling vec_map v0.8.0
   Compiling unicode-segmentation v1.1.0
   Compiling ansi_term v0.9.0
   Compiling matches v0.1.6
   Compiling strsim v0.6.0
   Compiling unicode-bidi v0.3.4
   Compiling kernel32-sys v0.2.2
error: linking with `link.exe` failed: exit code: 1181
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\bin\\amd64\\link.exe" "/LIBPATH:C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\lib\\amd64" "/LIBPATH:C:\\Program Files (x86)\\Windows Kits\\10\\lib\\10.0.15063.0\\ucrt\\x64" "/LIBPATH:C:\\Program Files (x86)\\Windows Kits\\10\\lib\\10.0.15063.0\\um\\x64" "/NOLOGO" "/NXCOMPAT" "/LIBPATH:C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\tochecch\\AppData\\Local\\Temp\\cargo-install.LWP8drBtUD59\\release\\build\\kernel32-sys-5ee1259db1228dbc\\build_script_build-5ee1259db1228dbc.0.o" "/OUT:C:\\Users\\tochecch\\AppData\\Local\\Temp\\cargo-install.LWP8drBtUD59\\release\\build\\kernel32-sys-5ee1259db1228dbc\\build_script_build-5ee1259db1228dbc.exe" "/OPT:REF,ICF" "/DEBUG" "/LIBPATH:C:\\Users\\tochecch\\AppData\\Local\\Temp\\cargo-install.LWP8drBtUD59\\release\\deps" "/LIBPATH:C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib" "C:\\Users\\tochecch\\AppData\\Local\\Temp\\cargo-install.LWP8drBtUD59\\release\\deps\\libbuild-cc558114ba27a576.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd-0a78323911070f99.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\librand-c279a51d66700350.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcollections-d7bf31a4ca1ea637.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libstd_unicode-d367c3ba0db49600.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libpanic_unwind-2d4bf02140c11dcb.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libunwind-add7a84d7e82d084.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liblibc-84688accbc86d6b7.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc-fe2e68b21f0bdd7a.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\liballoc_system-7fc0381594c93f56.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcore-ea9d77e7c23fe65c.rlib" "C:\\Users\\tochecch\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\lib\\rustlib\\x86_64-pc-windows-msvc\\lib\\libcompiler_builtins-91b619d34dd1f5aa.rlib" "advapi32.lib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib"
  = note: LINK : fatal error LNK1181: cannot open input file 'userenv.lib'


error: aborting due to previous error

error: Could not compile `kernel32-sys`.
Build failed, waiting for other jobs to finish...
