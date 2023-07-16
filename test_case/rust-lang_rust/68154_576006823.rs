plain
2020-01-19T13:51:39.8617016Z    Compiling wayland-sys v0.21.13
2020-01-19T13:51:40.8250136Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2020-01-19T13:51:40.8266632Z 
2020-01-19T13:51:40.8267025Z Caused by:
2020-01-19T13:51:40.8267798Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-0e9785db88371e3c/build-script-build` (exit code: 101)
2020-01-19T13:51:40.8271143Z --- stdout
2020-01-19T13:51:40.8271906Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-19T13:51:40.8272626Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out && \
2020-01-19T13:51:40.8273001Z  CC="gcc" \
2020-01-19T13:51:40.8273245Z  AR="ar" \
2020-01-19T13:51:40.8273501Z  FREETYPE_CFLAGS="" \
2020-01-19T13:51:40.8273763Z  FREETYPE_LIBS="" \
2020-01-19T13:51:40.8274252Z  CFLAGS=""" -fPIC" \
2020-01-19T13:51:40.8274883Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2020-01-19T13:51:40.8275431Z  --disable-docs \
2020-01-19T13:51:40.8275975Z  --disable-shared \
2020-01-19T13:51:40.8276275Z   \
2020-01-19T13:51:40.8276827Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2020-01-19T13:51:40.8277773Z checking whether build environment is sane... yes
2020-01-19T13:51:40.8278502Z checking for x86_64-unknown-linux-gnu-strip... no
2020-01-19T13:51:40.8279403Z checking for strip... strip
2020-01-19T13:51:40.8280272Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-19T13:51:40.8280272Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-01-19T13:51:40.8280567Z checking for gawk... no
2020-01-19T13:51:40.8280754Z checking for mawk... mawk
2020-01-19T13:51:40.8281338Z checking whether make supports nested variables... yes
2020-01-19T13:51:40.8281536Z checking whether make supports nested variables... (cached) yes
2020-01-19T13:51:40.8282033Z checking whether to enable maintainer-specific portions of Makefiles... no
2020-01-19T13:51:40.8282527Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2020-01-19T13:51:40.8327113Z checking whether make sets $(MAKE)... (cached) yes
2020-01-19T13:51:40.8327836Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2020-01-19T13:51:40.8328384Z checking for pkg-config... /usr/bin/pkg-config
2020-01-19T13:51:40.8328872Z checking pkg-config is at least version 0.9.0... yes
2020-01-19T13:51:40.8329315Z checking for RM macro... rm -f
2020-01-19T13:51:40.8330646Z checking host system type... x86_64-unknown-linux-gnu
2020-01-19T13:51:40.8330908Z checking how to print strings... printf
2020-01-19T13:51:40.8331101Z checking for a sed that does not truncate output... /bin/sed
2020-01-19T13:51:40.8331503Z checking for fgrep... /bin/grep -F
2020-01-19T13:51:40.8331503Z checking for fgrep... /bin/grep -F
2020-01-19T13:51:40.8331735Z checking for ld used by gcc... /usr/bin/ld
2020-01-19T13:51:40.8331916Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-01-19T13:51:40.8332365Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2020-01-19T13:51:40.8332851Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2020-01-19T13:51:40.8333262Z checking the maximum length of command line arguments... 1572864
2020-01-19T13:51:40.8333533Z checking whether the shell understands some XSI constructs... yes
2020-01-19T13:51:40.8333719Z checking whether the shell understands "+="... yes
2020-01-19T13:51:40.8334281Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2020-01-19T13:51:40.8335021Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2020-01-19T13:51:40.8335519Z checking for /usr/bin/ld option to reload object files... -r
2020-01-19T13:51:40.8335999Z checking for x86_64-unknown-linux-gnu-objdump... no
2020-01-19T13:51:40.8336413Z checking how to recognize dependent libraries... pass_all
2020-01-19T13:51:40.8336848Z checking for x86_64-unknown-linux-gnu-dlltool... no
2020-01-19T13:51:40.8337063Z checking for dlltool... no
2020-01-19T13:51:40.8337269Z checking how to associate runtime and link libraries... printf %s\n
2020-01-19T13:51:40.8337269Z checking how to associate runtime and link libraries... printf %s\n
2020-01-19T13:51:40.8337808Z checking for x86_64-unknown-linux-gnu-ar... ar
2020-01-19T13:51:40.8338053Z checking for archiver @FILE support... @
2020-01-19T13:51:40.8338487Z checking for x86_64-unknown-linux-gnu-strip... strip
2020-01-19T13:51:40.8338965Z checking for x86_64-unknown-linux-gnu-ranlib... no
2020-01-19T13:51:40.8339201Z checking for ranlib... ranlib
2020-01-19T13:51:40.8339955Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2020-01-19T13:51:40.8340263Z checking for sysroot... no
2020-01-19T13:51:40.8340712Z checking for x86_64-unknown-linux-gnu-mt... no
2020-01-19T13:51:40.8340937Z checking for mt... no
2020-01-19T13:51:40.8341142Z checking if : is a manifest tool... no
2020-01-19T13:51:40.8341310Z checking for dlfcn.h... yes
2020-01-19T13:51:40.8341475Z checking for objdir... .libs
2020-01-19T13:51:40.8341948Z checking if gcc supports -fno-rtti -fno-exceptions... no
2020-01-19T13:51:40.8342421Z checking for gcc option to produce PIC... -fPIC -DPIC
2020-01-19T13:51:40.8342887Z checking if gcc PIC flag -fPIC -DPIC works... yes
2020-01-19T13:51:40.8343372Z checking if gcc static flag -static works... yes
2020-01-19T13:51:40.8343805Z checking if gcc supports -c -o file.o... yes
2020-01-19T13:51:40.8344261Z checking if gcc supports -c -o file.o... (cached) yes
2020-01-19T13:51:40.8344781Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2020-01-19T13:51:40.8345038Z checking dynamic linker characteristics... GNU/Linux ld.so
2020-01-19T13:51:40.8345247Z checking how to hardcode library paths into programs... immediate
2020-01-19T13:51:40.8345439Z checking whether stripping libraries is possible... yes
2020-01-19T13:51:40.8345619Z checking if libtool supports shared libraries... yes
2020-01-19T13:51:40.8345815Z checking whether to build shared libraries... no
2020-01-19T13:51:40.8346190Z checking for dirent.h that defines DIR... yes
2020-01-19T13:51:40.8346385Z checking for library containing opendir... none required
2020-01-19T13:51:40.8346565Z checking for ANSI C header files... (cached) yes
2020-01-19T13:51:40.8346752Z checking fcntl.h usability... yes
---
2020-01-19T13:51:40.8350952Z checking for sys/param.h... yes
2020-01-19T13:51:40.8351136Z checking sys/mount.h usability... yes
2020-01-19T13:51:40.8351301Z checking sys/mount.h presence... yes
2020-01-19T13:51:40.8351462Z checking for sys/mount.h... yes
2020-01-19T13:51:40.8351645Z checking for stdint types... stdint.h (shortcircuit)
2020-01-19T13:51:40.8351845Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2020-01-19T13:51:40.8352611Z checking for inline... inline
2020-01-19T13:51:40.8352803Z checking for flexible array members... yes
2020-01-19T13:51:40.8352973Z checking for pid_t... yes
2020-01-19T13:51:40.8353153Z checking for vprintf... yes
---
2020-01-19T13:51:40.8354047Z checking for getpagesize... yes
2020-01-19T13:51:40.8354226Z checking for working mmap... yes
2020-01-19T13:51:40.8354392Z checking for link... yes
2020-01-19T13:51:40.8354551Z checking for mkstemp... yes
2020-01-19T13:51:40.8354732Z checking for mkostemp... yes
2020-01-19T13:51:40.8354915Z checking for _mktemp_s... no
2020-01-19T13:51:40.8355252Z checking for getopt... yes
2020-01-19T13:51:40.8355416Z checking for getopt_long... yes
2020-01-19T13:51:40.8355606Z checking for getprogname... no
2020-01-19T13:51:40.8355770Z checking for getexecname... no
---
2020-01-19T13:51:40.8357861Z checking for fstatfs... yes
2020-01-19T13:51:40.8358022Z checking for lstat... yes
2020-01-19T13:51:40.8358189Z checking for posix_fadvise in fcntl.h... fcntl.h
2020-01-19T13:51:40.8358388Z checking for scandir... yes
2020-01-19T13:51:40.8358563Z checking for struct statvfs.f_basetype... no
2020-01-19T13:51:40.8358738Z checking for struct statvfs.f_fstypename... no
2020-01-19T13:51:40.8407144Z checking for struct statfs.f_flags... yes
2020-01-19T13:51:40.8407684Z checking for struct statfs.f_fstypename... no
2020-01-19T13:51:40.8407908Z checking for struct dirent.d_type... yes
2020-01-19T13:51:40.8408083Z checking for FREETYPE... yes
2020-01-19T13:51:40.8408252Z checking for FT_Get_Next_Char... yes
2020-01-19T13:51:40.8408457Z checking for FT_Get_BDF_Property... yes
2020-01-19T13:51:40.8408648Z checking for FT_Get_PS_Font_Info... yes
2020-01-19T13:51:40.8408820Z checking for FT_Has_PS_Glyph_Names... yes
2020-01-19T13:51:40.8409010Z checking for FT_Get_X11_Font_Format... yes
2020-01-19T13:51:40.8409201Z checking for FT_Select_Size... yes
2020-01-19T13:51:40.8409369Z checking for FT_Bitmap_Size.y_ppem... yes
2020-01-19T13:51:40.8410081Z checking expat.h usability... yes
2020-01-19T13:51:40.8410299Z checking expat.h presence... yes
2020-01-19T13:51:40.8410632Z checking for expat.h... yes
2020-01-19T13:51:40.8410632Z checking for expat.h... yes
2020-01-19T13:51:40.8410868Z checking for XML_SetDoctypeDeclHandler... yes
2020-01-19T13:51:40.8487627Z checking for Intel atomic primitives... true
2020-01-19T13:51:40.8488049Z checking for Solaris atomic operations... false
2020-01-19T13:51:40.8488661Z checking if compiler needs -Werror to reject unknown flags... no
2020-01-19T13:51:40.8489533Z checking for the pthreads library -lpthreads... no
2020-01-19T13:51:40.8491235Z checking whether pthreads work without any flags... no
2020-01-19T13:51:40.8491655Z checking whether pthreads work with -Kthread... no
2020-01-19T13:51:40.8491936Z checking whether pthreads work with -kthread... no
2020-01-19T13:51:40.8492222Z checking for the pthreads library -llthread... no
2020-01-19T13:51:40.8492491Z checking whether pthreads work with -pthread... yes
2020-01-19T13:51:40.8492594Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2020-01-19T13:51:40.8492697Z checking if more special flags are required for pthreads... no
2020-01-19T13:51:40.8492793Z checking for PTHREAD_PRIO_INHERIT... yes
2020-01-19T13:51:40.8492865Z checking for docbook2html... no
2020-01-19T13:51:40.8492955Z checking whether byte ordering is bigendian... no
2020-01-19T13:51:40.8493028Z checking size of void *... 8
2020-01-19T13:51:40.8493111Z checking alignment of double... 8
2020-01-19T13:51:40.8493204Z checking that generated files are newer than configure... done
2020-01-19T13:51:40.8493376Z config.status: creating Makefile
2020-01-19T13:51:40.8493448Z config.status: creating fontconfig/Makefile
2020-01-19T13:51:40.8493736Z config.status: creating fc-lang/Makefile
2020-01-19T13:51:40.8493736Z config.status: creating fc-lang/Makefile
2020-01-19T13:51:40.8493998Z config.status: creating fc-glyphname/Makefile
2020-01-19T13:51:40.8494266Z config.status: creating fc-case/Makefile
2020-01-19T13:51:40.8494341Z config.status: creating src/Makefile
2020-01-19T13:51:40.8494426Z config.status: creating conf.d/Makefile
2020-01-19T13:51:40.8494687Z config.status: creating fc-cache/Makefile
2020-01-19T13:51:40.8494958Z config.status: creating fc-cat/Makefile
2020-01-19T13:51:40.8495224Z config.status: creating fc-list/Makefile
2020-01-19T13:51:40.8495478Z config.status: creating fc-match/Makefile
2020-01-19T13:51:40.8495747Z config.status: creating fc-pattern/Makefile
2020-01-19T13:51:40.8496001Z config.status: creating fc-query/Makefile
2020-01-19T13:51:40.8496276Z config.status: creating fc-scan/Makefile
2020-01-19T13:51:40.8496532Z config.status: creating fc-validate/Makefile
2020-01-19T13:51:40.8496695Z config.status: creating doc/version.sgml
2020-01-19T13:51:40.8496780Z config.status: creating test/Makefile
2020-01-19T13:51:40.8496850Z config.status: creating fontconfig.spec
2020-01-19T13:51:40.8496934Z config.status: creating fontconfig.pc
2020-01-19T13:51:40.8496934Z config.status: creating fontconfig.pc
2020-01-19T13:51:40.8497199Z config.status: creating fontconfig-zip
2020-01-19T13:51:40.8497274Z config.status: creating config.h
2020-01-19T13:51:40.8497369Z config.status: executing depfiles commands
2020-01-19T13:51:40.8497442Z config.status: executing libtool commands
2020-01-19T13:51:40.8497532Z config.status: executing src/fcstdint.h commands
2020-01-19T13:51:40.8497615Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-19T13:51:40.8497988Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out && make -j2
2020-01-19T13:51:40.8498389Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out'
2020-01-19T13:51:40.8499035Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out'
2020-01-19T13:51:40.8499135Z Making all in fontconfig
2020-01-19T13:51:40.8499520Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fontconfig'
2020-01-19T13:51:40.8500223Z make[4]: Nothing to be done for 'all'.
2020-01-19T13:51:40.8500223Z make[4]: Nothing to be done for 'all'.
2020-01-19T13:51:40.8500690Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fontconfig'
2020-01-19T13:51:40.8500974Z Making all in fc-case
2020-01-19T13:51:40.8501341Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-case'
2020-01-19T13:51:40.8501557Z   GEN      fcalias.h
2020-01-19T13:51:40.8501623Z   GEN      fcaliastail.h
2020-01-19T13:51:40.8502261Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-case'
2020-01-19T13:51:40.8502560Z make[5]: Nothing to be done for 'all-am'.
2020-01-19T13:51:40.8502925Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-case'
2020-01-19T13:51:40.8503345Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-case'
2020-01-19T13:51:40.8503345Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-case'
2020-01-19T13:51:40.8503626Z Making all in fc-lang
2020-01-19T13:51:40.8503989Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-lang'
2020-01-19T13:51:40.8504108Z   GEN      fcalias.h
2020-01-19T13:51:40.8504176Z   GEN      fcaliastail.h
2020-01-19T13:51:40.8504424Z make  all-am
2020-01-19T13:51:40.8504786Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-lang'
2020-01-19T13:51:40.8504904Z   GEN      fclang.h
2020-01-19T13:51:40.8505265Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-lang'
2020-01-19T13:51:40.8505539Z   GEN      fc-lang
2020-01-19T13:51:40.8505928Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-lang'
2020-01-19T13:51:40.8506328Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-lang'
2020-01-19T13:51:40.8506738Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-lang'
2020-01-19T13:51:40.8507005Z Making all in fc-glyphname
2020-01-19T13:51:40.8507403Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-glyphname'
2020-01-19T13:51:40.8507520Z   GEN      fcalias.h
2020-01-19T13:51:40.8507586Z   GEN      fcaliastail.h
2020-01-19T13:51:40.8507825Z make  all-am
2020-01-19T13:51:40.8508192Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-glyphname'
2020-01-19T13:51:40.8508492Z make[5]: Nothing to be done for 'all-am'.
2020-01-19T13:51:40.8508872Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-glyphname'
2020-01-19T13:51:40.8509291Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/fc-glyphname'
2020-01-19T13:51:40.8510057Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/src'
2020-01-19T13:51:40.8510195Z   GEN      fcalias.h
2020-01-19T13:51:40.8510195Z   GEN      fcalias.h
2020-01-19T13:51:40.8510261Z   GEN      fcftalias.h
2020-01-19T13:51:40.8510519Z   GEN      stamp-fcstdint
2020-01-19T13:51:40.8510598Z config.status: executing src/fcstdint.h commands
2020-01-19T13:51:40.8510698Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-01-19T13:51:40.8510779Z config.status: src/fcstdint.h is unchanged
2020-01-19T13:51:40.8510863Z   GEN      fcobjshash.gperf
2020-01-19T13:51:40.8510931Z   GEN      fcobjshash.h
2020-01-19T13:51:40.8511327Z Makefile:882: recipe for target 'fcobjshash.h' failed
2020-01-19T13:51:40.8511762Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/src'
2020-01-19T13:51:40.8512070Z Makefile:562: recipe for target 'all-recursive' failed
2020-01-19T13:51:40.8512446Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out'
2020-01-19T13:51:40.8512858Z Makefile:445: recipe for target 'all' failed
2020-01-19T13:51:40.8513241Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out'
2020-01-19T13:51:40.8513670Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/libfontconfig.a' failed
2020-01-19T13:51:40.8514063Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-01-19T13:51:40.8514374Z --- stderr
2020-01-19T13:51:40.8514669Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-19T13:51:40.8514669Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-01-19T13:51:40.8515033Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2020-01-19T13:51:40.8515332Z WARNING: 'gperf' is missing on your system.
2020-01-19T13:51:40.8515422Z          You might have modified some files without having the proper
2020-01-19T13:51:40.8515749Z          tools for further handling them.  Check the 'README' file, it
2020-01-19T13:51:40.8515844Z          often tells you about the needed prerequisites for installing
2020-01-19T13:51:40.8515951Z          this package.  You may also peek at any GNU archive site, in
2020-01-19T13:51:40.8516252Z          case some other package contains this missing 'gperf' program.
2020-01-19T13:51:40.8516352Z make[4]: *** [fcobjshash.h] Error 1
2020-01-19T13:51:40.8516441Z make[4]: *** Waiting for unfinished jobs....
2020-01-19T13:51:40.8516695Z make[3]: *** [all-recursive] Error 1
2020-01-19T13:51:40.8516793Z make[2]: *** [all] Error 2
2020-01-19T13:51:40.8517164Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-bfc18a431c840e1e/out/libfontconfig.a] Error 2
2020-01-19T13:51:40.8517512Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2020-01-19T13:51:40.8517834Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2020-01-19T13:51:40.8518182Z                                                                                  "-f",
2020-01-19T13:51:40.8518691Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2020-01-19T13:51:40.8518911Z 
2020-01-19T13:51:40.8518994Z warning: build failed, waiting for other jobs to finish...
2020-01-19T13:51:41.5217397Z error: build failed
2020-01-19T13:51:41.5248029Z thread 'main' panicked at 'tests failed for https://github.com/servo/webrender', src/tools/cargotest/main.rs:88:9
---
2020-01-19T13:51:41.5249330Z 
2020-01-19T13:51:41.5249386Z 
2020-01-19T13:51:41.5257180Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-01-19T13:51:41.5257592Z Build completed unsuccessfully in 1:56:03
2020-01-19T13:51:41.5312268Z Makefile:50: recipe for target 'check-aux' failed
2020-01-19T13:51:41.5312646Z make: *** [check-aux] Error 1
2020-01-19T13:51:41.5332044Z   local time: Sun Jan 19 13:51:41 UTC 2020
2020-01-19T13:51:42.0727317Z   network time: Sun, 19 Jan 2020 13:51:42 GMT
2020-01-19T13:51:42.0728125Z == end clock drift check ==
2020-01-19T13:51:45.5969012Z 
2020-01-19T13:51:45.5969012Z 
2020-01-19T13:51:45.6084798Z ##[error]Bash exited with code '2'.
2020-01-19T13:51:45.6124231Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-19T13:51:45.6126371Z ==============================================================================
2020-01-19T13:51:45.6126463Z Task         : Get sources
2020-01-19T13:51:45.6126561Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
