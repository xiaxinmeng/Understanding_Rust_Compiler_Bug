plain
[00:05:35]    Compiling semver-parser v0.6.2
[00:05:35]    Compiling env_logger v0.3.5
[00:05:35]    Compiling docopt v0.6.86
[00:05:37]    Compiling semver v0.5.1
[00:05:47] error: failed to compile `cargo-vendor v0.1.4`, intermediate artifacts can be found at `/tmp/cargo-installKGZx3G`
[00:05:47] Caused by:
[00:05:47]   failed to run custom build command for `curl-sys v0.4.3`
[00:05:47]   failed to run custom build command for `curl-sys v0.4.3`
[00:05:47] process didn't exit successfully: `/tmp/cargo-installKGZx3G/debug/build/curl-sys-0f0d144b97317dab/build-script-build` (exit code: 101)
[00:05:47] --- stdout
[00:05:47] Couldn't find libcurl from pkgconfig ("`\"pkg-config\" \"--libs\" \"--cflags\" \"libcurl\"` did not exit successfully: exit code: 1\n--- stderr\nPackage libcurl was not found in the pkg-config search path.\nPerhaps you should add the directory containing `libcurl.pc\'\nto the PKG_CONFIG_PATH environment variable\nNo package \'libcurl\' found\n"), compiling it from source...
[00:05:47] cargo:rustc-link-search=native=/tmp/cargo-installKGZx3G/debug/build/curl-sys-a574ea040976ba44/out/lib
[00:05:47] cargo:rustc-link-lib=static=curl
[00:05:47] cargo:root=/tmp/cargo-installKGZx3G/debug/build/curl-sys-a574ea040976ba44/out
[00:05:47] cargo:include=/tmp/cargo-installKGZx3G/debug/build/curl-sys-a574ea040976ba44/out/include
[00:05:47] TARGET = Some("x86_64-unknown-linux-gnu")
[00:05:47] HOST = Some("x86_64-unknown-linux-gnu")
[00:05:47] TARGET = Some("x86_64-unknown-linux-gnu")
[00:05:47] TARGET = Some("x86_64-unknown-linux-gnu")
---
[00:05:47] CFLAGS_x86_64-unknown-linux-gnu = None
[00:05:47] CFLAGS_x86_64_unknown_linux_gnu = None
[00:05:47] HOST_CFLAGS = None
[00:05:47] CFLAGS = None
[00:05:47] DEBUG = Some("true")
[00:05:47] running: "sh" "/cargo/registry/src/github.com-1ecc6299db9ec823/curl-sys-0.4.3/curl/configure" "--without-ca-bundle" "--without-ca-path" "--with-ssl=/checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install" "--enable-static=yes" "--enable-shared=no" "--enable-debug" "--disable-optimize" "--prefix=/tmp/cargo-installKGZx3G/debug/build/curl-sys-a574ea040976ba44/out" "--without-nghttp2" "--without-librtmp" "--without-libidn2" "--without-libssh2" "--without-libpsl" "--disable-ldap" "--disable-ldaps" "--disable-ftp" "--disable-rtsp" "--disable-dict" "--disable-telnet" "--disable-tftp" "--disable-pop3" "--disable-imap" "--disable-smtp" "--disable-gopher" "--disable-manual" "--disable-smb" "--disable-sspi" "--disable-manual" "--disable-unix-sockets" "--disable-versioned-symbols" "--enable-hidden-symbols" "--disable-libcurl-option"
[00:05:47] checking whether to enable maintainer-specific portions of Makefiles... no
[00:05:47] checking whether make supports nested variables... yes
[00:05:47] checking whether to enable debug build options... yes
[00:05:47] checking whether to enable compiler optimizer... no
[00:05:47] checking whether to enable strict compiler warnings... yes
[00:05:47] checking whether to enable compiler warnings as errors... no
[00:05:47] checking whether to enable curl debug memory tracking... (assumed) yes
[00:05:47] checking whether to enable hiding of library internal symbols... yes
[00:05:47] checking whether to enable c-ares for DNS lookups... no
[00:05:47] checking whether to disable dependency on -lrt... (assumed no)
[00:05:47] checking for path separator... :
[00:05:47] checking for sed... /bin/sed
[00:05:47] checking for egrep... /bin/grep -E
[00:05:47] checking for ar... /usr/bin/ar
[00:05:47] checking for a BSD-compatible install... /usr/bin/install -c
[00:05:47] checking for gcc... cc
---
[00:05:47] checking for cc option to accept ISO C89... none needed
[00:05:47] checking whether cc understands -c and -o together... yes
[00:05:47] checking how to run the C preprocessor... cc -E
[00:05:47] checking whether build environment is sane... yes
[00:05:47] checking for a thread-safe mkdir -p... /bin/mkdir -p
[00:05:47] checking for gawk... no
[00:05:47] checking for mawk... mawk
[00:05:47] checking for style of include used by make... GNU
[00:05:47] checking dependency style of cc... gcc3
[00:05:47] checking curl version... 7.54.0-DEV
[00:05:47] checking build system type... x86_64-pc-linux-gnu
---
[00:05:47] checking for strings.h... yes
[00:05:47] checking for inttypes.h... (cached) yes
[00:05:47] checking for stdint.h... (cached) yes
[00:05:47] checking for unistd.h... yes
[00:05:47] checking if cpp -P is needed... yes
[00:05:47] checking if cpp -P works... yes
[00:05:47] checking size of long... 8
[00:05:47] checking size of void*... 8
[00:05:47] checking for 64-bit curl_off_t data type... long
[00:05:47] checking size of curl_off_t... 8
[00:05:47] checking formatting string directive for curl_off_t... "ld"
[00:05:47] checking formatting string directive for unsigned curl_off_t... "lu"
[00:05:47] checking constant suffix string for curl_off_t... L
[00:05:47] checking constant suffix string for unsigned curl_off_t... UL
[00:05:47] checking if OS is AIX (to define _ALL_SOURCE)... no
[00:05:47] checking if _THREAD_SAFE is already defined... no
[00:05:47] checking if _THREAD_SAFE is actually needed... no
[00:05:47] checking if _THREAD_SAFE is onwards defined... no
[00:05:47] checking if _REENTRANT is already defined... no
[00:05:47] checking if _REENTRANT is actually needed... no
[00:05:47] checking if _REENTRANT is onwards defined... no
[00:05:47] checking for _FILE_OFFSET_BITS value needed for large files... no
[00:05:47] checking how to print strings... printf
[00:05:47] checking for a sed that does not truncate output... (cached) /bin/sed
[00:05:47] checking for fgrep... /bin/grep -F
[00:05:47] checking for fgrep... /bin/grep -F
[00:05:47] checking for ld used by cc... /usr/bin/ld
[00:05:47] checking if the linker (/usr/bin/ld) is GNU ld... yes
[00:05:47] checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
[00:05:47] checking the name lister (/usr/bin/nm -B) interface... BSD nm
[00:05:47] checking whether ln -s works... yes
[00:05:47] checking the maximum length of command line arguments... 1572864
[00:05:47] checking how to convert x86_64-pc-linux-gnu file names to x86_64-pc-linux-gnu format... func_convert_file_noop
[00:05:47] checking how to convert x86_64-pc-linux-gnu file names to toolchain format... func_convert_file_noop
[00:05:47] checking for /usr/bin/ld option to reload object files... -r
[00:05:47] checking for objdump... objdump
[00:05:47] checking how to recognize dependent libraries... pass_all
[00:05:47] checking for dlltool... no
[00:05:47] checking how to associate runtime and link libraries... printf %s\n
[00:05:47] checking for archiver @FILE support... @
[00:05:47] checking for strip... strip
[00:05:47] checking for ranlib... ranlib
[00:05:47] checking command to parse /usr/bin/nm -B output from cc object... ok
[00:05:47] checking for sysroot... no
[00:05:47] checking for a working dd... /bin/dd
[00:05:47] checking how to truncate binary pipes... /bin/dd bs=4096 count=1
[00:05:47] checking for mt... no
[00:05:47] checking if : is a manifest tool... no
[00:05:47] checking for dlfcn.h... yes
[00:05:47] checking for objdir... .libs
[00:05:47] checking if cc supports -fno-rtti -fno-exceptions... no
[00:05:47] checking for cc option to produce PIC... -fPIC -DPIC
[00:05:47] checking if cc PIC flag -fPIC -DPIC works... yes
[00:05:47] checking if cc static flag -static works... yes
[00:05:47] checking if cc supports -c -o file.o... yes
[00:05:47] checking if cc supports -c -o file.o... (cached) yes
[00:05:47] checking whether the cc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
[00:05:47] checking dynamic linker characteristics... GNU/Linux ld.so
[00:05:47] checking how to hardcode library paths into programs... immediate
[00:05:47] checking whether stripping libraries is possible... yes
[00:05:47] checking if libtool supports shared libraries... yes
[00:05:47] checking whether to build shared libraries... no
[00:05:47] checking whether to build static libraries... yes
[00:05:47] checking whether to build shared libraries with -version-info... yes
[00:05:47] checking whether to build shared libraries with -no-undefined... no
[00:05:47] checking whether to build shared libraries with -mimpure-text... no
[00:05:47] checking whether to build shared libraries with PIC... yes
[00:05:47] checking whether to build static libraries with PIC... yes
[00:05:47] checking whether to build shared libraries only... no
[00:05:47] checking whether to build static libraries only... yes
[00:05:47] checking for inline... inline
[00:05:47] checking if compiler is DEC/Compaq/HP C... no
[00:05:47] checking if compiler is HP-UX C... no
[00:05:47] checking if compiler is IBM C... no
[00:05:47] checking if compiler is Intel C... no
[00:05:47] checking if compiler is clang... no
[00:05:47] checking if compiler is GNU C... yes
[00:05:47] checking if compiler is LCC... no
[00:05:47] checking if compiler is SGI MIPSpro C... no
[00:05:47] checking if compiler is SGI MIPS C... no
[00:05:47] checking if compiler is SunPro C... no
[00:05:47] checking if compiler is Tiny C... no
[00:05:47] checking if compiler is Watcom C... no
[00:05:47] checking if compiler accepts some basic options... yes
[00:05:47] configure: compiler options added: -Werror-implicit-function-declaration 
[00:05:47] checking if compiler accepts debug enabling options... yes
[00:05:47] configure: compiler options added: -g
[00:05:47] checking if compiler accepts optimizer disabling options... yes
[00:05:47] configure: compiler options added: -O0
[00:05:47] checking if compiler accepts strict warning options... yes
[00:05:47] configure: compiler options added: -pedantic -Wall -W -Wpointer-arith -Wwrite-strings -Wunused -Wshadow -Winline -Wnested-externs -Wmissing-declarations -Wmissing-prototypes -Wno-long-long -Wfloat-equal -Wno-multichar -Wsign-compare -Wundef -Wno-format-nonliteral -Wendif-labels -Wstrict-prototypes -Wdeclaration-after-statement -Wstrict-aliasing=3 -Wcast-align -Wtype-limits -Wold-style-declaration -Wmissing-parameter-type -Wempty-body -Wclobbered -Wignored-qualifiers -Wconversion -Wno-sign-conversion -Wvla -Wno-system-headers 
[00:05:47] checking if compiler halts on compilation errors... yes
[00:05:47] checking if compiler halts on negative sized arrays... yes
[00:05:47] checking if compiler halts on function prototype mismatch... yes
[00:05:47] checking if compiler supports hiding library internal symbols... yes
[00:05:47] checking if curl debug memory tracking can be enabled... yes
[00:05:47] checking for windows.h... no
[00:05:47] checking whether build target is a native Windows one... no
[00:05:47] checking whether build target supports WIN32 file API... no
[00:05:47] checking for good-to-use Mac CFLAGS... no
[00:05:47] checking whether to support http... yes
[00:05:47] checking whether to support ftp... no
[00:05:47] checking whether to support file... yes
[00:05:47] checking whether to support ldap... no
[00:05:47] checking whether to support ldaps... no
[00:05:47] checking whether to support rtsp... no
[00:05:47] checking whether to support proxies... yes
[00:05:47] checking whether to support dict... no
[00:05:47] checking whether to support telnet... no
[00:05:47] checking whether to support tftp... no
[00:05:47] checking whether to support pop3... no
[00:05:47] checking whether to support imap... no
[00:05:47] checking whether to support smb... no
[00:05:47] checking whether to support smtp... no
[00:05:47] checking whether to support gopher... no
[00:05:47] checking whether to provide built-in manual... no
[00:05:47] checking whether to enable generation of C code... no
[00:05:47] checking whether to use libgcc... no
[00:05:47] checking if X/Open network library is required... no
[00:05:47] checking for gethostbyname... yes
[00:05:47] checking for windows.h... (cached) no
[00:05:47] checking for winsock.h... (cached) no
[00:05:47] checking for winsock2.h... (cached) no
[00:05:47] checking for connect in libraries... yes
[00:05:47] checking for sys/types.h... (cached) yes
[00:05:47] checking sys/time.h usability... yes
[00:05:47] checking sys/time.h presence... yes
[00:05:47] checking for sys/time.h... yes
[00:05:47] checking for sys/time.h... yes
[00:05:47] checking time.h usability... yes
[00:05:47] checking time.h presence... yes
[00:05:47] checking for time.h... yes
[00:05:47] checking for monotonic clock_gettime... yes
[00:05:47] checking for clock_gettime in libraries... no additional lib required
[00:05:47] checking if monotonic clock_gettime works... yes
[00:05:47] checking for pkg-config... /usr/bin/pkg-config
[00:05:47] checking for zlib options with pkg-config... found
[00:05:47] checking zlib.h presence... yes
[00:05:47] checking for zlib.h... yes
[00:05:47] checking for zlib.h... yes
[00:05:47] configure: found both libz and libz.h header
[00:05:47] checking whether to enable IPv6... yes
[00:05:47] checking if struct sockaddr_in6 has sin6_scope_id member... yes
[00:05:47] checking if argv can be written to... yes
[00:05:47] checking if GSS-API support is requested... no
[00:05:47] checking whether to enable Windows native SSL/TLS (Windows native builds only)... no
[00:05:47] checking whether to enable Apple OS native SSL/TLS... no
[00:05:47] configure: PKG_CONFIG_LIBDIR will be set to "/checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/lib/pkgconfig"
[00:05:47] checking for pkg-config... (cached) /usr/bin/pkg-config
[00:05:47] checking for openssl options with pkg-config... found
[00:05:47] configure: pkg-config: SSL_LIBS: "-lssl -lcrypto"
[00:05:47] configure: pkg-config: SSL_LDFLAGS: "-L/checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/lib"
[00:05:47] configure: pkg-config: SSL_CPPFLAGS: "-I/checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/include"
[00:05:47] checking for HMAC_Update in -lcrypto... yes
[00:05:47] checking OpenSSL linking without -ldl... yes
[00:05:47] checking for SSL_connect in -lssl... yes
[00:05:47] checking openssl/x509.h usability... yes
[00:05:47] checking openssl/x509.h presence... yes
[00:05:47] checking for openssl/x509.h... yes
[00:05:47] checking openssl/rsa.h usability... yes
[00:05:47] checking openssl/rsa.h presence... yes
[00:05:47] checking for openssl/rsa.h... yes
[00:05:47] checking openssl/crypto.h usability... yes
[00:05:47] checking openssl/crypto.h presence... yes
[00:05:47] checking for openssl/crypto.h... yes
[00:05:47] checking openssl/pem.h usability... yes
[00:05:47] checking openssl/pem.h presence... yes
[00:05:47] checking for openssl/pem.h... yes
[00:05:47] checking openssl/ssl.h usability... yes
[00:05:47] checking openssl/ssl.h presence... yes
[00:05:47] checking for openssl/ssl.h... yes
[00:05:47] checking openssl/err.h usability... yes
[00:05:47] checking openssl/err.h presence... yes
[00:05:47] checking for openssl/err.h... yes
[00:05:47] checking openssl/pkcs12.h usability... yes
[00:05:47] checking openssl/pkcs12.h presence... yes
[00:05:47] checking for openssl/pkcs12.h... yes
[00:05:47] checking for ENGINE_init... yes
[00:05:47] checking openssl/engine.h usability... yes
[00:05:47] checking openssl/engine.h presence... yes
[00:05:47] checking for openssl/engine.h... yes
[00:05:47] checking for ENGINE_load_builtin_engines... yes
[00:05:47] checking for RAND_egd... yes
[00:05:47] checking for ENGINE_cleanup... yes
[00:05:47] checking for SSL_get_shutdown... yes
[00:05:47] checking for SSLv2_client_method... yes
[00:05:47] checking for BoringSSL... no
[00:05:47] checking for libressl... no
[00:05:47] configure: Added /checkout/obj/build/x86_64-unknown-linux-gnu/openssl/install/lib to LD_LIBRARY_PATH
[00:05:47] checking for OpenSSL headers version... 1.0.2 - 0x100020efL
[00:05:47] checking for OpenSSL library version... 1.0.2
[00:05:47] checking for OpenSSL headers and library versions matching... yes
[00:05:47] checking for "/dev/urandom"... yes
[00:05:47] checking for SRP_Calc_client_key in -lcrypto... yes
[00:05:47] checking default CA cert bundle/path... no
[00:05:47] checking whether to use builtin CA store of SSL library... no
[00:05:47] checking whether versioned symbols are wanted... no
[00:05:47] checking whether to enable Windows native IDN (Windows native builds only)... no
[00:05:47] checking whether to build with libidn2... no
[00:05:47] checking for malloc.h... yes
[00:05:47] checking for memory.h... no
[00:05:47] checking for sys/types.h... (cached) yes
[00:05:47] checking for sys/time.h... (cached) yes
---
[00:05:47] checking for assert.h... yes
[00:05:47] checking for unistd.h... (cached) yes
[00:05:47] checking for stdlib.h... (cached) yes
[00:05:47] checking for limits.h... yes
[00:05:47] checking for arpa/inet.h... yes
[00:05:47] checking for net/if.h... yes
[00:05:47] checking for netinet/in.h... yes
[00:05:47] checking for sys/un.h... yes
[00:05:47] checking for netinet/tcp.h... yes
[00:05:47] checking for netdb.h... yes
[00:05:47] checking for sys/sockio.h... no
[00:05:47] checking for sys/param.h... yes
[00:05:47] checking for termios.h... yes
[00:05:47] checking for termio.h... yes
[00:05:47] checking for termio.h... yes
[00:05:47] checking for sgtty.h... yes
[00:05:47] checking for alloca.h... yes
[00:05:47] checking for time.h... (cached) yes
[00:05:47] checking for io.h... no
[00:05:47] checking for io.h... no
[00:05:47] checking for pwd.h... yes
[00:05:47] checking for utime.h... yes
[00:05:47] checking for sys/utime.h... no
[00:05:47] checking for sys/poll.h... yes
[00:05:47] checking for poll.h... yes
[00:05:47] checking for socket.h... no
[00:05:47] checking for libgen.h... yes
[00:05:47] checking for locale.h... yes
[00:05:47] checking for errno.h... yes
[00:05:47] checking for stdbool.h... yes
[00:05:47] checking for stdbool.h... yes
[00:05:47] checking for arpa/tftp.h... yes
[00:05:47] checking for sys/filio.h... no
[00:05:47] checking for setjmp.h... yes
[00:05:47] checking for an ANSI C-conforming const... yes
[00:05:47] checking for an ANSI C-conforming const... yes
[00:05:47] checking for compiler support of C99 variadic macro style... yes
[00:05:47] checking for compiler support of old gcc variadic macro style... yes
[00:05:47] checking whether time.h and sys/time.h may both be included... (cached) yes
[00:05:47] checking for sys/types.h... (cached) yes
[00:05:47] checking for sys/time.h... (cached) yes
[00:05:47] checking for time.h... (cached) yes
[00:05:47] checking for time.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for struct timeval... yes
[00:05:47] checking run-time libs availability... fine
[00:05:47] checking size of size_t... 8
[00:05:47] checking size of long... (cached) 8
[00:05:47] checking size of int... 4
[00:05:47] checking size of short... 2
[00:05:47] checking size of time_t... 8
[00:05:47] checking size of off_t... 8
[00:05:47] checking for long long... yes
[00:05:47] checking if numberLL works... yes
[00:05:47] checking for ssize_t... yes
[00:05:47] checking for bool... yes
[00:05:47] checking for windows.h... (cached) no
[00:05:47] checking for winsock2.h... (cached) no
[00:05:47] checking for ws2tcpip.h... (cached) no
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for curl_socklen_t data type... socklen_t
[00:05:47] checking size of curl_socklen_t... 4
[00:05:47] checking for poll.h... (cached) yes
[00:05:47] checking for sys/poll.h... (cached) yes
[00:05:47] checking for in_addr_t... yes
[00:05:47] checking for struct sockaddr_storage... yes
[00:05:47] checking for struct sockaddr_storage... yes
[00:05:47] checking signal.h usability... yes
[00:05:47] checking signal.h presence... yes
[00:05:47] checking for signal.h... yes
[00:05:47] checking for sig_atomic_t... yes
[00:05:47] checking if sig_atomic_t is already defined as volatile... no
[00:05:47] checking for sys/select.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for select... yes
[00:05:47] checking for select... yes
[00:05:47] checking types of args and return type for select... int,fd_set *,struct timeval *,int
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for recv... yes
[00:05:47] checking for recv... yes
[00:05:47] checking types of args and return type for recv... int,void *,size_t,int,ssize_t
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for send... yes
[00:05:47] checking for send... yes
[00:05:47] checking types of args and return type for send... int,const void *,size_t,int,ssize_t
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for MSG_NOSIGNAL... yes
[00:05:47] checking for unistd.h... (cached) yes
[00:05:47] checking for unistd.h... (cached) yes
[00:05:47] checking if alarm can be linked... yes
[00:05:47] checking if alarm is prototyped... yes
[00:05:47] checking if alarm is compilable... yes
[00:05:47] checking if alarm usage allowed... yes
[00:05:47] checking if alarm might be used... yes
[00:05:47] checking for string.h... (cached) yes
[00:05:47] checking for strings.h... (cached) yes
[00:05:47] checking for sys/types.h... (cached) yes
[00:05:47] checking for libgen.h... (cached) yes
[00:05:47] checking for libgen.h... (cached) yes
[00:05:47] checking if basename can be linked... yes
[00:05:47] checking if basename is prototyped... yes
[00:05:47] checking if basename is compilable... yes
[00:05:47] checking if basename usage allowed... yes
[00:05:47] checking if basename might be used... yes
[00:05:47] checking for socket.h... (cached) no
[00:05:47] checking for socket.h... (cached) no
[00:05:47] checking if closesocket can be linked... no
[00:05:47] checking if closesocket might be used... no
[00:05:47] checking if CloseSocket can be linked... no
[00:05:47] checking if CloseSocket might be used... no
[00:05:47] checking if connect can be linked... yes
[00:05:47] checking if connect is prototyped... yes
[00:05:47] checking if connect is compilable... yes
[00:05:47] checking if connect usage allowed... yes
[00:05:47] checking if connect might be used... yes
[00:05:47] checking for unistd.h... (cached) yes
[00:05:47] checking for fcntl.h... (cached) yes
[00:05:47] checking for fcntl.h... (cached) yes
[00:05:47] checking if fcntl can be linked... yes
[00:05:47] checking if fcntl is prototyped... yes
[00:05:47] checking if fcntl is compilable... yes
[00:05:47] checking if fcntl usage allowed... yes
[00:05:47] checking if fcntl might be used... yes
[00:05:47] checking if fcntl O_NONBLOCK is compilable... yes
[00:05:47] checking if fcntl O_NONBLOCK usage allowed... yes
[00:05:47] checking if fcntl O_NONBLOCK might be used... yes
[00:05:47] checking for stdio.h... yes
[00:05:47] checking for stdio.h... yes
[00:05:47] checking if fdopen can be linked... yes
[00:05:47] checking if fdopen is prototyped... yes
[00:05:47] checking if fdopen is compilable... yes
[00:05:47] checking if fdopen usage allowed... yes
[00:05:47] checking if fdopen might be used... yes
[00:05:47] checking for netdb.h... (cached) yes
[00:05:47] checking for netdb.h... (cached) yes
[00:05:47] checking if freeaddrinfo can be linked... yes
[00:05:47] checking if freeaddrinfo is prototyped... yes
[00:05:47] checking if freeaddrinfo is compilable... yes
[00:05:47] checking if freeaddrinfo usage allowed... yes
[00:05:47] checking if freeaddrinfo might be used... yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for netinet/in.h... (cached) yes
[00:05:47] checking for ifaddrs.h... yes
[00:05:47] checking if freeifaddrs can be linked... yes
[00:05:47] checking if freeifaddrs is prototyped... yes
[00:05:47] checking if freeifaddrs is compilable... yes
[00:05:47] checking if freeifaddrs usage allowed... yes
[00:05:47] checking if freeifaddrs might be used... yes
[00:05:47] checking for sys/types.h... (cached) yes
[00:05:47] checking for sys/xattr.h... yes
[00:05:47] checking if fsetxattr can be linked... yes
[00:05:47] checking if fsetxattr is prototyped... yes
[00:05:47] checking if fsetxattr takes 5 args.... yes
[00:05:47] checking if fsetxattr is compilable... yes
[00:05:47] checking if fsetxattr usage allowed... yes
[00:05:47] checking if fsetxattr might be used... yes
[00:05:47] checking if ftruncate can be linked... yes
[00:05:47] checking if ftruncate is prototyped... yes
[00:05:47] checking if ftruncate is compilable... yes
[00:05:47] checking if ftruncate usage allowed... yes
[00:05:47] checking if ftruncate might be used... yes
[00:05:47] checking for stdlib.h... (cached) yes
[00:05:47] checking for stdlib.h... (cached) yes
[00:05:47] checking if getaddrinfo can be linked... yes
[00:05:47] checking if getaddrinfo is prototyped... yes
[00:05:47] checking if getaddrinfo is compilable... yes
[00:05:47] checking if getaddrinfo seems to work... yes
[00:05:47] checking if getaddrinfo usage allowed... yes
[00:05:47] checking if getaddrinfo might be used... yes
[00:05:47] checking if getaddrinfo is threadsafe... yes
[00:05:47] checking if gai_strerror can be linked... yes
[00:05:47] checking if gai_strerror is prototyped... yes
[00:05:47] checking if gai_strerror is compilable... yes
[00:05:47] checking if gai_strerror usage allowed... yes
[00:05:47] checking if gai_strerror might be used... yes
[00:05:47] checking if gethostbyaddr can be linked... yes
[00:05:47] checking if gethostbyaddr is prototyped... yes
[00:05:47] checking if gethostbyaddr is compilable... yes
[00:05:47] checking if gethostbyaddr usage allowed... yes
[00:05:47] checking if gethostbyaddr might be used... yes
[00:05:47] checking if gethostbyaddr_r can be linked... yes
[00:05:47] checking if gethostbyaddr_r is prototyped... yes
[00:05:47] checking if gethostbyaddr_r takes 5 args.... no
[00:05:47] checking if gethostbyaddr_r takes 7 args.... no
[00:05:47] checking if gethostbyaddr_r takes 8 args.... yes
[00:05:47] checking if gethostbyaddr_r is compilable... yes
[00:05:47] checking if gethostbyaddr_r usage allowed... yes
[00:05:47] checking if gethostbyaddr_r might be used... yes
[00:05:47] checking if gethostbyname can be linked... yes
[00:05:47] checking if gethostbyname is prototyped... yes
[00:05:47] checking if gethostbyname is compilable... yes
[00:05:47] checking if gethostbyname usage allowed... yes
[00:05:47] checking if gethostbyname might be used... yes
[00:05:47] checking if gethostbyname_r can be linked... yes
[00:05:47] checking if gethostbyname_r is prototyped... yes
[00:05:47] checking if gethostbyname_r takes 3 args.... no
[00:05:47] checking if gethostbyname_r takes 5 args.... no
[00:05:47] checking if gethostbyname_r takes 6 args.... yes
[00:05:47] checking if gethostbyname_r is compilable... yes
[00:05:47] checking if gethostbyname_r usage allowed... yes
[00:05:47] checking if gethostbyname_r might be used... yes
[00:05:47] checking if gethostname can be linked... yes
[00:05:47] checking if gethostname is prototyped... yes
[00:05:47] checking if gethostname is compilable... yes
[00:05:47] checking for gethostname arg 2 data type... size_t
[00:05:47] checking if gethostname usage allowed... yes
[00:05:47] checking if gethostname might be used... yes
[00:05:47] checking if getifaddrs can be linked... yes
[00:05:47] checking if getifaddrs is prototyped... yes
[00:05:47] checking if getifaddrs is compilable... yes
[00:05:47] checking if getifaddrs seems to work... yes
[00:05:47] checking if getifaddrs usage allowed... yes
[00:05:47] checking if getifaddrs might be used... yes
[00:05:47] checking if getservbyport_r can be linked... yes
[00:05:47] checking if getservbyport_r is prototyped... yes
[00:05:47] checking if getservbyport_r takes 4 args.... no
[00:05:47] checking if getservbyport_r takes 5 args.... no
[00:05:47] checking if getservbyport_r takes 6 args.... yes
[00:05:47] checking if getservbyport_r is compilable... yes
[00:05:47] checking if getservbyport_r usage allowed... yes
[00:05:47] checking if getservbyport_r might be used... yes
[00:05:47] checking for sys/time.h... (cached) yes
[00:05:47] checking for time.h... (cached) yes
[00:05:47] checking for time.h... (cached) yes
[00:05:47] checking if gmtime_r can be linked... yes
[00:05:47] checking if gmtime_r is prototyped... yes
[00:05:47] checking if gmtime_r is compilable... yes
[00:05:47] checking if gmtime_r seems to work... yes
[00:05:47] checking if gmtime_r usage allowed... yes
[00:05:47] checking if gmtime_r might be used... yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for netinet/in.h... (cached) yes
[00:05:47] checking for arpa/inet.h... (cached) yes
[00:05:47] checking if inet_ntoa_r can be linked... no
[00:05:47] checking if inet_ntoa_r might be used... no
[00:05:47] checking if inet_ntop can be linked... yes
[00:05:47] checking if inet_ntop is prototyped... yes
[00:05:47] checking if inet_ntop is compilable... yes
[00:05:47] checking if inet_ntop seems to work... yes
[00:05:47] checking if inet_ntop usage allowed... yes
[00:05:47] checking if inet_ntop might be used... yes
[00:05:47] checking if inet_pton can be linked... yes
[00:05:47] checking if inet_pton is prototyped... yes
[00:05:47] checking if inet_pton is compilable... yes
[00:05:47] checking if inet_pton seems to work... yes
[00:05:47] checking if inet_pton usage allowed... yes
[00:05:47] checking if inet_pton might be used... yes
[00:05:47] checking for unistd.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for sys/ioctl.h... (cached) yes
[00:05:47] checking for stropts.h... yes
[00:05:47] checking for stropts.h... yes
[00:05:47] checking if ioctl can be linked... yes
[00:05:47] checking if ioctl is prototyped... yes
[00:05:47] checking if ioctl is compilable... yes
[00:05:47] checking if ioctl usage allowed... yes
[00:05:47] checking if ioctl might be used... yes
[00:05:47] checking if ioctl FIONBIO is compilable... yes
[00:05:47] checking if ioctl FIONBIO usage allowed... yes
[00:05:47] checking if ioctl FIONBIO might be used... yes
[00:05:47] checking if ioctl SIOCGIFADDR is compilable... yes
[00:05:47] checking if ioctl SIOCGIFADDR usage allowed... yes
[00:05:47] checking if ioctl SIOCGIFADDR might be used... yes
[00:05:47] checking if ioctlsocket can be linked... no
[00:05:47] checking if ioctlsocket might be used... no
[00:05:47] checking if IoctlSocket can be linked... no
[00:05:47] checking if IoctlSocket might be used... no
[00:05:47] checking if localtime_r can be linked... yes
[00:05:47] checking if localtime_r is prototyped... yes
[00:05:47] checking if localtime_r is compilable... yes
[00:05:47] checking if localtime_r seems to work... yes
[00:05:47] checking if localtime_r usage allowed... yes
[00:05:47] checking if localtime_r might be used... yes
[00:05:47] checking if memrchr can be linked... yes
[00:05:47] checking if memrchr is prototyped... no
[00:05:47] checking if memrchr might be used... no
[00:05:47] checking if poll can be linked... yes
[00:05:47] checking if poll is prototyped... yes
[00:05:47] checking if poll is compilable... yes
[00:05:47] checking if poll seems to work... yes
[00:05:47] checking if poll usage allowed... yes
[00:05:47] checking if poll might be used... yes
[00:05:47] checking if setsockopt can be linked... yes
[00:05:47] checking if setsockopt is prototyped... yes
[00:05:47] checking if setsockopt is compilable... yes
[00:05:47] checking if setsockopt usage allowed... yes
[00:05:47] checking if setsockopt might be used... yes
[00:05:47] checking if setsockopt SO_NONBLOCK is compilable... no
[00:05:47] checking if setsockopt SO_NONBLOCK might be used... no
[00:05:47] checking for signal.h... (cached) yes
[00:05:47] checking if sigaction can be linked... yes
[00:05:47] checking if sigaction is prototyped... yes
[00:05:47] checking if sigaction is compilable... yes
[00:05:47] checking if sigaction is compilable... yes
[00:05:47] checking if sigaction usage allowed... yes
[00:05:47] checking if sigaction might be used... yes
[00:05:47] checking if siginterrupt can be linked... yes
[00:05:47] checking if siginterrupt is prototyped... yes
[00:05:47] checking if siginterrupt is compilable... yes
[00:05:47] checking if siginterrupt usage allowed... yes
[00:05:47] checking if siginterrupt might be used... yes
[00:05:47] checking if signal can be linked... yes
[00:05:47] checking if signal is prototyped... yes
[00:05:47] checking if signal is compilable... yes
[00:05:47] checking if signal usage allowed... yes
[00:05:47] checking if signal might be used... yes
[00:05:47] checking for setjmp.h... (cached) yes
[00:05:47] checking for setjmp.h... (cached) yes
[00:05:47] checking if sigsetjmp can be linked... no
[00:05:47] checking if sigsetjmp seems a macro... yes
[00:05:47] checking if sigsetjmp is compilable... yes
[00:05:47] checking if sigsetjmp usage allowed... yes
[00:05:47] checking if sigsetjmp might be used... yes
[00:05:47] checking if socket can be linked... yes
[00:05:47] checking if socket is prototyped... yes
[00:05:47] checking if socket is compilable... yes
[00:05:47] checking if socket usage allowed... yes
[00:05:47] checking if socket might be used... yes
[00:05:47] checking if socketpair can be linked... yes
[00:05:47] checking if socketpair is prototyped... yes
[00:05:47] checking if socketpair is compilable... yes
[00:05:47] checking if socketpair usage allowed... yes
[00:05:47] checking if socketpair might be used... yes
[00:05:47] checking if strcasecmp can be linked... yes
[00:05:47] checking if strcasecmp is prototyped... yes
[00:05:47] checking if strcasecmp is compilable... yes
[00:05:47] checking if strcasecmp usage allowed... yes
[00:05:47] checking if strcasecmp might be used... yes
[00:05:47] checking if strcmpi can be linked... no
[00:05:47] checking if strcmpi might be used... no
[00:05:47] checking if strdup can be linked... yes
[00:05:47] checking if strdup is prototyped... yes
[00:05:47] checking if strdup is compilable... yes
[00:05:47] checking if strdup usage allowed... yes
[00:05:47] checking if strdup might be used... yes
[00:05:47] checking if strerror_r can be linked... yes
[00:05:47] checking if strerror_r is prototyped... yes
[00:05:47] checking if strerror_r is compilable... yes
[00:05:47] checking if strerror_r is glibc like... no
[00:05:47] checking if strerror_r is POSIX like... yes
[00:05:47] checking if strerror_r seems to work... yes
[00:05:47] checking if strerror_r usage allowed... yes
[00:05:47] checking if strerror_r might be used... yes
[00:05:47] checking if stricmp can be linked... no
[00:05:47] checking if stricmp might be used... no
[00:05:47] checking if strncasecmp can be linked... yes
[00:05:47] checking if strncasecmp is prototyped... yes
[00:05:47] checking if strncasecmp is compilable... yes
[00:05:47] checking if strncasecmp usage allowed... yes
[00:05:47] checking if strncasecmp might be used... yes
[00:05:47] checking if strncmpi can be linked... no
[00:05:47] checking if strncmpi might be used... no
[00:05:47] checking if strnicmp can be linked... no
[00:05:47] checking if strnicmp might be used... no
[00:05:47] checking if strstr can be linked... yes
[00:05:47] checking if strstr is prototyped... yes
[00:05:47] checking if strstr is compilable... yes
[00:05:47] checking if strstr usage allowed... yes
[00:05:47] checking if strstr might be used... yes
[00:05:47] checking if strtok_r can be linked... yes
[00:05:47] checking if strtok_r is prototyped... yes
[00:05:47] checking if strtok_r is compilable... yes
[00:05:47] checking if strtok_r usage allowed... yes
[00:05:47] checking if strtok_r might be used... yes
[00:05:47] checking if strtoll can be linked... yes
[00:05:47] checking if strtoll is prototyped... yes
[00:05:47] checking if strtoll is compilable... yes
[00:05:47] checking if strtoll usage allowed... yes
[00:05:47] checking if strtoll might be used... yes
[00:05:47] checking for sys/uio.h... (cached) yes
[00:05:47] checking for sys/uio.h... (cached) yes
[00:05:47] checking if writev can be linked... yes
[00:05:47] checking if writev is prototyped... yes
[00:05:47] checking if writev is compilable... yes
[00:05:47] checking if writev usage allowed... yes
[00:05:47] checking if writev might be used... yes
[00:05:47] checking for geteuid... yes
[00:05:47] checking for getpass_r... no
[00:05:47] checking for getpass_r... no
[00:05:47] checking deeper for getpass_r... but still no
[00:05:47] checking for getppid... yes
[00:05:47] checking for getprotobyname... yes
[00:05:47] checking for getpwuid... yes
[00:05:47] checking for getpwuid_r... yes
[00:05:47] checking for gettimeofday... yes
[00:05:47] checking for gettimeofday... yes
[00:05:47] checking for if_nametoindex... yes
[00:05:47] checking for inet_addr... yes
[00:05:47] checking for perror... yes
[00:05:47] checking for setlocale... yes
[00:05:47] checking for setlocale... yes
[00:05:47] checking for setmode... no
[00:05:47] checking deeper for setmode... but still no
[00:05:47] checking for uname... yes
[00:05:47] checking for utime... yes
[00:05:47] checking for sys/types.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for netdb.h... (cached) yes
[00:05:47] checking for getnameinfo... yes
[00:05:47] checking types of arguments for getnameinfo... const struct sockaddr *,socklen_t,socklen_t,int
[00:05:47] checking for stdio.h... (cached) yes
[00:05:47] checking for sys/socket.h... (cached) yes
[00:05:47] checking for netdb.h... (cached) yes
[00:05:47] checking for netdb.h... (cached) yes
[00:05:47] checking for netinet/in.h... (cached) yes
[00:05:47] checking for arpa/inet.h... (cached) yes
[00:05:47] checking for working NI_WITHSCOPEID... no
[00:05:47] checking how to set a socket into non-blocking mode... fcntl O_NONBLOCK
[00:05:47] checking for perl... /usr/bin/perl
[00:05:47] checking for gnroff... no
[00:05:47] checking for nroff... no
[00:05:47] checking whether to enable the threaded resolver... no
[00:05:47] checking whether to use POSIX threads for threaded resolver... auto
[00:05:47] checking whether to enable verbose strings... yes
[00:05:47] checking whether to enable SSPI support (Windows native builds only)... no
[00:05:47] checking whether to enable cryptographic authentication methods... yes
[00:05:47] checking whether to enable NTLM delegation to winbind's helper... yes
[00:05:47] checking whether to enable TLS-SRP authentication... yes
[00:05:47] checking whether to enable Unix domain sockets... no
[00:05:47] checking whether to enable support for cookies... yes
[00:05:47] checking whether hiding of library internal symbols will actually happen... yes
[00:05:47] checking whether to enforce SONAME bump... no
[00:05:47] checking that generated files are newer than configure... done
[00:05:47] config.status: creating Makefile
[00:05:47] 
[00:05:47] --- stderr
[00:05:47] --- stderr
[00:05:47] fatal: Not a git repository (or any parent up to mount point /cargo)
[00:05:47] Stopping at filesystem boundary (GIT_DISCOVERY_ACROSS_FILESYSTEM not set).
[00:05:47] configure: WARNING: disabling built-in manual
[00:05:47] config.status: error: cannot find input file: `docs/Makefile.in'
[00:05:47] command did not execute successfully, got: exit code: 1
[00:05:47] 
[00:05:47] 
[00:05:47] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/curl-sys-0.4.3/build.rs:224:5
[00:05:47] 
[00:05:47] 
[00:05:47] 
[00:05:47] 
[00:05:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "install" "--force" "--debug" "--vers" "0.1.4" "cargo-vendor"
[00:05:47] 
[00:05:47] 
[00:05:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test distcheck
[00:05:47] Build completed unsuccessfully in 0:03:21
