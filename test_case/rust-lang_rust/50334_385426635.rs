plain
[01:18:10]    Compiling semver-parser v0.6.2
[01:18:10]    Compiling env_logger v0.3.5
[01:18:10]    Compiling docopt v0.6.86
[01:18:13]    Compiling semver v0.5.1
[01:18:20] error: failed to compile `cargo-vendor v0.1.4`, intermediate artifacts can be found at `/tmp/cargo-install.a0sVULwVnhEJ`
[01:18:20] Caused by:
[01:18:20]   failed to run custom build command for `curl-sys v0.4.3`
[01:18:20]   failed to run custom build command for `curl-sys v0.4.3`
[01:18:20] process didn't exit successfully: `/tmp/cargo-install.a0sVULwVnhEJ/debug/build/curl-sys-73933b7dd4d5cf43/build-script-build` (exit code: 101)
[01:18:20] --- stdout
[01:18:20] Couldn't find libcurl from pkgconfig ("Aborted because LIBCURL_NO_PKG_CONFIG is set"), compiling it from source...
[01:18:20] cargo:rustc-link-search=native=/tmp/cargo-install.a0sVULwVnhEJ/debug/build/curl-sys-f62e1dc14344c926/out/lib
[01:18:20] cargo:rustc-link-lib=static=curl
[01:18:20] cargo:root=/tmp/cargo-install.a0sVULwVnhEJ/debug/build/curl-sys-f62e1dc14344c926/out
[01:18:20] cargo:include=/tmp/cargo-install.a0sVULwVnhEJ/debug/build/curl-sys-f62e1dc14344c926/out/include
[01:18:20] TARGET = Some("i686-unknown-linux-gnu")
[01:18:20] HOST = Some("i686-unknown-linux-gnu")
[01:18:20] TARGET = Some("i686-unknown-linux-gnu")
[01:18:20] TARGET = Some("i686-unknown-linux-gnu")
---
[01:18:20] CFLAGS_i686-unknown-linux-gnu = None
[01:18:20] CFLAGS_i686_unknown_linux_gnu = None
[01:18:20] HOST_CFLAGS = None
[01:18:20] CFLAGS = None
[01:18:20] DEBUG = Some("true")
[01:18:20] running: "sh" "/cargo/registry/src/github.com-1ecc6299db9ec823/curl-sys-0.4.3/curl/configure" "--without-ca-bundle" "--without-ca-path" "--with-ssl=/checkout/obj/build/i686-unknown-linux-gnu/openssl/install" "--enable-static=yes" "--enable-shared=no" "--enable-debug" "--disable-optimize" "--prefix=/tmp/cargo-install.a0sVULwVnhEJ/debug/build/curl-sys-f62e1dc14344c926/out" "--without-nghttp2" "--without-librtmp" "--without-libidn2" "--without-libssh2" "--without-libpsl" "--disable-ldap" "--disable-ldaps" "--disable-ftp" "--disable-rtsp" "--disable-dict" "--disable-telnet" "--disable-tftp" "--disable-pop3" "--disable-imap" "--disable-smtp" "--disable-gopher" "--disable-manual" "--disable-smb" "--disable-sspi" "--disable-manual" "--disable-unix-sockets" "--disable-versioned-symbols" "--enable-hidden-symbols" "--disable-libcurl-option"
[01:18:20] checking whether to enable maintainer-specific portions of Makefiles... no
[01:18:20] checking whether make supports nested variables... yes
[01:18:20] checking whether to enable debug build options... yes
[01:18:20] checking whether to enable compiler optimizer... no
[01:18:20] checking whether to enable strict compiler warnings... yes
[01:18:20] checking whether to enable compiler warnings as errors... no
[01:18:20] checking whether to enable curl debug memory tracking... (assumed) yes
[01:18:20] checking whether to enable hiding of library internal symbols... yes
[01:18:20] checking whether to enable c-ares for DNS lookups... no
[01:18:20] checking whether to disable dependency on -lrt... (assumed no)
[01:18:20] checking for path separator... :
[01:18:20] checking for sed... /bin/sed
[01:18:20] checking for egrep... /bin/grep -E
[01:18:20] checking for ar... /rustroot/bin/ar
[01:18:20] checking for a BSD-compatible install... /usr/bin/install -c
[01:18:20] checking for gcc... cc
---
[01:18:20] checking for cc option to accept ISO C89... none needed
[01:18:20] checking whether cc understands -c and -o together... yes
[01:18:20] checking how to run the C preprocessor... cc -E
[01:18:20] checking whether build environment is sane... yes
[01:18:20] checking for a thread-safe mkdir -p... /bin/mkdir -p
[01:18:20] checking whether make sets $(MAKE)... yes
[01:18:20] checking for style of include used by make... GNU
[01:18:20] checking dependency style of cc... gcc3
[01:18:20] checking curl version... 7.54.0-DEV
---
[01:18:20] checking for strings.h... yes
[01:18:20] checking for inttypes.h... (cached) yes
[01:18:20] checking for stdint.h... (cached) yes
[01:18:20] checking for unistd.h... yes
[01:18:20] checking if cpp -P is needed... no
[01:18:20] checking size of long... 4
[01:18:20] checking size of void*... 4
[01:18:20] checking for 64-bit curl_off_t data type... int64_t
[01:18:20] checking size of curl_off_t... 8
[01:18:20] checking formatting string directive for curl_off_t... "ld"
[01:18:20] checking formatting string directive for unsigned curl_off_t... "lu"
[01:18:20] checking constant suffix string for curl_off_t... LL
[01:18:20] checking constant suffix string for unsigned curl_off_t... ULL
[01:18:20] checking if OS is AIX (to define _ALL_SOURCE)... no
[01:18:20] checking if _THREAD_SAFE is already defined... no
[01:18:20] checking if _THREAD_SAFE is actually needed... no
[01:18:20] checking if _THREAD_SAFE is onwards defined... no
[01:18:20] checking if _REENTRANT is already defined... no
[01:18:20] checking if _REENTRANT is actually needed... no
[01:18:20] checking if _REENTRANT is onwards defined... no
[01:18:20] checking for _FILE_OFFSET_BITS value needed for large files... 64
[01:18:20] checking how to print strings... printf
[01:18:20] checking for a sed that does not truncate output... (cached) /bin/sed
[01:18:20] checking for fgrep... /bin/grep -F
[01:18:20] checking for fgrep... /bin/grep -F
[01:18:20] checking for ld used by cc... /rustroot/bin/ld
[01:18:20] checking if the linker (/rustroot/bin/ld) is GNU ld... yes
[01:18:20] checking for BSD- or MS-compatible name lister (nm)... /rustroot/bin/nm -B
[01:18:20] checking the name lister (/rustroot/bin/nm -B) interface... BSD nm
[01:18:20] checking whether ln -s works... yes
[01:18:20] checking the maximum length of command line arguments... 98304
[01:18:20] checking how to convert x86_64-pc-linux-gnu file names to x86_64-pc-linux-gnu format... func_convert_file_noop
[01:18:20] checking how to convert x86_64-pc-linux-gnu file names to toolchain format... func_convert_file_noop
[01:18:20] checking for /rustroot/bin/ld option to reload object files... -r
[01:18:20] checking for objdump... objdump
[01:18:20] checking how to recognize dependent libraries... pass_all
[01:18:20] checking for dlltool... no
[01:18:20] checking how to associate runtime and link libraries... printf %s\n
[01:18:20] checking for archiver @FILE support... @
[01:18:20] checking for strip... strip
[01:18:20] checking for ranlib... ranlib
[01:18:20] checking command to parse /rustroot/bin/nm -B output from cc object... ok
[01:18:20] checking for sysroot... no
[01:18:20] checking for a working dd... /bin/dd
[01:18:20] checking how to truncate binary pipes... /bin/dd bs=4096 count=1
[01:18:20] checking for mt... no
[01:18:20] checking if : is a manifest tool... no
[01:18:20] checking for dlfcn.h... yes
[01:18:20] checking for objdir... .libs
[01:18:20] checking if cc supports -fno-rtti -fno-exceptions... no
[01:18:20] checking for cc option to produce PIC... -fPIC -DPIC
[01:18:20] checking if cc PIC flag -fPIC -DPIC works... yes
[01:18:20] checking if cc static flag -static works... yes
[01:18:20] checking if cc supports -c -o file.o... yes
[01:18:20] checking if cc supports -c -o file.o... (cached) yes
[01:18:20] checking whether the cc linker (/rustroot/bin/ld -m elf_i386) supports shared libraries... yes
[01:18:20] checking dynamic linker characteristics... GNU/Linux ld.so
[01:18:20] checking how to hardcode library paths into programs... immediate
[01:18:20] checking whether stripping libraries is possible... yes
[01:18:20] checking if libtool supports shared libraries... yes
[01:18:20] checking whether to build shared libraries... no
[01:18:20] checking whether to build static libraries... yes
[01:18:20] checking whether to build shared libraries with -version-info... yes
[01:18:20] checking whether to build shared libraries with -no-undefined... no
[01:18:20] checking whether to build shared libraries with -mimpure-text... no
[01:18:20] checking whether to build shared libraries with PIC... yes
[01:18:20] checking whether to build static libraries with PIC... yes
[01:18:20] checking whether to build shared libraries only... no
[01:18:20] checking whether to build static libraries only... yes
[01:18:20] checking for inline... inline
[01:18:20] checking if compiler is DEC/Compaq/HP C... no
[01:18:20] checking if compiler is HP-UX C... no
[01:18:20] checking if compiler is IBM C... no
[01:18:20] checking if compiler is Intel C... no
[01:18:20] checking if compiler is clang... no
[01:18:20] checking if compiler is GNU C... yes
[01:18:20] checking if compiler is LCC... no
[01:18:20] checking if compiler is SGI MIPSpro C... no
[01:18:20] checking if compiler is SGI MIPS C... no
[01:18:20] checking if compiler is SunPro C... no
[01:18:20] checking if compiler is Tiny C... no
[01:18:20] checking if compiler is Watcom C... no
[01:18:20] checking if compiler accepts some basic options... yes
[01:18:20] configure: compiler options added: -Werror-implicit-function-declaration 
[01:18:20] checking if compiler accepts debug enabling options... yes
[01:18:20] configure: compiler options added: -g
[01:18:20] checking if compiler accepts optimizer disabling options... yes
[01:18:20] configure: compiler options added: -O0
[01:18:20] checking if compiler accepts strict warning options... yes
[01:18:20] configure: compiler options added: -pedantic -Wall -W -Wpointer-arith -Wwrite-strings -Wunused -Wshadow -Winline -Wnested-externs -Wmissing-declarations -Wmissing-prototypes -Wno-long-long -Wfloat-equal -Wno-multichar -Wsign-compare -Wundef -Wno-format-nonliteral -Wendif-labels -Wstrict-prototypes -Wdeclaration-after-statement -Wstrict-aliasing=3 -Wcast-align -Wtype-limits -Wold-style-declaration -Wmissing-parameter-type -Wempty-body -Wclobbered -Wignored-qualifiers -Wconversion -Wno-sign-conversion -Wvla -Wno-system-headers 
[01:18:20] checking if compiler halts on compilation errors... yes
[01:18:20] checking if compiler halts on negative sized arrays... yes
[01:18:20] checking if compiler halts on function prototype mismatch... yes
[01:18:20] checking if compiler supports hiding library internal symbols... yes
[01:18:20] checking if curl debug memory tracking can be enabled... yes
[01:18:20] checking for windows.h... no
[01:18:20] checking whether build target is a native Windows one... no
[01:18:20] checking whether build target supports WIN32 file API... no
[01:18:20] checking for good-to-use Mac CFLAGS... no
[01:18:20] checking whether to support http... yes
[01:18:20] checking whether to support ftp... no
[01:18:20] checking whether to support file... yes
[01:18:20] checking whether to support ldap... no
[01:18:20] checking whether to support ldaps... no
[01:18:20] checking whether to support rtsp... no
[01:18:20] checking whether to support proxies... yes
[01:18:20] checking whether to support dict... no
[01:18:20] checking whether to support telnet... no
[01:18:20] checking whether to support tftp... no
[01:18:20] checking whether to support pop3... no
[01:18:20] checking whether to support imap... no
[01:18:20] checking whether to support smb... no
[01:18:20] checking whether to support smtp... no
[01:18:20] checking whether to support gopher... no
[01:18:20] checking whether to provide built-in manual... no
[01:18:20] checking whether to enable generation of C code... no
[01:18:20] checking whether to use libgcc... no
[01:18:20] checking if X/Open network library is required... no
[01:18:20] checking for gethostbyname... yes
[01:18:20] checking for windows.h... (cached) no
[01:18:20] checking for winsock.h... (cached) no
[01:18:20] checking for winsock2.h... (cached) no
[01:18:20] checking for connect in libraries... yes
[01:18:20] checking for sys/types.h... (cached) yes
[01:18:20] checking sys/time.h usability... yes
[01:18:20] checking sys/time.h presence... yes
[01:18:20] checking for sys/time.h... yes
[01:18:20] checking for sys/time.h... yes
[01:18:20] checking time.h usability... yes
[01:18:20] checking time.h presence... yes
[01:18:20] checking for time.h... yes
[01:18:20] checking for monotonic clock_gettime... yes
[01:18:20] checking for clock_gettime in libraries... -lrt
[01:18:20] checking if monotonic clock_gettime works... yes
[01:18:20] checking for pkg-config... /usr/bin/pkg-config
[01:18:20] checking for zlib options with pkg-config... found
[01:18:20] checking zlib.h presence... yes
[01:18:20] checking for zlib.h... yes
[01:18:20] checking for zlib.h... yes
[01:18:20] configure: found both libz and libz.h header
[01:18:20] checking whether to enable IPv6... yes
[01:18:20] checking if struct sockaddr_in6 has sin6_scope_id member... yes
[01:18:20] checking if argv can be written to... yes
[01:18:20] checking if GSS-API support is requested... no
[01:18:20] checking whether to enable Windows native SSL/TLS (Windows native builds only)... no
[01:18:20] checking whether to enable Apple OS native SSL/TLS... no
[01:18:20] configure: PKG_CONFIG_LIBDIR will be set to "/checkout/obj/build/i686-unknown-linux-gnu/openssl/install/lib/pkgconfig"
[01:18:20] checking for pkg-config... (cached) /usr/bin/pkg-config
[01:18:20] checking for openssl options with pkg-config... found
[01:18:20] configure: pkg-config: SSL_LIBS: "-lssl -lcrypto  "
[01:18:20] configure: pkg-config: SSL_LDFLAGS: "-L/checkout/obj/build/i686-unknown-linux-gnu/openssl/install/lib  "
[01:18:20] configure: pkg-config: SSL_CPPFLAGS: "-I/checkout/obj/build/i686-unknown-linux-gnu/openssl/install/include  "
[01:18:20] checking for HMAC_Update in -lcrypto... yes
[01:18:20] checking OpenSSL linking without -ldl... yes
[01:18:20] checking for SSL_connect in -lssl... yes
[01:18:20] checking openssl/x509.h usability... yes
[01:18:20] checking openssl/x509.h presence... yes
[01:18:20] checking for openssl/x509.h... yes
[01:18:20] checking openssl/rsa.h usability... yes
[01:18:20] checking openssl/rsa.h presence... yes
[01:18:20] checking for openssl/rsa.h... yes
[01:18:20] checking openssl/crypto.h usability... yes
[01:18:20] checking openssl/crypto.h presence... yes
[01:18:20] checking for openssl/crypto.h... yes
[01:18:20] checking openssl/pem.h usability... yes
[01:18:20] checking openssl/pem.h presence... yes
[01:18:20] checking for openssl/pem.h... yes
[01:18:20] checking openssl/ssl.h usability... yes
[01:18:20] checking openssl/ssl.h presence... yes
[01:18:20] checking for openssl/ssl.h... yes
[01:18:20] checking openssl/err.h usability... yes
[01:18:20] checking openssl/err.h presence... yes
[01:18:20] checking for openssl/err.h... yes
[01:18:20] checking openssl/pkcs12.h usability... yes
[01:18:20] checking openssl/pkcs12.h presence... yes
[01:18:20] checking for openssl/pkcs12.h... yes
[01:18:20] checking for ENGINE_init... yes
[01:18:20] checking openssl/engine.h usability... yes
[01:18:20] checking openssl/engine.h presence... yes
[01:18:20] checking for openssl/engine.h... yes
[01:18:20] checking for ENGINE_load_builtin_engines... yes
[01:18:20] checking for RAND_egd... yes
[01:18:20] checking for ENGINE_cleanup... yes
[01:18:20] checking for SSL_get_shutdown... yes
[01:18:20] checking for SSLv2_client_method... yes
[01:18:20] checking for BoringSSL... no
[01:18:20] checking for libressl... no
[01:18:20] configure: Added /checkout/obj/build/i686-unknown-linux-gnu/openssl/install/lib to LD_LIBRARY_PATH
[01:18:20] checking for OpenSSL headers version... 1.0.2 - 0x100020efL
[01:18:20] checking for OpenSSL library version... 1.0.2
[01:18:20] checking for OpenSSL headers and library versions matching... yes
[01:18:20] checking for "/dev/urandom"... yes
[01:18:20] checking for SRP_Calc_client_key in -lcrypto... yes
[01:18:20] checking default CA cert bundle/path... no
[01:18:20] checking whether to use builtin CA store of SSL library... no
[01:18:20] checking whether versioned symbols are wanted... no
[01:18:20] checking whether to enable Windows native IDN (Windows native builds only)... no
[01:18:20] checking whether to build with libidn2... no
[01:18:20] checking for malloc.h... yes
[01:18:20] checking for memory.h... no
[01:18:20] checking for sys/types.h... (cached) yes
[01:18:20] checking for sys/time.h... (cached) yes
---
[01:18:20] checking for assert.h... yes
[01:18:20] checking for unistd.h... (cached) yes
[01:18:20] checking for stdlib.h... (cached) yes
[01:18:20] checking for limits.h... yes
[01:18:20] checking for arpa/inet.h... yes
[01:18:20] checking for net/if.h... yes
[01:18:20] checking for netinet/in.h... yes
[01:18:20] checking for sys/un.h... yes
[01:18:20] checking for netinet/tcp.h... yes
[01:18:20] checking for netdb.h... yes
[01:18:20] checking for sys/sockio.h... no
[01:18:20] checking for sys/param.h... yes
[01:18:20] checking for termios.h... yes
[01:18:20] checking for termio.h... yes
[01:18:20] checking for termio.h... yes
[01:18:20] checking for sgtty.h... yes
[01:18:20] checking for alloca.h... yes
[01:18:20] checking for time.h... (cached) yes
[01:18:20] checking for io.h... no
[01:18:20] checking for io.h... no
[01:18:20] checking for pwd.h... yes
[01:18:20] checking for utime.h... yes
[01:18:20] checking for sys/utime.h... no
[01:18:20] checking for sys/poll.h... yes
[01:18:20] checking for poll.h... yes
[01:18:20] checking for socket.h... no
[01:18:20] checking for libgen.h... yes
[01:18:20] checking for locale.h... yes
[01:18:20] checking for errno.h... yes
[01:18:20] checking for stdbool.h... yes
[01:18:20] checking for stdbool.h... yes
[01:18:20] checking for arpa/tftp.h... yes
[01:18:20] checking for sys/filio.h... no
[01:18:20] checking for setjmp.h... yes
[01:18:20] checking for an ANSI C-conforming const... yes
[01:18:20] checking for an ANSI C-conforming const... yes
[01:18:20] checking for compiler support of C99 variadic macro style... yes
[01:18:20] checking for compiler support of old gcc variadic macro style... yes
[01:18:20] checking whether time.h and sys/time.h may both be included... (cached) yes
[01:18:20] checking for sys/types.h... (cached) yes
[01:18:20] checking for sys/time.h... (cached) yes
[01:18:20] checking for time.h... (cached) yes
[01:18:20] checking for time.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for struct timeval... yes
[01:18:20] checking run-time libs availability... fine
[01:18:20] checking size of size_t... 4
[01:18:20] checking size of long... (cached) 4
[01:18:20] checking size of int... 4
[01:18:20] checking size of short... 2
[01:18:20] checking size of time_t... 4
[01:18:20] checking size of off_t... 8
[01:18:20] checking for long long... yes
[01:18:20] checking if numberLL works... yes
[01:18:20] checking for ssize_t... yes
[01:18:20] checking for bool... yes
[01:18:20] checking for windows.h... (cached) no
[01:18:20] checking for winsock2.h... (cached) no
[01:18:20] checking for ws2tcpip.h... (cached) no
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for curl_socklen_t data type... socklen_t
[01:18:20] checking size of curl_socklen_t... 4
[01:18:20] checking for poll.h... (cached) yes
[01:18:20] checking for sys/poll.h... (cached) yes
[01:18:20] checking for in_addr_t... yes
[01:18:20] checking for struct sockaddr_storage... yes
[01:18:20] checking for struct sockaddr_storage... yes
[01:18:20] checking signal.h usability... yes
[01:18:20] checking signal.h presence... yes
[01:18:20] checking for signal.h... yes
[01:18:20] checking for sig_atomic_t... yes
[01:18:20] checking if sig_atomic_t is already defined as volatile... no
[01:18:20] checking for sys/select.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for select... yes
[01:18:20] checking for select... yes
[01:18:20] checking types of args and return type for select... int,fd_set *,struct timeval *,int
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for recv... yes
[01:18:20] checking for recv... yes
[01:18:20] checking types of args and return type for recv... int,void *,size_t,int,int
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for send... yes
[01:18:20] checking for send... yes
[01:18:20] checking types of args and return type for send... int,const void *,size_t,int,int
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for MSG_NOSIGNAL... yes
[01:18:20] checking for unistd.h... (cached) yes
[01:18:20] checking for unistd.h... (cached) yes
[01:18:20] checking if alarm can be linked... yes
[01:18:20] checking if alarm is prototyped... yes
[01:18:20] checking if alarm is compilable... yes
[01:18:20] checking if alarm usage allowed... yes
[01:18:20] checking if alarm might be used... yes
[01:18:20] checking for string.h... (cached) yes
[01:18:20] checking for strings.h... (cached) yes
[01:18:20] checking for sys/types.h... (cached) yes
[01:18:20] checking for libgen.h... (cached) yes
[01:18:20] checking for libgen.h... (cached) yes
[01:18:20] checking if basename can be linked... yes
[01:18:20] checking if basename is prototyped... yes
[01:18:20] checking if basename is compilable... yes
[01:18:20] checking if basename usage allowed... yes
[01:18:20] checking if basename might be used... yes
[01:18:20] checking for socket.h... (cached) no
[01:18:20] checking for socket.h... (cached) no
[01:18:20] checking if closesocket can be linked... no
[01:18:20] checking if closesocket might be used... no
[01:18:20] checking if CloseSocket can be linked... no
[01:18:20] checking if CloseSocket might be used... no
[01:18:20] checking if connect can be linked... yes
[01:18:20] checking if connect is prototyped... yes
[01:18:20] checking if connect is compilable... yes
[01:18:20] checking if connect usage allowed... yes
[01:18:20] checking if connect might be used... yes
[01:18:20] checking for unistd.h... (cached) yes
[01:18:20] checking for fcntl.h... (cached) yes
[01:18:20] checking for fcntl.h... (cached) yes
[01:18:20] checking if fcntl can be linked... yes
[01:18:20] checking if fcntl is prototyped... yes
[01:18:20] checking if fcntl is compilable... yes
[01:18:20] checking if fcntl usage allowed... yes
[01:18:20] checking if fcntl might be used... yes
[01:18:20] checking if fcntl O_NONBLOCK is compilable... yes
[01:18:20] checking if fcntl O_NONBLOCK usage allowed... yes
[01:18:20] checking if fcntl O_NONBLOCK might be used... yes
[01:18:20] checking for stdio.h... yes
[01:18:20] checking for stdio.h... yes
[01:18:20] checking if fdopen can be linked... yes
[01:18:20] checking if fdopen is prototyped... yes
[01:18:20] checking if fdopen is compilable... yes
[01:18:20] checking if fdopen usage allowed... yes
[01:18:20] checking if fdopen might be used... yes
[01:18:20] checking for netdb.h... (cached) yes
[01:18:20] checking for netdb.h... (cached) yes
[01:18:20] checking if freeaddrinfo can be linked... yes
[01:18:20] checking if freeaddrinfo is prototyped... yes
[01:18:20] checking if freeaddrinfo is compilable... yes
[01:18:20] checking if freeaddrinfo usage allowed... yes
[01:18:20] checking if freeaddrinfo might be used... yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for netinet/in.h... (cached) yes
[01:18:20] checking for ifaddrs.h... yes
[01:18:20] checking if freeifaddrs can be linked... yes
[01:18:20] checking if freeifaddrs is prototyped... yes
[01:18:20] checking if freeifaddrs is compilable... yes
[01:18:20] checking if freeifaddrs usage allowed... yes
[01:18:20] checking if freeifaddrs might be used... yes
[01:18:20] checking for sys/types.h... (cached) yes
[01:18:20] checking for sys/xattr.h... yes
[01:18:20] checking if fsetxattr can be linked... yes
[01:18:20] checking if fsetxattr is prototyped... yes
[01:18:20] checking if fsetxattr takes 5 args.... yes
[01:18:20] checking if fsetxattr is compilable... yes
[01:18:20] checking if fsetxattr usage allowed... yes
[01:18:20] checking if fsetxattr might be used... yes
[01:18:20] checking if ftruncate can be linked... yes
[01:18:20] checking if ftruncate is prototyped... yes
[01:18:20] checking if ftruncate is compilable... yes
[01:18:20] checking if ftruncate usage allowed... yes
[01:18:20] checking if ftruncate might be used... yes
[01:18:20] checking for stdlib.h... (cached) yes
[01:18:20] checking for stdlib.h... (cached) yes
[01:18:20] checking if getaddrinfo can be linked... yes
[01:18:20] checking if getaddrinfo is prototyped... yes
[01:18:20] checking if getaddrinfo is compilable... yes
[01:18:20] checking if getaddrinfo seems to work... yes
[01:18:20] checking if getaddrinfo usage allowed... yes
[01:18:20] checking if getaddrinfo might be used... yes
[01:18:20] checking if getaddrinfo is threadsafe... yes
[01:18:20] checking if gai_strerror can be linked... yes
[01:18:20] checking if gai_strerror is prototyped... yes
[01:18:20] checking if gai_strerror is compilable... yes
[01:18:20] checking if gai_strerror usage allowed... yes
[01:18:20] checking if gai_strerror might be used... yes
[01:18:20] checking if gethostbyaddr can be linked... yes
[01:18:20] checking if gethostbyaddr is prototyped... yes
[01:18:20] checking if gethostbyaddr is compilable... yes
[01:18:20] checking if gethostbyaddr usage allowed... yes
[01:18:20] checking if gethostbyaddr might be used... yes
[01:18:20] checking if gethostbyaddr_r can be linked... yes
[01:18:20] checking if gethostbyaddr_r is prototyped... yes
[01:18:20] checking if gethostbyaddr_r takes 5 args.... no
[01:18:20] checking if gethostbyaddr_r takes 7 args.... no
[01:18:20] checking if gethostbyaddr_r takes 8 args.... yes
[01:18:20] checking if gethostbyaddr_r is compilable... yes
[01:18:20] checking if gethostbyaddr_r usage allowed... yes
[01:18:20] checking if gethostbyaddr_r might be used... yes
[01:18:20] checking if gethostbyname can be linked... yes
[01:18:20] checking if gethostbyname is prototyped... yes
[01:18:20] checking if gethostbyname is compilable... yes
[01:18:20] checking if gethostbyname usage allowed... yes
[01:18:20] checking if gethostbyname might be used... yes
[01:18:20] checking if gethostbyname_r can be linked... yes
[01:18:20] checking if gethostbyname_r is prototyped... yes
[01:18:20] checking if gethostbyname_r takes 3 args.... no
[01:18:20] checking if gethostbyname_r takes 5 args.... no
[01:18:20] checking if gethostbyname_r takes 6 args.... yes
[01:18:20] checking if gethostbyname_r is compilable... yes
[01:18:20] checking if gethostbyname_r usage allowed... yes
[01:18:20] checking if gethostbyname_r might be used... yes
[01:18:20] checking if gethostname can be linked... yes
[01:18:20] checking if gethostname is prototyped... yes
[01:18:20] checking if gethostname is compilable... yes
[01:18:20] checking for gethostname arg 2 data type... unsigned int
[01:18:20] checking if gethostname usage allowed... yes
[01:18:20] checking if gethostname might be used... yes
[01:18:20] checking if getifaddrs can be linked... yes
[01:18:20] checking if getifaddrs is prototyped... yes
[01:18:20] checking if getifaddrs is compilable... yes
[01:18:20] checking if getifaddrs seems to work... yes
[01:18:20] checking if getifaddrs usage allowed... yes
[01:18:20] checking if getifaddrs might be used... yes
[01:18:20] checking if getservbyport_r can be linked... yes
[01:18:20] checking if getservbyport_r is prototyped... yes
[01:18:20] checking if getservbyport_r takes 4 args.... no
[01:18:20] checking if getservbyport_r takes 5 args.... no
[01:18:20] checking if getservbyport_r takes 6 args.... yes
[01:18:20] checking if getservbyport_r is compilable... yes
[01:18:20] checking if getservbyport_r usage allowed... yes
[01:18:20] checking if getservbyport_r might be used... yes
[01:18:20] checking for sys/time.h... (cached) yes
[01:18:20] checking for time.h... (cached) yes
[01:18:20] checking for time.h... (cached) yes
[01:18:20] checking if gmtime_r can be linked... yes
[01:18:20] checking if gmtime_r is prototyped... yes
[01:18:20] checking if gmtime_r is compilable... yes
[01:18:20] checking if gmtime_r seems to work... yes
[01:18:20] checking if gmtime_r usage allowed... yes
[01:18:20] checking if gmtime_r might be used... yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for netinet/in.h... (cached) yes
[01:18:20] checking for arpa/inet.h... (cached) yes
[01:18:20] checking if inet_ntoa_r can be linked... no
[01:18:20] checking if inet_ntoa_r might be used... no
[01:18:20] checking if inet_ntop can be linked... yes
[01:18:20] checking if inet_ntop is prototyped... yes
[01:18:20] checking if inet_ntop is compilable... yes
[01:18:20] checking if inet_ntop seems to work... yes
[01:18:20] checking if inet_ntop usage allowed... yes
[01:18:20] checking if inet_ntop might be used... yes
[01:18:20] checking if inet_pton can be linked... yes
[01:18:20] checking if inet_pton is prototyped... yes
[01:18:20] checking if inet_pton is compilable... yes
[01:18:20] checking if inet_pton seems to work... yes
[01:18:20] checking if inet_pton usage allowed... yes
[01:18:20] checking if inet_pton might be used... yes
[01:18:20] checking for unistd.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for sys/ioctl.h... (cached) yes
[01:18:20] checking for stropts.h... yes
[01:18:20] checking for stropts.h... yes
[01:18:20] checking if ioctl can be linked... yes
[01:18:20] checking if ioctl is prototyped... yes
[01:18:20] checking if ioctl is compilable... yes
[01:18:20] checking if ioctl usage allowed... yes
[01:18:20] checking if ioctl might be used... yes
[01:18:20] checking if ioctl FIONBIO is compilable... yes
[01:18:20] checking if ioctl FIONBIO usage allowed... yes
[01:18:20] checking if ioctl FIONBIO might be used... yes
[01:18:20] checking if ioctl SIOCGIFADDR is compilable... yes
[01:18:20] checking if ioctl SIOCGIFADDR usage allowed... yes
[01:18:20] checking if ioctl SIOCGIFADDR might be used... yes
[01:18:20] checking if ioctlsocket can be linked... no
[01:18:20] checking if ioctlsocket might be used... no
[01:18:20] checking if IoctlSocket can be linked... no
[01:18:20] checking if IoctlSocket might be used... no
[01:18:20] checking if localtime_r can be linked... yes
[01:18:20] checking if localtime_r is prototyped... yes
[01:18:20] checking if localtime_r is compilable... yes
[01:18:20] checking if localtime_r seems to work... yes
[01:18:20] checking if localtime_r usage allowed... yes
[01:18:20] checking if localtime_r might be used... yes
[01:18:20] checking if memrchr can be linked... yes
[01:18:20] checking if memrchr is prototyped... no
[01:18:20] checking if memrchr might be used... no
[01:18:20] checking if poll can be linked... yes
[01:18:20] checking if poll is prototyped... yes
[01:18:20] checking if poll is compilable... yes
[01:18:20] checking if poll seems to work... yes
[01:18:20] checking if poll usage allowed... yes
[01:18:20] checking if poll might be used... yes
[01:18:20] checking if setsockopt can be linked... yes
[01:18:20] checking if setsockopt is prototyped... yes
[01:18:20] checking if setsockopt is compilable... yes
[01:18:20] checking if setsockopt usage allowed... yes
[01:18:20] checking if setsockopt might be used... yes
[01:18:20] checking if setsockopt SO_NONBLOCK is compilable... no
[01:18:20] checking if setsockopt SO_NONBLOCK might be used... no
[01:18:20] checking for signal.h... (cached) yes
[01:18:20] checking if sigaction can be linked... yes
[01:18:20] checking if sigaction is prototyped... yes
[01:18:20] checking if sigaction is compilable... yes
[01:18:20] checking if sigaction is compilable... yes
[01:18:20] checking if sigaction usage allowed... yes
[01:18:20] checking if sigaction might be used... yes
[01:18:20] checking if siginterrupt can be linked... yes
[01:18:20] checking if siginterrupt is prototyped... yes
[01:18:20] checking if siginterrupt is compilable... yes
[01:18:20] checking if siginterrupt usage allowed... yes
[01:18:20] checking if siginterrupt might be used... yes
[01:18:20] checking if signal can be linked... yes
[01:18:20] checking if signal is prototyped... yes
[01:18:20] checking if signal is compilable... yes
[01:18:20] checking if signal usage allowed... yes
[01:18:20] checking if signal might be used... yes
[01:18:20] checking for setjmp.h... (cached) yes
[01:18:20] checking for setjmp.h... (cached) yes
[01:18:20] checking if sigsetjmp can be linked... no
[01:18:20] checking if sigsetjmp seems a macro... yes
[01:18:20] checking if sigsetjmp is compilable... yes
[01:18:20] checking if sigsetjmp usage allowed... yes
[01:18:20] checking if sigsetjmp might be used... yes
[01:18:20] checking if socket can be linked... yes
[01:18:20] checking if socket is prototyped... yes
[01:18:20] checking if socket is compilable... yes
[01:18:20] checking if socket usage allowed... yes
[01:18:20] checking if socket might be used... yes
[01:18:20] checking if socketpair can be linked... yes
[01:18:20] checking if socketpair is prototyped... yes
[01:18:20] checking if socketpair is compilable... yes
[01:18:20] checking if socketpair usage allowed... yes
[01:18:20] checking if socketpair might be used... yes
[01:18:20] checking if strcasecmp can be linked... yes
[01:18:20] checking if strcasecmp is prototyped... yes
[01:18:20] checking if strcasecmp is compilable... yes
[01:18:20] checking if strcasecmp usage allowed... yes
[01:18:20] checking if strcasecmp might be used... yes
[01:18:20] checking if strcmpi can be linked... no
[01:18:20] checking if strcmpi might be used... no
[01:18:20] checking if strdup can be linked... yes
[01:18:20] checking if strdup is prototyped... yes
[01:18:20] checking if strdup is compilable... yes
[01:18:20] checking if strdup usage allowed... yes
[01:18:20] checking if strdup might be used... yes
[01:18:20] checking if strerror_r can be linked... yes
[01:18:20] checking if strerror_r is prototyped... yes
[01:18:20] checking if strerror_r is compilable... yes
[01:18:20] checking if strerror_r is glibc like... no
[01:18:20] checking if strerror_r is POSIX like... yes
[01:18:20] checking if strerror_r seems to work... yes
[01:18:20] checking if strerror_r usage allowed... yes
[01:18:20] checking if strerror_r might be used... yes
[01:18:20] checking if stricmp can be linked... no
[01:18:20] checking if stricmp might be used... no
[01:18:20] checking if strncasecmp can be linked... yes
[01:18:20] checking if strncasecmp is prototyped... yes
[01:18:20] checking if strncasecmp is compilable... yes
[01:18:20] checking if strncasecmp usage allowed... yes
[01:18:20] checking if strncasecmp might be used... yes
[01:18:20] checking if strncmpi can be linked... no
[01:18:20] checking if strncmpi might be used... no
[01:18:20] checking if strnicmp can be linked... no
[01:18:20] checking if strnicmp might be used... no
[01:18:20] checking if strstr can be linked... yes
[01:18:20] checking if strstr is prototyped... yes
[01:18:20] checking if strstr is compilable... yes
[01:18:20] checking if strstr usage allowed... yes
[01:18:20] checking if strstr might be used... yes
[01:18:20] checking if strtok_r can be linked... yes
[01:18:20] checking if strtok_r is prototyped... yes
[01:18:20] checking if strtok_r is compilable... yes
[01:18:20] checking if strtok_r usage allowed... yes
[01:18:20] checking if strtok_r might be used... yes
[01:18:20] checking if strtoll can be linked... yes
[01:18:20] checking if strtoll is prototyped... yes
[01:18:20] checking if strtoll is compilable... yes
[01:18:20] checking if strtoll usage allowed... yes
[01:18:20] checking if strtoll might be used... yes
[01:18:20] checking for sys/uio.h... (cached) yes
[01:18:20] checking for sys/uio.h... (cached) yes
[01:18:20] checking if writev can be linked... yes
[01:18:20] checking if writev is prototyped... yes
[01:18:20] checking if writev is compilable... yes
[01:18:20] checking if writev usage allowed... yes
[01:18:20] checking if writev might be used... yes
[01:18:20] checking for geteuid... yes
[01:18:20] checking for getpass_r... no
[01:18:20] checking for getpass_r... no
[01:18:20] checking deeper for getpass_r... but still no
[01:18:20] checking for getppid... yes
[01:18:20] checking for getprotobyname... yes
[01:18:20] checking for getpwuid... yes
[01:18:20] checking for getpwuid_r... yes
[01:18:20] checking for gettimeofday... yes
[01:18:20] checking for gettimeofday... yes
[01:18:20] checking for if_nametoindex... yes
[01:18:20] checking for inet_addr... yes
[01:18:20] checking for perror... yes
[01:18:20] checking for setlocale... yes
[01:18:20] checking for setlocale... yes
[01:18:20] checking for setmode... no
[01:18:20] checking deeper for setmode... but still no
[01:18:20] checking for uname... yes
[01:18:20] checking for utime... yes
[01:18:20] checking for sys/types.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for netdb.h... (cached) yes
[01:18:20] checking for getnameinfo... yes
[01:18:20] checking types of arguments for getnameinfo... const struct sockaddr *,socklen_t,size_t,unsigned int
[01:18:20] checking for stdio.h... (cached) yes
[01:18:20] checking for sys/socket.h... (cached) yes
[01:18:20] checking for netdb.h... (cached) yes
[01:18:20] checking for netdb.h... (cached) yes
[01:18:20] checking for netinet/in.h... (cached) yes
[01:18:20] checking for arpa/inet.h... (cached) yes
[01:18:20] checking for working NI_WITHSCOPEID... no
[01:18:20] checking how to set a socket into non-blocking mode... fcntl O_NONBLOCK
[01:18:20] checking for perl... /usr/bin/perl
[01:18:20] checking for gnroff... no
[01:18:20] checking for nroff... no
[01:18:20] checking whether to enable the threaded resolver... no
[01:18:20] checking whether to use POSIX threads for threaded resolver... auto
[01:18:20] checking whether to enable verbose strings... yes
[01:18:20] checking whether to enable SSPI support (Windows native builds only)... no
[01:18:20] checking whether to enable cryptographic authentication methods... yes
[01:18:20] checking whether to enable NTLM delegation to winbind's helper... yes
[01:18:20] checking whether to enable TLS-SRP authentication... yes
[01:18:20] checking whether to enable Unix domain sockets... no
[01:18:20] checking whether to enable support for cookies... yes
[01:18:20] checking whether hiding of library internal symbols will actually happen... yes
[01:18:20] checking whether to enforce SONAME bump... no
[01:18:20] checking that generated files are newer than configure... done
[01:18:20] config.status: creating Makefile
[01:18:20] 
[01:18:20] --- stderr
[01:18:20] --- stderr
[01:18:20] fatal: Not a git repository (or any parent up to mount point /cargo)
[01:18:20] Stopping at filesystem boundary (GIT_DISCOVERY_ACROSS_FILESYSTEM not set).
[01:18:20] configure: WARNING: disabling built-in manual
[01:18:20] config.status: error: cannot find input file: `docs/Makefile.in'
[01:18:20] command did not execute successfully, got: exit code: 1
[01:18:20] 
[01:18:20] 
[01:18:20] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/curl-sys-0.4.3/build.rs:224:5
[01:18:20] 
[01:18:20] 
[01:18:20] 
[01:18:20] 
[01:18:20] command did not execute successfully: "/checkout/obj/build/i686-unknown-linux-gnu/stage0/bin/cargo" "install" "--force" "--debug" "--vers" "0.1.4" "cargo-vendor"
[01:18:20] 
[01:18:20] 
[01:18:20] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build i686-unknown-linux-gnu --host i686-unknown-linux-gnu --target i686-unknown-linux-gnu
[01:18:20] Build completed unsuccessfully in 1:14:00
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:25b831f0
$ cat obj/tmp/sccache.log
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
WARN:sccache::cache::s3: Got AWS error: Error(BadHTTPStatus(NotFound), State { next_error: None })
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:3a6aca75
$ cat /tmp/sccache.log
