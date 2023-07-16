plain
2020-02-16T12:13:14.7493471Z    Compiling wayland-sys v0.21.13
2020-02-16T12:13:22.7630664Z error: failed to run custom build command for `servo-fontconfig-sys v4.0.4`
2020-02-16T12:13:23.4505125Z 
2020-02-16T12:13:23.4512070Z Caused by:
2020-02-16T12:13:23.4513229Z   process didn't exit successfully: `/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-8a7d29f7fc94bf3d/build-script-build` (exit code: 101)
2020-02-16T12:13:23.4513467Z --- stdout
2020-02-16T12:13:23.4513763Z make[1]: Entering directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-02-16T12:13:23.4514067Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out && \
2020-02-16T12:13:23.4514166Z  CC="gcc" \
2020-02-16T12:13:23.4514232Z  AR="ar" \
2020-02-16T12:13:23.4514282Z  FREETYPE_CFLAGS="" \
2020-02-16T12:13:23.4514354Z  FREETYPE_LIBS="" \
2020-02-16T12:13:23.4514859Z  CFLAGS=""" -fPIC" \
2020-02-16T12:13:23.4515190Z  /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/configure \
2020-02-16T12:13:23.4515388Z  --disable-docs \
2020-02-16T12:13:23.4515585Z  --disable-shared \
2020-02-16T12:13:23.4515640Z   \
2020-02-16T12:13:23.4515887Z  --host=x86_64-unknown-linux-gnu --sysconfdir=/etc --localstatedir=/var
2020-02-16T12:13:23.4516380Z checking whether build environment is sane... yes
2020-02-16T12:13:23.4516610Z checking for x86_64-unknown-linux-gnu-strip... no
2020-02-16T12:13:23.4516672Z checking for strip... strip
2020-02-16T12:13:23.4516895Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-02-16T12:13:23.4516895Z checking for a thread-safe mkdir -p... /bin/mkdir -p
2020-02-16T12:13:23.4517076Z checking for gawk... no
2020-02-16T12:13:23.4517146Z checking for mawk... mawk
2020-02-16T12:13:23.4517286Z checking whether make supports nested variables... yes
2020-02-16T12:13:23.4517353Z checking whether make supports nested variables... (cached) yes
2020-02-16T12:13:23.4517638Z checking whether to enable maintainer-specific portions of Makefiles... no
2020-02-16T12:13:23.4517870Z checking for x86_64-unknown-linux-gnu-gcc... gcc
---
2020-02-16T12:13:23.4521403Z checking whether make sets $(MAKE)... (cached) yes
2020-02-16T12:13:23.4521644Z checking for x86_64-unknown-linux-gnu-pkg-config... no
2020-02-16T12:13:23.4521861Z checking for pkg-config... /usr/bin/pkg-config
2020-02-16T12:13:23.4522097Z checking pkg-config is at least version 0.9.0... yes
2020-02-16T12:13:23.4522312Z checking for RM macro... rm -f
2020-02-16T12:13:23.4522969Z checking host system type... x86_64-unknown-linux-gnu
2020-02-16T12:13:23.4523289Z checking how to print strings... printf
2020-02-16T12:13:23.4523364Z checking for a sed that does not truncate output... /bin/sed
2020-02-16T12:13:23.4523773Z checking for fgrep... /bin/grep -F
2020-02-16T12:13:23.4523773Z checking for fgrep... /bin/grep -F
2020-02-16T12:13:23.4523849Z checking for ld used by gcc... /usr/bin/ld
2020-02-16T12:13:23.4523912Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-02-16T12:13:23.4524163Z checking for BSD- or MS-compatible name lister (nm)... /usr/bin/nm -B
2020-02-16T12:13:23.4524394Z checking the name lister (/usr/bin/nm -B) interface... BSD nm
2020-02-16T12:13:23.4524482Z checking the maximum length of command line arguments... 1572864
2020-02-16T12:13:23.4524553Z checking whether the shell understands some XSI constructs... yes
2020-02-16T12:13:23.4524711Z checking whether the shell understands "+="... yes
2020-02-16T12:13:23.4525024Z checking how to convert x86_64-unknown-linux-gnu file names to x86_64-unknown-linux-gnu format... func_convert_file_noop
2020-02-16T12:13:23.4525338Z checking how to convert x86_64-unknown-linux-gnu file names to toolchain format... func_convert_file_noop
2020-02-16T12:13:23.4525797Z checking for /usr/bin/ld option to reload object files... -r
2020-02-16T12:13:23.4526012Z checking for x86_64-unknown-linux-gnu-objdump... no
2020-02-16T12:13:23.4526333Z checking how to recognize dependent libraries... pass_all
2020-02-16T12:13:23.4526566Z checking for x86_64-unknown-linux-gnu-dlltool... no
2020-02-16T12:13:23.4526628Z checking for dlltool... no
2020-02-16T12:13:23.4526708Z checking how to associate runtime and link libraries... printf %s\n
2020-02-16T12:13:23.4526708Z checking how to associate runtime and link libraries... printf %s\n
2020-02-16T12:13:23.4526926Z checking for x86_64-unknown-linux-gnu-ar... ar
2020-02-16T12:13:23.4527012Z checking for archiver @FILE support... @
2020-02-16T12:13:23.4527242Z checking for x86_64-unknown-linux-gnu-strip... strip
2020-02-16T12:13:23.4527459Z checking for x86_64-unknown-linux-gnu-ranlib... no
2020-02-16T12:13:23.4527536Z checking for ranlib... ranlib
2020-02-16T12:13:23.4527767Z checking command to parse /usr/bin/nm -B output from gcc object... ok
2020-02-16T12:13:23.4527854Z checking for sysroot... no
2020-02-16T12:13:23.4528061Z checking for x86_64-unknown-linux-gnu-mt... no
2020-02-16T12:13:23.4528138Z checking for mt... no
2020-02-16T12:13:23.4528193Z checking if : is a manifest tool... no
2020-02-16T12:13:23.4528265Z checking for dlfcn.h... yes
2020-02-16T12:13:23.4528319Z checking for objdir... .libs
2020-02-16T12:13:23.4528554Z checking if gcc supports -fno-rtti -fno-exceptions... no
2020-02-16T12:13:23.4528786Z checking for gcc option to produce PIC... -fPIC -DPIC
2020-02-16T12:13:23.4529183Z checking if gcc PIC flag -fPIC -DPIC works... yes
2020-02-16T12:13:23.4529413Z checking if gcc static flag -static works... yes
2020-02-16T12:13:23.4529633Z checking if gcc supports -c -o file.o... yes
2020-02-16T12:13:23.4529870Z checking if gcc supports -c -o file.o... (cached) yes
2020-02-16T12:13:23.4530142Z checking whether the gcc linker (/usr/bin/ld -m elf_x86_64) supports shared libraries... yes
2020-02-16T12:13:23.4530242Z checking dynamic linker characteristics... GNU/Linux ld.so
2020-02-16T12:13:23.4530315Z checking how to hardcode library paths into programs... immediate
2020-02-16T12:13:23.4530404Z checking whether stripping libraries is possible... yes
2020-02-16T12:13:23.4530486Z checking if libtool supports shared libraries... yes
2020-02-16T12:13:23.4530553Z checking whether to build shared libraries... no
2020-02-16T12:13:23.4530702Z checking for dirent.h that defines DIR... yes
2020-02-16T12:13:23.4530784Z checking for library containing opendir... none required
2020-02-16T12:13:23.4530850Z checking for ANSI C header files... (cached) yes
2020-02-16T12:13:23.4531107Z checking fcntl.h usability... yes
---
2020-02-16T12:13:23.4532626Z checking for sys/param.h... yes
2020-02-16T12:13:23.4532893Z checking sys/mount.h usability... yes
2020-02-16T12:13:23.4532971Z checking sys/mount.h presence... yes
2020-02-16T12:13:23.4533032Z checking for sys/mount.h... yes
2020-02-16T12:13:23.4533113Z checking for stdint types... stdint.h (shortcircuit)
2020-02-16T12:13:23.4533186Z make use of stdint.h in src/fcstdint.h (assuming C99 compatible system)
2020-02-16T12:13:23.4533559Z checking for inline... inline
2020-02-16T12:13:23.4533635Z checking for flexible array members... yes
2020-02-16T12:13:23.4533714Z checking for pid_t... yes
2020-02-16T12:13:23.4533961Z checking for vprintf... yes
---
2020-02-16T12:13:23.4534482Z checking for getpagesize... yes
2020-02-16T12:13:23.4534542Z checking for working mmap... yes
2020-02-16T12:13:23.4534618Z checking for link... yes
2020-02-16T12:13:23.4534675Z checking for mkstemp... yes
2020-02-16T12:13:23.4534749Z checking for mkostemp... yes
2020-02-16T12:13:23.4534807Z checking for _mktemp_s... no
2020-02-16T12:13:23.4534938Z checking for getopt... yes
2020-02-16T12:13:23.4535010Z checking for getopt_long... yes
2020-02-16T12:13:23.4535084Z checking for getprogname... no
2020-02-16T12:13:23.4535142Z checking for getexecname... no
---
2020-02-16T12:13:23.4536111Z checking for fstatfs... yes
2020-02-16T12:13:23.4536167Z checking for lstat... yes
2020-02-16T12:13:23.4536243Z checking for posix_fadvise in fcntl.h... fcntl.h
2020-02-16T12:13:23.4536304Z checking for scandir... yes
2020-02-16T12:13:23.4536378Z checking for struct statvfs.f_basetype... no
2020-02-16T12:13:23.4536441Z checking for struct statvfs.f_fstypename... no
2020-02-16T12:13:23.4536519Z checking for struct statfs.f_flags... yes
2020-02-16T12:13:23.4536587Z checking for struct statfs.f_fstypename... no
2020-02-16T12:13:23.4536662Z checking for struct dirent.d_type... yes
2020-02-16T12:13:23.4536720Z checking for FREETYPE... yes
2020-02-16T12:13:23.4536793Z checking for FT_Get_Next_Char... yes
2020-02-16T12:13:23.4536940Z checking for FT_Get_BDF_Property... yes
2020-02-16T12:13:23.4537032Z checking for FT_Get_PS_Font_Info... yes
2020-02-16T12:13:23.4537108Z checking for FT_Has_PS_Glyph_Names... yes
2020-02-16T12:13:23.4537168Z checking for FT_Get_X11_Font_Format... yes
2020-02-16T12:13:23.4537243Z checking for FT_Select_Size... yes
2020-02-16T12:13:23.4537301Z checking for FT_Bitmap_Size.y_ppem... yes
2020-02-16T12:13:23.4537431Z checking expat.h usability... yes
2020-02-16T12:13:23.4537507Z checking expat.h presence... yes
2020-02-16T12:13:23.4537564Z checking for expat.h... yes
2020-02-16T12:13:23.4537564Z checking for expat.h... yes
2020-02-16T12:13:23.4537639Z checking for XML_SetDoctypeDeclHandler... yes
2020-02-16T12:13:23.4537700Z checking for Intel atomic primitives... true
2020-02-16T12:13:23.4537846Z checking for Solaris atomic operations... false
2020-02-16T12:13:23.4538137Z checking if compiler needs -Werror to reject unknown flags... no
2020-02-16T12:13:23.4538368Z checking for the pthreads library -lpthreads... no
2020-02-16T12:13:23.4538461Z checking whether pthreads work without any flags... no
2020-02-16T12:13:23.4538788Z checking whether pthreads work with -Kthread... no
2020-02-16T12:13:23.4539211Z checking whether pthreads work with -kthread... no
2020-02-16T12:13:23.4539617Z checking for the pthreads library -llthread... no
2020-02-16T12:13:23.4540029Z checking whether pthreads work with -pthread... yes
2020-02-16T12:13:23.4540104Z checking for joinable pthread attribute... PTHREAD_CREATE_JOINABLE
2020-02-16T12:13:23.4540195Z checking if more special flags are required for pthreads... no
2020-02-16T12:13:23.4540264Z checking for PTHREAD_PRIO_INHERIT... yes
2020-02-16T12:13:23.4540341Z checking for docbook2html... no
2020-02-16T12:13:23.4540428Z checking whether byte ordering is bigendian... no
2020-02-16T12:13:23.4540491Z checking size of void *... 8
2020-02-16T12:13:23.4540566Z checking alignment of double... 8
2020-02-16T12:13:23.4540632Z checking that generated files are newer than configure... done
2020-02-16T12:13:23.4540784Z config.status: creating Makefile
2020-02-16T12:13:23.4540860Z config.status: creating fontconfig/Makefile
2020-02-16T12:13:23.4541085Z config.status: creating fc-lang/Makefile
2020-02-16T12:13:23.4541085Z config.status: creating fc-lang/Makefile
2020-02-16T12:13:23.4541321Z config.status: creating fc-glyphname/Makefile
2020-02-16T12:13:23.4541534Z config.status: creating fc-case/Makefile
2020-02-16T12:13:23.4541614Z config.status: creating src/Makefile
2020-02-16T12:13:23.4541690Z config.status: creating conf.d/Makefile
2020-02-16T12:13:23.4541902Z config.status: creating fc-cache/Makefile
2020-02-16T12:13:23.4542290Z config.status: creating fc-cat/Makefile
2020-02-16T12:13:23.4542679Z config.status: creating fc-list/Makefile
2020-02-16T12:13:23.4542916Z config.status: creating fc-match/Makefile
2020-02-16T12:13:23.4543132Z config.status: creating fc-pattern/Makefile
2020-02-16T12:13:23.4543360Z config.status: creating fc-query/Makefile
2020-02-16T12:13:23.4543576Z config.status: creating fc-scan/Makefile
2020-02-16T12:13:23.4543811Z config.status: creating fc-validate/Makefile
2020-02-16T12:13:23.4544132Z config.status: creating doc/version.sgml
2020-02-16T12:13:23.4544211Z config.status: creating test/Makefile
2020-02-16T12:13:23.4544272Z config.status: creating fontconfig.spec
2020-02-16T12:13:23.4544350Z config.status: creating fontconfig.pc
2020-02-16T12:13:23.4544350Z config.status: creating fontconfig.pc
2020-02-16T12:13:23.4544567Z config.status: creating fontconfig-zip
2020-02-16T12:13:23.4544649Z config.status: creating config.h
2020-02-16T12:13:23.4544981Z config.status: executing depfiles commands
2020-02-16T12:13:23.4545065Z config.status: executing libtool commands
2020-02-16T12:13:23.4545134Z config.status: executing src/fcstdint.h commands
2020-02-16T12:13:23.4545237Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-02-16T12:13:23.4545559Z cd /checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out && make -j2
2020-02-16T12:13:23.4546236Z make[2]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out'
2020-02-16T12:13:23.4547057Z make[3]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out'
2020-02-16T12:13:23.4547177Z Making all in fontconfig
2020-02-16T12:13:23.4547538Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fontconfig'
2020-02-16T12:13:23.4547824Z make[4]: Nothing to be done for 'all'.
2020-02-16T12:13:23.4547824Z make[4]: Nothing to be done for 'all'.
2020-02-16T12:13:23.4548505Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fontconfig'
2020-02-16T12:13:23.4549176Z Making all in fc-case
2020-02-16T12:13:23.4549685Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-case'
2020-02-16T12:13:23.4549782Z   GEN      fcalias.h
2020-02-16T12:13:23.4549855Z   GEN      fcaliastail.h
2020-02-16T12:13:23.4550365Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-case'
2020-02-16T12:13:23.4550767Z make[5]: Nothing to be done for 'all-am'.
2020-02-16T12:13:23.4551084Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-case'
2020-02-16T12:13:23.4551423Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-case'
2020-02-16T12:13:23.4551423Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-case'
2020-02-16T12:13:23.4551660Z Making all in fc-lang
2020-02-16T12:13:23.4552184Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-lang'
2020-02-16T12:13:23.4552272Z   GEN      fcalias.h
2020-02-16T12:13:23.4552346Z   GEN      fcaliastail.h
2020-02-16T12:13:23.4552721Z make  all-am
2020-02-16T12:13:23.4553053Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-lang'
2020-02-16T12:13:23.4553142Z   GEN      fclang.h
2020-02-16T12:13:23.4553717Z make[6]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-lang'
2020-02-16T12:13:23.4553963Z   GEN      fc-lang
2020-02-16T12:13:23.4554290Z make[6]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-lang'
2020-02-16T12:13:23.4554685Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-lang'
2020-02-16T12:13:23.4555074Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-lang'
2020-02-16T12:13:23.4555348Z Making all in fc-glyphname
2020-02-16T12:13:23.4555878Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-glyphname'
2020-02-16T12:13:23.4555972Z   GEN      fcaliastail.h
2020-02-16T12:13:23.4556049Z   GEN      fcalias.h
2020-02-16T12:13:23.4556241Z make  all-am
2020-02-16T12:13:23.4556578Z make[5]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-glyphname'
2020-02-16T12:13:23.4557010Z make[5]: Nothing to be done for 'all-am'.
2020-02-16T12:13:23.4557364Z make[5]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-glyphname'
2020-02-16T12:13:23.4557915Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/fc-glyphname'
2020-02-16T12:13:23.4558346Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/src'
2020-02-16T12:13:23.4558346Z make[4]: Entering directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/src'
2020-02-16T12:13:23.4558511Z   GEN      fcftalias.h
2020-02-16T12:13:23.4558596Z   GEN      fcalias.h
2020-02-16T12:13:23.4559005Z   GEN      stamp-fcstdint
2020-02-16T12:13:23.4559094Z config.status: executing src/fcstdint.h commands
2020-02-16T12:13:23.4559171Z config.status: creating src/fcstdint.h : _FONTCONFIG_SRC_FCSTDINT_H
2020-02-16T12:13:23.4559263Z config.status: src/fcstdint.h is unchanged
2020-02-16T12:13:23.4559499Z   GEN      fcobjshash.gperf
2020-02-16T12:13:23.4559761Z   GEN      fcobjshash.h
2020-02-16T12:13:23.4560025Z Makefile:882: recipe for target 'fcobjshash.h' failed
2020-02-16T12:13:23.4560358Z make[4]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/src'
2020-02-16T12:13:23.4560910Z Makefile:562: recipe for target 'all-recursive' failed
2020-02-16T12:13:23.4561230Z make[3]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out'
2020-02-16T12:13:23.4561505Z Makefile:445: recipe for target 'all' failed
2020-02-16T12:13:23.4561991Z make[2]: Leaving directory '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out'
2020-02-16T12:13:23.4562370Z makefile.cargo:61: recipe for target '/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/libfontconfig.a' failed
2020-02-16T12:13:23.4562700Z make[1]: Leaving directory '/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4'
2020-02-16T12:13:23.4562953Z --- stderr
2020-02-16T12:13:23.4563186Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-02-16T12:13:23.4563186Z make[2]: warning: -jN forced in submake: disabling jobserver mode.
2020-02-16T12:13:23.4563516Z /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
2020-02-16T12:13:23.4563765Z WARNING: 'gperf' is missing on your system.
2020-02-16T12:13:23.4563857Z          You might have modified some files without having the proper
2020-02-16T12:13:23.4564132Z          tools for further handling them.  Check the 'README' file, it
2020-02-16T12:13:23.4564235Z          often tells you about the needed prerequisites for installing
2020-02-16T12:13:23.4564317Z          this package.  You may also peek at any GNU archive site, in
2020-02-16T12:13:23.4564608Z          case some other package contains this missing 'gperf' program.
2020-02-16T12:13:23.4564704Z make[4]: *** [fcobjshash.h] Error 1
2020-02-16T12:13:23.4564771Z make[4]: *** Waiting for unfinished jobs....
2020-02-16T12:13:23.4565016Z make[3]: *** [all-recursive] Error 1
2020-02-16T12:13:23.4565083Z make[2]: *** [all] Error 2
2020-02-16T12:13:23.4565430Z make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-56d358c3df5cd366/out/libfontconfig.a] Error 2
2020-02-16T12:13:23.4566082Z thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
2020-02-16T12:13:23.4566374Z                          env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
2020-02-16T12:13:23.4566847Z                                                                                  "-f",
2020-02-16T12:13:23.4567287Z                                                                                  "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
2020-02-16T12:13:23.4567489Z 
2020-02-16T12:13:23.4567738Z warning: build failed, waiting for other jobs to finish...
2020-02-16T12:13:25.4739772Z warning: use of deprecated item 'try': use the `?` operator instead
2020-02-16T12:13:25.4740063Z    --> webrender/src/batch.rs:342:1
---
2020-02-16T12:15:06.4356093Z 
2020-02-16T12:15:06.4356226Z 
2020-02-16T12:15:06.4361108Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2020-02-16T12:15:06.4361406Z Build completed unsuccessfully in 1:52:44
2020-02-16T12:15:06.4406516Z Makefile:50: recipe for target 'check-aux' failed
2020-02-16T12:15:06.4409503Z make: *** [check-aux] Error 1
2020-02-16T12:15:06.4415777Z   local time: Sun Feb 16 12:15:06 UTC 2020
2020-02-16T12:15:07.0992158Z   network time: Sun, 16 Feb 2020 12:15:07 GMT
2020-02-16T12:15:07.0997697Z == end clock drift check ==
2020-02-16T12:15:15.3189805Z 
2020-02-16T12:15:15.3189805Z 
2020-02-16T12:15:15.3287842Z ##[error]Bash exited with code '2'.
2020-02-16T12:15:15.3326996Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-16T12:15:15.3328817Z ==============================================================================
2020-02-16T12:15:15.3328913Z Task         : Get sources
2020-02-16T12:15:15.3328983Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
