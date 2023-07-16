plain
2019-12-03T07:06:56.0457523Z    Compiling wayland-sys v0.21.13
2019-12-03T07:06:56.6930042Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2019-12-03T07:06:56.6944581Z 
2019-12-03T07:06:56.6944713Z Caused by:
2019-12-03T07:06:56.6945225Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-78c2a4c8d0afddf5/build-script-build` (exit code: 101)
2019-12-03T07:06:56.6945545Z --- stdout
2019-12-03T07:06:56.6946159Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2019-12-03T07:06:56.6947178Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out && \
2019-12-03T07:06:56.6947316Z  CC="gcc" \
2019-12-03T07:06:56.6947381Z  AR="ar" \
2019-12-03T07:06:56.6947476Z  FREETYPE_CFLAGS="" \
2019-12-03T07:06:56.6947564Z  FREETYPE_LIBS="" \
2019-12-03T07:06:56.6947823Z  CFLAGS=""" -fPIC" \
2019-12-03T07:06:56.6948156Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2019-12-03T07:06:56.6948394Z  --disable-docs \
2019-12-03T07:06:56.6948629Z  --disable-shared \
2019-12-03T07:06:56.6948698Z   \
2019-12-03T07:06:56.6948993Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2019-12-03T07:06:56.6949398Z checking whether build environment is sane... yes
2019-12-03T07:06:56.6949685Z checking for x86_64-unknown-linux-gnu-strip... no
2019-12-03T07:06:56.6949765Z checking for strip... strip
2019-12-03T07:06:56.6950207Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2019-12-03T07:06:56.6950207Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2019-12-03T07:06:56.6950441Z checking for gawk... no
2019-12-03T07:06:56.6950564Z checking for mawk... mawk
2019-12-03T07:06:56.6950704Z checking whether make supports nested variables... yes
2019-12-03T07:06:56.6950772Z checking whether make supports nested variables... (cached) yes
2019-12-03T07:06:56.6951235Z checking whether to enable maintainer-specific portions of Makefiles... no
2019-12-03T07:06:56.6951472Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2019-12-03T07:06:56.6955712Z checking whether make sets $(MAKE)... (cached) yes
2019-12-03T07:06:56.6955923Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2019-12-03T07:06:56.6956139Z checking for pkg-config... /usr/bin/pkg-config
2019-12-03T07:06:56.6956347Z checking pkg-config is at least version 0.9.0... yes
2019-12-03T07:06:56.6956548Z checking for RM macro... rm -f
2019-12-03T07:06:56.6956989Z checking host system type... x86_64-unknown-linux-gnu
2019-12-03T07:06:56.6957067Z checking how to print strings... printf
2019-12-03T07:06:56.6957131Z checking for a sed that does not truncate output... /bin/sed
2019-12-03T07:06:56.6957341Z checking for fgrep... /bin/grep -F
2019-12-03T07:06:56.6957341Z checking for fgrep... /bin/grep -F
2019-12-03T07:06:56.6957403Z checking for ld used by gcc... /usr/bin/ld
2019-12-03T07:06:56.6957481Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2019-12-03T07:06:56.6957706Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2019-12-03T07:06:56.6958125Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2019-12-03T07:06:56.6958209Z checking the maximum length of command line arguments... 1572864
2019-12-03T07:06:56.6958276Z checking whether the shell understands some XSI constructs... yes
2019-12-03T07:06:56.6958356Z checking whether the shell understands "+="... yes
2019-12-03T07:06:56.6958629Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2019-12-03T07:06:56.6958914Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2019-12-03T07:06:56.6959150Z checking for /usr/bin/ld option to reload object files... -r
2019-12-03T07:06:56.6959383Z checking for x86_64-unknown-linux-gnu-objdump... no
2019-12-03T07:06:56.6959522Z checking how to recognize dependent libraries... pass_all
2019-12-03T07:06:56.6959766Z checking for x86_64-unknown-linux-gnu-dlltool... no
2019-12-03T07:06:56.6959827Z checking for dlltool... no
2019-12-03T07:06:56.6959906Z checking how to associate runtime and link libraries... printf %s\n
2019-12-03T07:06:56.6959906Z checking how to associate runtime and link libraries... printf %s\n
2019-12-03T07:06:56.6963710Z checking for x86_64-unknown-linux-gnu-ar... ar
2019-12-03T07:06:56.6963803Z checking for archiver @FILE support... @
2019-12-03T07:06:56.6964025Z checking for x86_64-unknown-linux-gnu-strip... strip
2019-12-03T07:06:56.6964373Z checking for x86_64-unknown-linux-gnu-ranlib... no
2019-12-03T07:06:56.6964450Z checking for ranlib... ranlib
2019-12-03T07:06:56.6965049Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2019-12-03T07:06:56.6965114Z checking for sysroot... no
2019-12-03T07:06:56.6965317Z checking for x86_64-unknown-linux-gnu-mt... no
2019-12-03T07:06:56.6965390Z checking for mt... no
2019-12-03T07:06:56.6965441Z checking if : is a manifest tool... no
2019-12-03T07:06:56.6965511Z checking for dlfcn.h... yes
2019-12-03T07:06:56.6965634Z checking for objdir... .libs
2019-12-03T07:06:56.6965868Z checking if gcc supports -fno-rtti -fno-exceptions... no
2019-12-03T07:06:56.6966069Z checking for gcc option to produce PIC... -fPIC -DPIC
2019-12-03T07:06:56.6966279Z checking if gcc PIC flag -fPIC -DPIC works... yes
2019-12-03T07:06:56.6966471Z checking if gcc static flag -static works... yes
2019-12-03T07:06:56.6966676Z checking if gcc supports -c -o file.o... yes
2019-12-03T07:06:56.6966895Z checking if gcc supports -c -o file.o... (cached) yes
2019-12-03T07:06:56.6967133Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2019-12-03T07:06:56.6967225Z checking dynamic linker characteristics... GNU/Linux ld.so
2019-12-03T07:06:56.6967293Z checking how to hardcode library paths into programs... immediate
2019-12-03T07:06:56.6967375Z checking whether stripping libraries is possible... yes
2019-12-03T07:06:56.6967436Z checking if libtool supports shared libraries... yes
2019-12-03T07:06:56.6967513Z checking whether to build shared libraries... no
2019-12-03T07:06:56.6967653Z checking for dirent.h that defines DIR... yes
2019-12-03T07:06:56.6967733Z checking for library containing opendir... none required
2019-12-03T07:06:56.6967793Z checking for ANSI C header files... (cached) yes
2019-12-03T07:06:56.6967864Z checking fcntl.h usability... yes
---
2019-12-03T07:06:56.6968906Z checking for sys/statfs.h... yes
2019-12-03T07:06:56.6968979Z checking sys/param.h usability... yes
2019-12-03T07:06:56.6969032Z checking sys/param.h presence... yes
2019-12-03T07:06:56.6969100Z checking for sys/param.h... yes
2019-12-03T07:06:56.6969152Z checking sys/mount.h usability... yes
2019-12-03T07:06:56.6969220Z checking sys/mount.h presence... yes
2019-12-03T07:06:56.6969287Z checking for sys/mount.h... yes
2019-12-03T07:06:56.6969344Z checking for stdint types... stdint.h (shortcircuit)
2019-12-03T07:06:56.6969424Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2019-12-03T07:06:56.6969724Z checking for inline... inline
2019-12-03T07:06:56.6969777Z checking for flexible array members... yes
2019-12-03T07:06:56.6969845Z checking for pid_t... yes
2019-12-03T07:06:56.6969911Z checking for vprintf... yes
---
2019-12-03T07:06:56.6970276Z checking for getpagesize... yes
2019-12-03T07:06:56.6970342Z checking for working mmap... yes
2019-12-03T07:06:56.6970393Z checking for link... yes
2019-12-03T07:06:56.6970461Z checking for mkstemp... yes
2019-12-03T07:06:56.6970526Z checking for mkostemp... yes
2019-12-03T07:06:56.6970577Z checking for _mktemp_s... no
2019-12-03T07:06:56.6970748Z checking for getopt... yes
2019-12-03T07:06:56.6970812Z checking for getopt_long... yes
2019-12-03T07:06:56.6970864Z checking for getprogname... no
2019-12-03T07:06:56.6970930Z checking for getexecname... no
---
2019-12-03T07:06:56.6971938Z checking for regfree... yes
2019-12-03T07:06:56.6972007Z checking for fstatvfs... yes
2019-12-03T07:06:56.6972062Z checking for fstatfs... yes
2019-12-03T07:06:56.6972135Z checking for lstat... yes
2019-12-03T07:06:56.6972209Z checking for posix_fadvise in fcntl.h... fcntl.h
2019-12-03T07:06:56.6972275Z checking for scandir... yes
2019-12-03T07:06:56.6972348Z checking for struct statvfs.f_basetype... no
2019-12-03T07:06:56.6972411Z checking for struct statvfs.f_fstypename... no
2019-12-03T07:06:56.6972489Z checking for struct statfs.f_flags... yes
2019-12-03T07:06:56.6972551Z checking for struct statfs.f_fstypename... no
2019-12-03T07:06:56.6972627Z checking for struct dirent.d_type... yes
2019-12-03T07:06:56.6972684Z checking for FREETYPE... yes
2019-12-03T07:06:56.6972761Z checking for FT_Get_Next_Char... yes
2019-12-03T07:06:56.6972833Z checking for FT_Get_BDF_Property... yes
2019-12-03T07:06:56.6972890Z checking for FT_Get_PS_Font_Info... yes
2019-12-03T07:06:56.6972967Z checking for FT_Has_PS_Glyph_Names... yes
2019-12-03T07:06:56.6973026Z checking for FT_Get_X11_Font_Format... yes
2019-12-03T07:06:56.6973099Z checking for FT_Select_Size... yes
2019-12-03T07:06:56.6973157Z checking for FT_Bitmap_Size.y_ppem... yes
2019-12-03T07:06:56.6973290Z checking expat.h usability... yes
2019-12-03T07:06:56.6973362Z checking expat.h presence... yes
2019-12-03T07:06:56.6973434Z checking for expat.h... yes
2019-12-03T07:06:56.6973434Z checking for expat.h... yes
2019-12-03T07:06:56.6973492Z checking for XML_SetDoctypeDeclHandler... yes
2019-12-03T07:06:56.6973569Z checking for Intel atomic primitives... true
2019-12-03T07:06:56.6973633Z checking for Solaris atomic operations... false
2019-12-03T07:06:56.6973915Z checking if compiler needs -Werror to reject unknown flags... no
2019-12-03T07:06:56.6974134Z checking for the pthreads library -lpthreads... no
2019-12-03T07:06:56.6974219Z checking whether pthreads work without any flags... no
2019-12-03T07:06:56.6974429Z checking whether pthreads work with -Kthread... no
2019-12-03T07:06:56.6974817Z checking whether pthreads work with -kthread... no
2019-12-03T07:06:56.6975200Z checking for the pthreads library -llthread... no
2019-12-03T07:06:56.6975396Z checking whether pthreads work with -pthread... yes
2019-12-03T07:06:56.6975485Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2019-12-03T07:06:56.6975552Z checking if more special flags are required for pthreads... no
2019-12-03T07:06:56.6975629Z checking for PTHREAD_PRIO_INHERIT... yes
2019-12-03T07:06:56.6975683Z checking for docbook2html... no
2019-12-03T07:06:56.6975754Z checking whether byte ordering is bigendian... no
2019-12-03T07:06:56.6975824Z checking size of void *... 8
2019-12-03T07:06:56.6975874Z checking alignment of double... 8
2019-12-03T07:06:56.6976027Z checking that generated files are newer than configure... done
2019-12-03T07:06:56.6976161Z config.status: creating Makefile
2019-12-03T07:06:56.6976215Z config.status: creating fontconfig/Makefile
2019-12-03T07:06:56.6976438Z config.status: creating fc-lang/Makefile
2019-12-03T07:06:56.6976438Z config.status: creating fc-lang/Makefile
2019-12-03T07:06:56.6976626Z config.status: creating fc-glyphname/Makefile
2019-12-03T07:06:56.6976823Z config.status: creating fc-case/Makefile
2019-12-03T07:06:56.6976959Z config.status: creating src/Makefile
2019-12-03T07:06:56.6977013Z config.status: creating conf.d/Makefile
2019-12-03T07:06:56.6977228Z config.status: creating fc-cache/Makefile
2019-12-03T07:06:56.6977409Z config.status: creating fc-cat/Makefile
2019-12-03T07:06:56.6977605Z config.status: creating fc-list/Makefile
2019-12-03T07:06:56.6977787Z config.status: creating fc-match/Makefile
2019-12-03T07:06:56.6977987Z config.status: creating fc-pattern/Makefile
2019-12-03T07:06:56.6978190Z config.status: creating fc-query/Makefile
2019-12-03T07:06:56.6978376Z config.status: creating fc-scan/Makefile
2019-12-03T07:06:56.6978578Z config.status: creating fc-validate/Makefile
2019-12-03T07:06:56.6978706Z config.status: creating doc/version.sgml
2019-12-03T07:06:56.6978759Z config.status: creating test/Makefile
2019-12-03T07:06:56.6978826Z config.status: creating fontconfig.spec
2019-12-03T07:06:56.6978879Z config.status: creating fontconfig.pc
2019-12-03T07:06:56.6978879Z config.status: creating fontconfig.pc
2019-12-03T07:06:56.6979085Z config.status: creating fontconfig-zip
2019-12-03T07:06:56.6979156Z config.status: creating config.h
2019-12-03T07:06:56.6979210Z config.status: executing depfiles commands
2019-12-03T07:06:56.6979283Z config.status: executing libtool commands
2019-12-03T07:06:56.6979341Z config.status: executing src/fcstdint.h commands
2019-12-03T07:06:56.6979420Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2019-12-03T07:06:56.6979685Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out && make -j2
2019-12-03T07:06:56.6979982Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out'
2019-12-03T07:06:56.6980436Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out'
2019-12-03T07:06:56.6980527Z Making all in fontconfig
2019-12-03T07:06:56.6980798Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fontconfig'
2019-12-03T07:06:56.6981205Z make[4]: Nothing to be done for 'all'.
2019-12-03T07:06:56.6981205Z make[4]: Nothing to be done for 'all'.
2019-12-03T07:06:56.6981751Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fontconfig'
2019-12-03T07:06:56.6981958Z Making all in fc-case
2019-12-03T07:06:56.6982264Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-case'
2019-12-03T07:06:56.6982349Z   GEN      fcalias.h
2019-12-03T07:06:56.6982419Z   GEN      fcaliastail.h
2019-12-03T07:06:56.6982593Z make  all-am
2019-12-03T07:06:56.6982892Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-case'
2019-12-03T07:06:56.6983127Z make[5]: Nothing to be done for 'all-am'.
2019-12-03T07:06:56.6983417Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-case'
2019-12-03T07:06:56.6983775Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-case'
2019-12-03T07:06:56.6983997Z Making all in fc-lang
2019-12-03T07:06:56.6984494Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-lang'
2019-12-03T07:06:56.6984987Z   GEN      fcalias.h
2019-12-03T07:06:56.6985050Z   GEN      fcaliastail.h
2019-12-03T07:06:56.6985475Z make  all-am
2019-12-03T07:06:56.6985964Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-lang'
2019-12-03T07:06:56.6986067Z   GEN      fclang.h
2019-12-03T07:06:56.6986547Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-lang'
2019-12-03T07:06:56.6986777Z   GEN      fc-lang
2019-12-03T07:06:56.6987365Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-lang'
2019-12-03T07:06:56.6987730Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-lang'
2019-12-03T07:06:56.6988098Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-lang'
2019-12-03T07:06:56.6988353Z Making all in fc-glyphname
2019-12-03T07:06:56.6988705Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-glyphname'
2019-12-03T07:06:56.6988820Z   GEN      fcaliastail.h
2019-12-03T07:06:56.6988884Z   GEN      fcalias.h
2019-12-03T07:06:56.6989265Z make  all-am
2019-12-03T07:06:56.6989576Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-glyphname'
2019-12-03T07:06:56.6989689Z   GEN      fcglyphname.h
2019-12-03T07:06:56.6990001Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-glyphname'
2019-12-03T07:06:56.6990233Z   GEN      fc-glyphname
2019-12-03T07:06:56.6990556Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-glyphname'
2019-12-03T07:06:56.6991098Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-glyphname'
2019-12-03T07:06:56.6991973Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/fc-glyphname'
2019-12-03T07:06:56.6992364Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/src'
2019-12-03T07:06:56.6992466Z   GEN      fcalias.h
2019-12-03T07:06:56.6992466Z   GEN      fcalias.h
2019-12-03T07:06:56.6992531Z   GEN      fcftalias.h
2019-12-03T07:06:56.6992736Z   GEN      stamp-fcstdint
2019-12-03T07:06:56.6992804Z config.status: executing src/fcstdint.h commands
2019-12-03T07:06:56.6992892Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2019-12-03T07:06:56.6992961Z config.status: src/fcstdint.h is unchanged
2019-12-03T07:06:56.6993036Z   GEN      fcobjshash.gperf
2019-12-03T07:06:56.6993094Z   GEN      fcobjshash.h
2019-12-03T07:06:56.6993338Z Makefile:882: recipe for target 'fcobjshash.h' failed
2019-12-03T07:06:56.6993815Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/src'
2019-12-03T07:06:56.6994055Z Makefile:562: recipe for target 'all-recursive' failed
2019-12-03T07:06:56.6994515Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out'
2019-12-03T07:06:56.6995070Z Makefile:445: recipe for target 'all' failed
2019-12-03T07:06:56.6995368Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out'
2019-12-03T07:06:56.6995710Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/libfontconfig.a' failed
2019-12-03T07:06:56.6996171Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2019-12-03T07:06:56.6996477Z --- stderr
2019-12-03T07:06:56.6996951Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2019-12-03T07:06:56.6996951Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2019-12-03T07:06:56.6997254Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2019-12-03T07:06:56.6997496Z WARNING: 'gperf' is missing on your system.
2019-12-03T07:06:56.6997572Z          You might have modified some files without having the proper
2019-12-03T07:06:56.6997844Z          tools for further handling them.  Check the 'README' file, it
2019-12-03T07:06:56.6998182Z          often tells you about the needed prerequisites for installing
2019-12-03T07:06:56.6998260Z          this package.  You may also peek at any GNU archive site, in
2019-12-03T07:06:56.6998530Z          case some other package contains this missing 'gperf' program.
2019-12-03T07:06:56.6998600Z make[4]: *** [fcobjshash.h] Error 1
2019-12-03T07:06:56.6998680Z make[4]: *** Waiting for unfinished jobs....
2019-12-03T07:06:56.6998885Z make[3]: *** [all-recursive] Error 1
2019-12-03T07:06:56.6998961Z make[2]: *** [all] Error 2
2019-12-03T07:06:56.6999266Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-4be2d8fbe70edaeb/out/libfontconfig.a] Error 2
2019-12-03T07:06:56.6999533Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2019-12-03T07:06:56.6999805Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2019-12-03T07:06:56.7000059Z                                                                                  "-f",
2019-12-03T07:06:56.7000484Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2019-12-03T07:06:56.7000847Z 
2019-12-03T07:06:56.7000937Z warning: build failed, waiting for other jobs to finish...
2019-12-03T07:06:57.3979977Z error: build failed
2019-12-03T07:06:57.3997388Z thread 'main' panicked at 'tests failed for https://github.com/servo/webrender', src/tools/cargotest/main.rs:88:9
---
2019-12-03T07:06:57.4004180Z 
2019-12-03T07:06:57.4004290Z 
2019-12-03T07:06:57.4015705Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-12-03T07:06:57.4015819Z Build completed unsuccessfully in 1:41:48
2019-12-03T07:06:57.4086922Z Makefile:50: recipe for target 'check-aux' failed
2019-12-03T07:06:57.4087364Z == clock drift check ==
2019-12-03T07:06:57.4087832Z   local time: make: *** [check-aux] Error 1
2019-12-03T07:06:58.5309182Z   network time: Tue, 03 Dec 2019 07:06:57 GMT
2019-12-03T07:06:58.5309806Z == end clock drift check ==
2019-12-03T07:07:05.3591007Z 
2019-12-03T07:07:05.3591007Z 
2019-12-03T07:07:05.3780186Z ##[error]Bash exited with code '2'.
2019-12-03T07:07:05.4008053Z ##[section]Starting: Checkout
2019-12-03T07:07:05.4010374Z ==============================================================================
2019-12-03T07:07:05.4010636Z Task         : Get sources
2019-12-03T07:07:05.4010700Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
