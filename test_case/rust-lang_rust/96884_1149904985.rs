
windows-gnu:
	//MinGW libs
	"libgcc.a",
	"libgcc_eh.a",
	"libgcc_s.a",
	"libm.a",
	"libmingw32.a",
	"libmingwex.a",
	"libstdc++.a",
	"libiconv.a",
	"libmoldname.a",
	"libpthread.a",

	//Windows import libs
	"libadvapi32.a",
	"libbcrypt.a",
	"libcomctl32.a",
	"libcomdlg32.a",
	"libcredui.a",
	"libcrypt32.a",
	"libdbghelp.a",
	"libgdi32.a",
	"libimagehlp.a",
	"libiphlpapi.a",
	"libkernel32.a",
	"libmsimg32.a",
	"libmsvcrt.a",
	"libodbc32.a",
	"libole32.a",
	"liboleaut32.a",
	"libopengl32.a",
	"libpsapi.a",
	"librpcrt4.a",
	"libsecur32.a",
	"libsetupapi.a",
	"libshell32.a",
	"libsynchronization.a",
	"libuser32.a",
	"libuserenv.a",
	"libuuid.a",
	"libwinhttp.a",
	"libwinmm.a",
	"libwinspool.a",
	"libws2_32.a",
	"libwsock32.a",

	// Linker and dlltool
	"(i686,x86_64)-w64-mingw32-gcc.exe"
	"ld.exe"
	"dlltool.exe"
	"libwinpthread-1.dll" // needed by the exes

	// Crt objects
	"crt2.o", "dllcrt2.o"
musl:
	"libc.a"
	"crt1.o", "Scrt1.o", "rcrt1.o", "crti.o", "crtn.o", "crtbegin.o", "crtbeginS.o", "crtend.o", "crtendS.o"
	"libunwind.a"
wasi:
	"libc.a"
	"crt1-command.o", "crt1-reactor.o"
fuchsia, fortanix, gnullvm (do not go through the self-contained dir but should):
	"libunwind.a"
other:
	sanitizer libs
