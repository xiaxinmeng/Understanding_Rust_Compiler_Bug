bash
cargo 0.18.0-nightly (5db6d64 2017-03-03)
rustc 1.17.0-nightly (b1e31766d 2017-03-03)

cargo test --target i686-pc-windows-msvc

error: linking with `link.exe` failed: exit code: 1318
  |
  = note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC\\BIN\\link.
exe" "/NOLOGO" "/NXCOMPAT" "/LARGEADDRESSAWARE" "/SAFESEH" "/LIBPATH:C:\\Users\\
n\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\rustlib\\i
686-pc-windows-msvc\\lib" "C:\\Users\\n\\Documents\\imageflow\\target\\debug\\bu
ild\\hyper-d6d1e087e41dda9f\\build_script_build-d6d1e087e41dda9f.build_script_bu
ild.o" "/OUT:C:\\Users\\n\\Documents\\imageflow\\target\\debug\\build\\hyper-d6d
1e087e41dda9f\\build_script_build-d6d1e087e41dda9f.exe" "/OPT:REF,NOICF" "/DEBUG
" "/LIBPATH:C:\\Users\\n\\Documents\\imageflow\\target\\debug\\deps" "/LIBPATH:C
:\\Users\\n\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\
rustlib\\i686-pc-windows-msvc\\lib" "C:\\Users\\n\\Documents\\imageflow\\target\
\debug\\deps\\librustc_version-1324ff5c3ace67a8.rlib" "C:\\Users\\n\\Documents\\
imageflow\\target\\debug\\deps\\libsemver-f260b12f97c06fb2.rlib" "C:\\Users\\n\\
.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\rustlib\\i686
-pc-windows-msvc\\lib\\libstd-13ee61d82f393677.rlib" "C:\\Users\\n\\.rustup\\too
lchains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-
msvc\\lib\\libpanic_unwind-48ea9b793aac0c7c.rlib" "C:\\Users\\n\\.rustup\\toolch
ains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-msv
c\\lib\\libunwind-d47ebac29f9bd14f.rlib" "C:\\Users\\n\\.rustup\\toolchains\\nig
htly-2017-03-04-i686-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib\\l
iblibc-c9fd286480cea820.rlib" "C:\\Users\\n\\.rustup\\toolchains\\nightly-2017-0
3-04-i686-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib\\librand-83ea
7d0fdec3fdde.rlib" "C:\\Users\\n\\.rustup\\toolchains\\nightly-2017-03-04-i686-p
c-windows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib\\libcollections-6abea59f
9a3a2e81.rlib" "C:\\Users\\n\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-wi
ndows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib\\liballoc-18b5b7bd96a82034.r
lib" "C:\\Users\\n\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc
\\lib\\rustlib\\i686-pc-windows-msvc\\lib\\liballoc_system-a1b08813244b60c5.rlib
" "C:\\Users\\n\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc\\l
ib\\rustlib\\i686-pc-windows-msvc\\lib\\libstd_unicode-1caf05b3f5a5e05c.rlib" "C
:\\Users\\n\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\
rustlib\\i686-pc-windows-msvc\\lib\\libcore-046c133680ee59d2.rlib" "C:\\Users\\n
\\.rustup\\toolchains\\nightly-2017-03-04-i686-pc-windows-msvc\\lib\\rustlib\\i6
86-pc-windows-msvc\\lib\\libcompiler_builtins-056daeb08ac9c69e.rlib" "advapi32.l
ib" "ws2_32.lib" "userenv.lib" "shell32.lib" "msvcrt.lib"
  = note: LINK : fatal error LNK1318: Unexpected PDB error; RPC (23) '(0x000006B
A)'


error: aborting due to previous error

error: Could not compile `hyper`.
Build failed, waiting for other jobs to finish...
error: build failed
