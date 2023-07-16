
Александр@VAIO-FZ13 MINGW64 /c/link-crash
$ which rustc
/c/Program Files/rust-nightly-i686-pc-windows-msvc/rustc/bin/rustc

Александр@VAIO-FZ13 MINGW64 /c/link-crash
$ which cargo
/c/Program Files/rust-nightly-i686-pc-windows-msvc/cargo/bin/cargo

Александр@VAIO-FZ13 MINGW64 /c/link-crash
$ rustc -vV
rustc 1.5.0-nightly (6108e8c3e 2015-09-28)
binary: rustc
commit-hash: 6108e8c3e5e5768d9862ba0f66f44cbca098d015
commit-date: 2015-09-28
host: i686-pc-windows-msvc
release: 1.5.0-nightly

Александр@VAIO-FZ13 MINGW64 /c/link-crash
$ cargo build
   Compiling libc v0.1.10
   Compiling log v0.3.2
   Compiling link-crash v0.1.0 (file:///C:/link-crash)
error: linking with `link.exe` failed: exit code: 1120
note: "C:\\Program Files (x86)\\Microsoft Visual Studio 14.0\\VC/bin\\link.exe" "/LIBPATH:C:\\Program Files (x86
)\\Microsoft Visual Studio 14.0\\VC/lib\\" "/LIBPATH:C:\\Program Files (x86)\\Windows Kits\\10\\Lib\\10.0.10150.
0\\ucrt\\x86" "/LIBPATH:C:\\Program Files (x86)\\Windows Kits\\8.1\\Lib\\winv6.3\\um\\x86" "/NOLOGO" "/NXCOMPAT"
 "/LIBPATH:C:\\Program Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib"
 "C:\\link-crash\\target\\debug\\link_crash.0.o" "/OUT:C:\\link-crash\\target\\debug\\link_crash.exe" "/OPT:REF,
ICF" "/DEBUG" "C:\\link-crash\\target\\debug\\liblink_crash.rlib" "C:\\link-crash\\target\\debug\\deps\\liblog-6
e27e2da5513596d.rlib" "C:\\link-crash\\target\\debug\\deps\\liblibc-165b5479aec0459c.rlib" "C:\\Program Files\\r
ust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib\\libstd-10cbabc2.rlib" "C:\\Pro
gram Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib\\libcollections-10
cbabc2.rlib" "C:\\Program Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\l
ib\\librustc_unicode-10cbabc2.rlib" "C:\\Program Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\
i686-pc-windows-msvc\\lib\\librand-10cbabc2.rlib" "C:\\Program Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\
bin\\rustlib\\i686-pc-windows-msvc\\lib\\liballoc-10cbabc2.rlib" "C:\\Program Files\\rust-nightly-i686-pc-window
s-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib\\liballoc_system-10cbabc2.rlib" "C:\\Program Files\\rust-
nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib\\liblibc-10cbabc2.rlib" "C:\\Progra
m Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib\\libcore-10cbabc2.rli
b" "/LIBPATH:C:\\link-crash\\target\\debug" "/LIBPATH:C:\\link-crash\\target\\debug\\deps" "/LIBPATH:C:\\Program
 Files\\rust-nightly-i686-pc-windows-msvc\\rustc\\bin\\rustlib\\i686-pc-windows-msvc\\lib" "/LIBPATH:C:\\link-cr
ash\\.rust\\bin\\i686-pc-windows-msvc" "/LIBPATH:C:\\link-crash\\bin\\i686-pc-windows-msvc" "kernel32.lib" "shel
l32.lib" "msvcrt.lib" "ws2_32.lib" "userenv.lib" "advapi32.lib" "kernel32.lib" "shell32.lib" "msvcrt.lib" "compi
ler-rt.lib"
note: link_crash.0.o : warning LNK4217: locally defined symbol __ZN20MAX_LOG_LEVEL_FILTER20h6a83dca2b222a128EaaE
 imported in function __ZN15Logger$LT$E$GT$6do_log21h11022588445185383603E
