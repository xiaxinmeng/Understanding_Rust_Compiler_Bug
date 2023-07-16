plain
2020-01-17T21:41:30.7613870Z 
2020-01-17T21:41:38.2029255Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2020-01-17T21:41:38.2030517Z 
2020-01-17T21:41:38.2030938Z Caused by:
2020-01-17T21:41:38.2031709Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4afac713bc9da71e/build-script-build` (exit code: 101)
2020-01-17T21:41:38.2032053Z --- stdout
2020-01-17T21:41:38.2032688Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-17T21:41:38.2033448Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out && \
2020-01-17T21:41:38.2033636Z  CC="gcc" \
2020-01-17T21:41:38.2035923Z  AR="ar" \
2020-01-17T21:41:38.2036000Z  FREETYPE_CFLAGS="" \
2020-01-17T21:41:38.2036088Z  FREETYPE_LIBS="" \
2020-01-17T21:41:38.2036456Z  CFLAGS=""" -fPIC" \
2020-01-17T21:41:38.2036832Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2020-01-17T21:41:38.2037266Z  --disable-docs \
2020-01-17T21:41:38.2037543Z  --disable-shared \
2020-01-17T21:41:38.2037614Z   \
2020-01-17T21:41:38.2037942Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2020-01-17T21:41:38.2038365Z checking whether build environment is sane... yes
2020-01-17T21:41:38.2038662Z checking for x86_64-unknown-linux-gnu-strip... no
2020-01-17T21:41:38.2038764Z checking for strip... strip
2020-01-17T21:41:38.2039082Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-17T21:41:38.2039082Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-17T21:41:38.2039166Z checking for gawk... no
2020-01-17T21:41:38.2039252Z checking for mawk... mawk
2020-01-17T21:41:38.2039550Z checking whether make supports nested variables... yes
2020-01-17T21:41:38.2039638Z checking whether make supports nested variables... (cached) yes
2020-01-17T21:41:38.2039996Z checking whether to enable maintainer-specific portions of Makefiles... no
2020-01-17T21:41:38.2040613Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2020-01-17T21:41:38.2045004Z checking whether make sets $(MAKE)... (cached) yes
2020-01-17T21:41:38.2045311Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2020-01-17T21:41:38.2045627Z checking for pkg-config... /usr/bin/pkg-config
2020-01-17T21:41:38.2045943Z checking pkg-config is at least version 0.9.0... yes
2020-01-17T21:41:38.2046248Z checking for RM macro... rm -f
2020-01-17T21:41:38.2046886Z checking host system type... x86_64-unknown-linux-gnu
2020-01-17T21:41:38.2046992Z checking how to print strings... printf
2020-01-17T21:41:38.2047074Z checking for a sed that does not truncate output... /bin/sed
2020-01-17T21:41:38.2047386Z checking for fgrep... /bin/grep -F
2020-01-17T21:41:38.2047386Z checking for fgrep... /bin/grep -F
2020-01-17T21:41:38.2047467Z checking for ld used by gcc... /usr/bin/ld
2020-01-17T21:41:38.2047569Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-01-17T21:41:38.2047889Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2020-01-17T21:41:38.2048232Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2020-01-17T21:41:38.2048349Z checking the maximum length of command line arguments... 1572864
2020-01-17T21:41:38.2048456Z checking whether the shell understands some XSI constructs... yes
2020-01-17T21:41:38.2048564Z checking whether the shell understands "+="... yes
2020-01-17T21:41:38.2048960Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2020-01-17T21:41:38.2049382Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2020-01-17T21:41:38.2049708Z checking for /usr/bin/ld option to reload object files... -r
2020-01-17T21:41:38.2050184Z checking for x86_64-unknown-linux-gnu-objdump... no
2020-01-17T21:41:38.2050373Z checking how to recognize dependent libraries... pass_all
2020-01-17T21:41:38.2050699Z checking for x86_64-unknown-linux-gnu-dlltool... no
2020-01-17T21:41:38.2050781Z checking for dlltool... no
2020-01-17T21:41:38.2050880Z checking how to associate runtime and link libraries... printf %s\n
2020-01-17T21:41:38.2050880Z checking how to associate runtime and link libraries... printf %s\n
2020-01-17T21:41:38.2051291Z checking for x86_64-unknown-linux-gnu-ar... ar
2020-01-17T21:41:38.2051412Z checking for archiver @FILE support... @
2020-01-17T21:41:38.2051751Z checking for x86_64-unknown-linux-gnu-strip... strip
2020-01-17T21:41:38.2052081Z checking for x86_64-unknown-linux-gnu-ranlib... no
2020-01-17T21:41:38.2052163Z checking for ranlib... ranlib
2020-01-17T21:41:38.2052630Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2020-01-17T21:41:38.2052736Z checking for sysroot... no
2020-01-17T21:41:38.2053169Z checking for x86_64-unknown-linux-gnu-mt... no
2020-01-17T21:41:38.2053265Z checking for mt... no
2020-01-17T21:41:38.2053331Z checking if : is a manifest tool... no
2020-01-17T21:41:38.2053416Z checking for dlfcn.h... yes
2020-01-17T21:41:38.2053482Z checking for objdir... .libs
2020-01-17T21:41:38.2053783Z checking if gcc supports -fno-rtti -fno-exceptions... no
2020-01-17T21:41:38.2054068Z checking for gcc option to produce PIC... -fPIC -DPIC
2020-01-17T21:41:38.2054368Z checking if gcc PIC flag -fPIC -DPIC works... yes
2020-01-17T21:41:38.2054650Z checking if gcc static flag -static works... yes
2020-01-17T21:41:38.2054952Z checking if gcc supports -c -o file.o... yes
2020-01-17T21:41:38.2055248Z checking if gcc supports -c -o file.o... (cached) yes
2020-01-17T21:41:38.2055582Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2020-01-17T21:41:38.2055701Z checking dynamic linker characteristics... GNU/Linux ld.so
2020-01-17T21:41:38.2055798Z checking how to hardcode library paths into programs... immediate
2020-01-17T21:41:38.2055900Z checking whether stripping libraries is possible... yes
2020-01-17T21:41:38.2055977Z checking if libtool supports shared libraries... yes
2020-01-17T21:41:38.2056073Z checking whether to build shared libraries... no
2020-01-17T21:41:38.2056241Z checking for dirent.h that defines DIR... yes
2020-01-17T21:41:38.2056335Z checking for library containing opendir... none required
2020-01-17T21:41:38.2056420Z checking for ANSI C header files... (cached) yes
2020-01-17T21:41:38.2056510Z checking fcntl.h usability... yes
---
2020-01-17T21:41:38.2058032Z checking for sys/param.h... yes
2020-01-17T21:41:38.2058114Z checking sys/mount.h usability... yes
2020-01-17T21:41:38.2058284Z checking sys/mount.h presence... yes
2020-01-17T21:41:38.2058369Z checking for sys/mount.h... yes
2020-01-17T21:41:38.2058458Z checking for stdint types... stdint.h (shortcircuit)
2020-01-17T21:41:38.2058541Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2020-01-17T21:41:38.2100012Z checking for inline... inline
2020-01-17T21:41:38.2100107Z checking for flexible array members... yes
2020-01-17T21:41:38.2100356Z checking for pid_t... yes
2020-01-17T21:41:38.2100462Z checking for vprintf... yes
---
2020-01-17T21:41:38.2100873Z checking for getpagesize... yes
2020-01-17T21:41:38.2100943Z checking for working mmap... yes
2020-01-17T21:41:38.2101042Z checking for link... yes
2020-01-17T21:41:38.2101111Z checking for mkstemp... yes
2020-01-17T21:41:38.2101197Z checking for mkostemp... yes
2020-01-17T21:41:38.2101266Z checking for _mktemp_s... no
2020-01-17T21:41:38.2101420Z checking for getopt... yes
2020-01-17T21:41:38.2101505Z checking for getopt_long... yes
2020-01-17T21:41:38.2101575Z checking for getprogname... no
2020-01-17T21:41:38.2101660Z checking for getexecname... no
---
2020-01-17T21:41:38.2102600Z checking for fstatfs... yes
2020-01-17T21:41:38.2102686Z checking for lstat... yes
2020-01-17T21:41:38.2102762Z checking for posix_fadvise in fcntl.h... fcntl.h
2020-01-17T21:41:38.2102851Z checking for scandir... yes
2020-01-17T21:41:38.2102924Z checking for struct statvfs.f_basetype... no
2020-01-17T21:41:38.2103021Z checking for struct statvfs.f_fstypename... no
2020-01-17T21:41:38.2103253Z checking for struct statfs.f_flags... yes
2020-01-17T21:41:38.2103333Z checking for struct statfs.f_fstypename... no
2020-01-17T21:41:38.2103418Z checking for struct dirent.d_type... yes
2020-01-17T21:41:38.2103485Z checking for FREETYPE... yes
2020-01-17T21:41:38.2103565Z checking for FT_Get_Next_Char... yes
2020-01-17T21:41:38.2103632Z checking for FT_Get_BDF_Property... yes
2020-01-17T21:41:38.2103716Z checking for FT_Get_PS_Font_Info... yes
2020-01-17T21:41:38.2103783Z checking for FT_Has_PS_Glyph_Names... yes
2020-01-17T21:41:38.2103876Z checking for FT_Get_X11_Font_Format... yes
2020-01-17T21:41:38.2103942Z checking for FT_Select_Size... yes
2020-01-17T21:41:38.2104027Z checking for FT_Bitmap_Size.y_ppem... yes
2020-01-17T21:41:38.2104175Z checking expat.h usability... yes
2020-01-17T21:41:38.2104256Z checking expat.h presence... yes
2020-01-17T21:41:38.2104322Z checking for expat.h... yes
2020-01-17T21:41:38.2104322Z checking for expat.h... yes
2020-01-17T21:41:38.2104407Z checking for XML_SetDoctypeDeclHandler... yes
2020-01-17T21:41:38.2104485Z checking for Intel atomic primitives... true
2020-01-17T21:41:38.2104574Z checking for Solaris atomic operations... false
2020-01-17T21:41:38.2104962Z checking if compiler needs -Werror to reject unknown flags... no
2020-01-17T21:41:38.2105282Z checking for the pthreads library -lpthreads... no
2020-01-17T21:41:38.2105367Z checking whether pthreads work without any flags... no
2020-01-17T21:41:38.2105671Z checking whether pthreads work with -Kthread... no
2020-01-17T21:41:38.2106117Z checking whether pthreads work with -kthread... no
2020-01-17T21:41:38.2106403Z checking for the pthreads library -llthread... no
2020-01-17T21:41:38.2106836Z checking whether pthreads work with -pthread... yes
2020-01-17T21:41:38.2106927Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2020-01-17T21:41:38.2107034Z checking if more special flags are required for pthreads... no
2020-01-17T21:41:38.2107113Z checking for PTHREAD_PRIO_INHERIT... yes
2020-01-17T21:41:38.2107289Z checking for docbook2html... no
2020-01-17T21:41:38.2107511Z checking whether byte ordering is bigendian... no
2020-01-17T21:41:38.2107605Z checking size of void *... 8
2020-01-17T21:41:38.2107672Z checking alignment of double... 8
2020-01-17T21:41:38.2107769Z checking that generated files are newer than configure... done
2020-01-17T21:41:38.2107935Z config.status: creating Makefile
2020-01-17T21:41:38.2108035Z config.status: creating fontconfig/Makefile
2020-01-17T21:41:38.2108371Z config.status: creating fc-lang/Makefile
2020-01-17T21:41:38.2108371Z config.status: creating fc-lang/Makefile
2020-01-17T21:41:38.2108684Z config.status: creating fc-glyphname/Makefile
2020-01-17T21:41:38.2108973Z config.status: creating fc-case/Makefile
2020-01-17T21:41:38.2109074Z config.status: creating src/Makefile
2020-01-17T21:41:38.2109145Z config.status: creating conf.d/Makefile
2020-01-17T21:41:38.2109448Z config.status: creating fc-cache/Makefile
2020-01-17T21:41:38.2109727Z config.status: creating fc-cat/Makefile
2020-01-17T21:41:38.2110041Z config.status: creating fc-list/Makefile
2020-01-17T21:41:38.2110347Z config.status: creating fc-match/Makefile
2020-01-17T21:41:38.2110638Z config.status: creating fc-pattern/Makefile
2020-01-17T21:41:38.2110942Z config.status: creating fc-query/Makefile
2020-01-17T21:41:38.2111229Z config.status: creating fc-scan/Makefile
2020-01-17T21:41:38.2111536Z config.status: creating fc-validate/Makefile
2020-01-17T21:41:38.2111724Z config.status: creating doc/version.sgml
2020-01-17T21:41:38.2111797Z config.status: creating test/Makefile
2020-01-17T21:41:38.2111890Z config.status: creating fontconfig.spec
2020-01-17T21:41:38.2111962Z config.status: creating fontconfig.pc
2020-01-17T21:41:38.2111962Z config.status: creating fontconfig.pc
2020-01-17T21:41:38.2112265Z config.status: creating fontconfig-zip
2020-01-17T21:41:38.2112345Z config.status: creating config.h
2020-01-17T21:41:38.2112440Z config.status: executing depfiles commands
2020-01-17T21:41:38.2112532Z config.status: executing libtool commands
2020-01-17T21:41:38.2112622Z config.status: executing src/fcstdint.h commands
2020-01-17T21:41:38.2112728Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-17T21:41:38.2113385Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out && make -j2
2020-01-17T21:41:38.2113963Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out'
2020-01-17T21:41:38.2114803Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out'
2020-01-17T21:41:38.2114928Z Making all in fontconfig
2020-01-17T21:41:38.2115338Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fontconfig'
2020-01-17T21:41:38.2115699Z make[4]: Nothing to be done for 'all'.
2020-01-17T21:41:38.2115699Z make[4]: Nothing to be done for 'all'.
2020-01-17T21:41:38.2116146Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fontconfig'
2020-01-17T21:41:38.2116489Z Making all in fc-case
2020-01-17T21:41:38.2116911Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-case'
2020-01-17T21:41:38.2117047Z   GEN      fcalias.h
2020-01-17T21:41:38.2117138Z   GEN      fcaliastail.h
2020-01-17T21:41:38.2117541Z make  all-am
2020-01-17T21:41:38.2118316Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-case'
2020-01-17T21:41:38.2118606Z make[5]: Nothing to be done for 'all-am'.
2020-01-17T21:41:38.2118976Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-case'
2020-01-17T21:41:38.2119483Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-case'
2020-01-17T21:41:38.2120022Z Making all in fc-lang
2020-01-17T21:41:38.2120486Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-lang'
2020-01-17T21:41:38.2120598Z   GEN      fcalias.h
2020-01-17T21:41:38.2120685Z   GEN      fcaliastail.h
2020-01-17T21:41:38.2120943Z make  all-am
2020-01-17T21:41:38.2121352Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-lang'
2020-01-17T21:41:38.2121474Z   GEN      fclang.h
2020-01-17T21:41:38.2121891Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-lang'
2020-01-17T21:41:38.2122178Z   GEN      fc-lang
2020-01-17T21:41:38.2122594Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-lang'
2020-01-17T21:41:38.2123295Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-lang'
2020-01-17T21:41:38.2123681Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-lang'
2020-01-17T21:41:38.2123962Z Making all in fc-glyphname
2020-01-17T21:41:38.2124316Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-glyphname'
2020-01-17T21:41:38.2124431Z   GEN      fcalias.h
2020-01-17T21:41:38.2124502Z   GEN      fcaliastail.h
2020-01-17T21:41:38.2124744Z make  all-am
2020-01-17T21:41:38.2125111Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-glyphname'
2020-01-17T21:41:38.2125394Z make[5]: Nothing to be done for 'all-am'.
2020-01-17T21:41:38.2125771Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-glyphname'
2020-01-17T21:41:38.2126163Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/fc-glyphname'
2020-01-17T21:41:38.2126618Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/src'
2020-01-17T21:41:38.2126618Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/src'
2020-01-17T21:41:38.2126733Z   GEN      fcftalias.h
2020-01-17T21:41:38.2126812Z   GEN      fcalias.h
2020-01-17T21:41:38.2127042Z   GEN      stamp-fcstdint
2020-01-17T21:41:38.2127148Z config.status: executing src/fcstdint.h commands
2020-01-17T21:41:38.2127224Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-17T21:41:38.2127317Z config.status: src/fcstdint.h is unchanged
2020-01-17T21:41:38.2127381Z   GEN      fcobjshash.gperf
2020-01-17T21:41:38.2127461Z   GEN      fcobjshash.h
2020-01-17T21:41:38.2127729Z Makefile:882: recipe for target 'fcobjshash.h' failed
2020-01-17T21:41:38.2128105Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/src'
2020-01-17T21:41:38.2128417Z Makefile:562: recipe for target 'all-recursive' failed
2020-01-17T21:41:38.2128763Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out'
2020-01-17T21:41:38.2129061Z Makefile:445: recipe for target 'all' failed
2020-01-17T21:41:38.2129539Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out'
2020-01-17T21:41:38.2130281Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/libfontconfig.a' failed
2020-01-17T21:41:38.2130715Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-17T21:41:38.2131081Z --- stderr
2020-01-17T21:41:38.2131433Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-17T21:41:38.2131433Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-17T21:41:38.2131934Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2020-01-17T21:41:38.2132339Z WARNING: 'gperf' is missing on your system.
2020-01-17T21:41:38.2132460Z          You might have modified some files without having the proper
2020-01-17T21:41:38.2132808Z          tools for further handling them.  Check the 'README' file, it
2020-01-17T21:41:38.2132933Z          often tells you about the needed prerequisites for installing
2020-01-17T21:41:38.2133165Z          this package.  You may also peek at any GNU archive site, in
2020-01-17T21:41:38.2133500Z          case some other package contains this missing 'gperf' program.
2020-01-17T21:41:38.2133585Z make[4]: *** [fcobjshash.h] Error 1
2020-01-17T21:41:38.2133677Z make[4]: *** Waiting for unfinished jobs....
2020-01-17T21:41:38.2133957Z make[3]: *** [all-recursive] Error 1
2020-01-17T21:41:38.2134057Z make[2]: *** [all] Error 2
2020-01-17T21:41:38.2134479Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2a9512960d08e10/out/libfontconfig.a] Error 2
2020-01-17T21:41:38.2134848Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2020-01-17T21:41:38.2135299Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2020-01-17T21:41:38.2135657Z                                                                                  "-f",
2020-01-17T21:41:38.2136200Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2020-01-17T21:41:38.2136433Z 
2020-01-17T21:41:38.2136532Z warning: build failed, waiting for other jobs to finish...
2020-01-17T21:41:38.3914453Z error: build failed
2020-01-17T21:41:38.3940117Z thread 'main' panicked at 'tests failed for https://github.com/servo/webrender', src/tools/cargotest/main.rs:88:9
---
2020-01-17T21:41:38.3941235Z 
2020-01-17T21:41:38.3941270Z 
2020-01-17T21:41:38.3951736Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-01-17T21:41:38.3951935Z Build completed unsuccessfully in 1:52:09
2020-01-17T21:41:38.4014159Z make: *** [check-aux] Error 1
2020-01-17T21:41:38.4014509Z Makefile:50: recipe for target 'check-aux' failed
2020-01-17T21:41:38.4025082Z   local time: Fri Jan 17 21:41:38 UTC 2020
2020-01-17T21:41:38.5393705Z   network time: Fri, 17 Jan 2020 21:41:38 GMT
2020-01-17T21:41:38.5394023Z == end clock drift check ==
2020-01-17T21:41:42.3114596Z 
2020-01-17T21:41:42.3114596Z 
2020-01-17T21:41:42.3257435Z ##[error]Bash exited with code '2'.
2020-01-17T21:41:42.3350838Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-17T21:41:42.3353336Z ==============================================================================
2020-01-17T21:41:42.3353451Z Task         : Get sources
2020-01-17T21:41:42.3353538Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
