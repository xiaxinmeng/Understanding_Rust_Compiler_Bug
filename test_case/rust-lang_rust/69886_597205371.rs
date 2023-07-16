plain
2020-03-10T17:08:56.8248960Z    Compiling wayland-sys v0.21.13
2020-03-10T17:09:04.8926679Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2020-03-10T17:09:04.8933402Z 
2020-03-10T17:09:04.8934141Z Caused by:
2020-03-10T17:09:04.8935647Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-a3bc2049c985c42b/build-script-build` (exit code: 101)
2020-03-10T17:09:04.8941237Z --- stdout
2020-03-10T17:09:04.8942250Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-03-10T17:09:04.8943318Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out && \
2020-03-10T17:09:04.8943736Z  CC="gcc" \
2020-03-10T17:09:04.8943939Z  AR="ar" \
2020-03-10T17:09:04.8944140Z  FREETYPE_CFLAGS="" \
2020-03-10T17:09:04.8944378Z  FREETYPE_LIBS="" \
2020-03-10T17:09:04.8944789Z  CFLAGS=""" -fPIC" \
2020-03-10T17:09:04.8945436Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2020-03-10T17:09:04.8945959Z  --disable-docs \
2020-03-10T17:09:04.8946376Z  --disable-shared \
2020-03-10T17:09:04.8946568Z   \
2020-03-10T17:09:04.8947102Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2020-03-10T17:09:04.8948102Z checking whether build environment is sane... yes
2020-03-10T17:09:04.8948663Z checking for x86_64-unknown-linux-gnu-strip... no
2020-03-10T17:09:04.8948958Z checking for strip... strip
2020-03-10T17:09:04.8949691Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-03-10T17:09:04.8949691Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-03-10T17:09:04.8949970Z checking for gawk... no
2020-03-10T17:09:04.8950224Z checking for mawk... mawk
2020-03-10T17:09:04.8950817Z checking whether make supports nested variables... yes
2020-03-10T17:09:04.8951628Z checking whether make supports nested variables... (cached) yes
2020-03-10T17:09:04.8952324Z checking whether to enable maintainer-specific portions of Makefiles... no
2020-03-10T17:09:04.8952939Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2020-03-10T17:09:04.8970583Z checking whether make sets $(MAKE)... (cached) yes
2020-03-10T17:09:04.8971124Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2020-03-10T17:09:04.8971657Z checking for pkg-config... /usr/bin/pkg-config
2020-03-10T17:09:04.8972177Z checking pkg-config is at least version 0.9.0... yes
2020-03-10T17:09:04.8972656Z checking for RM macro... rm -f
2020-03-10T17:09:04.8973713Z checking host system type... x86_64-unknown-linux-gnu
2020-03-10T17:09:04.8974041Z checking how to print strings... printf
2020-03-10T17:09:04.8974393Z checking for a sed that does not truncate output... /bin/sed
2020-03-10T17:09:04.8974903Z checking for fgrep... /bin/grep -F
2020-03-10T17:09:04.8974903Z checking for fgrep... /bin/grep -F
2020-03-10T17:09:04.8975195Z checking for ld used by gcc... /usr/bin/ld
2020-03-10T17:09:04.8975538Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-03-10T17:09:04.8976117Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2020-03-10T17:09:04.8976732Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2020-03-10T17:09:04.8977122Z checking the maximum length of command line arguments... 1572864
2020-03-10T17:09:04.8977531Z checking whether the shell understands some XSI constructs... yes
2020-03-10T17:09:04.8977909Z checking whether the shell understands "+="... yes
2020-03-10T17:09:04.8978642Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2020-03-10T17:09:04.8979486Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2020-03-10T17:09:04.8980387Z checking for /usr/bin/ld option to reload object files... -r
2020-03-10T17:09:04.8980958Z checking for x86_64-unknown-linux-gnu-objdump... no
2020-03-10T17:09:04.8981595Z checking how to recognize dependent libraries... pass_all
2020-03-10T17:09:04.8982151Z checking for x86_64-unknown-linux-gnu-dlltool... no
2020-03-10T17:09:04.8982445Z checking for dlltool... no
2020-03-10T17:09:04.8982787Z checking how to associate runtime and link libraries... printf %s\n
2020-03-10T17:09:04.8982787Z checking how to associate runtime and link libraries... printf %s\n
2020-03-10T17:09:04.8983332Z checking for x86_64-unknown-linux-gnu-ar... ar
2020-03-10T17:09:04.8983659Z checking for archiver @FILE support... @
2020-03-10T17:09:04.8984259Z checking for x86_64-unknown-linux-gnu-strip... strip
2020-03-10T17:09:04.8984851Z checking for x86_64-unknown-linux-gnu-ranlib... no
2020-03-10T17:09:04.8985148Z checking for ranlib... ranlib
2020-03-10T17:09:04.8985703Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2020-03-10T17:09:04.8986061Z checking for sysroot... no
2020-03-10T17:09:04.8986529Z checking for x86_64-unknown-linux-gnu-mt... no
2020-03-10T17:09:04.8986820Z checking for mt... no
2020-03-10T17:09:04.8987082Z checking if : is a manifest tool... no
2020-03-10T17:09:04.8987374Z checking for dlfcn.h... yes
2020-03-10T17:09:04.8987621Z checking for objdir... .libs
2020-03-10T17:09:04.8988135Z checking if gcc supports -fno-rtti -fno-exceptions... no
2020-03-10T17:09:04.8988669Z checking for gcc option to produce PIC... -fPIC -DPIC
2020-03-10T17:09:04.8989207Z checking if gcc PIC flag -fPIC -DPIC works... yes
2020-03-10T17:09:04.8989731Z checking if gcc static flag -static works... yes
2020-03-10T17:09:04.8990376Z checking if gcc supports -c -o file.o... yes
2020-03-10T17:09:04.8990894Z checking if gcc supports -c -o file.o... (cached) yes
2020-03-10T17:09:04.8991709Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2020-03-10T17:09:04.8992178Z checking dynamic linker characteristics... GNU/Linux ld.so
2020-03-10T17:09:04.8992557Z checking how to hardcode library paths into programs... immediate
2020-03-10T17:09:04.8992947Z checking whether stripping libraries is possible... yes
2020-03-10T17:09:04.8993312Z checking if libtool supports shared libraries... yes
2020-03-10T17:09:04.8993988Z checking whether to build static libraries... yes
2020-03-10T17:09:04.8994311Z checking for dirent.h that defines DIR... yes
2020-03-10T17:09:04.8994666Z checking for library containing opendir... none required
2020-03-10T17:09:04.8995010Z checking for ANSI C header files... (cached) yes
---
2020-03-10T17:09:04.9001197Z checking for sys/param.h... yes
2020-03-10T17:09:04.9001652Z checking sys/mount.h usability... yes
2020-03-10T17:09:04.9002084Z checking sys/mount.h presence... yes
2020-03-10T17:09:04.9002359Z checking for sys/mount.h... yes
2020-03-10T17:09:04.9002823Z checking for stdint types... stdint.h (shortcircuit)
2020-03-10T17:09:04.9003222Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2020-03-10T17:09:04.9004134Z checking for inline... inline
2020-03-10T17:09:04.9004412Z checking for flexible array members... yes
2020-03-10T17:09:04.9004697Z checking for pid_t... yes
2020-03-10T17:09:04.9004955Z checking for vprintf... yes
---
2020-03-10T17:09:04.9006448Z checking for getpagesize... yes
2020-03-10T17:09:04.9006726Z checking for working mmap... yes
2020-03-10T17:09:04.9006973Z checking for link... yes
2020-03-10T17:09:04.9007229Z checking for mkstemp... yes
2020-03-10T17:09:04.9007475Z checking for mkostemp... yes
2020-03-10T17:09:04.9007740Z checking for _mktemp_s... no
2020-03-10T17:09:04.9008243Z checking for getopt... yes
2020-03-10T17:09:04.9008510Z checking for getopt_long... yes
2020-03-10T17:09:04.9008766Z checking for getprogname... no
2020-03-10T17:09:04.9009037Z checking for getexecname... no
---
2020-03-10T17:09:04.9012796Z checking for fstatfs... yes
2020-03-10T17:09:04.9013053Z checking for lstat... yes
2020-03-10T17:09:04.9013335Z checking for posix_fadvise in fcntl.h... fcntl.h
2020-03-10T17:09:04.9013636Z checking for scandir... yes
2020-03-10T17:09:04.9013913Z checking for struct statvfs.f_basetype... no
2020-03-10T17:09:04.9014247Z checking for struct statvfs.f_fstypename... no
2020-03-10T17:09:04.9014568Z checking for struct statfs.f_flags... yes
2020-03-10T17:09:04.9014874Z checking for struct statfs.f_fstypename... no
2020-03-10T17:09:04.9015197Z checking for struct dirent.d_type... yes
2020-03-10T17:09:04.9015468Z checking for FREETYPE... yes
2020-03-10T17:09:04.9015748Z checking for FT_Get_Next_Char... yes
2020-03-10T17:09:04.9016029Z checking for FT_Get_BDF_Property... yes
2020-03-10T17:09:04.9016491Z checking for FT_Get_PS_Font_Info... yes
2020-03-10T17:09:04.9016771Z checking for FT_Has_PS_Glyph_Names... yes
2020-03-10T17:09:04.9017229Z checking for FT_Get_X11_Font_Format... yes
2020-03-10T17:09:04.9017668Z checking for FT_Select_Size... yes
2020-03-10T17:09:04.9018076Z checking for FT_Bitmap_Size.y_ppem... yes
2020-03-10T17:09:04.9018542Z checking expat.h usability... yes
2020-03-10T17:09:04.9018787Z checking expat.h presence... yes
2020-03-10T17:09:04.9019008Z checking for expat.h... yes
2020-03-10T17:09:04.9019008Z checking for expat.h... yes
2020-03-10T17:09:04.9019440Z checking for XML_SetDoctypeDeclHandler... yes
2020-03-10T17:09:04.9019717Z checking for Intel atomic primitives... true
2020-03-10T17:09:04.9020016Z checking for Solaris atomic operations... false
2020-03-10T17:09:04.9020584Z checking if compiler needs -Werror to reject unknown flags... no
2020-03-10T17:09:04.9021086Z checking for the pthreads library -lpthreads... no
2020-03-10T17:09:04.9021693Z checking whether pthreads work without any flags... no
2020-03-10T17:09:04.9022387Z checking whether pthreads work with -Kthread... no
2020-03-10T17:09:04.9023043Z checking whether pthreads work with -kthread... no
2020-03-10T17:09:04.9023683Z checking for the pthreads library -llthread... no
2020-03-10T17:09:04.9024390Z checking whether pthreads work with -pthread... yes
2020-03-10T17:09:04.9024929Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2020-03-10T17:09:04.9025305Z checking if more special flags are required for pthreads... no
2020-03-10T17:09:04.9026017Z checking for PTHREAD_PRIO_INHERIT... yes
2020-03-10T17:09:04.9026521Z checking for docbook2html... no
2020-03-10T17:09:04.9026836Z checking whether byte ordering is bigendian... no
2020-03-10T17:09:04.9027223Z checking size of void *... 8
2020-03-10T17:09:04.9027515Z checking alignment of double... 8
2020-03-10T17:09:04.9027840Z checking that generated files are newer than configure... done
2020-03-10T17:09:04.9028491Z config.status: creating Makefile
2020-03-10T17:09:04.9028790Z config.status: creating fontconfig/Makefile
2020-03-10T17:09:04.9029330Z config.status: creating fc-lang/Makefile
2020-03-10T17:09:04.9029330Z config.status: creating fc-lang/Makefile
2020-03-10T17:09:04.9029828Z config.status: creating fc-glyphname/Makefile
2020-03-10T17:09:04.9030333Z config.status: creating fc-case/Makefile
2020-03-10T17:09:04.9030633Z config.status: creating src/Makefile
2020-03-10T17:09:04.9030947Z config.status: creating conf.d/Makefile
2020-03-10T17:09:04.9031425Z config.status: creating fc-cache/Makefile
2020-03-10T17:09:04.9031920Z config.status: creating fc-cat/Makefile
2020-03-10T17:09:04.9032416Z config.status: creating fc-list/Makefile
2020-03-10T17:09:04.9032898Z config.status: creating fc-match/Makefile
2020-03-10T17:09:04.9033410Z config.status: creating fc-pattern/Makefile
2020-03-10T17:09:04.9033901Z config.status: creating fc-query/Makefile
2020-03-10T17:09:04.9034396Z config.status: creating fc-scan/Makefile
2020-03-10T17:09:04.9034894Z config.status: creating fc-validate/Makefile
2020-03-10T17:09:04.9035521Z config.status: creating doc/version.sgml
2020-03-10T17:09:04.9035839Z config.status: creating test/Makefile
2020-03-10T17:09:04.9036154Z config.status: creating fontconfig.spec
2020-03-10T17:09:04.9036451Z config.status: creating fontconfig.pc
2020-03-10T17:09:04.9036451Z config.status: creating fontconfig.pc
2020-03-10T17:09:04.9036939Z config.status: creating fontconfig-zip
2020-03-10T17:09:04.9037229Z config.status: creating config.h
2020-03-10T17:09:04.9037542Z config.status: executing depfiles commands
2020-03-10T17:09:04.9037854Z config.status: executing libtool commands
2020-03-10T17:09:04.9038200Z config.status: executing src/fcstdint.h commands
2020-03-10T17:09:04.9038590Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-03-10T17:09:04.9039356Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out && make -j2
2020-03-10T17:09:04.9040230Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out'
2020-03-10T17:09:04.9041509Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out'
2020-03-10T17:09:04.9041943Z Making all in fontconfig
2020-03-10T17:09:04.9042683Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fontconfig'
2020-03-10T17:09:04.9043354Z make[4]: Nothing to be done for 'all'.
2020-03-10T17:09:04.9043354Z make[4]: Nothing to be done for 'all'.
2020-03-10T17:09:04.9044105Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fontconfig'
2020-03-10T17:09:04.9044723Z Making all in fc-case
2020-03-10T17:09:04.9045436Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-case'
2020-03-10T17:09:04.9045898Z   GEN      fcalias.h
2020-03-10T17:09:04.9046221Z   GEN      fcaliastail.h
2020-03-10T17:09:04.9046630Z make  all-am
2020-03-10T17:09:04.9047338Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-case'
2020-03-10T17:09:04.9047995Z make[5]: Nothing to be done for 'all-am'.
2020-03-10T17:09:04.9048756Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-case'
2020-03-10T17:09:04.9051542Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-case'
2020-03-10T17:09:04.9052312Z Making all in fc-lang
2020-03-10T17:09:04.9053178Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-lang'
2020-03-10T17:09:04.9053667Z   GEN      fcalias.h
2020-03-10T17:09:04.9053914Z   GEN      fcaliastail.h
2020-03-10T17:09:04.9054484Z make  all-am
2020-03-10T17:09:04.9055354Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-lang'
2020-03-10T17:09:04.9055796Z   GEN      fclang.h
2020-03-10T17:09:04.9056516Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-lang'
2020-03-10T17:09:04.9057099Z   GEN      fc-lang
2020-03-10T17:09:04.9057820Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-lang'
2020-03-10T17:09:04.9058788Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-lang'
2020-03-10T17:09:04.9059716Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-lang'
2020-03-10T17:09:04.9060342Z Making all in fc-glyphname
2020-03-10T17:09:04.9061069Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-glyphname'
2020-03-10T17:09:04.9061542Z   GEN      fcalias.h
2020-03-10T17:09:04.9061788Z   GEN      fcaliastail.h
2020-03-10T17:09:04.9062155Z make  all-am
2020-03-10T17:09:04.9062866Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-glyphname'
2020-03-10T17:09:04.9063533Z make[5]: Nothing to be done for 'all-am'.
2020-03-10T17:09:04.9064639Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-glyphname'
2020-03-10T17:09:04.9065965Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/fc-glyphname'
2020-03-10T17:09:04.9067749Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/src'
2020-03-10T17:09:04.9068190Z   GEN      fcalias.h
2020-03-10T17:09:04.9068435Z   GEN      fcftalias.h
2020-03-10T17:09:04.9068830Z   GEN      stamp-fcstdint
2020-03-10T17:09:04.9068830Z   GEN      stamp-fcstdint
2020-03-10T17:09:04.9069144Z config.status: executing src/fcstdint.h commands
2020-03-10T17:09:04.9069531Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-03-10T17:09:04.9069921Z config.status: src/fcstdint.h is unchanged
2020-03-10T17:09:04.9070219Z   GEN      fcobjshash.gperf
2020-03-10T17:09:04.9070459Z   GEN      fcobjshash.h
2020-03-10T17:09:04.9070966Z Makefile:882: recipe for target 'fcobjshash.h' failed
2020-03-10T17:09:04.9071726Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/src'
2020-03-10T17:09:04.9072435Z Makefile:562: recipe for target 'all-recursive' failed
2020-03-10T17:09:04.9073188Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out'
2020-03-10T17:09:04.9073869Z Makefile:445: recipe for target 'all' failed
2020-03-10T17:09:04.9074779Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out'
2020-03-10T17:09:04.9075771Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/libfontconfig.a' failed
2020-03-10T17:09:04.9076691Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-03-10T17:09:04.9077344Z --- stderr
2020-03-10T17:09:04.9077853Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-03-10T17:09:04.9077853Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-03-10T17:09:04.9078657Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2020-03-10T17:09:04.9079382Z WARNING: 'gperf' is missing on your system.
2020-03-10T17:09:04.9079782Z          You might have modified some files without having the proper
2020-03-10T17:09:04.9080435Z          tools for further handling them.  Check the 'README' file, it
2020-03-10T17:09:04.9080862Z          often tells you about the needed prerequisites for installing
2020-03-10T17:09:04.9081293Z          this package.  You may also peek at any GNU archive site, in
2020-03-10T17:09:04.9081911Z          case some other package contains this missing 'gperf' program.
2020-03-10T17:09:04.9082284Z make[4]: *** [fcobjshash.h] Error 1
2020-03-10T17:09:04.9082589Z make[4]: *** Waiting for unfinished jobs....
2020-03-10T17:09:04.9083084Z make[3]: *** [all-recursive] Error 1
2020-03-10T17:09:04.9083379Z make[2]: *** [all] Error 2
2020-03-10T17:09:04.9084107Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/libfontconfig.a] Error 2
2020-03-10T17:09:04.9084933Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2020-03-10T17:09:04.9086310Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2020-03-10T17:09:04.9087125Z                                                                                  "-f",
2020-03-10T17:09:04.9088485Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2020-03-10T17:09:04.9089681Z 
2020-03-10T17:09:04.9089945Z warning: build failed, waiting for other jobs to finish...
2020-03-10T17:09:07.8095611Z warning: use of deprecated item 'try': use the `?` operator instead
2020-03-10T17:09:07.8110836Z    --> webrender/src/batch.rs:342:1
---
2020-03-10T17:11:02.4128540Z 
2020-03-10T17:11:02.4128651Z 
2020-03-10T17:11:02.4146015Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-03-10T17:11:02.4146627Z Build completed unsuccessfully in 2:17:28
2020-03-10T17:11:02.4199269Z Makefile:50: recipe for target 'check-aux' failed
2020-03-10T17:11:02.4199805Z make: *** [check-aux] Error 1
2020-03-10T17:11:02.4656801Z   local time: Tue Mar 10 17:11:02 UTC 2020
2020-03-10T17:11:02.5588032Z   network time: Tue, 10 Mar 2020 17:11:02 GMT
2020-03-10T17:11:02.5593039Z == end clock drift check ==
2020-03-10T17:11:10.8277992Z 
2020-03-10T17:11:10.8277992Z 
2020-03-10T17:11:10.8359790Z ##[error]Bash exited with code '2'.
2020-03-10T17:11:10.8456015Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-10T17:11:10.8462562Z ==============================================================================
2020-03-10T17:11:10.8462953Z Task         : Get sources
2020-03-10T17:11:10.8463369Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