link_crash.0.o : warning LNK4217: locally defined symbol __ZN4sync6atomic11atomic_load14_MSG_FILE_LINE20hb23f8a4
c7daca952RbKE imported in function __ZN4sync6atomic11atomic_load20h6038998186637377380E
liblog-6e27e2da5513596d.rlib(log-6e27e2da5513596d.0.o) : warning LNK4217: locally defined symbol __ZN4sync6atomi
c11atomic_load14_MSG_FILE_LINE20hb23f8a4c7daca952RbKE imported in function __ZN4iter27Enumerate$LT$I$GT$.Iterato
r4next20h3246140536557949847E
link_crash.0.o : warning LNK4217: locally defined symbol __ZN4sync6atomic11atomic_load14_MSG_FILE_LINE20hb23f8a4
c7daca952ccKE imported in function __ZN4sync6atomic11atomic_load20h6038998186637377380E
liblog-6e27e2da5513596d.rlib(log-6e27e2da5513596d.0.o) : warning LNK4049: locally defined symbol __ZN4sync6atomi
c11atomic_load14_MSG_FILE_LINE20hb23f8a4c7daca952ccKE imported
liblog-6e27e2da5513596d.rlib(log-6e27e2da5513596d.0.o) : warning LNK4217: locally defined symbol __ZN5ascii19ASC
II_LOWERCASE_MAP20hac5b35c34bc02cd2r8aE imported in function __ZN5ascii11u8.AsciiExt18to_ascii_lowercase20hc17d3
0bbfc4cf83fA2aE
liblog-6e27e2da5513596d.rlib(log-6e27e2da5513596d.0.o) : warning LNK4217: locally defined symbol __ZN6option15Op
tion$LT$T$GT$6unwrap14_MSG_FILE_LINE20hb23f8a4c7daca9524tNE imported in function __ZN6option15Option$LT$T$GT$6un
wrap20h3963608452739003467E
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN6option15Option$LT$T$GT$6un
wrap14_MSG_FILE_LINE20hb23f8a4c7daca9524tNE imported
liblog-6e27e2da5513596d.rlib(log-6e27e2da5513596d.0.o) : warning LNK4217: locally defined symbol __ZN4sync6atomi
c12atomic_store14_MSG_FILE_LINE20hb23f8a4c7daca952LaKE imported in function __ZN4sync6atomic12atomic_store21h142
34137121443170045E
liblog-6e27e2da5513596d.rlib(log-6e27e2da5513596d.0.o) : warning LNK4217: locally defined symbol __ZN4sync6atomi
c12atomic_store14_MSG_FILE_LINE20hb23f8a4c7daca9526aKE imported in function __ZN4sync6atomic12atomic_store21h142
34137121443170045E
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN7raw_vec11alloc_guard14_MSG
_FILE_LINE20h23e862067ca87a82TsbE imported in function __ZN5error101Box$LT$Error$u2b$$u20$Send$u20$$u2b$$u20$Syn
c$u20$$u2b$$u20$$u27$a$GT$.From$LT$$RF$$u27$b$u20$str$GT$4from20h29fbba5b1fad06c78daE
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN7raw_vec11a
lloc_guard14_MSG_FILE_LINE20h23e862067ca87a82TsbE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN6option15Option$LT$T$GT$6ex
pect15__STATIC_FMTSTR20h3dea4d7f9b96dce1ptNE imported in function __ZN3vec12Vec$LT$T$GT$7reserve21h1607928223866
7312862E
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN6option15Op
tion$LT$T$GT$6expect15__STATIC_FMTSTR20h3dea4d7f9b96dce1ptNE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN6option15Option$LT$T$GT$6ex
pect10_FILE_LINE20h428926c3c48c3a419sNE imported in function __ZN3vec12Vec$LT$T$GT$7reserve21h160792822386673128
62E
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN6option15Op
tion$LT$T$GT$6expect10_FILE_LINE20h428926c3c48c3a419sNE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN6result24Result$LT$T$C$$u20
$E$GT$6unwrap15__STATIC_FMTSTR20h3dea4d7f9b96dce1DbOE imported in function __ZN6thread4park20h68824b9a40d94b79FA
bE
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN6result24Result$LT$T$C$$u20
$E$GT$6unwrap10_FILE_LINE20h428926c3c48c3a41nbOE imported in function __ZN6thread4park20h68824b9a40d94b79FAbE
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN4cell16RefCell$LT$T$GT$10bo
rrow_mut14_MSG_FILE_LINE20hb23f8a4c7daca9529tKE imported in function __ZN4rand13ThreadRng.Rng8next_u6420h5206e2e
b7fbb98d1lwxE
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5slice54_$u5b$T$u5d$.ops..I
ndex$LT$ops..Range$LT$usize$GT$$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952rOPE imported in function __ZN3sys2o
s12error_string20hecf08faf6ec2bfdfKHvE
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN5slice54_$u
5b$T$u5d$.ops..Index$LT$ops..Range$LT$usize$GT$$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952rOPE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN6string6String8truncate14_M
SG_FILE_LINE20hea0af16c5f6bd6f9eOfE imported in function __ZN3sys2os12error_string20hecf08faf6ec2bfdfKHvE
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5slice54_$u5b$T$u5d$.ops..I
ndex$LT$ops..Range$LT$usize$GT$$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952YNPE imported in function __ZN3sys2o
s12Env.Iterator4next20h88d9e8399861ea3c6KvE
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN5slice54_$u
5b$T$u5d$.ops..Index$LT$ops..Range$LT$usize$GT$$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952YNPE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN7raw_vec15RawVec$LT$T$GT$13
shrink_to_fit14_MSG_FILE_LINE20h23e862067ca87a82BpbE imported in function __ZN3ffi5c_str7CString18from_vec_unche
cked20h360ec22bac5862a7wDeE
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN7raw_vec15R
awVec$LT$T$GT$13shrink_to_fit14_MSG_FILE_LINE20h23e862067ca87a82BpbE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5slice57_$u5b$T$u5d$.ops..I
ndexMut$LT$ops..Range$LT$usize$GT$$GT$9index_mut14_MSG_FILE_LINE20hb23f8a4c7daca952RQPE imported in function __Z
N2io4Read14read_to_string20h5208782761598478823E
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN3vec12Vec$LT$T$GT$5drain14_
MSG_FILE_LINE20hea0af16c5f6bd6f9GzgE imported in function __ZN2io8buffered18BufWriter$LT$W$GT$9flush_buf20h87407
03657607558764E
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5slice39_$u5b$T$u5d$.ops..I
ndexMut$LT$usize$GT$9index_mut14_MSG_FILE_LINE20hb23f8a4c7daca9523MPE imported in function __ZN3net6parser20Pars
er$LT$$u27$a$GT$19read_ipv6_addr_impl11read_groups20h670e4c18482dcebe6vjE
librand-10cbabc2.rlib(rand-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN5slice39_$u5b$T$u5d$.ops.
.IndexMut$LT$usize$GT$9index_mut14_MSG_FILE_LINE20hb23f8a4c7daca9523MPE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN4cell16RefCell$LT$T$GT$6bor
row14_MSG_FILE_LINE20hb23f8a4c7daca952ptKE imported in function __ZN10sys_common11thread_info3set20h7b1ab3da9c63
92c5c3rE
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5slice36_$u5b$T$u5d$.ops..I
ndex$LT$usize$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952eMPE imported in function __ZN4hash3sip16SipHasher.Has
her5write20he995307452470973XGTE
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN5slice36_$u
5b$T$u5d$.ops..Index$LT$usize$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952eMPE imported
librand-10cbabc2.rlib(rand-10cbabc2.0.o) : warning LNK4049: locally defined symbol __ZN5slice36_$u5b$T$u5d$.ops.
.Index$LT$usize$GT$5index14_MSG_FILE_LINE20hb23f8a4c7daca952eMPE imported
libstd-10cbabc2.rlib(std-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5isaac5EMPTY20h7c76c371cbc6
037aFhbE imported in function __ZN4rand6StdRng3new20hfd7a7d67da92a33bTqxE
libcollections-10cbabc2.rlib(collections-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN5u_str15UTF
8_CHAR_WIDTH20h448f3234fdd35abd79gE imported in function __ZN6string6String15from_utf8_lossy20hd447a128cc475435X
AfE
librustc_unicode-10cbabc2.rlib(rustc_unicode-10cbabc2.0.o) : warning LNK4217: locally defined symbol __ZN4char12
char.CharExt8to_digit14_MSG_FILE_LINE20hb23f8a4c7daca952CJKE imported in function __ZN4char4char8is_digit20h41d2
1f466880a7d5MthE
link_crash.0.o : error LNK2019: unresolved external symbol __imp___ZN15Logger$LT$E$GT$6do_log15__STATIC_FMTSTR20
hf5db1df99b097f02HbaE referenced in function __ZN15Logger$LT$E$GT$6do_log21h11022588445185383603E
link_crash.0.o : error LNK2019: unresolved external symbol __imp___ZN15Logger$LT$E$GT$6do_log4_LOC20h021bff66b31
ba9b7HaaE referenced in function __ZN15Logger$LT$E$GT$6do_log21h11022588445185383603E
C:\link-crash\target\debug\link_crash.exe : fatal error LNK1120: 2 unresolved externals

error: aborting due to previous error
Could not compile `link-crash`.

To learn more, run the command again with --verbose.

