plain
2020-01-22T09:44:29.7221132Z    Compiling wayland-sys v0.21.13
2020-01-22T09:44:31.6052144Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2020-01-22T09:44:31.6078604Z 
2020-01-22T09:44:31.6081179Z Caused by:
2020-01-22T09:44:31.6081919Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a2cf3df689aa121b/build-script-build` (exit code: 101)
2020-01-22T09:44:31.6082741Z --- stdout
2020-01-22T09:44:31.6083329Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-22T09:44:31.6083969Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out && \
2020-01-22T09:44:31.6084107Z  CC="gcc" \
2020-01-22T09:44:31.6084172Z  AR="ar" \
2020-01-22T09:44:31.6084254Z  FREETYPE_CFLAGS="" \
2020-01-22T09:44:31.6084327Z  FREETYPE_LIBS="" \
2020-01-22T09:44:31.6084869Z  CFLAGS=""" -fPIC" \
2020-01-22T09:44:31.6085194Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2020-01-22T09:44:31.6085461Z  --disable-docs \
2020-01-22T09:44:31.6085685Z  --disable-shared \
2020-01-22T09:44:31.6085775Z   \
2020-01-22T09:44:31.6086059Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2020-01-22T09:44:31.6087148Z checking whether build environment is sane... yes
2020-01-22T09:44:31.6087546Z checking for x86_64-unknown-linux-gnu-strip... no
2020-01-22T09:44:31.6087650Z checking for strip... strip
2020-01-22T09:44:31.6087918Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-22T09:44:31.6087918Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-22T09:44:31.6088018Z checking for gawk... no
2020-01-22T09:44:31.6088199Z checking for mawk... mawk
2020-01-22T09:44:31.6088520Z checking whether make supports nested variables... yes
2020-01-22T09:44:31.6088822Z checking whether make supports nested variables... (cached) yes
2020-01-22T09:44:31.6089157Z checking whether to enable maintainer-specific portions of Makefiles... no
2020-01-22T09:44:31.6115081Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2020-01-22T09:44:31.6119430Z checking whether make sets $(MAKE)... (cached) yes
2020-01-22T09:44:31.6119689Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2020-01-22T09:44:31.6119927Z checking for pkg-config... /usr/bin/pkg-config
2020-01-22T09:44:31.6120177Z checking pkg-config is at least version 0.9.0... yes
2020-01-22T09:44:31.6120403Z checking for RM macro... rm -f
2020-01-22T09:44:31.6121378Z checking host system type... x86_64-unknown-linux-gnu
2020-01-22T09:44:31.6121444Z checking how to print strings... printf
2020-01-22T09:44:31.6121516Z checking for a sed that does not truncate output... /bin/sed
2020-01-22T09:44:31.6121717Z checking for fgrep... /bin/grep -F
2020-01-22T09:44:31.6121717Z checking for fgrep... /bin/grep -F
2020-01-22T09:44:31.6121788Z checking for ld used by gcc... /usr/bin/ld
2020-01-22T09:44:31.6121852Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-01-22T09:44:31.6122090Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2020-01-22T09:44:31.6122457Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2020-01-22T09:44:31.6122552Z checking the maximum length of command line arguments... 1572864
2020-01-22T09:44:31.6122623Z checking whether the shell understands some XSI constructs... yes
2020-01-22T09:44:31.6122697Z checking whether the shell understands "+="... yes
2020-01-22T09:44:31.6123098Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2020-01-22T09:44:31.6123391Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2020-01-22T09:44:31.6123639Z checking for /usr/bin/ld option to reload object files... -r
2020-01-22T09:44:31.6123859Z checking for x86_64-unknown-linux-gnu-objdump... no
2020-01-22T09:44:31.6123996Z checking how to recognize dependent libraries... pass_all
2020-01-22T09:44:31.6125285Z checking for x86_64-unknown-linux-gnu-dlltool... no
2020-01-22T09:44:31.6125383Z checking for dlltool... no
2020-01-22T09:44:31.6125451Z checking how to associate runtime and link libraries... printf %s\n
2020-01-22T09:44:31.6125451Z checking how to associate runtime and link libraries... printf %s\n
2020-01-22T09:44:31.6125695Z checking for x86_64-unknown-linux-gnu-ar... ar
2020-01-22T09:44:31.6125764Z checking for archiver @FILE support... @
2020-01-22T09:44:31.6126013Z checking for x86_64-unknown-linux-gnu-strip... strip
2020-01-22T09:44:31.6126242Z checking for x86_64-unknown-linux-gnu-ranlib... no
2020-01-22T09:44:31.6126382Z checking for ranlib... ranlib
2020-01-22T09:44:31.6126624Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2020-01-22T09:44:31.6126705Z checking for sysroot... no
2020-01-22T09:44:31.6127099Z checking for x86_64-unknown-linux-gnu-mt... no
2020-01-22T09:44:31.6127177Z checking for mt... no
2020-01-22T09:44:31.6127242Z checking if : is a manifest tool... no
2020-01-22T09:44:31.6127305Z checking for dlfcn.h... yes
2020-01-22T09:44:31.6127371Z checking for objdir... .libs
2020-01-22T09:44:31.6127618Z checking if gcc supports -fno-rtti -fno-exceptions... no
2020-01-22T09:44:31.6127872Z checking for gcc option to produce PIC... -fPIC -DPIC
2020-01-22T09:44:31.6128270Z checking if gcc PIC flag -fPIC -DPIC works... yes
2020-01-22T09:44:31.6128506Z checking if gcc static flag -static works... yes
2020-01-22T09:44:31.6128743Z checking if gcc supports -c -o file.o... yes
2020-01-22T09:44:31.6129138Z checking if gcc supports -c -o file.o... (cached) yes
2020-01-22T09:44:31.6129578Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2020-01-22T09:44:31.6129672Z checking dynamic linker characteristics... GNU/Linux ld.so
2020-01-22T09:44:31.6129754Z checking how to hardcode library paths into programs... immediate
2020-01-22T09:44:31.6129830Z checking whether stripping libraries is possible... yes
2020-01-22T09:44:31.6129912Z checking if libtool supports shared libraries... yes
2020-01-22T09:44:31.6130140Z checking whether to build shared libraries... no
2020-01-22T09:44:31.6130530Z checking for dirent.h that defines DIR... yes
2020-01-22T09:44:31.6130604Z checking for library containing opendir... none required
2020-01-22T09:44:31.6130673Z checking for ANSI C header files... (cached) yes
2020-01-22T09:44:31.6130756Z checking fcntl.h usability... yes
---
2020-01-22T09:44:31.6132334Z checking for sys/param.h... yes
2020-01-22T09:44:31.6132393Z checking sys/mount.h usability... yes
2020-01-22T09:44:31.6132467Z checking sys/mount.h presence... yes
2020-01-22T09:44:31.6132527Z checking for sys/mount.h... yes
2020-01-22T09:44:31.6132601Z checking for stdint types... stdint.h (shortcircuit)
2020-01-22T09:44:31.6132685Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2020-01-22T09:44:31.6133050Z checking for inline... inline
2020-01-22T09:44:31.6133264Z checking for flexible array members... yes
2020-01-22T09:44:31.6133328Z checking for pid_t... yes
2020-01-22T09:44:31.6133385Z checking for vprintf... yes
---
2020-01-22T09:44:31.6133699Z checking for getpagesize... yes
2020-01-22T09:44:31.6133756Z checking for working mmap... yes
2020-01-22T09:44:31.6133819Z checking for link... yes
2020-01-22T09:44:31.6133886Z checking for mkstemp... yes
2020-01-22T09:44:31.6133944Z checking for mkostemp... yes
2020-01-22T09:44:31.6134010Z checking for _mktemp_s... no
2020-01-22T09:44:31.6134129Z checking for getopt... yes
2020-01-22T09:44:31.6134185Z checking for getopt_long... yes
2020-01-22T09:44:31.6134260Z checking for getprogname... no
2020-01-22T09:44:31.6134317Z checking for getexecname... no
---
2020-01-22T09:44:31.6135197Z checking for fstatfs... yes
2020-01-22T09:44:31.6135252Z checking for lstat... yes
2020-01-22T09:44:31.6135493Z checking for posix_fadvise in fcntl.h... fcntl.h
2020-01-22T09:44:31.6135556Z checking for scandir... yes
2020-01-22T09:44:31.6135627Z checking for struct statvfs.f_basetype... no
2020-01-22T09:44:31.6135693Z checking for struct statvfs.f_fstypename... no
2020-01-22T09:44:31.6135766Z checking for struct statfs.f_flags... yes
2020-01-22T09:44:31.6135829Z checking for struct statfs.f_fstypename... no
2020-01-22T09:44:31.6135910Z checking for struct dirent.d_type... yes
2020-01-22T09:44:31.6136138Z checking for FREETYPE... yes
2020-01-22T09:44:31.6136210Z checking for FT_Get_Next_Char... yes
2020-01-22T09:44:31.6136271Z checking for FT_Get_BDF_Property... yes
2020-01-22T09:44:31.6136338Z checking for FT_Get_PS_Font_Info... yes
2020-01-22T09:44:31.6136410Z checking for FT_Has_PS_Glyph_Names... yes
2020-01-22T09:44:31.6136473Z checking for FT_Get_X11_Font_Format... yes
2020-01-22T09:44:31.6136545Z checking for FT_Select_Size... yes
2020-01-22T09:44:31.6136606Z checking for FT_Bitmap_Size.y_ppem... yes
2020-01-22T09:44:31.6136832Z checking expat.h usability... yes
2020-01-22T09:44:31.6136976Z checking expat.h presence... yes
2020-01-22T09:44:31.6137222Z checking for expat.h... yes
2020-01-22T09:44:31.6137222Z checking for expat.h... yes
2020-01-22T09:44:31.6137523Z checking for XML_SetDoctypeDeclHandler... yes
2020-01-22T09:44:31.6137593Z checking for Intel atomic primitives... true
2020-01-22T09:44:31.6137670Z checking for Solaris atomic operations... false
2020-01-22T09:44:31.6138042Z checking if compiler needs -Werror to reject unknown flags... no
2020-01-22T09:44:31.6138296Z checking for the pthreads library -lpthreads... no
2020-01-22T09:44:31.6138381Z checking whether pthreads work without any flags... no
2020-01-22T09:44:31.6138623Z checking whether pthreads work with -Kthread... no
2020-01-22T09:44:31.6139041Z checking whether pthreads work with -kthread... no
2020-01-22T09:44:31.6139487Z checking for the pthreads library -llthread... no
2020-01-22T09:44:31.6139801Z checking whether pthreads work with -pthread... yes
2020-01-22T09:44:31.6139882Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2020-01-22T09:44:31.6139983Z checking if more special flags are required for pthreads... no
2020-01-22T09:44:31.6140063Z checking for PTHREAD_PRIO_INHERIT... yes
2020-01-22T09:44:31.6140129Z checking for docbook2html... no
2020-01-22T09:44:31.6140207Z checking whether byte ordering is bigendian... no
2020-01-22T09:44:31.6140281Z checking size of void *... 8
2020-01-22T09:44:31.6140527Z checking alignment of double... 8
2020-01-22T09:44:31.6140602Z checking that generated files are newer than configure... done
2020-01-22T09:44:31.6140905Z config.status: creating Makefile
2020-01-22T09:44:31.6140979Z config.status: creating fontconfig/Makefile
2020-01-22T09:44:31.6141400Z config.status: creating fc-lang/Makefile
2020-01-22T09:44:31.6141400Z config.status: creating fc-lang/Makefile
2020-01-22T09:44:31.6141813Z config.status: creating fc-glyphname/Makefile
2020-01-22T09:44:31.6142217Z config.status: creating fc-case/Makefile
2020-01-22T09:44:31.6142751Z config.status: creating src/Makefile
2020-01-22T09:44:31.6142883Z config.status: creating conf.d/Makefile
2020-01-22T09:44:31.6143349Z config.status: creating fc-cache/Makefile
2020-01-22T09:44:31.6143592Z config.status: creating fc-cat/Makefile
2020-01-22T09:44:31.6143826Z config.status: creating fc-list/Makefile
2020-01-22T09:44:31.6144247Z config.status: creating fc-match/Makefile
2020-01-22T09:44:31.6144730Z config.status: creating fc-pattern/Makefile
2020-01-22T09:44:31.6145173Z config.status: creating fc-query/Makefile
2020-01-22T09:44:31.6145413Z config.status: creating fc-scan/Makefile
2020-01-22T09:44:31.6145673Z config.status: creating fc-validate/Makefile
2020-01-22T09:44:31.6146000Z config.status: creating doc/version.sgml
2020-01-22T09:44:31.6146235Z config.status: creating test/Makefile
2020-01-22T09:44:31.6146305Z config.status: creating fontconfig.spec
2020-01-22T09:44:31.6146388Z config.status: creating fontconfig.pc
2020-01-22T09:44:31.6146388Z config.status: creating fontconfig.pc
2020-01-22T09:44:31.6146784Z config.status: creating fontconfig-zip
2020-01-22T09:44:31.6147049Z config.status: creating config.h
2020-01-22T09:44:31.6147293Z config.status: executing depfiles commands
2020-01-22T09:44:31.6147377Z config.status: executing libtool commands
2020-01-22T09:44:31.6147455Z config.status: executing src/fcstdint.h commands
2020-01-22T09:44:31.6147547Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-22T09:44:31.6148314Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out && make -j2
2020-01-22T09:44:31.6148830Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out'
2020-01-22T09:44:31.6149749Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out'
2020-01-22T09:44:31.6149848Z Making all in fontconfig
2020-01-22T09:44:31.6150465Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fontconfig'
2020-01-22T09:44:31.6150785Z make[4]: Nothing to be done for 'all'.
2020-01-22T09:44:31.6150785Z make[4]: Nothing to be done for 'all'.
2020-01-22T09:44:31.6151339Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fontconfig'
2020-01-22T09:44:31.6151686Z Making all in fc-case
2020-01-22T09:44:31.6152186Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-case'
2020-01-22T09:44:31.6152280Z   GEN      fcalias.h
2020-01-22T09:44:31.6152348Z   GEN      fcaliastail.h
2020-01-22T09:44:31.6153228Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-case'
2020-01-22T09:44:31.6153647Z make[5]: Nothing to be done for 'all-am'.
2020-01-22T09:44:31.6154191Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-case'
2020-01-22T09:44:31.6154722Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-case'
2020-01-22T09:44:31.6154722Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-case'
2020-01-22T09:44:31.6154961Z Making all in fc-lang
2020-01-22T09:44:31.6155450Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-lang'
2020-01-22T09:44:31.6155548Z   GEN      fcalias.h
2020-01-22T09:44:31.6155612Z   GEN      fcaliastail.h
2020-01-22T09:44:31.6155964Z make  all-am
2020-01-22T09:44:31.6156271Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-lang'
2020-01-22T09:44:31.6156364Z   GEN      fclang.h
2020-01-22T09:44:31.6156665Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-lang'
2020-01-22T09:44:31.6156892Z   GEN      fc-lang
2020-01-22T09:44:31.6157377Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-lang'
2020-01-22T09:44:31.6157736Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-lang'
2020-01-22T09:44:31.6158077Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-lang'
2020-01-22T09:44:31.6158331Z Making all in fc-glyphname
2020-01-22T09:44:31.6158838Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-glyphname'
2020-01-22T09:44:31.6158934Z   GEN      fcalias.h
2020-01-22T09:44:31.6159001Z   GEN      fcaliastail.h
2020-01-22T09:44:31.6159195Z make  all-am
2020-01-22T09:44:31.6159529Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-glyphname'
2020-01-22T09:44:31.6159783Z make[5]: Nothing to be done for 'all-am'.
2020-01-22T09:44:31.6160292Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-glyphname'
2020-01-22T09:44:31.6161744Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/fc-glyphname'
2020-01-22T09:44:31.6162229Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/src'
2020-01-22T09:44:31.6162229Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/src'
2020-01-22T09:44:31.6162315Z   GEN      fcftalias.h
2020-01-22T09:44:31.6162378Z   GEN      fcalias.h
2020-01-22T09:44:31.6162571Z   GEN      stamp-fcstdint
2020-01-22T09:44:31.6162645Z config.status: executing src/fcstdint.h commands
2020-01-22T09:44:31.6162716Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-22T09:44:31.6162790Z config.status: src/fcstdint.h is unchanged
2020-01-22T09:44:31.6162856Z   GEN      fcobjshash.gperf
2020-01-22T09:44:31.6162913Z   GEN      fcobjshash.h
2020-01-22T09:44:31.6163258Z Makefile:882: recipe for target 'fcobjshash.h' failed
2020-01-22T09:44:31.6163607Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/src'
2020-01-22T09:44:31.6163872Z Makefile:562: recipe for target 'all-recursive' failed
2020-01-22T09:44:31.6164279Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out'
2020-01-22T09:44:31.6164526Z Makefile:445: recipe for target 'all' failed
2020-01-22T09:44:31.6164823Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out'
2020-01-22T09:44:31.6165194Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/libfontconfig.a' failed
2020-01-22T09:44:31.6165520Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-22T09:44:31.6165783Z --- stderr
2020-01-22T09:44:31.6166013Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-22T09:44:31.6166013Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-22T09:44:31.6166319Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2020-01-22T09:44:31.6166559Z WARNING: 'gperf' is missing on your system.
2020-01-22T09:44:31.6166642Z          You might have modified some files without having the proper
2020-01-22T09:44:31.6166887Z          tools for further handling them.  Check the 'README' file, it
2020-01-22T09:44:31.6166974Z          often tells you about the needed prerequisites for installing
2020-01-22T09:44:31.6167052Z          this package.  You may also peek at any GNU archive site, in
2020-01-22T09:44:31.6167303Z          case some other package contains this missing 'gperf' program.
2020-01-22T09:44:31.6167382Z make[4]: *** [fcobjshash.h] Error 1
2020-01-22T09:44:31.6167445Z make[4]: *** Waiting for unfinished jobs....
2020-01-22T09:44:31.6167664Z make[3]: *** [all-recursive] Error 1
2020-01-22T09:44:31.6167728Z make[2]: *** [all] Error 2
2020-01-22T09:44:31.6168042Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-3d2cba7536d3cc06/out/libfontconfig.a] Error 2
2020-01-22T09:44:31.6168325Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2020-01-22T09:44:31.6168602Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2020-01-22T09:44:31.6168879Z                                                                                  "-f",
2020-01-22T09:44:31.6169288Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2020-01-22T09:44:31.6169476Z 
2020-01-22T09:44:31.6169559Z warning: build failed, waiting for other jobs to finish...
2020-01-22T09:44:38.9752144Z warning: use of deprecated item 'try': use the `?` operator instead
2020-01-22T09:44:38.9753274Z    --> webrender/src/batch.rs:342:1
---
2020-01-22T09:46:22.5860460Z 
2020-01-22T09:46:22.5860515Z 
2020-01-22T09:46:22.5874638Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-01-22T09:46:22.5875014Z Build completed unsuccessfully in 1:46:03
2020-01-22T09:46:22.5919325Z Makefile:50: recipe for target 'check-aux' failed
2020-01-22T09:46:22.5921795Z make: *** [check-aux] Error 1
2020-01-22T09:46:24.5269726Z   local time: Wed Jan 22 09:46:24 UTC 2020
2020-01-22T09:46:24.7927836Z   network time: Wed, 22 Jan 2020 09:46:24 GMT
2020-01-22T09:46:24.7928042Z == end clock drift check ==
2020-01-22T09:46:31.3803502Z 
2020-01-22T09:46:31.3803502Z 
2020-01-22T09:46:31.3889776Z ##[error]Bash exited with code '2'.
2020-01-22T09:46:31.3928810Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-22T09:46:31.3930561Z ==============================================================================
2020-01-22T09:46:31.3930650Z Task         : Get sources
2020-01-22T09:46:31.3930731Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
