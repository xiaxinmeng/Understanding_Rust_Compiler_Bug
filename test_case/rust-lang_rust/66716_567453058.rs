plain
2019-12-19T09:28:46.3394887Z   llvm-3.8-runtime llvm-runtime mesa-common-dev mime-support openssl patch
2019-12-19T09:28:46.3395403Z   perl perl-modules-5.22 python2.7-minimal x11proto-core-dev
2019-12-19T09:28:46.3395975Z   x11proto-damage-dev x11proto-dri2-dev x11proto-fixes-dev x11proto-gl-dev
2019-12-19T09:28:46.3396476Z   x11proto-input-dev x11proto-kb-dev x11proto-xext-dev
2019-12-19T09:28:46.3397046Z   x11proto-xf86vidmode-dev xorg-sgml-doctools xtrans-dev zlib1g-dev
2019-12-19T09:28:46.3397968Z   binutils-doc bzip2-doc codeblocks eclipse ninja-build cpp-doc gcc-5-locales
2019-12-19T09:28:46.3398533Z   debian-keyring g++-multilib g++-5-multilib gcc-5-doc libstdc++6-5-dbg
2019-12-19T09:28:46.3399058Z   gcc-multilib manpages-dev autoconf automake libtool flex bison gdb gcc-doc
2019-12-19T09:28:46.3399633Z   gcc-5-multilib libgcc1-dbg libgomp1-dbg libitm1-dbg libatomic1-dbg
2019-12-19T09:28:46.3399633Z   gcc-5-multilib libgcc1-dbg libgomp1-dbg libitm1-dbg libatomic1-dbg
2019-12-19T09:28:46.3408099Z   libasan2-dbg liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg
2019-12-19T09:28:46.3410443Z   libmpx0-dbg libquadmath0-dbg gettext-base git-daemon-run
2019-12-19T09:28:46.3412322Z   | git-daemon-sysvinit git-doc git-el git-email git-gui gitk gitweb git-arch
2019-12-19T09:28:46.3414122Z   git-cvs git-mediawiki git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user
2019-12-19T09:28:46.3416192Z   pciutils lm-sensors libstdc++-5-doc libxcb-doc libxext-doc llvm-3.8-doc
2019-12-19T09:28:46.3419545Z   | libterm-readline-perl-perl python2.7-doc
2019-12-19T09:28:46.3421525Z Recommended packages:
2019-12-19T09:28:46.3423301Z   build-essential fakeroot libalgorithm-merge-perl less rsync ssh-client
2019-12-19T09:28:46.3425013Z   manpages manpages-dev libfile-fcntllock-perl libtxc-dxtn-s2tc
---
2019-12-19T09:29:54.8629417Z Setting up libxcb-glx0:amd64 (1.11.1-1ubuntu1) ...
2019-12-19T09:29:54.9076605Z Setting up libxcb-present0:amd64 (1.11.1-1ubuntu1) ...
2019-12-19T09:29:54.9475210Z Setting up libxcb-sync1:amd64 (1.11.1-1ubuntu1) ...
2019-12-19T09:29:54.9934179Z Setting up libgl1-mesa-glx:amd64 (18.0.5-0ubuntu0~16.04.1) ...
2019-12-19T09:29:55.0230431Z update-alternatives: using /usr/lib/x86_64-linux-gnu/mesa/ld.so.conf to provide /etc/ld.so.conf.d/x86_64-linux-gnu_GL.conf (x86_64-linux-gnu_gl_conf) in auto mode
2019-12-19T09:29:55.1092676Z Setting up libpthread-stubs0-dev:amd64 (0.3-4) ...
2019-12-19T09:29:55.1558754Z Setting up libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.9) ...
2019-12-19T09:29:55.2027949Z Setting up libssl-dev:amd64 (1.0.2g-1ubuntu4.15) ...
2019-12-19T09:29:55.2453389Z Setting up libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
---
2019-12-19T11:28:23.3093645Z    Compiling wayland-sys v0.21.13
2019-12-19T11:28:25.8679916Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2019-12-19T11:28:25.8698452Z 
2019-12-19T11:28:25.8698594Z Caused by:
2019-12-19T11:28:25.8699188Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-c7284c5c6a30d3ba/build-script-build` (exit code: 101)
2019-12-19T11:28:25.8699498Z --- stdout
2019-12-19T11:28:25.8699851Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2019-12-19T11:28:25.8700200Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out && \
2019-12-19T11:28:25.8700313Z  CC="gcc" \
2019-12-19T11:28:25.8700374Z  AR="ar" \
2019-12-19T11:28:25.8700455Z  FREETYPE_CFLAGS="" \
2019-12-19T11:28:25.8700522Z  FREETYPE_LIBS="" \
2019-12-19T11:28:25.8700767Z  CFLAGS=""" -fPIC" \
2019-12-19T11:28:25.8701114Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2019-12-19T11:28:25.8701367Z  --disable-docs \
2019-12-19T11:28:25.8701601Z  --disable-shared \
2019-12-19T11:28:25.8701668Z   \
2019-12-19T11:28:25.8701959Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2019-12-19T11:28:25.8702341Z checking whether build environment is sane... yes
2019-12-19T11:28:25.8702606Z checking for x86_64-unknown-linux-gnu-strip... no
2019-12-19T11:28:25.8702699Z checking for strip... strip
2019-12-19T11:28:25.8702955Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2019-12-19T11:28:25.8702955Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2019-12-19T11:28:25.8703050Z checking for gawk... no
2019-12-19T11:28:25.8703134Z checking for mawk... mawk
2019-12-19T11:28:25.8703296Z checking whether make supports nested variables... yes
2019-12-19T11:28:25.8703379Z checking whether make supports nested variables... (cached) yes
2019-12-19T11:28:25.8703717Z checking whether to enable maintainer-specific portions of Makefiles... no
2019-12-19T11:28:25.8703989Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2019-12-19T11:28:25.8708828Z checking whether make sets $(MAKE)... (cached) yes
2019-12-19T11:28:25.8709098Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2019-12-19T11:28:25.8709374Z checking for pkg-config... /usr/bin/pkg-config
2019-12-19T11:28:25.8709632Z checking pkg-config is at least version 0.9.0... yes
2019-12-19T11:28:25.8709887Z checking for RM macro... rm -f
2019-12-19T11:28:25.8710435Z checking host system type... x86_64-unknown-linux-gnu
2019-12-19T11:28:25.8710531Z checking how to print strings... printf
2019-12-19T11:28:25.8710624Z checking for a sed that does not truncate output... /bin/sed
2019-12-19T11:28:25.8710889Z checking for fgrep... /bin/grep -F
2019-12-19T11:28:25.8710889Z checking for fgrep... /bin/grep -F
2019-12-19T11:28:25.8710962Z checking for ld used by gcc... /usr/bin/ld
2019-12-19T11:28:25.8711056Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2019-12-19T11:28:25.8711336Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2019-12-19T11:28:25.8711662Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2019-12-19T11:28:25.8711756Z checking the maximum length of command line arguments... 1572864
2019-12-19T11:28:25.8711869Z checking whether the shell understands some XSI constructs... yes
2019-12-19T11:28:25.8711973Z checking whether the shell understands "+="... yes
2019-12-19T11:28:25.8712433Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2019-12-19T11:28:25.8712907Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2019-12-19T11:28:25.8713215Z checking for /usr/bin/ld option to reload object files... -r
2019-12-19T11:28:25.8713612Z checking for x86_64-unknown-linux-gnu-objdump... no
2019-12-19T11:28:25.8713778Z checking how to recognize dependent libraries... pass_all
2019-12-19T11:28:25.8714035Z checking for x86_64-unknown-linux-gnu-dlltool... no
2019-12-19T11:28:25.8714127Z checking for dlltool... no
2019-12-19T11:28:25.8714219Z checking how to associate runtime and link libraries... printf %s\n
2019-12-19T11:28:25.8714219Z checking how to associate runtime and link libraries... printf %s\n
2019-12-19T11:28:25.8714473Z checking for x86_64-unknown-linux-gnu-ar... ar
2019-12-19T11:28:25.8714566Z checking for archiver @FILE support... @
2019-12-19T11:28:25.8714816Z checking for x86_64-unknown-linux-gnu-strip... strip
2019-12-19T11:28:25.8715087Z checking for x86_64-unknown-linux-gnu-ranlib... no
2019-12-19T11:28:25.8715160Z checking for ranlib... ranlib
2019-12-19T11:28:25.8715443Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2019-12-19T11:28:25.8715530Z checking for sysroot... no
2019-12-19T11:28:25.8715904Z checking for x86_64-unknown-linux-gnu-mt... no
2019-12-19T11:28:25.8716007Z checking for mt... no
2019-12-19T11:28:25.8716071Z checking if : is a manifest tool... no
2019-12-19T11:28:25.8716155Z checking for dlfcn.h... yes
2019-12-19T11:28:25.8716220Z checking for objdir... .libs
2019-12-19T11:28:25.8716530Z checking if gcc supports -fno-rtti -fno-exceptions... no
2019-12-19T11:28:25.8716908Z checking for gcc option to produce PIC... -fPIC -DPIC
2019-12-19T11:28:25.8717186Z checking if gcc PIC flag -fPIC -DPIC works... yes
2019-12-19T11:28:25.8717441Z checking if gcc static flag -static works... yes
2019-12-19T11:28:25.8718018Z checking if gcc supports -c -o file.o... yes
2019-12-19T11:28:25.8718300Z checking if gcc supports -c -o file.o... (cached) yes
2019-12-19T11:28:25.8718641Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2019-12-19T11:28:25.8718759Z checking dynamic linker characteristics... GNU/Linux ld.so
2019-12-19T11:28:25.8718974Z checking how to hardcode library paths into programs... immediate
2019-12-19T11:28:25.8719077Z checking whether stripping libraries is possible... yes
2019-12-19T11:28:25.8719156Z checking if libtool supports shared libraries... yes
2019-12-19T11:28:25.8719252Z checking whether to build shared libraries... no
2019-12-19T11:28:25.8719422Z checking for dirent.h that defines DIR... yes
2019-12-19T11:28:25.8719500Z checking for library containing opendir... none required
2019-12-19T11:28:25.8719596Z checking for ANSI C header files... (cached) yes
2019-12-19T11:28:25.8719686Z checking fcntl.h usability... yes
---
2019-12-19T11:28:25.8721119Z checking for sys/statfs.h... yes
2019-12-19T11:28:25.8721177Z checking sys/param.h usability... yes
2019-12-19T11:28:25.8721254Z checking sys/param.h presence... yes
2019-12-19T11:28:25.8721313Z checking for sys/param.h... yes
2019-12-19T11:28:25.8721394Z checking sys/mount.h usability... yes
2019-12-19T11:28:25.8721459Z checking sys/mount.h presence... yes
2019-12-19T11:28:25.8721534Z checking for sys/mount.h... yes
2019-12-19T11:28:25.8721598Z checking for stdint types... stdint.h (shortcircuit)
2019-12-19T11:28:25.8721687Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2019-12-19T11:28:25.8722041Z checking for inline... inline
2019-12-19T11:28:25.8722120Z checking for flexible array members... yes
2019-12-19T11:28:25.8722180Z checking for pid_t... yes
2019-12-19T11:28:25.8722255Z checking for vprintf... yes
---
2019-12-19T11:28:25.8722582Z checking for getpagesize... yes
2019-12-19T11:28:25.8722665Z checking for working mmap... yes
2019-12-19T11:28:25.8722795Z checking for link... yes
2019-12-19T11:28:25.8722878Z checking for mkstemp... yes
2019-12-19T11:28:25.8722952Z checking for mkostemp... yes
2019-12-19T11:28:25.8723010Z checking for _mktemp_s... no
2019-12-19T11:28:25.8723141Z checking for getopt... yes
2019-12-19T11:28:25.8723216Z checking for getopt_long... yes
2019-12-19T11:28:25.8723275Z checking for getprogname... no
2019-12-19T11:28:25.8723350Z checking for getexecname... no
---
2019-12-19T11:28:25.8724207Z checking for fstatfs... yes
2019-12-19T11:28:25.8724281Z checking for lstat... yes
2019-12-19T11:28:25.8724344Z checking for posix_fadvise in fcntl.h... fcntl.h
2019-12-19T11:28:25.8724422Z checking for scandir... yes
2019-12-19T11:28:25.8724483Z checking for struct statvfs.f_basetype... no
2019-12-19T11:28:25.8724565Z checking for struct statvfs.f_fstypename... no
2019-12-19T11:28:25.8724627Z checking for struct statfs.f_flags... yes
2019-12-19T11:28:25.8724708Z checking for struct statfs.f_fstypename... no
2019-12-19T11:28:25.8724787Z checking for struct dirent.d_type... yes
2019-12-19T11:28:25.8724847Z checking for FREETYPE... yes
2019-12-19T11:28:25.8724922Z checking for FT_Get_Next_Char... yes
2019-12-19T11:28:25.8724981Z checking for FT_Get_BDF_Property... yes
2019-12-19T11:28:25.8725058Z checking for FT_Get_PS_Font_Info... yes
2019-12-19T11:28:25.8725131Z checking for FT_Has_PS_Glyph_Names... yes
2019-12-19T11:28:25.8725211Z checking for FT_Get_X11_Font_Format... yes
2019-12-19T11:28:25.8725270Z checking for FT_Select_Size... yes
2019-12-19T11:28:25.8725348Z checking for FT_Bitmap_Size.y_ppem... yes
2019-12-19T11:28:25.8725480Z checking expat.h usability... yes
2019-12-19T11:28:25.8725538Z checking expat.h presence... yes
2019-12-19T11:28:25.8725612Z checking for expat.h... yes
2019-12-19T11:28:25.8725612Z checking for expat.h... yes
2019-12-19T11:28:25.8725713Z checking for XML_SetDoctypeDeclHandler... yes
2019-12-19T11:28:25.8725776Z checking for Intel atomic primitives... true
2019-12-19T11:28:25.8725859Z checking for Solaris atomic operations... false
2019-12-19T11:28:25.8726140Z checking if compiler needs -Werror to reject unknown flags... no
2019-12-19T11:28:25.8726393Z checking for the pthreads library -lpthreads... no
2019-12-19T11:28:25.8726464Z checking whether pthreads work without any flags... no
2019-12-19T11:28:25.8726722Z checking whether pthreads work with -Kthread... no
2019-12-19T11:28:25.8726950Z checking whether pthreads work with -kthread... no
2019-12-19T11:28:25.8727194Z checking for the pthreads library -llthread... no
2019-12-19T11:28:25.8727419Z checking whether pthreads work with -pthread... yes
2019-12-19T11:28:25.8727511Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2019-12-19T11:28:25.8727940Z checking if more special flags are required for pthreads... no
2019-12-19T11:28:25.8728032Z checking for PTHREAD_PRIO_INHERIT... yes
2019-12-19T11:28:25.8728119Z checking for docbook2html... no
2019-12-19T11:28:25.8728191Z checking whether byte ordering is bigendian... no
2019-12-19T11:28:25.8728279Z checking size of void *... 8
2019-12-19T11:28:25.8728345Z checking alignment of double... 8
2019-12-19T11:28:25.8728439Z checking that generated files are newer than configure... done
2019-12-19T11:28:25.8728610Z config.status: creating Makefile
2019-12-19T11:28:25.8728799Z config.status: creating fontconfig/Makefile
2019-12-19T11:28:25.8729137Z config.status: creating fc-lang/Makefile
2019-12-19T11:28:25.8729137Z config.status: creating fc-lang/Makefile
2019-12-19T11:28:25.8729409Z config.status: creating fc-glyphname/Makefile
2019-12-19T11:28:25.8729653Z config.status: creating fc-case/Makefile
2019-12-19T11:28:25.8729744Z config.status: creating src/Makefile
2019-12-19T11:28:25.8729812Z config.status: creating conf.d/Makefile
2019-12-19T11:28:25.8730073Z config.status: creating fc-cache/Makefile
2019-12-19T11:28:25.8730311Z config.status: creating fc-cat/Makefile
2019-12-19T11:28:25.8730570Z config.status: creating fc-list/Makefile
2019-12-19T11:28:25.8730810Z config.status: creating fc-match/Makefile
2019-12-19T11:28:25.8731074Z config.status: creating fc-pattern/Makefile
2019-12-19T11:28:25.8731413Z config.status: creating fc-query/Makefile
2019-12-19T11:28:25.8731647Z config.status: creating fc-scan/Makefile
2019-12-19T11:28:25.8731880Z config.status: creating fc-validate/Makefile
2019-12-19T11:28:25.8732123Z config.status: creating doc/version.sgml
2019-12-19T11:28:25.8732183Z config.status: creating test/Makefile
2019-12-19T11:28:25.8732260Z config.status: creating fontconfig.spec
2019-12-19T11:28:25.8732320Z config.status: creating fontconfig.pc
2019-12-19T11:28:25.8732320Z config.status: creating fontconfig.pc
2019-12-19T11:28:25.8732579Z config.status: creating fontconfig-zip
2019-12-19T11:28:25.8732643Z config.status: creating config.h
2019-12-19T11:28:25.8732722Z config.status: executing depfiles commands
2019-12-19T11:28:25.8732784Z config.status: executing libtool commands
2019-12-19T11:28:25.8732867Z config.status: executing src/fcstdint.h commands
2019-12-19T11:28:25.8732955Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2019-12-19T11:28:25.8733262Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out && make -j2
2019-12-19T11:28:25.8733605Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out'
2019-12-19T11:28:25.8734161Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out'
2019-12-19T11:28:25.8734247Z Making all in fontconfig
2019-12-19T11:28:25.8775565Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fontconfig'
2019-12-19T11:28:25.8776063Z make[4]: Nothing to be done for 'all'.
2019-12-19T11:28:25.8776063Z make[4]: Nothing to be done for 'all'.
2019-12-19T11:28:25.8776431Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fontconfig'
2019-12-19T11:28:25.8776731Z Making all in fc-case
2019-12-19T11:28:25.8777102Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-case'
2019-12-19T11:28:25.8777224Z   GEN      fcalias.h
2019-12-19T11:28:25.8777291Z   GEN      fcaliastail.h
2019-12-19T11:28:25.8778058Z make  all-am
2019-12-19T11:28:25.8778439Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-case'
2019-12-19T11:28:25.8778753Z make[5]: Nothing to be done for 'all-am'.
2019-12-19T11:28:25.8779132Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-case'
2019-12-19T11:28:25.8779543Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-case'
2019-12-19T11:28:25.8779837Z Making all in fc-lang
2019-12-19T11:28:25.8780214Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-lang'
2019-12-19T11:28:25.8780338Z   GEN      fcalias.h
2019-12-19T11:28:25.8780424Z   GEN      fcaliastail.h
2019-12-19T11:28:25.8780659Z make  all-am
2019-12-19T11:28:25.8781343Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-lang'
2019-12-19T11:28:25.8781464Z   GEN      fclang.h
2019-12-19T11:28:25.8781860Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-lang'
2019-12-19T11:28:25.8782095Z   GEN      fc-lang
2019-12-19T11:28:25.8782444Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-lang'
2019-12-19T11:28:25.8782802Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-lang'
2019-12-19T11:28:25.8783183Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-lang'
2019-12-19T11:28:25.8783461Z Making all in fc-glyphname
2019-12-19T11:28:25.8784051Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-glyphname'
2019-12-19T11:28:25.8784283Z   GEN      fcaliastail.h
2019-12-19T11:28:25.8784350Z   GEN      fcalias.h
2019-12-19T11:28:25.8784610Z make  all-am
2019-12-19T11:28:25.8784967Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-glyphname'
2019-12-19T11:28:25.8785262Z make[5]: Nothing to be done for 'all-am'.
2019-12-19T11:28:25.8785644Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-glyphname'
2019-12-19T11:28:25.8786034Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/fc-glyphname'
2019-12-19T11:28:25.8786502Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/src'
2019-12-19T11:28:25.8786620Z   GEN      fcalias.h
2019-12-19T11:28:25.8786620Z   GEN      fcalias.h
2019-12-19T11:28:25.8786692Z   GEN      fcftalias.h
2019-12-19T11:28:25.8787046Z   GEN      stamp-fcstdint
2019-12-19T11:28:25.8787140Z config.status: executing src/fcstdint.h commands
2019-12-19T11:28:25.8787220Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2019-12-19T11:28:25.8787316Z config.status: src/fcstdint.h is unchanged
2019-12-19T11:28:25.8787383Z   GEN      fcobjshash.gperf
2019-12-19T11:28:25.8787950Z   GEN      fcobjshash.h
2019-12-19T11:28:25.8788296Z Makefile:882: recipe for target 'fcobjshash.h' failed
2019-12-19T11:28:25.8788678Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/src'
2019-12-19T11:28:25.8788970Z Makefile:562: recipe for target 'all-recursive' failed
2019-12-19T11:28:25.8789342Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out'
2019-12-19T11:28:25.8789636Z Makefile:445: recipe for target 'all' failed
2019-12-19T11:28:25.8789996Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out'
2019-12-19T11:28:25.8790441Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/libfontconfig.a' failed
2019-12-19T11:28:25.8790807Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2019-12-19T11:28:25.8791104Z --- stderr
2019-12-19T11:28:25.8791387Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2019-12-19T11:28:25.8791387Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2019-12-19T11:28:25.8791737Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2019-12-19T11:28:25.8792028Z WARNING: 'gperf' is missing on your system.
2019-12-19T11:28:25.8792115Z          You might have modified some files without having the proper
2019-12-19T11:28:25.8792421Z          tools for further handling them.  Check the 'README' file, it
2019-12-19T11:28:25.8792759Z          often tells you about the needed prerequisites for installing
2019-12-19T11:28:25.8792855Z          this package.  You may also peek at any GNU archive site, in
2019-12-19T11:28:25.8793180Z          case some other package contains this missing 'gperf' program.
2019-12-19T11:28:25.8793259Z make[4]: *** [fcobjshash.h] Error 1
2019-12-19T11:28:25.8793348Z make[4]: *** Waiting for unfinished jobs....
2019-12-19T11:28:25.8793582Z make[3]: *** [all-recursive] Error 1
2019-12-19T11:28:25.8793671Z make[2]: *** [all] Error 2
2019-12-19T11:28:25.8794016Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-6c491ff7095db0e4/out/libfontconfig.a] Error 2
2019-12-19T11:28:25.8794347Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2019-12-19T11:28:25.8794664Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2019-12-19T11:28:25.8794963Z                                                                                  "-f",
2019-12-19T11:28:25.8795570Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2019-12-19T11:28:25.8795791Z 
2019-12-19T11:28:25.8795863Z warning: build failed, waiting for other jobs to finish...
2019-12-19T11:28:34.5166331Z warning: use of deprecated item 'try': use the `?` operator instead
2019-12-19T11:28:34.5167416Z    --> webrender/src/batch.rs:342:1
---
2019-12-19T11:30:36.9657853Z 
2019-12-19T11:30:36.9657904Z 
2019-12-19T11:30:36.9672153Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-12-19T11:30:36.9672571Z Build completed unsuccessfully in 1:57:43
2019-12-19T11:30:36.9721576Z Makefile:50: recipe for target 'check-aux' failed
2019-12-19T11:30:36.9722497Z make: *** [check-aux] Error 1
2019-12-19T11:30:36.9739533Z   local time: Thu Dec 19 11:30:36 UTC 2019
2019-12-19T11:30:37.2515660Z   network time: Thu, 19 Dec 2019 11:30:37 GMT
2019-12-19T11:30:37.2516423Z == end clock drift check ==
2019-12-19T11:30:51.8218795Z 
2019-12-19T11:30:51.8218795Z 
2019-12-19T11:30:51.8338577Z ##[error]Bash exited with code '2'.
2019-12-19T11:30:51.8387372Z ##[section]Starting: Checkout
2019-12-19T11:30:51.8389473Z ==============================================================================
2019-12-19T11:30:51.8389962Z Task         : Get sources
2019-12-19T11:30:51.8390106Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
