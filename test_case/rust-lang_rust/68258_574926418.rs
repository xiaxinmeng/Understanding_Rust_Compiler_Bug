plain
2020-01-16T00:41:20.0055951Z    Compiling freetype v0.4.0
2020-01-16T00:41:21.1558176Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2020-01-16T00:41:21.1558324Z 
2020-01-16T00:41:21.1558420Z Caused by:
2020-01-16T00:41:21.1558924Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-2419a686963e5b7d/build-script-build` (exit code: 101)
2020-01-16T00:41:21.1559474Z --- stdout
2020-01-16T00:41:21.1559930Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-16T00:41:21.1560517Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out && \
2020-01-16T00:41:21.1560640Z  CC="gcc" \
2020-01-16T00:41:21.1560710Z  AR="ar" \
2020-01-16T00:41:21.1560796Z  FREETYPE_CFLAGS="" \
2020-01-16T00:41:21.1560871Z  FREETYPE_LIBS="" \
2020-01-16T00:41:21.1561151Z  CFLAGS=""" -fPIC" \
2020-01-16T00:41:21.1561505Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2020-01-16T00:41:21.1561794Z  --disable-docs \
2020-01-16T00:41:21.1562047Z  --disable-shared \
2020-01-16T00:41:21.1562140Z   \
2020-01-16T00:41:21.1562841Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2020-01-16T00:41:21.1563329Z checking whether build environment is sane... yes
2020-01-16T00:41:21.1563661Z checking for x86_64-unknown-linux-gnu-strip... no
2020-01-16T00:41:21.1563778Z checking for strip... strip
2020-01-16T00:41:21.1564077Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-16T00:41:21.1564077Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-16T00:41:21.1564181Z checking for gawk... no
2020-01-16T00:41:21.1564257Z checking for mawk... mawk
2020-01-16T00:41:21.1564437Z checking whether make supports nested variables... yes
2020-01-16T00:41:21.1564549Z checking whether make supports nested variables... (cached) yes
2020-01-16T00:41:21.1564891Z checking whether to enable maintainer-specific portions of Makefiles... no
2020-01-16T00:41:21.1565222Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2020-01-16T00:41:21.1569654Z checking whether make sets $(MAKE)... (cached) yes
2020-01-16T00:41:21.1570036Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2020-01-16T00:41:21.1570458Z checking for pkg-config... /usr/bin/pkg-config
2020-01-16T00:41:21.1570787Z checking pkg-config is at least version 0.9.0... yes
2020-01-16T00:41:21.1571075Z checking for RM macro... rm -f
2020-01-16T00:41:21.1571704Z checking host system type... x86_64-unknown-linux-gnu
2020-01-16T00:41:21.1571792Z checking how to print strings... printf
2020-01-16T00:41:21.1571895Z checking for a sed that does not truncate output... /bin/sed
2020-01-16T00:41:21.1572175Z checking for fgrep... /bin/grep -F
2020-01-16T00:41:21.1572175Z checking for fgrep... /bin/grep -F
2020-01-16T00:41:21.1572551Z checking for ld used by gcc... /usr/bin/ld
2020-01-16T00:41:21.1572640Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-01-16T00:41:21.1573067Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2020-01-16T00:41:21.1573423Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2020-01-16T00:41:21.1573527Z checking the maximum length of command line arguments... 1572864
2020-01-16T00:41:21.1573651Z checking whether the shell understands some XSI constructs... yes
2020-01-16T00:41:21.1573746Z checking whether the shell understands "+="... yes
2020-01-16T00:41:21.1574178Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2020-01-16T00:41:21.1574583Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2020-01-16T00:41:21.1574936Z checking for /usr/bin/ld option to reload object files... -r
2020-01-16T00:41:21.1575244Z checking for x86_64-unknown-linux-gnu-objdump... no
2020-01-16T00:41:21.1575452Z checking how to recognize dependent libraries... pass_all
2020-01-16T00:41:21.1575762Z checking for x86_64-unknown-linux-gnu-dlltool... no
2020-01-16T00:41:21.1575877Z checking for dlltool... no
2020-01-16T00:41:21.1575966Z checking how to associate runtime and link libraries... printf %s\n
2020-01-16T00:41:21.1575966Z checking how to associate runtime and link libraries... printf %s\n
2020-01-16T00:41:21.1576300Z checking for x86_64-unknown-linux-gnu-ar... ar
2020-01-16T00:41:21.1576387Z checking for archiver @FILE support... @
2020-01-16T00:41:21.1576706Z checking for x86_64-unknown-linux-gnu-strip... strip
2020-01-16T00:41:21.1577010Z checking for x86_64-unknown-linux-gnu-ranlib... no
2020-01-16T00:41:21.1577113Z checking for ranlib... ranlib
2020-01-16T00:41:21.1577446Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2020-01-16T00:41:21.1577539Z checking for sysroot... no
2020-01-16T00:41:21.1577847Z checking for x86_64-unknown-linux-gnu-mt... no
2020-01-16T00:41:21.1577933Z checking for mt... no
2020-01-16T00:41:21.1578025Z checking if : is a manifest tool... no
2020-01-16T00:41:21.1578104Z checking for dlfcn.h... yes
2020-01-16T00:41:21.1578197Z checking for objdir... .libs
2020-01-16T00:41:21.1578502Z checking if gcc supports -fno-rtti -fno-exceptions... no
2020-01-16T00:41:21.1578840Z checking for gcc option to produce PIC... -fPIC -DPIC
2020-01-16T00:41:21.1579150Z checking if gcc PIC flag -fPIC -DPIC works... yes
2020-01-16T00:41:21.1579466Z checking if gcc static flag -static works... yes
2020-01-16T00:41:21.1579775Z checking if gcc supports -c -o file.o... yes
2020-01-16T00:41:21.1580081Z checking if gcc supports -c -o file.o... (cached) yes
2020-01-16T00:41:21.1580457Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2020-01-16T00:41:21.1580570Z checking dynamic linker characteristics... GNU/Linux ld.so
2020-01-16T00:41:21.1580682Z checking how to hardcode library paths into programs... immediate
2020-01-16T00:41:21.1580781Z checking whether stripping libraries is possible... yes
2020-01-16T00:41:21.1580887Z checking if libtool supports shared libraries... yes
2020-01-16T00:41:21.1580975Z checking whether to build shared libraries... no
2020-01-16T00:41:21.1581315Z checking for dirent.h that defines DIR... yes
2020-01-16T00:41:21.1581475Z checking for library containing opendir... none required
2020-01-16T00:41:21.1581583Z checking for ANSI C header files... (cached) yes
2020-01-16T00:41:21.1581665Z checking fcntl.h usability... yes
---
2020-01-16T00:41:21.1583678Z checking for sys/param.h... yes
2020-01-16T00:41:21.1583753Z checking sys/mount.h usability... yes
2020-01-16T00:41:21.1583847Z checking sys/mount.h presence... yes
2020-01-16T00:41:21.1583939Z checking for sys/mount.h... yes
2020-01-16T00:41:21.1584022Z checking for stdint types... stdint.h (shortcircuit)
2020-01-16T00:41:21.1584133Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2020-01-16T00:41:21.1584667Z checking for inline... inline
2020-01-16T00:41:21.1584745Z checking for flexible array members... yes
2020-01-16T00:41:21.1584848Z checking for pid_t... yes
2020-01-16T00:41:21.1584922Z checking for vprintf... yes
---
2020-01-16T00:41:21.1585359Z checking for getpagesize... yes
2020-01-16T00:41:21.1585454Z checking for working mmap... yes
2020-01-16T00:41:21.1585531Z checking for link... yes
2020-01-16T00:41:21.1585622Z checking for mkstemp... yes
2020-01-16T00:41:21.1585697Z checking for mkostemp... yes
2020-01-16T00:41:21.1585786Z checking for _mktemp_s... no
2020-01-16T00:41:21.1585952Z checking for getopt... yes
2020-01-16T00:41:21.1586027Z checking for getopt_long... yes
2020-01-16T00:41:21.1586127Z checking for getprogname... no
2020-01-16T00:41:21.1586204Z checking for getexecname... no
---
2020-01-16T00:41:21.1587476Z checking for fstatfs... yes
2020-01-16T00:41:21.1587549Z checking for lstat... yes
2020-01-16T00:41:21.1587644Z checking for posix_fadvise in fcntl.h... fcntl.h
2020-01-16T00:41:21.1587724Z checking for scandir... yes
2020-01-16T00:41:21.1587944Z checking for struct statvfs.f_basetype... no
2020-01-16T00:41:21.1588059Z checking for struct statvfs.f_fstypename... no
2020-01-16T00:41:21.1588229Z checking for struct statfs.f_flags... yes
2020-01-16T00:41:21.1588328Z checking for struct statfs.f_fstypename... no
2020-01-16T00:41:21.1588409Z checking for struct dirent.d_type... yes
2020-01-16T00:41:21.1588503Z checking for FREETYPE... yes
2020-01-16T00:41:21.1588579Z checking for FT_Get_Next_Char... yes
2020-01-16T00:41:21.1588673Z checking for FT_Get_BDF_Property... yes
2020-01-16T00:41:21.1588751Z checking for FT_Get_PS_Font_Info... yes
2020-01-16T00:41:21.1588851Z checking for FT_Has_PS_Glyph_Names... yes
2020-01-16T00:41:21.1588931Z checking for FT_Get_X11_Font_Format... yes
2020-01-16T00:41:21.1589026Z checking for FT_Select_Size... yes
2020-01-16T00:41:21.1589122Z checking for FT_Bitmap_Size.y_ppem... yes
2020-01-16T00:41:21.1589292Z checking expat.h usability... yes
2020-01-16T00:41:21.1589379Z checking expat.h presence... yes
2020-01-16T00:41:21.1589472Z checking for expat.h... yes
2020-01-16T00:41:21.1589472Z checking for expat.h... yes
2020-01-16T00:41:21.1589561Z checking for XML_SetDoctypeDeclHandler... yes
2020-01-16T00:41:21.1589661Z checking for Intel atomic primitives... true
2020-01-16T00:41:21.1589746Z checking for Solaris atomic operations... false
2020-01-16T00:41:21.1590198Z checking if compiler needs -Werror to reject unknown flags... no
2020-01-16T00:41:21.1590512Z checking for the pthreads library -lpthreads... no
2020-01-16T00:41:21.1590624Z checking whether pthreads work without any flags... no
2020-01-16T00:41:21.1590927Z checking whether pthreads work with -Kthread... no
2020-01-16T00:41:21.1591249Z checking whether pthreads work with -kthread... no
2020-01-16T00:41:21.1591569Z checking for the pthreads library -llthread... no
2020-01-16T00:41:21.1591870Z checking whether pthreads work with -pthread... yes
2020-01-16T00:41:21.1591985Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2020-01-16T00:41:21.1592094Z checking if more special flags are required for pthreads... no
2020-01-16T00:41:21.1592199Z checking for PTHREAD_PRIO_INHERIT... yes
2020-01-16T00:41:21.1592539Z checking for docbook2html... no
2020-01-16T00:41:21.1592641Z checking whether byte ordering is bigendian... no
2020-01-16T00:41:21.1592720Z checking size of void *... 8
2020-01-16T00:41:21.1592813Z checking alignment of double... 8
2020-01-16T00:41:21.1592916Z checking that generated files are newer than configure... done
2020-01-16T00:41:21.1593097Z config.status: creating Makefile
2020-01-16T00:41:21.1593177Z config.status: creating fontconfig/Makefile
2020-01-16T00:41:21.1593566Z config.status: creating fc-lang/Makefile
2020-01-16T00:41:21.1593566Z config.status: creating fc-lang/Makefile
2020-01-16T00:41:21.1593862Z config.status: creating fc-glyphname/Makefile
2020-01-16T00:41:21.1594163Z config.status: creating fc-case/Makefile
2020-01-16T00:41:21.1594247Z config.status: creating src/Makefile
2020-01-16T00:41:21.1594341Z config.status: creating conf.d/Makefile
2020-01-16T00:41:21.1594637Z config.status: creating fc-cache/Makefile
2020-01-16T00:41:21.1594949Z config.status: creating fc-cat/Makefile
2020-01-16T00:41:21.1595248Z config.status: creating fc-list/Makefile
2020-01-16T00:41:21.1595535Z config.status: creating fc-match/Makefile
2020-01-16T00:41:21.1595840Z config.status: creating fc-pattern/Makefile
2020-01-16T00:41:21.1632994Z config.status: creating fc-query/Makefile
2020-01-16T00:41:21.1633417Z config.status: creating fc-scan/Makefile
2020-01-16T00:41:21.1633717Z config.status: creating fc-validate/Makefile
2020-01-16T00:41:21.1633904Z config.status: creating doc/version.sgml
2020-01-16T00:41:21.1633999Z config.status: creating test/Makefile
2020-01-16T00:41:21.1634078Z config.status: creating fontconfig.spec
2020-01-16T00:41:21.1634174Z config.status: creating fontconfig.pc
2020-01-16T00:41:21.1634174Z config.status: creating fontconfig.pc
2020-01-16T00:41:21.1634458Z config.status: creating fontconfig-zip
2020-01-16T00:41:21.1634795Z config.status: creating config.h
2020-01-16T00:41:21.1634911Z config.status: executing depfiles commands
2020-01-16T00:41:21.1635086Z config.status: executing libtool commands
2020-01-16T00:41:21.1635188Z config.status: executing src/fcstdint.h commands
2020-01-16T00:41:21.1635282Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-16T00:41:21.1635761Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out && make -j2
2020-01-16T00:41:21.1636463Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out'
2020-01-16T00:41:21.1637253Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out'
2020-01-16T00:41:21.1637374Z Making all in fontconfig
2020-01-16T00:41:21.1637853Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fontconfig'
2020-01-16T00:41:21.1638203Z make[4]: Nothing to be done for 'all'.
2020-01-16T00:41:21.1638203Z make[4]: Nothing to be done for 'all'.
2020-01-16T00:41:21.1638678Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fontconfig'
2020-01-16T00:41:21.1638996Z Making all in fc-case
2020-01-16T00:41:21.1639445Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-case'
2020-01-16T00:41:21.1639581Z   GEN      fcalias.h
2020-01-16T00:41:21.1639663Z   GEN      fcaliastail.h
2020-01-16T00:41:21.1639944Z make  all-am
2020-01-16T00:41:21.1640371Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-case'
2020-01-16T00:41:21.1640727Z make[5]: Nothing to be done for 'all-am'.
2020-01-16T00:41:21.1641165Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-case'
2020-01-16T00:41:21.1641658Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-case'
2020-01-16T00:41:21.1641985Z Making all in fc-lang
2020-01-16T00:41:21.1642771Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-lang'
2020-01-16T00:41:21.1642922Z   GEN      fcalias.h
2020-01-16T00:41:21.1643000Z   GEN      fcaliastail.h
2020-01-16T00:41:21.1643284Z make  all-am
2020-01-16T00:41:21.1643691Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-lang'
2020-01-16T00:41:21.1643827Z   GEN      fclang.h
2020-01-16T00:41:21.1644254Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-lang'
2020-01-16T00:41:21.1644584Z   GEN      fc-lang
2020-01-16T00:41:21.1645045Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-lang'
2020-01-16T00:41:21.1645519Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-lang'
2020-01-16T00:41:21.1646015Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-lang'
2020-01-16T00:41:21.1646333Z Making all in fc-glyphname
2020-01-16T00:41:21.1646790Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-glyphname'
2020-01-16T00:41:21.1646912Z   GEN      fcaliastail.h
2020-01-16T00:41:21.1647009Z   GEN      fcalias.h
2020-01-16T00:41:21.1647293Z make  all-am
2020-01-16T00:41:21.1647732Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-glyphname'
2020-01-16T00:41:21.1648090Z make[5]: Nothing to be done for 'all-am'.
2020-01-16T00:41:21.1648681Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-glyphname'
2020-01-16T00:41:21.1649309Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/fc-glyphname'
2020-01-16T00:41:21.1649870Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/src'
2020-01-16T00:41:21.1649870Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/src'
2020-01-16T00:41:21.1650005Z   GEN      fcftalias.h
2020-01-16T00:41:21.1650084Z   GEN      fcalias.h
2020-01-16T00:41:21.1650384Z   GEN      stamp-fcstdint
2020-01-16T00:41:21.1650476Z config.status: executing src/fcstdint.h commands
2020-01-16T00:41:21.1650591Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-16T00:41:21.1650688Z config.status: src/fcstdint.h is unchanged
2020-01-16T00:41:21.1650789Z   GEN      fcobjshash.gperf
2020-01-16T00:41:21.1650868Z   GEN      fcobjshash.h
2020-01-16T00:41:21.1651224Z Makefile:882: recipe for target 'fcobjshash.h' failed
2020-01-16T00:41:21.1651660Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/src'
2020-01-16T00:41:21.1652059Z Makefile:562: recipe for target 'all-recursive' failed
2020-01-16T00:41:21.1652836Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out'
2020-01-16T00:41:21.1653192Z Makefile:445: recipe for target 'all' failed
2020-01-16T00:41:21.1653632Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out'
2020-01-16T00:41:21.1654139Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/libfontconfig.a' failed
2020-01-16T00:41:21.1654601Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-16T00:41:21.1654981Z --- stderr
2020-01-16T00:41:21.1655321Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-16T00:41:21.1655321Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-16T00:41:21.1655786Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2020-01-16T00:41:21.1656141Z WARNING: 'gperf' is missing on your system.
2020-01-16T00:41:21.1656248Z          You might have modified some files without having the proper
2020-01-16T00:41:21.1656622Z          tools for further handling them.  Check the 'README' file, it
2020-01-16T00:41:21.1656733Z          often tells you about the needed prerequisites for installing
2020-01-16T00:41:21.1656855Z          this package.  You may also peek at any GNU archive site, in
2020-01-16T00:41:21.1657203Z          case some other package contains this missing 'gperf' program.
2020-01-16T00:41:21.1657322Z make[4]: *** [fcobjshash.h] Error 1
2020-01-16T00:41:21.1657408Z make[4]: *** Waiting for unfinished jobs....
2020-01-16T00:41:21.1657743Z make[3]: *** [all-recursive] Error 1
2020-01-16T00:41:21.1657847Z make[2]: *** [all] Error 2
2020-01-16T00:41:21.1658296Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-eaeda8bed72c81f5/out/libfontconfig.a] Error 2
2020-01-16T00:41:21.1658714Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2020-01-16T00:41:21.1659101Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2020-01-16T00:41:21.1659505Z                                                                                  "-f",
2020-01-16T00:41:21.1660077Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2020-01-16T00:41:21.1660357Z 
2020-01-16T00:41:21.1660578Z warning: build failed, waiting for other jobs to finish...
2020-01-16T00:41:21.5074096Z error: build failed
2020-01-16T00:41:21.5093059Z thread 'main' panicked at 'tests failed for https://github.com/servo/webrender', src/tools/cargotest/main.rs:88:9
---
2020-01-16T00:41:21.5100714Z 
2020-01-16T00:41:21.5100859Z 
2020-01-16T00:41:21.5115156Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-01-16T00:41:21.5115352Z Build completed unsuccessfully in 1:57:16
2020-01-16T00:41:21.5169664Z make: *** [check-aux] Error 1
2020-01-16T00:41:21.5170007Z Makefile:50: recipe for target 'check-aux' failed
2020-01-16T00:41:21.5185551Z   local time: Thu Jan 16 00:41:21 UTC 2020
2020-01-16T00:41:21.7949699Z   network time: Thu, 16 Jan 2020 00:41:21 GMT
2020-01-16T00:41:21.7950807Z == end clock drift check ==
2020-01-16T00:41:24.0997085Z 
2020-01-16T00:41:24.0997085Z 
2020-01-16T00:41:24.1078990Z ##[error]Bash exited with code '2'.
2020-01-16T00:41:24.1117967Z ##[section]Starting: Checkout
2020-01-16T00:41:24.1120006Z ==============================================================================
2020-01-16T00:41:24.1120115Z Task         : Get sources
2020-01-16T00:41:24.1120197Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
