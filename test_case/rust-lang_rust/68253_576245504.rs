plain
2020-01-20T09:20:14.9776273Z Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
2020-01-20T09:20:14.9776372Z 
2020-01-20T09:20:14.9776989Z Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
2020-01-20T09:20:14.9777335Z  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
2020-01-20T09:20:15.4090043Z gpg: keyring `/tmp/tmppt2j8g__/secring.gpg' created
2020-01-20T09:20:15.4090851Z gpg: keyring `/tmp/tmppt2j8g__/pubring.gpg' created
2020-01-20T09:20:15.4091457Z gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
2020-01-20T09:20:15.8006775Z gpg: /tmp/tmppt2j8g__/trustdb.gpg: trustdb created
2020-01-20T09:20:15.8012524Z gpg: no ultimately trusted keys found
2020-01-20T09:20:15.8013350Z gpg: Total number processed: 1
2020-01-20T09:20:15.8013588Z gpg:               imported: 1  (RSA: 1)
2020-01-20T09:20:15.8595205Z OK
---
2020-01-20T09:20:47.8388508Z 100 2427k  100 2427k    0     0  2547k      0 --:--:-- --:--:-- --:--:-- 2549k
2020-01-20T09:20:47.8503253Z + cd crosstool-ng-crosstool-ng-1.24.0
2020-01-20T09:20:47.8503650Z + ./bootstrap
2020-01-20T09:20:47.8555828Z INFO  :: *** Generating package version descriptions
2020-01-20T09:20:48.3932996Z INFO  :: Master packages: android-ndk autoconf automake avr-libc binutils bison cloog dtc duma elf2flt expat gcc gdb gettext glibc-ports glibc gmp isl libelf libiconv libtool linux ltrace m4 make mingw-w64 moxiebox mpc mpfr musl ncurses newlib strace uClibc zlib
2020-01-20T09:20:48.3940145Z INFO  :: Generating 'config/versions/android-ndk.in'
2020-01-20T09:20:49.3630186Z INFO  :: Generating 'config/versions/automake.in'
2020-01-20T09:20:49.7149278Z INFO  :: Generating 'config/versions/avr-libc.in'
2020-01-20T09:20:49.9013937Z INFO  :: Generating 'config/versions/binutils.in'
2020-01-20T09:20:52.8216486Z INFO  :: Generating 'config/versions/bison.in'
---
2020-01-20T09:22:23.8814913Z checking lex output file root... lex.yy
2020-01-20T09:22:24.4114350Z checking lex library... -lfl
2020-01-20T09:22:24.6807035Z checking whether yytext is a pointer... yes
2020-01-20T09:22:24.6822595Z checking for bison... bison -y
2020-01-20T09:22:24.7346779Z checking whether g++ supports C++11 features with -std=gnu++11... yes
2020-01-20T09:22:24.8046180Z checking if gcc can static link... yes
2020-01-20T09:22:24.8066382Z checking for gobjcopy... no
2020-01-20T09:22:24.8072802Z checking for objcopy... objcopy
2020-01-20T09:22:24.8114181Z checking for absolute path to objcopy... /usr/bin/objcopy
2020-01-20T09:22:24.8122731Z checking for gobjdump... no
---
2020-01-20T09:22:26.1136849Z checking for ld used by gcc... /usr/bin/ld
2020-01-20T09:22:26.1162098Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-01-20T09:22:26.1192209Z checking for shared library run path origin... done
2020-01-20T09:22:26.1361157Z checking whether NLS is requested... yes
2020-01-20T09:22:26.1547442Z checking for CFPreferencesCopyAppValue... no
2020-01-20T09:22:26.1764126Z checking for CFLocaleCopyCurrent... no
2020-01-20T09:22:26.2287916Z checking whether to use NLS... yes
2020-01-20T09:22:26.2293132Z checking where the gettext function comes from... libc
2020-01-20T09:22:26.2315110Z checking for pkg-config... /usr/bin/pkg-config
2020-01-20T09:22:26.2335890Z checking pkg-config is at least version 0.9.0... yes
---
2020-01-20T09:22:26.8893655Z /usr/bin/make  all-recursive
2020-01-20T09:22:26.9000766Z make[1]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0'
2020-01-20T09:22:26.9041664Z Making all in kconfig
2020-01-20T09:22:26.9080126Z make[2]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-20T09:22:26.9080430Z bison -y -l -b zconf -p zconf  -ozconf.c zconf.y
2020-01-20T09:22:26.9084781Z flex -L -Pzconf  -ozconf.lex.c zconf.l
2020-01-20T09:22:26.9801892Z make[3]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-20T09:22:26.9802407Z depbase=`echo conf.o | sed 's|[^/]*$|.deps/&|;s|\.o$||'`;\
2020-01-20T09:22:26.9802842Z gcc -DHAVE_CONFIG_H -I. -I..  -include config.h -DCONFIG_=\"CT_\"   -g -O2 -MT conf.o -MD -MP -MF $depbase.Tpo -c -o conf.o conf.c &&\
2020-01-20T09:22:26.9803135Z mv -f $depbase.Tpo $depbase.Po
---
2020-01-20T09:22:30.2097047Z libtool: link: gcc -g -O2 -o mconf mconf.o zconf.o lxdialog/checklist.o lxdialog/inputbox.o lxdialog/menubox.o lxdialog/textbox.o lxdialog/util.o lxdialog/yesno.o  -lncurses
2020-01-20T09:22:30.2445014Z make[3]: Leaving directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-20T09:22:30.2445405Z make[2]: Leaving directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-20T09:22:30.2535351Z make[2]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0'
2020-01-20T09:22:30.2536187Z ( /bin/sed -e 's,[@]docdir[@],/usr/local/share/doc/crosstool-ng,g' -e 's,[@]pkgdatadir[@],/usr/local/share/crosstool-ng,g' -e 's,[@]pkglibexecdir[@],/usr/local/libexec/crosstool-ng,g' -e 's,[@]progname[@],'`echo ct-ng | sed 's,x,x,'`',g' | /bin/bash config.status --file=- ) < ct-ng.in >ct-ng-t && chmod a-w,a+x ct-ng-t && mv -f ct-ng-t ct-ng
2020-01-20T09:22:30.2537068Z /bin/mkdir -p bash-completion && ( /bin/sed -e 's,[@]docdir[@],/usr/local/share/doc/crosstool-ng,g' -e 's,[@]pkgdatadir[@],/usr/local/share/crosstool-ng,g' -e 's,[@]pkglibexecdir[@],/usr/local/libexec/crosstool-ng,g' -e 's,[@]progname[@],'`echo ct-ng | sed 's,x,x,'`',g' | /bin/bash config.status --file=- ) < bash-completion/ct-ng.in >bash-completion/ct-ng-t && mv -f bash-completion/ct-ng-t bash-completion/ct-ng
2020-01-20T09:22:30.3373362Z /bin/mkdir -p docs && ( /bin/sed -e 's,[@]docdir[@],/usr/local/share/doc/crosstool-ng,g' -e 's,[@]pkgdatadir[@],/usr/local/share/crosstool-ng,g' -e 's,[@]pkglibexecdir[@],/usr/local/libexec/crosstool-ng,g' -e 's,[@]progname[@],'`echo ct-ng | sed 's,x,x,'`',g' | /bin/bash config.status --file=- ) < docs/ct-ng.1.in >docs/ct-ng.1-t && mv -f docs/ct-ng.1-t docs/ct-ng.1
2020-01-20T09:22:30.4216963Z make[1]: Leaving directory '/build/crosstool-ng-crosstool-ng-1.24.0'
2020-01-20T09:22:30.4226151Z + make install
2020-01-20T09:22:30.4356417Z Making install in kconfig
2020-01-20T09:22:30.4414658Z make[1]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
---
2020-01-20T09:22:30.6901692Z  /usr/bin/install -c -m 644  packages/glibc/2.14.1/0000-respect-env-CPPFLAGS.patch packages/glibc/2.14.1/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.14.1/0002-fix-signed-shift-overlow.patch packages/glibc/2.14.1/0003-dl-openat64-variadic.patch packages/glibc/2.14.1/0004-unused-variables.patch packages/glibc/2.14.1/0005-misleading-indentation.patch packages/glibc/2.14.1/0006-dl-open-array-bounds.patch packages/glibc/2.14.1/0007-i386-x86_64-revert-clone-cfi.patch packages/glibc/2.14.1/0008-disable-ldconfig.patch packages/glibc/2.14.1/0009-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.14.1/0010-queue-header-updates.patch packages/glibc/2.14.1/0011-manual-no-perl.patch packages/glibc/2.14.1/0012-localedef-fix-trampoline.patch packages/glibc/2.14.1/0013-resolv-dynamic.patch packages/glibc/2.14.1/0014-localedef-mmap.patch packages/glibc/2.14.1/0015-fadvise64_64.patch packages/glibc/2.14.1/0016-section-comments.patch packages/glibc/2.14.1/0017-no-inline-gmon.patch packages/glibc/2.14.1/0018-assume-pipe2.patch packages/glibc/2.14.1/0019-china.patch packages/glibc/2.14.1/0020-new-valencian-locale.patch packages/glibc/2.14.1/0021-macos-cross-rpcgen.patch packages/glibc/2.14.1/0022-nscd-one-fork.patch packages/glibc/2.14.1/0023-hppa-nptl-carlos.patch packages/glibc/2.14.1/0024-dl_execstack-PaX-support.patch packages/glibc/2.14.1/0025-pre20040117-pt_pax.patch packages/glibc/2.14.1/0026-tests-sandbox-libdl-paths.patch packages/glibc/2.14.1/0027-dont-build-timezone.patch packages/glibc/2.14.1/0028-alpha-xstat.patch packages/glibc/2.14.1/0029-alpha-creat.patch packages/glibc/2.14.1/0030-alpha_alpha-add-fdatasync-support.patch packages/glibc/2.14.1/0031-ppc-atomic.patch packages/glibc/2.14.1/0032-mips_shn_undef-hack.patch packages/glibc/2.14.1/0033-alpha-atfcts.patch packages/glibc/2.14.1/0034-syslog.patch packages/glibc/2.14.1/0035-debug-readlink_chk-readklinkat_chk.patch packages/glibc/2.14.1/0036-cpuid-include.patch packages/glibc/2.14.1/0037-asm-i686.patch packages/glibc/2.14.1/0038-fix-rpc_parse-format.patch packages/glibc/2.14.1/0039-nis-bogus-conditional.patch '/usr/local/share/crosstool-ng/packages/glibc/2.14.1'
2020-01-20T09:22:30.6964947Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-20T09:22:30.6982505Z  /usr/bin/install -c -m 644  packages/glibc/2.15/0000-respect-env-CPPFLAGS.patch packages/glibc/2.15/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.15/0002-fix-signed-shift-overlow.patch packages/glibc/2.15/0003-dl-openat64-variadic.patch packages/glibc/2.15/0004-unused-variables.patch packages/glibc/2.15/0005-misleading-indentation.patch packages/glibc/2.15/0006-dl-open-array-bounds.patch packages/glibc/2.15/0007-i386-x86_64-revert-clone-cfi.patch packages/glibc/2.15/0008-disable-ldconfig.patch packages/glibc/2.15/0009-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.15/0010-queue-header-updates.patch packages/glibc/2.15/0011-manual-no-perl.patch packages/glibc/2.15/0012-localedef-fix-trampoline.patch packages/glibc/2.15/0013-resolv-dynamic.patch packages/glibc/2.15/0014-fadvise64_64.patch packages/glibc/2.15/0015-section-comments.patch packages/glibc/2.15/0016-no-inline-gmon.patch packages/glibc/2.15/0017-assume-pipe2.patch packages/glibc/2.15/0018-china.patch packages/glibc/2.15/0019-new-valencian-locale.patch packages/glibc/2.15/0020-macos-cross-rpcgen.patch packages/glibc/2.15/0021-nscd-one-fork.patch packages/glibc/2.15/0022-hppa-nptl-carlos.patch packages/glibc/2.15/0023-dl_execstack-PaX-support.patch packages/glibc/2.15/0024-pre20040117-pt_pax.patch packages/glibc/2.15/0025-tests-sandbox-libdl-paths.patch packages/glibc/2.15/0026-dont-build-timezone.patch packages/glibc/2.15/0027-alpha-xstat.patch packages/glibc/2.15/0028-alpha-creat.patch packages/glibc/2.15/0029-alpha_alpha-add-fdatasync-support.patch packages/glibc/2.15/0030-ppc-atomic.patch packages/glibc/2.15/0031-mips_shn_undef-hack.patch packages/glibc/2.15/0032-alpha-atfcts.patch packages/glibc/2.15/0033-syslog.patch packages/glibc/2.15/0034-debug-readlink_chk-readklinkat_chk.patch packages/glibc/2.15/0035-cpuid-include.patch packages/glibc/2.15/0036-asm-i686.patch packages/glibc/2.15/0037-fix-rpc_parse-format.patch packages/glibc/2.15/0038-nis-bogus-conditional.patch packages/glibc/2.15/0039-try-link-static.patch '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-20T09:22:30.7050639Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.16.0'
2020-01-20T09:22:30.7068219Z  /usr/bin/install -c -m 644  packages/glibc/2.16.0/0000-respect-env-CPPFLAGS.patch packages/glibc/2.16.0/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.16.0/0002-fix-signed-shift-overlow.patch packages/glibc/2.16.0/0003-dl-openat64-variadic.patch packages/glibc/2.16.0/0004-unused-variables.patch packages/glibc/2.16.0/0005-misleading-indentation.patch packages/glibc/2.16.0/0006-dl-open-array-bounds.patch packages/glibc/2.16.0/0007-i386-x86_64-revert-clone-cfi.patch packages/glibc/2.16.0/0008-disable-ldconfig.patch packages/glibc/2.16.0/0009-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.16.0/0010-queue-header-updates.patch packages/glibc/2.16.0/0011-localedef-fix-trampoline.patch packages/glibc/2.16.0/0012-resolv-dynamic.patch packages/glibc/2.16.0/0013-fadvise64_64.patch packages/glibc/2.16.0/0014-assume-pipe2.patch packages/glibc/2.16.0/0015-china.patch packages/glibc/2.16.0/0016-new-valencian-locale.patch packages/glibc/2.16.0/0017-macos-cross-rpcgen.patch packages/glibc/2.16.0/0018-nscd-one-fork.patch packages/glibc/2.16.0/0019-hppa-nptl-carlos.patch packages/glibc/2.16.0/0020-dl_execstack-PaX-support.patch packages/glibc/2.16.0/0021-pre20040117-pt_pax.patch packages/glibc/2.16.0/0022-tests-sandbox-libdl-paths.patch packages/glibc/2.16.0/0023-dont-build-timezone.patch packages/glibc/2.16.0/0024-alpha-xstat.patch packages/glibc/2.16.0/0025-alpha-creat.patch packages/glibc/2.16.0/0026-alpha_alpha-add-fdatasync-support.patch packages/glibc/2.16.0/0027-fix-parsing-of-numeric-hosts-in-gethostbyname_r.patch packages/glibc/2.16.0/0028-ppc-atomic.patch packages/glibc/2.16.0/0029-mips_shn_undef-hack.patch packages/glibc/2.16.0/0030-alpha-atfcts.patch packages/glibc/2.16.0/0031-syslog.patch packages/glibc/2.16.0/0032-debug-readlink_chk-readklinkat_chk.patch packages/glibc/2.16.0/0033-fix-rpc_parse-format.patch packages/glibc/2.16.0/0034-nis-bogus-conditional.patch packages/glibc/2.16.0/0035-obstack-common.patch packages/glibc/2.16.0/0036-new-tools.patch packages/glibc/2.16.0/0037-strftime-multiple-stmts.patch packages/glibc/2.16.0/0038-if_nametoindex-size-check.patch packages/glibc/2.16.0/0039-utmp-nonstring.patch '/usr/local/share/crosstool-ng/packages/glibc/2.16.0'
2020-01-20T09:22:30.7108148Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/libiconv/1.14'
2020-01-20T09:22:30.7122660Z  /usr/bin/install -c -m 644  packages/libiconv/1.14/0000-srclib_stdio.in.h-remove-gets-declarations.patch packages/libiconv/1.14/chksum packages/libiconv/1.14/version.desc '/usr/local/share/crosstool-ng/packages/libiconv/1.14'
2020-01-20T09:22:30.7154691Z  /usr/bin/install -c -m 644  packages/libiconv/1.15/chksum packages/libiconv/1.15/version.desc '/usr/local/share/crosstool-ng/packages/libiconv/1.15'
2020-01-20T09:22:30.7172038Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/cloog/0.18.1'
2020-01-20T09:22:30.7187273Z  /usr/bin/install -c -m 644  packages/cloog/0.18.1/chksum packages/cloog/0.18.1/version.desc '/usr/local/share/crosstool-ng/packages/cloog/0.18.1'
2020-01-20T09:22:30.7203340Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc'
---
2020-01-20T09:22:30.7586243Z  /usr/bin/install -c -m 644  packages/mpfr/package.desc '/usr/local/share/crosstool-ng/packages/mpfr'
2020-01-20T09:22:30.7600468Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/avr-libc/2.0.0'
2020-01-20T09:22:30.7612770Z  /usr/bin/install -c -m 644  packages/avr-libc/2.0.0/chksum packages/avr-libc/2.0.0/version.desc '/usr/local/share/crosstool-ng/packages/avr-libc/2.0.0'
2020-01-20T09:22:30.7631613Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mpfr/2.4.2'
2020-01-20T09:22:30.7645166Z  /usr/bin/install -c -m 644  packages/mpfr/2.4.2/0000-sin_cos_underflow.patch packages/mpfr/2.4.2/0001-longlong.h.patch packages/mpfr/2.4.2/0002-gmp5.patch packages/mpfr/2.4.2/chksum packages/mpfr/2.4.2/version.desc '/usr/local/share/crosstool-ng/packages/mpfr/2.4.2'
2020-01-20T09:22:30.7675972Z  /usr/bin/install -c -m 644  packages/bison/3.0.5/chksum packages/bison/3.0.5/version.desc '/usr/local/share/crosstool-ng/packages/bison/3.0.5'
2020-01-20T09:22:30.7712981Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-linaro/6.4-2018.05'
2020-01-20T09:22:30.7728314Z  /usr/bin/install -c -m 644  packages/gcc-linaro/6.4-2018.05/0000-libtool-leave-framework-alone.patch packages/gcc-linaro/6.4-2018.05/0001-uclibc-conf.patch packages/gcc-linaro/6.4-2018.05/0002-missing-execinfo_h.patch packages/gcc-linaro/6.4-2018.05/0003-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc-linaro/6.4-2018.05/0004-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc-linaro/6.4-2018.05/0005-arm-softfloat-libgcc.patch packages/gcc-linaro/6.4-2018.05/0006-arm_unbreak_armv4t.patch packages/gcc-linaro/6.4-2018.05/0007-cilk-wchar.patch packages/gcc-linaro/6.4-2018.05/0008-fix-m68k-compile.patch packages/gcc-linaro/6.4-2018.05/0009-fix-m68k-uclinux.patch packages/gcc-linaro/6.4-2018.05/0010-libgcc-mkmap-symver-support-skip_underscore.patch packages/gcc-linaro/6.4-2018.05/0011-libgcc-config-bfin-use-the-generic-linker-version-in.patch packages/gcc-linaro/6.4-2018.05/0012-libgcc-fix-DWARF-compilation-with-FDPIC-targets.patch packages/gcc-linaro/6.4-2018.05/0013-bfin-define-REENTRANT.patch packages/gcc-linaro/6.4-2018.05/0014-libgfortran-missing-include.patch packages/gcc-linaro/6.4-2018.05/0015-nios2-bad-multilib-default.patch packages/gcc-linaro/6.4-2018.05/0016-libgcc-disable-split-stack-nothreads.patch packages/gcc-linaro/6.4-2018.05/0017-uclinux-enable-threads.patch packages/gcc-linaro/6.4-2018.05/0018-bionic-ndk.patch packages/gcc-linaro/6.4-2018.05/0019-bionic-errno.patch packages/gcc-linaro/6.4-2018.05/0020-crystax.patch packages/gcc-linaro/6.4-2018.05/0021-crystax.patch packages/gcc-linaro/6.4-2018.05/0022-crystax.patch packages/gcc-linaro/6.4-2018.05/0023-crystax.patch packages/gcc-linaro/6.4-2018.05/0024-crystax.patch packages/gcc-linaro/6.4-2018.05/0025-crystax.patch packages/gcc-linaro/6.4-2018.05/0026-crystax.patch packages/gcc-linaro/6.4-2018.05/0027-crystax.patch packages/gcc-linaro/6.4-2018.05/0028-isl-0.20.patch packages/gcc-linaro/6.4-2018.05/chksum packages/gcc-linaro/6.4-2018.05/version.desc '/usr/local/share/crosstool-ng/packages/gcc-linaro/6.4-2018.05'
2020-01-20T09:22:30.7762520Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/avr-libc/1.8.1'
---
2020-01-20T09:22:30.8949406Z  /usr/bin/install -c -m 644  samples/x86_64-multilib-linux-uclibc/crosstool.config samples/x86_64-multilib-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc'
2020-01-20T09:22:30.8968182Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/libc'
2020-01-20T09:22:30.9023318Z  /usr/bin/install -c -m 644  scripts/build/libc/avr-libc.sh scripts/build/libc/bionic.sh scripts/build/libc/glibc.sh scripts/build/libc/mingw-w64.sh scripts/build/libc/moxiebox.sh scripts/build/libc/musl.sh scripts/build/libc/newlib.sh scripts/build/libc/none.sh scripts/build/libc/uClibc.sh '/usr/local/share/crosstool-ng/scripts/build/libc'
2020-01-20T09:22:30.9060463Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/6.5.0'
2020-01-20T09:22:30.9079152Z  /usr/bin/install -c -m 644  packages/gcc/6.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/6.5.0/0001-uclibc-conf.patch packages/gcc/6.5.0/0002-missing-execinfo_h.patch packages/gcc/6.5.0/0003-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/6.5.0/0004-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/6.5.0/0005-arm-softfloat-libgcc.patch packages/gcc/6.5.0/0006-arm_unbreak_armv4t.patch packages/gcc/6.5.0/0007-ARM-PR-target-70473-Reduce-size-of-Cortex-A8-automat.patch packages/gcc/6.5.0/0008-cilk-wchar.patch packages/gcc/6.5.0/0009-fix-m68k-compile.patch packages/gcc/6.5.0/0010-fix-m68k-uclinux.patch packages/gcc/6.5.0/0011-libgcc-mkmap-symver-support-skip_underscore.patch packages/gcc/6.5.0/0012-libgcc-config-bfin-use-the-generic-linker-version-in.patch packages/gcc/6.5.0/0013-libgcc-fix-DWARF-compilation-with-FDPIC-targets.patch packages/gcc/6.5.0/0014-bfin-define-REENTRANT.patch packages/gcc/6.5.0/0015-libgfortran-missing-include.patch packages/gcc/6.5.0/0016-nios2-bad-multilib-default.patch packages/gcc/6.5.0/0017-libgcc-disable-split-stack-nothreads.patch packages/gcc/6.5.0/0018-uclinux-enable-threads.patch packages/gcc/6.5.0/0019-bionic-ndk.patch packages/gcc/6.5.0/0020-bionic-errno.patch packages/gcc/6.5.0/0021-crystax.patch packages/gcc/6.5.0/0022-crystax.patch packages/gcc/6.5.0/0023-crystax.patch packages/gcc/6.5.0/0024-crystax.patch packages/gcc/6.5.0/0025-crystax.patch packages/gcc/6.5.0/0026-crystax.patch packages/gcc/6.5.0/0027-crystax.patch packages/gcc/6.5.0/0028-crystax.patch packages/gcc/6.5.0/0029-msp430-fix.patch packages/gcc/6.5.0/0030-isl-0.20.patch packages/gcc/6.5.0/chksum packages/gcc/6.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/6.5.0'
2020-01-20T09:22:30.9129916Z  /usr/bin/install -c -m 644  samples/arm-unknown-eabi/crosstool.config samples/arm-unknown-eabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-eabi'
2020-01-20T09:22:30.9170492Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/automake/1.11.6'
2020-01-20T09:22:30.9171975Z  /usr/bin/install -c -m 644  packages/automake/1.11.6/0000-escape-left-brace.patch packages/automake/1.11.6/chksum packages/automake/1.11.6/version.desc '/usr/local/share/crosstool-ng/packages/automake/1.11.6'
2020-01-20T09:22:30.9190785Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.11.1'
---
2020-01-20T09:22:30.9295827Z  /usr/bin/install -c -m 644  samples/x86_64-centos6-linux-gnu/crosstool.config samples/x86_64-centos6-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-centos6-linux-gnu'
2020-01-20T09:22:30.9314825Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-uclibcgnueabihf'
2020-01-20T09:22:30.9327854Z  /usr/bin/install -c -m 644  samples/arm-unknown-linux-uclibcgnueabihf/crosstool.config samples/arm-unknown-linux-uclibcgnueabihf/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-uclibcgnueabihf'
2020-01-20T09:22:30.9364749Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc-ports/2.13'
2020-01-20T09:22:30.9366402Z  /usr/bin/install -c -m 644  packages/glibc-ports/2.13/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc-ports/2.13/0001-m68k-sys-user.patch packages/glibc-ports/2.13/0002-alpha-SETPIPE-GETPIPE.patch packages/glibc-ports/2.13/0003-alpha-statfs.patch packages/glibc-ports/2.13/0004-alpha-cache-shape.patch packages/glibc-ports/2.13/0005-alpha-DEFAULT_STACK_PERMS.patch packages/glibc-ports/2.13/0006-alpha-fix-gcc-4.1-warnings.patch packages/glibc-ports/2.13/0007-alpha-feupdateenv.patch packages/glibc-ports/2.13/0008-alpha-fix-rtld-fPIC.patch packages/glibc-ports/2.13/0009-arm-cirrus-ep93xx-maverick-crunch-fpu.patch packages/glibc-ports/2.13/0010-nptl-lowlevellock.patch packages/glibc-ports/2.13/0011-fpu-cw-mips.patch packages/glibc-ports/2.13/0012-support-hard-float-eabi.patch packages/glibc-ports/2.13/chksum packages/glibc-ports/2.13/version.desc '/usr/local/share/crosstool-ng/packages/glibc-ports/2.13'
2020-01-20T09:22:30.9427326Z  /usr/bin/install -c -m 644  packages/glibc-ports/2.15/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc-ports/2.15/0001-libmemusage-link-failure.patch packages/glibc-ports/2.15/0002-m68k-sys-user.patch packages/glibc-ports/2.15/0003-alpha-cache-shape.patch packages/glibc-ports/2.15/0004-alpha-fix-gcc-4.1-warnings.patch packages/glibc-ports/2.15/0005-alpha-fix-rtld-fPIC.patch packages/glibc-ports/2.15/0006-arm-cirrus-ep93xx-maverick-crunch-fpu.patch packages/glibc-ports/2.15/0007-nptl-lowlevellock.patch packages/glibc-ports/2.15/0008-fpu-cw-mips.patch packages/glibc-ports/2.15/chksum packages/glibc-ports/2.15/version.desc '/usr/local/share/crosstool-ng/packages/glibc-ports/2.15'
2020-01-20T09:22:30.9430909Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mips-malta-linux-gnu'
2020-01-20T09:22:30.9444488Z  /usr/bin/install -c -m 644  samples/mips-malta-linux-gnu/crosstool.config samples/mips-malta-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mips-malta-linux-gnu'
2020-01-20T09:22:30.9489650Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-linaro/4.8-2015.06'
---
2020-01-20T09:22:30.9624462Z  /usr/bin/install -c -m 644  packages/glibc/2.13/0040-nis-bogus-conditional.patch packages/glibc/2.13/0041-initfini-ppc64.patch packages/glibc/2.13/0042-obstack-common.patch packages/glibc/2.13/0043-new-tools.patch packages/glibc/2.13/0044-strftime-multiple-stmts.patch packages/glibc/2.13/0045-if_nametoindex-size-check.patch packages/glibc/2.13/0046-utmp-nonstring.patch packages/glibc/2.13/0047-getlogin_r-use-strnlen.patch packages/glibc/2.13/0048-zic.c-use-memcpy.patch packages/glibc/2.13/chksum packages/glibc/2.13/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.13'
2020-01-20T09:22:30.9654373Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.32'
2020-01-20T09:22:30.9670968Z  /usr/bin/install -c -m 644  packages/binutils/2.32/0000-sh-conf.patch packages/binutils/2.32/0001-ld_makefile_patch.patch packages/binutils/2.32/0002-check_ldrunpath_length.patch packages/binutils/2.32/0003-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.32/0004-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.32/0005-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.32/0006-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.32/0007-sysroot.patch packages/binutils/2.32/0008-poison-system-directories.patch packages/binutils/2.32/chksum packages/binutils/2.32/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.32'
2020-01-20T09:22:30.9707240Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-20T09:22:30.9714071Z  /usr/bin/install -c -m 644  packages/glibc/2.15/0040-builtin_expect.patch packages/glibc/2.15/0041-gcc_s-suffix.patch packages/glibc/2.15/0042-obsolete-rpc.patch packages/glibc/2.15/0043-obstack-common.patch packages/glibc/2.15/0044-new-tools.patch packages/glibc/2.15/0045-strftime-multiple-stmts.patch packages/glibc/2.15/0046-if_nametoindex-size-check.patch packages/glibc/2.15/0047-utmp-nonstring.patch packages/glibc/2.15/0048-getlogin_r-use-strnlen.patch packages/glibc/2.15/0049-zic.c-use-memcpy.patch packages/glibc/2.15/0050-fdivp-order.patch packages/glibc/2.15/chksum packages/glibc/2.15/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-20T09:22:30.9750729Z  /usr/bin/install -c -m 644  packages/linux/3.9.11/chksum packages/linux/3.9.11/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.9.11'
2020-01-20T09:22:30.9779002Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.17'
2020-01-20T09:22:30.9794274Z  /usr/bin/install -c -m 644  packages/glibc/2.17/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc/2.17/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.17/0002-fix-signed-shift-overlow.patch packages/glibc/2.17/0003-dl-openat64-variadic.patch packages/glibc/2.17/0004-unused-variables.patch packages/glibc/2.17/0005-misleading-indentation.patch packages/glibc/2.17/0006-dl-open-array-bounds.patch packages/glibc/2.17/0007-support-make4.patch packages/glibc/2.17/0008-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.17/0009-macos-cross-rpcgen.patch packages/glibc/2.17/0010-fix-rpc_parse-format.patch packages/glibc/2.17/0011-nis-bogus-conditional.patch packages/glibc/2.17/0012-obstack-common.patch packages/glibc/2.17/0013-strftime-multiple-stmts.patch packages/glibc/2.17/0014-if_nametoindex-size-check.patch packages/glibc/2.17/0015-utmp-nonstring.patch packages/glibc/2.17/0016-getlogin_r-use-strnlen.patch packages/glibc/2.17/0017-zic.c-use-memcpy.patch packages/glibc/2.17/chksum packages/glibc/2.17/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.17'
2020-01-20T09:22:30.9836254Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.18'
2020-01-20T09:22:30.9836254Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.18'
2020-01-20T09:22:30.9850116Z  /usr/bin/install -c -m 644  packages/glibc/2.18/0000-4f2bcda-ARM-Fix-clone-code-when-built-for-Thumb.patch packages/glibc/2.18/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.18/0002-fix-signed-shift-overlow.patch packages/glibc/2.18/0003-dl-openat64-variadic.patch packages/glibc/2.18/0004-unused-variables.patch packages/glibc/2.18/0005-misleading-indentation.patch packages/glibc/2.18/0006-dl-open-array-bounds.patch packages/glibc/2.18/0007-2770d15-Fix-PI-mutex-check-in-pthread_cond_broadcast-and-pthread_cond_signal.patch packages/glibc/2.18/0008-support-make4.patch packages/glibc/2.18/0009-arm-unwind.patch packages/glibc/2.18/0010-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.18/0011-macos-cross-rpcgen.patch packages/glibc/2.18/0012-fix-rpc_parse-format.patch packages/glibc/2.18/0013-nis-bogus-conditional.patch packages/glibc/2.18/0014-strftime-multiple-stmts.patch packages/glibc/2.18/0015-if_nametoindex-size-check.patch packages/glibc/2.18/0016-utmp-nonstring.patch packages/glibc/2.18/0017-getlogin_r-use-strnlen.patch packages/glibc/2.18/0018-zic.c-use-memcpy.patch packages/glibc/2.18/chksum packages/glibc/2.18/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.18'
2020-01-20T09:22:30.9895380Z  /usr/bin/install -c -m 644  samples/powerpc64-unknown-linux-gnu/crosstool.config samples/powerpc64-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc64-unknown-linux-gnu'
2020-01-20T09:22:30.9926192Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.19'
2020-01-20T09:22:30.9940336Z  /usr/bin/install -c -m 644  packages/glibc/2.19/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.19/0001-fix-signed-shift-overlow.patch packages/glibc/2.19/0002-dl-openat64-variadic.patch packages/glibc/2.19/0003-unused-variables.patch packages/glibc/2.19/0004-misleading-indentation.patch packages/glibc/2.19/0005-dl-open-array-bounds.patch packages/glibc/2.19/0006-arm-unwind.patch packages/glibc/2.19/0007-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.19/0008-typedef-caddr.patch packages/glibc/2.19/0009-fix-rpc_parse-format.patch packages/glibc/2.19/0010-explicit-boolean.patch packages/glibc/2.19/0011-nis-bogus-conditional.patch packages/glibc/2.19/0012-strftime-multiple-stmts.patch packages/glibc/2.19/0013-if_nametoindex-size-check.patch packages/glibc/2.19/0014-utmp-nonstring.patch packages/glibc/2.19/0015-getlogin_r-use-strnlen.patch packages/glibc/2.19/0016-zic.c-use-memcpy.patch packages/glibc/2.19/chksum packages/glibc/2.19/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.19'
2020-01-20T09:22:30.9960505Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/musl'
---
2020-01-20T09:22:31.0295365Z  /usr/bin/install -c -m 644  samples/msp430-unknown-elf/crosstool.config samples/msp430-unknown-elf/reported.by '/usr/local/share/crosstool-ng/samples/msp430-unknown-elf'
2020-01-20T09:22:31.0315332Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.3.0.20160226'
2020-01-20T09:22:31.0327689Z  /usr/bin/install -c -m 644  packages/newlib/2.3.0.20160226/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.3.0.20160226/0001-fix-eabihf.patch packages/newlib/2.3.0.20160226/0002-fix-mt-cflags.patch packages/newlib/2.3.0.20160226/chksum packages/newlib/2.3.0.20160226/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.3.0.20160226'
2020-01-20T09:22:31.0348651Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/zlib/1.2.11'
2020-01-20T09:22:31.0364110Z  /usr/bin/install -c -m 644  packages/zlib/1.2.11/0000-make-check-fail.patch packages/zlib/1.2.11/0001-no-_wopen-cygwin.patch packages/zlib/1.2.11/0002-mingw-static-only.patch packages/zlib/1.2.11/chksum packages/zlib/1.2.11/version.desc '/usr/local/share/crosstool-ng/packages/zlib/1.2.11'
2020-01-20T09:22:31.0395175Z  /usr/bin/install -c -m 644  packages/gdb/7.6.1/chksum packages/gdb/7.6.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.6.1'
2020-01-20T09:22:31.0415463Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc-ports/2.16.0'
2020-01-20T09:22:31.0429020Z  /usr/bin/install -c -m 644  packages/glibc-ports/2.16.0/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc-ports/2.16.0/0001-m68k-sys-user.patch packages/glibc-ports/2.16.0/0002-alpha-cache-shape.patch packages/glibc-ports/2.16.0/0003-alpha-fix-gcc-4.1-warnings.patch packages/glibc-ports/2.16.0/0004-alpha-fix-rtld-fPIC.patch packages/glibc-ports/2.16.0/0005-nptl-lowlevellock.patch packages/glibc-ports/2.16.0/0006-fpu-cw-mips.patch packages/glibc-ports/2.16.0/chksum packages/glibc-ports/2.16.0/version.desc '/usr/local/share/crosstool-ng/packages/glibc-ports/2.16.0'
2020-01-20T09:22:31.0449192Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
2020-01-20T09:22:31.0449192Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
2020-01-20T09:22:31.0461118Z  /usr/bin/install -c -m 644  packages/linux/4.13.16/chksum packages/linux/4.13.16/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
2020-01-20T09:22:31.0478462Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.6'
2020-01-20T09:22:31.0490018Z  /usr/bin/install -c -m 644  packages/strace/4.6/chksum packages/strace/4.6/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.6'
2020-01-20T09:22:31.0509243Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.7'
2020-01-20T09:22:31.0523082Z  /usr/bin/install -c -m 644  packages/strace/4.7/chksum packages/strace/4.7/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.7'
2020-01-20T09:22:31.0543606Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.8'
2020-01-20T09:22:31.0556538Z  /usr/bin/install -c -m 644  packages/strace/4.8/0000-strace-4.8-glibc_2.18_build_fix-1.patch packages/strace/4.8/chksum packages/strace/4.8/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.8'
2020-01-20T09:22:31.0586927Z  /usr/bin/install -c -m 644  packages/strace/4.9/chksum packages/strace/4.9/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.9'
2020-01-20T09:22:31.0603328Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils-linaro/2.25.0-2015.01-2'
2020-01-20T09:22:31.0617688Z  /usr/bin/install -c -m 644  packages/binutils-linaro/2.25.0-2015.01-2/chksum packages/binutils-linaro/2.25.0-2015.01-2/version.desc '/usr/local/share/crosstool-ng/packages/binutils-linaro/2.25.0-2015.01-2'
2020-01-20T09:22:31.0639971Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
2020-01-20T09:22:31.0639971Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
2020-01-20T09:22:31.0654307Z  /usr/bin/install -c -m 644  samples/mipsel-multilib-linux-gnu/crosstool.config samples/mipsel-multilib-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
2020-01-20T09:22:31.0675370Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/duma/2_5_15'
2020-01-20T09:22:31.0688209Z  /usr/bin/install -c -m 644  packages/duma/2_5_15/0000-cross-compile.patch packages/duma/2_5_15/0001-separate_cpp.patch packages/duma/2_5_15/0002-cpp11-new-operator.patch packages/duma/2_5_15/chksum packages/duma/2_5_15/version.desc '/usr/local/share/crosstool-ng/packages/duma/2_5_15'
2020-01-20T09:22:31.0720288Z  /usr/bin/install -c -m 644  packages/cloog/package.desc '/usr/local/share/crosstool-ng/packages/cloog'
2020-01-20T09:22:31.0762243Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/4.9.4'
2020-01-20T09:22:31.0779344Z  /usr/bin/install -c -m 644  packages/gcc/4.9.4/0000-Use-ucontext_t-not-struct-ucontext-in-linux-unwind.h.patch packages/gcc/4.9.4/0001-gcc_bug_62231.patch packages/gcc/4.9.4/0002-gcc_bug_62231.patch packages/gcc/4.9.4/0003-libtool-leave-framework-alone.patch packages/gcc/4.9.4/0004-uclibc-conf.patch packages/gcc/4.9.4/0005-msp430-string-literals.patch packages/gcc/4.9.4/0006-alpha-bad-eh_frame.patch packages/gcc/4.9.4/0007-pr65730.patch packages/gcc/4.9.4/0008-gcc-config.gcc-fix-typo-for-powerpc-e6500-cpu_is_64b.patch packages/gcc/4.9.4/0009-pr43538.patch packages/gcc/4.9.4/0010-mt-ospace-preserve-FLAGS_FOR_TARGET.patch packages/gcc/4.9.4/0011-sanitizer-Fix-build-with-_FILE_OFFSET_BITS-64.patch packages/gcc/4.9.4/0012-missing-execinfo_h.patch packages/gcc/4.9.4/0013-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/4.9.4/0014-arm-softfloat-libgcc.patch packages/gcc/4.9.4/0015-arm_unbreak_armv4t.patch packages/gcc/4.9.4/0016-microblaze-enable-dwarf-eh-support.patch packages/gcc/4.9.4/0017-libstdcxx-uclibc-c99.patch packages/gcc/4.9.4/0018-cilk-wchar.patch packages/gcc/4.9.4/0019-xtensa-add-mauto-litpools-option.patch packages/gcc/4.9.4/0020-xtensa-reimplement-register-spilling.patch packages/gcc/4.9.4/0021-xtensa-use-unwind-dw2-fde-dip-instead-of-unwind-dw2-.patch packages/gcc/4.9.4/0022-xtensa-fix-_Unwind_GetCFA.patch packages/gcc/4.9.4/0023-xtensa-add-uclinux-support.patch packages/gcc/4.9.4/0024-gcc-xtensa-fix-fprintf-format-specifiers.patch packages/gcc/4.9.4/0025-xtensa-fix-PR-target-82181.patch packages/gcc/4.9.4/0026-nios2_legitimize_address.patch packages/gcc/4.9.4/0027-fix-m68k-compile.patch packages/gcc/4.9.4/0028-fix-m68k-uclinux.patch packages/gcc/4.9.4/0029-musl-support.patch packages/gcc/4.9.4/0030-microblaze-uclibc.patch packages/gcc/4.9.4/0031-libgcc-disable-split-stack-nothreads.patch packages/gcc/4.9.4/0032-uclinux-enable-threads.patch packages/gcc/4.9.4/1000-powerpc-link-with-math-lib.patch.conditional packages/gcc/4.9.4/chksum packages/gcc/4.9.4/version.desc '/usr/local/share/crosstool-ng/packages/gcc/4.9.4'
2020-01-20T09:22:31.0811603Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-sde-elf'
2020-01-20T09:22:31.0811603Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-sde-elf'
2020-01-20T09:22:31.0825748Z  /usr/bin/install -c -m 644  samples/mipsel-sde-elf/crosstool.config samples/mipsel-sde-elf/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-sde-elf'
2020-01-20T09:22:31.0854131Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/8.3.0'
2020-01-20T09:22:31.0870117Z  /usr/bin/install -c -m 644  packages/gcc/8.3.0/0000-libtool-leave-framework-alone.patch packages/gcc/8.3.0/0001-uclibc-conf.patch packages/gcc/8.3.0/0002-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/8.3.0/0003-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/8.3.0/0004-arm-softfloat-libgcc.patch packages/gcc/8.3.0/0005-fix-m68k-uclinux.patch packages/gcc/8.3.0/0006-libgfortran-missing-include.patch packages/gcc/8.3.0/0007-nios2-bad-multilib-default.patch packages/gcc/8.3.0/0008-libgcc-disable-split-stack-nothreads.patch packages/gcc/8.3.0/0009-bionic-ndk.patch packages/gcc/8.3.0/0010-crystax.patch packages/gcc/8.3.0/0011-crystax.patch packages/gcc/8.3.0/0012-crystax.patch packages/gcc/8.3.0/0013-crystax.patch packages/gcc/8.3.0/0014-crystax.patch packages/gcc/8.3.0/0015-crystax.patch packages/gcc/8.3.0/0016-crystax.patch packages/gcc/8.3.0/0017-crystax.patch packages/gcc/8.3.0/0018-ARC-Add-multilib-support-for-linux-targets.patch packages/gcc/8.3.0/0019-isl-0.20.patch packages/gcc/8.3.0/0020-ARM-fix-cmse.patch packages/gcc/8.3.0/0021-arm-Make-arm_cmse.h-C99-compatible.patch packages/gcc/8.3.0/chksum packages/gcc/8.3.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/8.3.0'
2020-01-20T09:22:31.0911705Z  /usr/bin/install -c -m 644  packages/newlib/3.1.0.20181231/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/3.1.0.20181231/0001-fix-mt-cflags.patch packages/newlib/3.1.0.20181231/chksum packages/newlib/3.1.0.20181231/version.desc '/usr/local/share/crosstool-ng/packages/newlib/3.1.0.20181231'
2020-01-20T09:22:31.0932168Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/2.6.38.8'
2020-01-20T09:22:31.0945255Z  /usr/bin/install -c -m 644  packages/linux/2.6.38.8/chksum packages/linux/2.6.38.8/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.38.8'
2020-01-20T09:22:31.0963366Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.0.1a'
---
2020-01-20T09:22:31.1347428Z  /usr/bin/install -c -m 644  packages/isl/0.14.1/chksum packages/isl/0.14.1/version.desc '/usr/local/share/crosstool-ng/packages/isl/0.14.1'
2020-01-20T09:22:31.1364276Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arc-multilib-elf32'
2020-01-20T09:22:31.1378964Z  /usr/bin/install -c -m 644  samples/arc-multilib-elf32/crosstool.config samples/arc-multilib-elf32/reported.by '/usr/local/share/crosstool-ng/samples/arc-multilib-elf32'
2020-01-20T09:22:31.1407244Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/arch'
2020-01-20T09:22:31.1422415Z  /usr/bin/install -c -m 644  config/arch/alpha.in config/arch/arc.in config/arch/arm.in config/arch/avr.in config/arch/m68k.in config/arch/microblaze.in config/arch/mips.in config/arch/moxie.in config/arch/msp430.in config/arch/nios2.in config/arch/powerpc.in config/arch/riscv.in config/arch/s390.in config/arch/sh.in config/arch/sparc.in config/arch/x86.in config/arch/xtensa.in '/usr/local/share/crosstool-ng/config/arch'
2020-01-20T09:22:31.1468968Z  /usr/bin/install -c -m 644  packages/linux/3.0.101/chksum packages/linux/3.0.101/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.0.101'
2020-01-20T09:22:31.1487104Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-ubuntu16.04-linux-gnu'
2020-01-20T09:22:31.1498925Z  /usr/bin/install -c -m 644  samples/i686-ubuntu16.04-linux-gnu/crosstool.config samples/i686-ubuntu16.04-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/i686-ubuntu16.04-linux-gnu'
2020-01-20T09:22:31.1522904Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/1.20.0'
---
2020-01-20T09:22:31.2265234Z  /usr/bin/install -c -m 644  samples/powerpc-e300c3-linux-gnu/crosstool.config samples/powerpc-e300c3-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-e300c3-linux-gnu'
2020-01-20T09:22:31.2284972Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.17.19'
2020-01-20T09:22:31.2299402Z  /usr/bin/install -c -m 644  packages/linux/4.17.19/chksum packages/linux/4.17.19/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.17.19'
2020-01-20T09:22:31.2324080Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mpfr/3.0.1'
2020-01-20T09:22:31.2339298Z  /usr/bin/install -c -m 644  packages/mpfr/3.0.1/0000-asin_exprange.patch packages/mpfr/3.0.1/0001-rec_sqrt-carry.patch packages/mpfr/3.0.1/0002-atan-expo-range.patch packages/mpfr/3.0.1/0003-texp-zero.patch packages/mpfr/3.0.1/chksum packages/mpfr/3.0.1/version.desc '/usr/local/share/crosstool-ng/packages/mpfr/3.0.1'
2020-01-20T09:22:31.2375913Z  /usr/bin/install -c -m 644  packages/linux/2.6.33.7/chksum packages/linux/2.6.33.7/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.33.7'
2020-01-20T09:22:31.2405498Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.26.1'
2020-01-20T09:22:31.2420374Z  /usr/bin/install -c -m 644  packages/binutils/2.26.1/0000-sh-conf.patch packages/binutils/2.26.1/0001-ld_makefile_patch.patch packages/binutils/2.26.1/0002-check_ldrunpath_length.patch packages/binutils/2.26.1/0003-fix-gold-pthreads-typo.patch packages/binutils/2.26.1/0004-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.26.1/0005-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.26.1/0006-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.26.1/0007-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.26.1/0008-sysroot.patch packages/binutils/2.26.1/0009-poison-system-directories.patch packages/binutils/2.26.1/0010-Fix-library-paths-on-PowerPC.patch packages/binutils/2.26.1/0011-xtensa-fix-signedness-of-gas-relocations.patch packages/binutils/2.26.1/0012-xtensa-fix-.init-.fini-literals-moving.patch packages/binutils/2.26.1/chksum packages/binutils/2.26.1/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.26.1'
2020-01-20T09:22:31.2448486Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/gen'
---
2020-01-20T09:22:31.3080328Z  /usr/bin/install -c -m 644  packages/glibc/2.21/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.21/0001-fix-signed-shift-overlow.patch packages/glibc/2.21/0002-dl-openat64-variadic.patch packages/glibc/2.21/0003-unused-variables.patch packages/glibc/2.21/0004-misleading-indentation.patch packages/glibc/2.21/0005-dl-open-array-bounds.patch packages/glibc/2.21/0006-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.21/0007-typedef-caddr.patch packages/glibc/2.21/0008-fix-rpc_parse-format.patch packages/glibc/2.21/0009-explicit-boolean.patch packages/glibc/2.21/0010-nis-bogus-conditional.patch packages/glibc/2.21/0011-dlclose-assert.patch packages/glibc/2.21/0012-strftime-multiple-stmts.patch packages/glibc/2.21/0013-if_nametoindex-size-check.patch packages/glibc/2.21/0014-utmp-nonstring.patch packages/glibc/2.21/0015-getlogin_r-use-strnlen.patch packages/glibc/2.21/0016-zic.c-use-memcpy.patch packages/glibc/2.21/chksum packages/glibc/2.21/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.21'
2020-01-20T09:22:31.3105841Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gettext'
2020-01-20T09:22:31.3117469Z  /usr/bin/install -c -m 644  packages/gettext/package.desc '/usr/local/share/crosstool-ng/packages/gettext'
2020-01-20T09:22:31.3145123Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.22'
2020-01-20T09:22:31.3159666Z  /usr/bin/install -c -m 644  packages/glibc/2.22/0000-sparc32-sem_open-missing-include.patch packages/glibc/2.22/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.22/0002-fix-signed-shift-overlow.patch packages/glibc/2.22/0003-dl-openat64-variadic.patch packages/glibc/2.22/0004-unused-variables.patch packages/glibc/2.22/0005-misleading-indentation.patch packages/glibc/2.22/0006-cve-2105-7547-getaddrinfo-stack.patch packages/glibc/2.22/0007-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.22/0008-typedef-caddr.patch packages/glibc/2.22/0009-fix-rpc_parse-format.patch packages/glibc/2.22/0010-explicit-boolean.patch packages/glibc/2.22/0011-nis-bogus-conditional.patch packages/glibc/2.22/0012-strftime-multiple-stmts.patch packages/glibc/2.22/0013-if_nametoindex-size-check.patch packages/glibc/2.22/0014-utmp-nonstring.patch packages/glibc/2.22/0015-getlogin_r-use-strnlen.patch packages/glibc/2.22/0016-zic.c-use-memcpy.patch packages/glibc/2.22/chksum packages/glibc/2.22/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.22'
2020-01-20T09:22:31.3206450Z  /usr/bin/install -c -m 644  packages/glibc/2.23/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.23/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.23/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.23/0003-typedef-caddr.patch packages/glibc/2.23/0004-fix-rpc_parse-format.patch packages/glibc/2.23/0005-explicit-boolean.patch packages/glibc/2.23/0006-nis-bogus-conditional.patch packages/glibc/2.23/0007-regexp-common.patch packages/glibc/2.23/0008-strftime-multiple-stmts.patch packages/glibc/2.23/0009-if_nametoindex-size-check.patch packages/glibc/2.23/0010-utmp-nonstring.patch packages/glibc/2.23/0011-getlogin_r-use-strnlen.patch packages/glibc/2.23/0012-zic.c-use-memcpy.patch packages/glibc/2.23/chksum packages/glibc/2.23/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.23'
2020-01-20T09:22:31.3236986Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.24'
2020-01-20T09:22:31.3251882Z  /usr/bin/install -c -m 644  packages/glibc/2.24/0000-sh-fix-gcc6.patch packages/glibc/2.24/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.24/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.24/0003-typedef-caddr.patch packages/glibc/2.24/0004-fix-rpc_parse-format.patch packages/glibc/2.24/0005-explicit-boolean.patch packages/glibc/2.24/0006-nis-bogus-conditional.patch packages/glibc/2.24/0007-regexp-common.patch packages/glibc/2.24/0008-strftime-multiple-stmts.patch packages/glibc/2.24/0009-if_nametoindex-size-check.patch packages/glibc/2.24/0010-utmp-nonstring.patch packages/glibc/2.24/0011-getlogin_r-use-strnlen.patch packages/glibc/2.24/0012-zic.c-use-memcpy.patch packages/glibc/2.24/chksum packages/glibc/2.24/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.24'
2020-01-20T09:22:31.3283169Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.25'
2020-01-20T09:22:31.3283169Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.25'
2020-01-20T09:22:31.3296737Z  /usr/bin/install -c -m 644  packages/glibc/2.25/0000-sh-fix-gcc6.patch packages/glibc/2.25/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.25/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.25/0003-typedef-caddr.patch packages/glibc/2.25/0004-sh4-trap-divdi3.patch packages/glibc/2.25/0005-sparc-extra-plt-call.patch packages/glibc/2.25/0006-regexp-common.patch packages/glibc/2.25/0007-strftime-multiple-stmts.patch packages/glibc/2.25/0008-if_nametoindex-size-check.patch packages/glibc/2.25/0009-utmp-nonstring.patch packages/glibc/2.25/0010-getlogin_r-use-strnlen.patch packages/glibc/2.25/0011-zic.c-use-memcpy.patch packages/glibc/2.25/chksum packages/glibc/2.25/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.25'
2020-01-20T09:22:31.3338005Z  /usr/bin/install -c -m 644  packages/glibc/2.26/0000-typedef-caddr.patch packages/glibc/2.26/0001-aarch64-rewrite-elf_machine_load_address.patch packages/glibc/2.26/0002-if_nametoindex-size-check.patch packages/glibc/2.26/0003-utmp-nonstring.patch packages/glibc/2.26/0004-getlogin_r-use-strnlen.patch packages/glibc/2.26/0005-zic.c-use-memcpy.patch packages/glibc/2.26/chksum packages/glibc/2.26/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.26'
2020-01-20T09:22:31.3359102Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.27'
2020-01-20T09:22:31.3372153Z  /usr/bin/install -c -m 644  packages/glibc/2.27/0000-typedef-caddr.patch packages/glibc/2.27/chksum packages/glibc/2.27/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.27'
2020-01-20T09:22:31.3390551Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-ubuntu14.04-linux-gnu'
---
2020-01-20T09:22:31.3932534Z  /usr/bin/install -c -m 644  packages/linux/3.5.7/chksum packages/linux/3.5.7/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.5.7'
2020-01-20T09:22:31.3960454Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/binutils'
2020-01-20T09:22:31.3961411Z  /usr/bin/install -c -m 644  scripts/build/binutils/binutils.sh '/usr/local/share/crosstool-ng/scripts/build/binutils'
2020-01-20T09:22:31.3984859Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.31.1'
2020-01-20T09:22:31.3999958Z  /usr/bin/install -c -m 644  packages/binutils/2.31.1/0000-sh-conf.patch packages/binutils/2.31.1/0001-ld_makefile_patch.patch packages/binutils/2.31.1/0002-check_ldrunpath_length.patch packages/binutils/2.31.1/0003-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.31.1/0004-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.31.1/0005-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.31.1/0006-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.31.1/0007-sysroot.patch packages/binutils/2.31.1/0008-poison-system-directories.patch packages/binutils/2.31.1/0009-xtensa-fix-relaxation-of-undefined-weak-references-i.patch packages/binutils/2.31.1/0010-xtensa-move-dynamic-relocations-sections-consistency.patch packages/binutils/2.31.1/0011-Restore-build-on-x86_64-w64-mingw32.patch packages/binutils/2.31.1/chksum packages/binutils/2.31.1/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.31.1'
2020-01-20T09:22:31.4046093Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/5.5.0'
2020-01-20T09:22:31.4062380Z  /usr/bin/install -c -m 644  packages/gcc/5.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/5.5.0/0001-uclibc-conf.patch packages/gcc/5.5.0/0002-msp430-string-literals.patch packages/gcc/5.5.0/0003-xtensa-implement-trap-pattern.patch packages/gcc/5.5.0/0004-alpha-bad-eh_frame.patch packages/gcc/5.5.0/0005-gcc-config.gcc-fix-typo-for-powerpc-e6500-cpu_is_64b.patch packages/gcc/5.5.0/0006-missing-execinfo_h.patch packages/gcc/5.5.0/0007-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/5.5.0/0008-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/5.5.0/0009-arm-softfloat-libgcc.patch packages/gcc/5.5.0/0010-arm_unbreak_armv4t.patch packages/gcc/5.5.0/0011-microblaze-enable-dwarf-eh-support.patch packages/gcc/5.5.0/0012-libstdcxx-uclibc-c99.patch packages/gcc/5.5.0/0013-cilk-wchar.patch packages/gcc/5.5.0/0014-xtensa-add-mauto-litpools-option.patch packages/gcc/5.5.0/0015-xtensa-reimplement-register-spilling.patch packages/gcc/5.5.0/0016-xtensa-add-uclinux-support.patch packages/gcc/5.5.0/0017-fix-m68k-compile.patch packages/gcc/5.5.0/0018-fix-m68k-uclinux.patch packages/gcc/5.5.0/0019-microblaze-uclibc.patch packages/gcc/5.5.0/0020-libitm-fixes-for-musl-support.patch packages/gcc/5.5.0/0021-fixincludes-update-for-musl-support.patch packages/gcc/5.5.0/0022-unwind-fix-for-musl.patch packages/gcc/5.5.0/0023-libstdc++-libgfortran-gthr-workaround-for-musl.patch packages/gcc/5.5.0/0024-musl-libc-config.patch packages/gcc/5.5.0/0025-add-musl-support-to-gcc.patch packages/gcc/5.5.0/0026-mips-musl-support.patch packages/gcc/5.5.0/0027-x86-musl-support.patch packages/gcc/5.5.0/0028-arm-musl-support.patch packages/gcc/5.5.0/0029-aarch64-musl-support.patch packages/gcc/5.5.0/0030-nios2-bad-multilib-default.patch packages/gcc/5.5.0/0031-libgcc-disable-split-stack-nothreads.patch packages/gcc/5.5.0/0032-uclinux-enable-threads.patch packages/gcc/5.5.0/0033-msp430-fix.patch packages/gcc/5.5.0/0034-xtensa-fix-PR-target-65416.patch packages/gcc/5.5.0/chksum packages/gcc/5.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/5.5.0'
2020-01-20T09:22:31.4100753Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts'
2020-01-20T09:22:31.4117768Z  /usr/bin/install -c -m 644  scripts/config.guess scripts/config.rpath scripts/config.sub scripts/crosstool-NG.sh scripts/functions scripts/populate.in scripts/saveSample.sh scripts/scripts.mk scripts/show-config.sh scripts/show-tuple.sh scripts/toolchain-config.in scripts/version-check.sh scripts/xldd.in '/usr/local/share/crosstool-ng/scripts'
2020-01-20T09:22:31.4157688Z  /usr/bin/install -c -m 644  packages/gdb/7.3.1/chksum packages/gdb/7.3.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.3.1'
2020-01-20T09:22:31.4174796Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/aarch64-unknown-linux-gnu'
2020-01-20T09:22:31.4187943Z  /usr/bin/install -c -m 644  samples/aarch64-unknown-linux-gnu/crosstool.config samples/aarch64-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-unknown-linux-gnu'
2020-01-20T09:22:31.4204378Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib'
---
2020-01-20T09:22:31.4526351Z  /usr/bin/install -c -m 644  packages/musl/1.1.21/chksum packages/musl/1.1.21/version.desc '/usr/local/share/crosstool-ng/packages/musl/1.1.21'
2020-01-20T09:22:31.4543356Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc-860-linux-gnu'
2020-01-20T09:22:31.4555380Z  /usr/bin/install -c -m 644  samples/powerpc-860-linux-gnu/crosstool.config samples/powerpc-860-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-860-linux-gnu'
2020-01-20T09:22:31.4576339Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/6.8a'
2020-01-20T09:22:31.4588287Z  /usr/bin/install -c -m 644  packages/gdb/6.8a/0000-dwarf-stack-overflow.patch packages/gdb/6.8a/0001-security-errata-20050610.patch packages/gdb/6.8a/0002-tdep-opcode-include-workaround.patch packages/gdb/6.8a/0003-reg-no-longer-active.patch packages/gdb/6.8a/0004-sim-ppc-have-config-h.patch packages/gdb/6.8a/0005-handle-stpcpy-define.patch packages/gdb/6.8a/chksum packages/gdb/6.8a/version.desc '/usr/local/share/crosstool-ng/packages/gdb/6.8a'
2020-01-20T09:22:31.4620762Z  /usr/bin/install -c -m 644  scripts/build/test_suite/gcc.sh '/usr/local/share/crosstool-ng/scripts/build/test_suite'
2020-01-20T09:22:31.4642307Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.0.0'
2020-01-20T09:22:31.4656357Z  /usr/bin/install -c -m 644  packages/newlib/2.0.0/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.0.0/0001-fix-eabihf.patch packages/newlib/2.0.0/0002-fix-mt-cflags.patch packages/newlib/2.0.0/chksum packages/newlib/2.0.0/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.0.0'
2020-01-20T09:22:31.4675325Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/isl/0.15'
---
2020-01-20T09:22:31.5135327Z  /usr/bin/install -c -m 644  samples/aarch64-unknown-linux-uclibc/crosstool.config samples/aarch64-unknown-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-unknown-linux-uclibc'
2020-01-20T09:22:31.5155167Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-bare_newlib_cortex_m3_nommu-eabi'
2020-01-20T09:22:31.5168791Z  /usr/bin/install -c -m 644  samples/arm-bare_newlib_cortex_m3_nommu-eabi/crosstool.config samples/arm-bare_newlib_cortex_m3_nommu-eabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-bare_newlib_cortex_m3_nommu-eabi'
2020-01-20T09:22:31.5194331Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/arch'
2020-01-20T09:22:31.5208217Z  /usr/bin/install -c -m 644  scripts/build/arch/alpha.sh scripts/build/arch/arc.sh scripts/build/arch/arm.sh scripts/build/arch/avr.sh scripts/build/arch/m68k.sh scripts/build/arch/microblaze.sh scripts/build/arch/mips.sh scripts/build/arch/moxie.sh scripts/build/arch/msp430.sh scripts/build/arch/nios2.sh scripts/build/arch/powerpc.sh scripts/build/arch/riscv.sh scripts/build/arch/s390.sh scripts/build/arch/sh.sh scripts/build/arch/sparc.sh scripts/build/arch/x86.sh scripts/build/arch/xtensa.sh '/usr/local/share/crosstool-ng/scripts/build/arch'
2020-01-20T09:22:31.5249842Z  /usr/bin/install -c -m 644  packages/gdb-linaro/7.8-2014.09/chksum packages/gdb-linaro/7.8-2014.09/version.desc '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.8-2014.09'
2020-01-20T09:22:31.5267304Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc64le-unknown-linux-gnu'
2020-01-20T09:22:31.5279214Z  /usr/bin/install -c -m 644  samples/powerpc64le-unknown-linux-gnu/crosstool.config samples/powerpc64le-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc64le-unknown-linux-gnu'
2020-01-20T09:22:31.5297068Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/duma'
---
2020-01-20T09:22:31.5645920Z  /usr/bin/install -c -m 644  packages/linux/3.17.8/chksum packages/linux/3.17.8/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.17.8'
2020-01-20T09:22:31.5676738Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/8.2.1'
2020-01-20T09:22:31.5689143Z  /usr/bin/install -c -m 644  packages/gdb/8.2.1/0000-musl_fix.patch packages/gdb/8.2.1/0001-uclibc-no-gettimeofday-clobber.patch packages/gdb/8.2.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/8.2.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/8.2.1/0004-allow-android.patch packages/gdb/8.2.1/chksum packages/gdb/8.2.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/8.2.1'
2020-01-20T09:22:31.5709975Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.10.1'
2020-01-20T09:22:31.5721828Z  /usr/bin/install -c -m 644  packages/gdb/7.10.1/0000-musl_fix.patch packages/gdb/7.10.1/0001-xtensa-initialize-call_abi-in-xtensa_tdep.patch packages/gdb/7.10.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/7.10.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/7.10.1/chksum packages/gdb/7.10.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.10.1'
2020-01-20T09:22:31.5753986Z  /usr/bin/install -c -m 644  packages/expat/2.2.6/chksum packages/expat/2.2.6/version.desc '/usr/local/share/crosstool-ng/packages/expat/2.2.6'
2020-01-20T09:22:31.5776143Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.10'
2020-01-20T09:22:31.5776143Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.10'
2020-01-20T09:22:31.5790447Z  /usr/bin/install -c -m 644  packages/strace/4.10/0000-aarch64_rt_sigreturn.patch packages/strace/4.10/0001-arm_mmap2.patch packages/strace/4.10/0002-aarch64_arch_regs.patch packages/strace/4.10/0003-stat64-v.test.patch packages/strace/4.10/0004-select_test.patch packages/strace/4.10/0005-fix_aarch64_ioctl_decoding.patch packages/strace/4.10/0006-fix_bexecve64_test.patch packages/strace/4.10/0007-decode_mips_indirect_syscall.patch packages/strace/4.10/0008-upstream-musl_includes.patch packages/strace/4.10/0009-use-host-ioctl.patch packages/strace/4.10/chksum packages/strace/4.10/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.10'
2020-01-20T09:22:31.5830429Z  /usr/bin/install -c -m 644  packages/strace/4.11/0000-use-host-ioctl.patch packages/strace/4.11/chksum packages/strace/4.11/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.11'
2020-01-20T09:22:31.5849973Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.12'
2020-01-20T09:22:31.5865732Z  /usr/bin/install -c -m 644  packages/strace/4.12/0000-use-host-ioctl.patch packages/strace/4.12/chksum packages/strace/4.12/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.12'
2020-01-20T09:22:31.5882654Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/debug'
2020-01-20T09:22:31.5882654Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/debug'
2020-01-20T09:22:31.5897033Z  /usr/bin/install -c -m 644  scripts/build/debug/200-duma.sh scripts/build/debug/300-gdb.sh scripts/build/debug/400-ltrace.sh scripts/build/debug/500-strace.sh scripts/build/debug/duma.in scripts/build/debug/gdbinit.in '/usr/local/share/crosstool-ng/scripts/build/debug'
2020-01-20T09:22:31.5931683Z  /usr/bin/install -c -m 644  packages/strace/4.13/0000-use-host-ioctl.patch packages/strace/4.13/chksum packages/strace/4.13/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.13'
2020-01-20T09:22:31.5949957Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.14'
2020-01-20T09:22:31.5964315Z  /usr/bin/install -c -m 644  packages/strace/4.14/0000-use-host-ioctl.patch packages/strace/4.14/chksum packages/strace/4.14/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.14'
2020-01-20T09:22:31.5985434Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.15'
---
2020-01-20T09:22:31.6525020Z  /usr/bin/install -c -m 644  samples/riscv32-hifive1-elf/crosstool.config samples/riscv32-hifive1-elf/reported.by '/usr/local/share/crosstool-ng/samples/riscv32-hifive1-elf'
2020-01-20T09:22:31.6544171Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/armv7-rpi2-linux-gnueabihf'
2020-01-20T09:22:31.6562452Z  /usr/bin/install -c -m 644  samples/armv7-rpi2-linux-gnueabihf/crosstool.config samples/armv7-rpi2-linux-gnueabihf/reported.by '/usr/local/share/crosstool-ng/samples/armv7-rpi2-linux-gnueabihf'
2020-01-20T09:22:31.6585905Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages'
2020-01-20T09:22:31.6597279Z  /usr/bin/install -c -m 644  packages/GNU.help packages/Linaro.help '/usr/local/share/crosstool-ng/packages'
2020-01-20T09:22:31.6627259Z  /usr/bin/install -c -m 644  config/cc/gcc.in config/cc/gcc.in.mips '/usr/local/share/crosstool-ng/config/cc'
2020-01-20T09:22:31.6644733Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-unknown-linux-gnu'
2020-01-20T09:22:31.6658170Z  /usr/bin/install -c -m 644  samples/mipsel-unknown-linux-gnu/crosstool.config samples/mipsel-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-unknown-linux-gnu'
2020-01-20T09:22:31.6675872Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/nios2-altera-linux-gnu'
---
2020-01-20T09:22:31.7171441Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.25/0000-gdb8.patch packages/uClibc-ng/1.0.25/0001-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.25/chksum packages/uClibc-ng/1.0.25/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.25'
2020-01-20T09:22:31.7192758Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.26'
2020-01-20T09:22:31.7209163Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.26/0000-gdb8.patch packages/uClibc-ng/1.0.26/0001-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.26/chksum packages/uClibc-ng/1.0.26/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.26'
2020-01-20T09:22:31.7232756Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.27'
2020-01-20T09:22:31.7247038Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.27/0000-bessel-link-failure.patch packages/uClibc-ng/1.0.27/0001-feraiseexcept-2.patch packages/uClibc-ng/1.0.27/0002-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.27/0003-feraiseexcept.patch packages/uClibc-ng/1.0.27/chksum packages/uClibc-ng/1.0.27/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.27'
2020-01-20T09:22:31.7279807Z  /usr/bin/install -c -m 644  packages/mpc/1.1.0/chksum packages/mpc/1.1.0/version.desc '/usr/local/share/crosstool-ng/packages/mpc/1.1.0'
2020-01-20T09:22:31.7298603Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.28'
2020-01-20T09:22:31.7309924Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.28/0000-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.28/chksum packages/uClibc-ng/1.0.28/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.28'
2020-01-20T09:22:31.7328652Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.29'
2020-01-20T09:22:31.7328652Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.29'
2020-01-20T09:22:31.7339795Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.29/chksum packages/uClibc-ng/1.0.29/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.29'
2020-01-20T09:22:31.7358694Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/licenses.d'
2020-01-20T09:22:31.7373370Z  /usr/bin/install -c -m 644  licenses.d/gpl.txt licenses.d/lgpl.txt '/usr/local/share/crosstool-ng/licenses.d'
2020-01-20T09:22:31.7405703Z  /usr/bin/install -c -m 644  packages/linux/2.6.37.6/chksum packages/linux/2.6.37.6/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.37.6'
2020-01-20T09:22:31.7440731Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/4.8.5'
2020-01-20T09:22:31.7458081Z  /usr/bin/install -c -m 644  packages/gcc/4.8.5/0000-gcc_bug_62231.patch packages/gcc/4.8.5/0001-gcc_bug_62231.patch packages/gcc/4.8.5/0002-libtool-leave-framework-alone.patch packages/gcc/4.8.5/0003-uclibc-conf.patch packages/gcc/4.8.5/0004-alpha-bad-eh_frame.patch packages/gcc/4.8.5/0005-pr65730.patch packages/gcc/4.8.5/0006-pr43538.patch packages/gcc/4.8.5/0007-mt-ospace-preserve-FLAGS_FOR_TARGET.patch packages/gcc/4.8.5/0008-build_gcc-5_with_gcc-6.patch packages/gcc/4.8.5/0009-missing-execinfo_h.patch packages/gcc/4.8.5/0010-libmudflap-susv3-legacy.patch packages/gcc/4.8.5/0011-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/4.8.5/0012-arm-softfloat-libgcc.patch packages/gcc/4.8.5/0013-arm_unbreak_armv4t.patch packages/gcc/4.8.5/0014-PR57717-E500v2.patch packages/gcc/4.8.5/0015-PR60155.patch packages/gcc/4.8.5/0016-aarch64-vmlaq_lane_s32-typo.patch packages/gcc/4.8.5/0017-libstdcxx-uclibc-c99.patch packages/gcc/4.8.5/0018-PR-other-56780.patch packages/gcc/4.8.5/0019-xtensa-add-mauto-litpools-option.patch packages/gcc/4.8.5/0020-xtensa-reimplement-register-spilling.patch packages/gcc/4.8.5/0021-xtensa-use-unwind-dw2-fde-dip-instead-of-unwind-dw2-.patch packages/gcc/4.8.5/0022-xtensa-fix-_Unwind_GetCFA.patch packages/gcc/4.8.5/0023-gcc-xtensa-fix-fprintf-format-specifiers.patch packages/gcc/4.8.5/0024-xtensa-fix-PR-target-82181.patch packages/gcc/4.8.5/0025-musl-support.patch packages/gcc/4.8.5/0026-cygwin64.patch packages/gcc/4.8.5/1000-powerpc-link-with-math-lib.patch.conditional packages/gcc/4.8.5/chksum packages/gcc/4.8.5/version.desc '/usr/local/share/crosstool-ng/packages/gcc/4.8.5'
2020-01-20T09:22:31.7494757Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/alphaev67-unknown-linux-gnu'
---
2020-01-20T09:22:31.7885268Z  /usr/bin/install -c -m 644  packages/glibc-ports/package.desc '/usr/local/share/crosstool-ng/packages/glibc-ports'
2020-01-20T09:22:31.7909641Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.12.1'
2020-01-20T09:22:31.7923106Z  /usr/bin/install -c -m 644  packages/gdb/7.12.1/0000-musl_fix.patch packages/gdb/7.12.1/0001-uclibc-no-gettimeofday-clobber.patch packages/gdb/7.12.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/7.12.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/7.12.1/0004-allow-android.patch packages/gdb/7.12.1/0005-include-order.patch packages/gdb/7.12.1/0006-duplicate-typedef.patch packages/gdb/7.12.1/chksum packages/gdb/7.12.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.12.1'
2020-01-20T09:22:31.7944913Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/debug'
2020-01-20T09:22:31.7956751Z  /usr/bin/install -c -m 644  config/debug/duma.in config/debug/gdb.in config/debug/gdb.in.cross config/debug/gdb.in.native config/debug/ltrace.in config/debug/strace.in '/usr/local/share/crosstool-ng/config/debug'
2020-01-20T09:22:31.7988699Z  /usr/bin/install -c -m 644  packages/gdb-linaro/7.3-2011.12/chksum packages/gdb-linaro/7.3-2011.12/version.desc '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.3-2011.12'
2020-01-20T09:22:31.8010105Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.2.0.20151023'
2020-01-20T09:22:31.8024241Z  /usr/bin/install -c -m 644  packages/newlib/2.2.0.20151023/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.2.0.20151023/0001-fix-eabihf.patch packages/newlib/2.2.0.20151023/0002-fix-mt-cflags.patch packages/newlib/2.2.0.20151023/chksum packages/newlib/2.2.0.20151023/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.2.0.20151023'
2020-01-20T09:22:31.8040266Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/android-ndk'
---
2020-01-20T09:22:31.9478436Z  /usr/bin/install -c -m 644  packages/ncurses/6.1/0000-ncurses-6.1-20180129.patch packages/ncurses/6.1/chksum packages/ncurses/6.1/version.desc '/usr/local/share/crosstool-ng/packages/ncurses/6.1'
2020-01-20T09:22:31.9497697Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples'
2020-01-20T09:22:31.9509285Z  /usr/bin/install -c -m 644  samples/samples.mk '/usr/local/share/crosstool-ng/samples'
2020-01-20T09:22:31.9530589Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/ltrace/0.5.3'
2020-01-20T09:22:31.9547569Z  /usr/bin/install -c -m 644  packages/ltrace/0.5.3/0000-allow-cross-compile.patch packages/ltrace/0.5.3/0001-alpha-support.patch packages/ltrace/0.5.3/0002-debian-ltrace_0.5.3-2.patch packages/ltrace/0.5.3/0003-add-sysdep.patch packages/ltrace/0.5.3/0004-mips.patch packages/ltrace/0.5.3/0005-mips-remove-CP.patch packages/ltrace/0.5.3/0006-allow-configurable-arch.patch packages/ltrace/0.5.3/0007-fix-missing-ptrace-defines.patch packages/ltrace/0.5.3/0008-lib-supcc.patch packages/ltrace/0.5.3/0009-libltrace-genindex.patch packages/ltrace/0.5.3/0010-ar-configurable.patch packages/ltrace/0.5.3/0011-configure-hostos.patch packages/ltrace/0.5.3/chksum packages/ltrace/0.5.3/version.desc '/usr/local/share/crosstool-ng/packages/ltrace/0.5.3'
2020-01-20T09:22:31.9585287Z  /usr/bin/install -c -m 644  samples/arm-cortex_a15-linux-gnueabihf/crosstool.config samples/arm-cortex_a15-linux-gnueabihf/reported.by '/usr/local/share/crosstool-ng/samples/arm-cortex_a15-linux-gnueabihf'
2020-01-20T09:22:31.9602575Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/moxiebox/git-9a79ac54'
2020-01-20T09:22:31.9616455Z  /usr/bin/install -c -m 644  packages/moxiebox/git-9a79ac54/0001-Remove-PKG_CONFIG-check.patch '/usr/local/share/crosstool-ng/packages/moxiebox/git-9a79ac54'
2020-01-20T09:22:31.9634427Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/2.6.39.4'
2020-01-20T09:22:31.9634427Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/2.6.39.4'
2020-01-20T09:22:31.9648327Z  /usr/bin/install -c -m 644  packages/linux/2.6.39.4/chksum packages/linux/2.6.39.4/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.39.4'
2020-01-20T09:22:31.9668754Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/make/3.81'
2020-01-20T09:22:31.9680725Z  /usr/bin/install -c -m 644  packages/make/3.81/chksum packages/make/3.81/version.desc '/usr/local/share/crosstool-ng/packages/make/3.81'
2020-01-20T09:22:31.9700456Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc-unknown-linux-uclibc'
2020-01-20T09:22:31.9716213Z  /usr/bin/install -c -m 644  samples/powerpc-unknown-linux-uclibc/crosstool.config samples/powerpc-unknown-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-unknown-linux-uclibc'
2020-01-20T09:22:31.9740359Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/companion_libs'
2020-01-20T09:22:31.9753793Z  /usr/bin/install -c -m 644  scripts/build/companion_libs/050-zlib.sh scripts/build/companion_libs/100-gmp.sh scripts/build/companion_libs/110-mpfr.sh scripts/build/companion_libs/121-isl.sh scripts/build/companion_libs/130-cloog.sh scripts/build/companion_libs/140-mpc.sh scripts/build/companion_libs/200-libelf.sh scripts/build/companion_libs/210-expat.sh scripts/build/companion_libs/220-ncurses.sh scripts/build/companion_libs/320-libiconv.sh scripts/build/companion_libs/330-gettext.sh '/usr/local/share/crosstool-ng/scripts/build/companion_libs'
2020-01-20T09:22:31.9792810Z  /usr/bin/install -c -m 644  packages/gmp/4.3.2/chksum packages/gmp/4.3.2/version.desc '/usr/local/share/crosstool-ng/packages/gmp/4.3.2'
2020-01-20T09:22:31.9810246Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/contrib'
2020-01-20T09:22:31.9810246Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/contrib'
2020-01-20T09:22:31.9822612Z  /usr/bin/install -c -m 644  contrib/openrisc-or32.patch.lzma '/usr/local/share/crosstool-ng/contrib'
2020-01-20T09:22:31.9858966Z  /usr/bin/install -c -m 644  samples/x86_64-w64-mingw32/crosstool.config samples/x86_64-w64-mingw32/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-w64-mingw32'
2020-01-20T09:22:31.9878580Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/libc'
2020-01-20T09:22:31.9891626Z  /usr/bin/install -c -m 644  config/libc/avr-libc.in config/libc/bionic.in config/libc/glibc.in config/libc/mingw-w64.in config/libc/moxiebox.in config/libc/musl.in config/libc/newlib.in config/libc/none.in config/libc/uClibc.in '/usr/local/share/crosstool-ng/config/libc'
2020-01-20T09:22:31.9916843Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc64-multilib-linux-gnu'
---
2020-01-20T09:22:32.0027346Z  /usr/bin/install -c -m 644  config/binutils.in config/cc.in config/comp_libs.in config/comp_tools.in config/config.in config/debug.in config/global.in config/kernel.in config/libc.in config/target.in config/test_suite.in config/toolchain.in '/usr/local/share/crosstool-ng/config'
2020-01-20T09:22:32.0053003Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/automake/1.14.1'
2020-01-20T09:22:32.0068163Z  /usr/bin/install -c -m 644  packages/automake/1.14.1/0000-escape-left-brace.patch packages/automake/1.14.1/chksum packages/automake/1.14.1/version.desc '/usr/local/share/crosstool-ng/packages/automake/1.14.1'
2020-01-20T09:22:32.0087987Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc/0.9.33.2'
2020-01-20T09:22:32.0100799Z  /usr/bin/install -c -m 644  packages/uClibc/0.9.33.2/0000-m68k-ice.patch packages/uClibc/0.9.33.2/0001-fix-kernel-3.4plus-build.patch packages/uClibc/0.9.33.2/0002-fix-darwin-build.patch packages/uClibc/0.9.33.2/0003-arm-unwind.patch packages/uClibc/0.9.33.2/0004-no-install-D.patch packages/uClibc/0.9.33.2/0005-prefer-multilib.patch packages/uClibc/0.9.33.2/0006-dlopen-static.patch packages/uClibc/0.9.33.2/0007-make-olddefconfig.patch packages/uClibc/0.9.33.2/chksum packages/uClibc/0.9.33.2/version.desc '/usr/local/share/crosstool-ng/packages/uClibc/0.9.33.2'
2020-01-20T09:22:32.0134562Z  /usr/bin/install -c -m 644  samples/armeb-unknown-linux-uclibcgnueabi/crosstool.config samples/armeb-unknown-linux-uclibcgnueabi/reported.by '/usr/local/share/crosstool-ng/samples/armeb-unknown-linux-uclibcgnueabi'
2020-01-20T09:22:32.0152403Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/alphaev56-unknown-linux-gnu'
2020-01-20T09:22:32.0167531Z  /usr/bin/install -c -m 644  samples/alphaev56-unknown-linux-gnu/crosstool.config samples/alphaev56-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/alphaev56-unknown-linux-gnu'
2020-01-20T09:22:32.0185777Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-ubuntu12.04-linux-gnu'
---
2020-01-20T09:22:32.0454803Z  /usr/bin/install -c -m 644  samples/armeb-unknown-eabi/crosstool.config samples/armeb-unknown-eabi/reported.by '/usr/local/share/crosstool-ng/samples/armeb-unknown-eabi'
2020-01-20T09:22:32.0470804Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gmp/6.0.0a'
2020-01-20T09:22:32.0484927Z  /usr/bin/install -c -m 644  packages/gmp/6.0.0a/chksum packages/gmp/6.0.0a/version.desc '/usr/local/share/crosstool-ng/packages/gmp/6.0.0a'
2020-01-20T09:22:32.0507067Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/global'
2020-01-20T09:22:32.0520184Z  /usr/bin/install -c -m 644  config/global/build-behave.in config/global/ct-behave.in config/global/download.in config/global/extract.in config/global/logging.in config/global/paths.in '/usr/local/share/crosstool-ng/config/global'
2020-01-20T09:22:32.0560845Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.24'
2020-01-20T09:22:32.0578766Z  /usr/bin/install -c -m 644  packages/binutils/2.24/0000-fix-enable-install-libiberty-flag.patch packages/binutils/2.24/0001-dont-segv-on-initial-instructions-overflow.patch packages/binutils/2.24/0002-sh-conf.patch packages/binutils/2.24/0003-ld_makefile_patch.patch packages/binutils/2.24/0004-check_ldrunpath_length.patch packages/binutils/2.24/0005-fix-gold-pthreads-typo.patch packages/binutils/2.24/0006-sysroot.patch packages/binutils/2.24/0007-poison-system-directories.patch packages/binutils/2.24/0008-Fix-library-paths-on-PowerPC.patch packages/binutils/2.24/0009-xtensa-trampolines.patch packages/binutils/2.24/0010-xtensa-gas-first-frag-alignment.patch packages/binutils/2.24/0011-xtensa-gas-ld-diff-relocation-signed.patch packages/binutils/2.24/0012-xtensa-fix-ld-segfault-when-linking-linux-modules.patch packages/binutils/2.24/0013-Fix-call8-call-target-out-of-range-xtensa-ld-relaxation.patch packages/binutils/2.24/0014-Fix-trampolines-search-code-for-conditional-branches.patch packages/binutils/2.24/0015-xtensa-optimize-check_section_ebb_pcrels_fit.patch packages/binutils/2.24/0016-xtensa-optimize-removed_by_actions.patch packages/binutils/2.24/0017-xtensa-optimize-find_removed_literal.patch packages/binutils/2.24/0018-xtensa-replace-action-list-with-splay-tree.patch packages/binutils/2.24/0019-xtensa-optimize-trampolines-relaxation.patch packages/binutils/2.24/0020-xtensa-fix-localized-symbol-refcounting-with-gc-sect.patch packages/binutils/2.24/0021-xtensa-fix-gas-segfault-with-text-section-literals.patch packages/binutils/2.24/0022-xtensa-add-auto-litpools-option.patch packages/binutils/2.24/0023-xtensa-fix-signedness-of-gas-relocations.patch packages/binutils/2.24/0024-xtensa-fix-.init-.fini-literals-moving.patch packages/binutils/2.24/chksum packages/binutils/2.24/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.24'
2020-01-20T09:22:32.0617257Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.30/chksum packages/uClibc-ng/1.0.30/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.30'
2020-01-20T09:22:32.0644266Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.27'
2020-01-20T09:22:32.0658074Z  /usr/bin/install -c -m 644  packages/binutils/2.27/0000-missing-break.patch packages/binutils/2.27/0001-sh-conf.patch packages/binutils/2.27/0002-ld_makefile_patch.patch packages/binutils/2.27/0003-check_ldrunpath_length.patch packages/binutils/2.27/0004-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.27/0005-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.27/0006-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.27/0007-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.27/0008-sysroot.patch packages/binutils/2.27/0009-poison-system-directories.patch packages/binutils/2.27/0010-Fix-library-paths-on-PowerPC.patch packages/binutils/2.27/0011-xtensa-reverse-shift-count.patch packages/binutils/2.27/chksum packages/binutils/2.27/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.27'
2020-01-20T09:22:32.0687042Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/companion_tools'
2020-01-20T09:22:32.0687042Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/companion_tools'
2020-01-20T09:22:32.0698962Z  /usr/bin/install -c -m 644  scripts/build/companion_tools/050-make.sh scripts/build/companion_tools/100-m4.sh scripts/build/companion_tools/200-autoconf.sh scripts/build/companion_tools/300-automake.sh scripts/build/companion_tools/400-libtool.sh scripts/build/companion_tools/500-dtc.sh scripts/build/companion_tools/510-bison.sh '/usr/local/share/crosstool-ng/scripts/build/companion_tools'
2020-01-20T09:22:32.0736409Z  /usr/bin/install -c -m 644  packages/mingw-w64/v3.1.0/0000-mingw64-malloc.patch packages/mingw-w64/v3.1.0/0001-gendef-explicit-fallthrough.patch packages/mingw-w64/v3.1.0/0002-genpeimg-explicit-fallthrough.patch packages/mingw-w64/v3.1.0/chksum packages/mingw-w64/v3.1.0/version.desc '/usr/local/share/crosstool-ng/packages/mingw-w64/v3.1.0'
2020-01-20T09:22:32.0756256Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.31'
2020-01-20T09:22:32.0772173Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.31/chksum packages/uClibc-ng/1.0.31/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.31'
2020-01-20T09:22:32.0788382Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/cc'
---
2020-01-20T09:22:32.1411995Z  /usr/bin/install -c -m 644  packages/expat/2.1.1/chksum packages/expat/2.1.1/version.desc '/usr/local/share/crosstool-ng/packages/expat/2.1.1'
2020-01-20T09:22:32.1441557Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.7.1-2014.06-1'
2020-01-20T09:22:32.1455256Z  /usr/bin/install -c -m 644  packages/gdb-linaro/7.7.1-2014.06-1/chksum packages/gdb-linaro/7.7.1-2014.06-1/version.desc '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.7.1-2014.06-1'
2020-01-20T09:22:32.1483981Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/make/4.2.1'
2020-01-20T09:22:32.1498075Z  /usr/bin/install -c -m 644  packages/make/4.2.1/0000-glob-v2.patch packages/make/4.2.1/0001-glob-v2-gl_lstat.patch packages/make/4.2.1/chksum packages/make/4.2.1/version.desc '/usr/local/share/crosstool-ng/packages/make/4.2.1'
2020-01-20T09:22:32.1535483Z  /usr/bin/install -c -m 644  packages/gdb/8.1.1/0000-musl_fix.patch packages/gdb/8.1.1/0001-uclibc-no-gettimeofday-clobber.patch packages/gdb/8.1.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/8.1.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/8.1.1/0004-allow-android.patch packages/gdb/8.1.1/chksum packages/gdb/8.1.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/8.1.1'
2020-01-20T09:22:32.1573739Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc'
2020-01-20T09:22:32.1589637Z  /usr/bin/install -c -m 644  packages/uClibc/config packages/uClibc/package.desc '/usr/local/share/crosstool-ng/packages/uClibc'
2020-01-20T09:22:32.1610347Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.9.1'
2020-01-20T09:22:32.1610347Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.9.1'
2020-01-20T09:22:32.1624876Z  /usr/bin/install -c -m 644  packages/gdb/7.9.1/chksum packages/gdb/7.9.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.9.1'
2020-01-20T09:22:32.1645474Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/s390x-ibm-linux-gnu'
2020-01-20T09:22:32.1661982Z  /usr/bin/install -c -m 644  samples/s390x-ibm-linux-gnu/crosstool.config samples/s390x-ibm-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/s390x-ibm-linux-gnu'
2020-01-20T09:22:32.1679940Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/moxie-unknown-elf'
2020-01-20T09:22:32.1695318Z  /usr/bin/install -c -m 644  samples/moxie-unknown-elf/crosstool.config samples/moxie-unknown-elf/reported.by '/usr/local/share/crosstool-ng/samples/moxie-unknown-elf'
2020-01-20T09:22:32.1718493Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/ltrace/0.7.3'
2020-01-20T09:22:32.1731567Z  /usr/bin/install -c -m 644  packages/ltrace/0.7.3/0000-avoid-libstdc++.patch packages/ltrace/0.7.3/0001-printf-p.patch packages/ltrace/0.7.3/0002-alpha-debug.h.patch packages/ltrace/0.7.3/0003-compile-warning.patch packages/ltrace/0.7.3/0004-sparc-ftbfs.patch packages/ltrace/0.7.3/0005-unexpected-breakpoint.patch packages/ltrace/0.7.3/0006-gcc-5.patch packages/ltrace/0.7.3/0007-glibc-2.24.patch packages/ltrace/0.7.3/chksum packages/ltrace/0.7.3/version.desc '/usr/local/share/crosstool-ng/packages/ltrace/0.7.3'
2020-01-20T09:22:32.1774147Z  /usr/bin/install -c -m 644  scripts/build/kernel/bare-metal.sh scripts/build/kernel/linux.sh scripts/build/kernel/windows.sh '/usr/local/share/crosstool-ng/scripts/build/kernel'
2020-01-20T09:22:32.1793653Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mingw-w64/v4.0.6'
2020-01-20T09:22:32.1806650Z  /usr/bin/install -c -m 644  packages/mingw-w64/v4.0.6/0000-mingw64-malloc.patch packages/mingw-w64/v4.0.6/0001-gendef-explicit-fallthrough.patch packages/mingw-w64/v4.0.6/0002-genpeimg-explicit-fallthrough.patch packages/mingw-w64/v4.0.6/chksum packages/mingw-w64/v4.0.6/version.desc '/usr/local/share/crosstool-ng/packages/mingw-w64/v4.0.6'
2020-01-20T09:22:32.1824943Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/3.14.79'
---
2020-01-20T09:22:37.4063356Z  ---> Running in f01d540d9d72
2020-01-20T09:22:38.9974042Z Removing intermediate container f01d540d9d72
2020-01-20T09:22:38.9975110Z  ---> ef7e28d20d95
2020-01-20T09:22:38.9975341Z Step 9/67 : RUN ./build-riscv-toolchain.sh
2020-01-20T09:22:39.0544323Z  ---> Running in 7ed457c7be91
2020-01-20T09:22:39.5033173Z + mkdir -p /tmp/build-riscv
2020-01-20T09:22:39.5118489Z + cp riscv64-unknown-linux-gnu.config /tmp/build-riscv/.config
2020-01-20T09:22:39.5118809Z + cd /tmp/build-riscv
2020-01-20T09:22:39.5119115Z + set +x
2020-01-20T09:23:09.5145760Z Mon Jan 20 09:23:09 UTC 2020 - building ...
2020-01-20T09:23:39.5177214Z Mon Jan 20 09:23:39 UTC 2020 - building ...
2020-01-20T09:24:09.5336797Z Mon Jan 20 09:24:09 UTC 2020 - building ...
---
2020-01-20T10:12:11.9424065Z  ---> Running in adcde61c8b16
2020-01-20T10:12:12.3704281Z + mkdir /usr/local/mips-linux-musl
2020-01-20T10:12:12.3785796Z + URL=https://ci-mirrors.rust-lang.org/rustc
2020-01-20T10:12:12.3786463Z + FILE=OpenWrt-Toolchain-ar71xx-generic_gcc-5.3.0_musl-1.1.16.Linux-x86_64.tar.bz2
2020-01-20T10:12:12.3794972Z + + curltar -L xjf - -C https://ci-mirrors.rust-lang.org/rustc/OpenWrt-Toolchain-ar71xx-generic_gcc-5.3.0_musl-1.1.16.Linux-x86_64.tar.bz2 /usr/local/mips-linux-musl
2020-01-20T10:12:12.3795631Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2020-01-20T10:12:12.3797727Z                                  Dload  Upload   Total   Spent    Left  Speed
2020-01-20T10:12:12.3798204Z 
2020-01-20T10:12:12.9612518Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2020-01-20T10:18:13.1661800Z Step 60/67 : ENV TARGETS=$TARGETS,thumbv7neon-unknown-linux-gnueabihf
2020-01-20T10:18:13.2350991Z  ---> Running in 36374dfd942a
2020-01-20T10:18:15.3248827Z Removing intermediate container 36374dfd942a
2020-01-20T10:18:15.3249644Z  ---> a021e19440fb
2020-01-20T10:18:15.3249955Z Step 61/67 : ENV TARGETS=$TARGETS,armv7a-none-eabi
2020-01-20T10:18:15.3824063Z  ---> Running in 6c8421c6b7ea
2020-01-20T10:18:17.4259944Z Removing intermediate container 6c8421c6b7ea
2020-01-20T10:18:17.4260800Z  ---> 0d72c47c44d3
2020-01-20T10:18:17.4261239Z Step 62/67 : ENV TARGETS=$TARGETS,armv7a-none-eabihf
2020-01-20T10:18:19.4372965Z Removing intermediate container c86c49965823
2020-01-20T10:18:19.4374398Z  ---> a47f8a297ac1
2020-01-20T10:18:19.4374398Z  ---> a47f8a297ac1
2020-01-20T10:18:19.4376004Z Step 63/67 : ENV CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc     CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc     CC_mips64el_unknown_linux_muslabi64=mips64el-linux-gnuabi64-gcc     CC_mips64_unknown_linux_muslabi64=mips64-linux-gnuabi64-gcc     CC_sparc64_unknown_linux_gnu=sparc64-linux-gnu-gcc     CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc     CC_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc     AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar     CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++     CC_armv7a_none_eabi=arm-none-eabi-gcc     CC_armv7a_none_eabihf=arm-none-eabi-gcc     CC_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-gcc     AR_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-ar     CXX_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-g++     CC_riscv32i_unknown_none_elf=false     CC_riscv32imc_unknown_none_elf=false     CC_riscv32imac_unknown_none_elf=false     CC_riscv64imac_unknown_none_elf=false     CC_riscv64gc_unknown_none_elf=false
2020-01-20T10:18:21.6085036Z Removing intermediate container 437c684308d0
2020-01-20T10:18:21.6085851Z  ---> da900c1eda58
2020-01-20T10:18:21.6086485Z Step 64/67 : ENV RUST_CONFIGURE_ARGS       --musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7hf=/musl-armv7hf       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --musl-root-mips64=/musl-mips64       --musl-root-mips64el=/musl-mips64el       --disable-docs
2020-01-20T10:18:21.6821563Z  ---> Running in b9e24280bf56
---
2020-01-20T11:26:01.0038105Z 
2020-01-20T11:26:01.0040712Z  finished in 0.530
2020-01-20T11:26:01.0044226Z [TIMING] Compiletest { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7em-none-eabihf", mode: "run-make", suite: "run-make", path: Some("src/test/run-make"), compare_mode: None } -- 0.543
2020-01-20T11:26:01.0101728Z Build completed successfully in 1:01:50
2020-01-20T11:26:01.0114984Z + python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,riscv64gc-unknown-linux-gnu,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi,armv7a-none-eabihf
2020-01-20T11:26:02.7873838Z Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-20T11:26:03.0689899Z [RUSTC-TIMING] core test:false 22.685
2020-01-20T11:26:03.0697616Z [RUSTC-TIMING] build_script_build test:false 0.414
2020-01-20T11:26:03.0708269Z [RUSTC-TIMING] build_script_build test:false 0.875
---
2020-01-20T11:58:49.7727533Z [TIMING] Std { target: "thumbv7neon-unknown-linux-gnueabihf", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 64.015
2020-01-20T11:58:49.7740226Z Dist std stage2 (x86_64-unknown-linux-gnu -> thumbv7neon-unknown-linux-gnueabihf)
2020-01-20T11:59:05.2972758Z  finished in 15.523
2020-01-20T11:59:05.2974612Z [TIMING] Std { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "thumbv7neon-unknown-linux-gnueabihf" } -- 15.524
2020-01-20T11:59:05.2984512Z Building stage2 std artifacts (x86_64-unknown-linux-gnu -> armv7a-none-eabi)
2020-01-20T11:59:05.4845827Z    Compiling compiler_builtins v0.1.24
2020-01-20T11:59:17.3492612Z    Compiling rustc-std-workspace-core v1.99.0 (/checkout/src/tools/rustc-std-workspace-core)
2020-01-20T11:59:17.3763309Z [RUSTC-TIMING] rustc_std_workspace_core test:false 0.028
2020-01-20T11:59:20.2788612Z [RUSTC-TIMING] compiler_builtins test:false 2.897
2020-01-20T11:59:20.2788612Z [RUSTC-TIMING] compiler_builtins test:false 2.897
2020-01-20T11:59:20.2835372Z    Compiling alloc v0.0.0 (/checkout/src/liballoc)
2020-01-20T11:59:23.5517735Z [RUSTC-TIMING] core test:false 18.064
2020-01-20T11:59:23.5792828Z [RUSTC-TIMING] alloc test:false 3.292
2020-01-20T11:59:23.5802341Z     Finished release [optimized + debuginfo] target(s) in 18.27s
2020-01-20T11:59:23.5895740Z Copying stage2 std from stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / armv7a-none-eabi)
2020-01-20T11:59:23.5900778Z [TIMING] Std { target: "armv7a-none-eabi", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } } -- 18.291
2020-01-20T11:59:23.5909887Z Dist std stage2 (x86_64-unknown-linux-gnu -> armv7a-none-eabi)
2020-01-20T11:59:34.3142734Z  finished in 10.723
2020-01-20T11:59:34.3144047Z [TIMING] Std { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "armv7a-none-eabi" } -- 10.724
2020-01-20T11:59:34.3156805Z Building stage2 std artifacts (x86_64-unknown-linux-gnu -> armv7a-none-eabihf)
2020-01-20T11:59:34.4898892Z    Compiling compiler_builtins v0.1.24
2020-01-20T11:59:36.3732778Z error: failed to run custom build command for `compiler_builtins v0.1.24`
2020-01-20T11:59:36.3736399Z 
2020-01-20T11:59:36.3736530Z Caused by:
2020-01-20T11:59:36.3736530Z Caused by:
2020-01-20T11:59:36.3746766Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/compiler_builtins-31cd637e77517490/build-script-build` (exit code: 1)
2020-01-20T11:59:36.3747010Z --- stdout
2020-01-20T11:59:36.3747216Z cargo:rerun-if-changed=build.rs
2020-01-20T11:59:36.3747526Z cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.24/compiler-rt
2020-01-20T11:59:36.3747751Z cargo:rustc-cfg=feature="unstable"
2020-01-20T11:59:36.3748179Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c
2020-01-20T11:59:36.3748794Z cargo:rustc-cfg=__absvdi2="optimized-c"
2020-01-20T11:59:36.3749158Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c
2020-01-20T11:59:36.3749423Z cargo:rustc-cfg=__absvsi2="optimized-c"
2020-01-20T11:59:36.3749744Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c
2020-01-20T11:59:36.3750024Z cargo:rustc-cfg=__absvti2="optimized-c"
2020-01-20T11:59:36.3750331Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c
2020-01-20T11:59:36.3750609Z cargo:rustc-cfg=__addvdi3="optimized-c"
2020-01-20T11:59:36.3750923Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c
2020-01-20T11:59:36.3751203Z cargo:rustc-cfg=__addvsi3="optimized-c"
2020-01-20T11:59:36.3751504Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c
2020-01-20T11:59:36.3751791Z cargo:rustc-cfg=__addvti3="optimized-c"
2020-01-20T11:59:36.3752244Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S
2020-01-20T11:59:36.3752468Z cargo:rustc-cfg=__aeabi_cdcmp="optimized-c"
2020-01-20T11:59:36.3752736Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c
2020-01-20T11:59:36.3752962Z cargo:rustc-cfg=__aeabi_cdcmpeq_check_nan="optimized-c"
2020-01-20T11:59:36.3753221Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S
2020-01-20T11:59:36.3753429Z cargo:rustc-cfg=__aeabi_cfcmp="optimized-c"
2020-01-20T11:59:36.3753700Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c
2020-01-20T11:59:36.3753928Z cargo:rustc-cfg=__aeabi_cfcmpeq_check_nan="optimized-c"
2020-01-20T11:59:36.3754189Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_div0.c
2020-01-20T11:59:36.3754419Z cargo:rustc-cfg=__aeabi_div0="optimized-c"
2020-01-20T11:59:36.3754659Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_drsub.c
2020-01-20T11:59:36.3754876Z cargo:rustc-cfg=__aeabi_drsub="optimized-c"
2020-01-20T11:59:36.3755115Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_frsub.c
2020-01-20T11:59:36.3755338Z cargo:rustc-cfg=__aeabi_frsub="optimized-c"
2020-01-20T11:59:36.3755574Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapdi2.S
2020-01-20T11:59:36.3755795Z cargo:rustc-cfg=__bswapdi2="optimized-c"
2020-01-20T11:59:36.3756031Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapsi2.S
2020-01-20T11:59:36.3756400Z cargo:rustc-cfg=__bswapsi2="optimized-c"
2020-01-20T11:59:36.3756701Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzdi2.S
2020-01-20T11:59:36.3757044Z cargo:rustc-cfg=__clzdi2="optimized-c"
2020-01-20T11:59:36.3757322Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzsi2.S
2020-01-20T11:59:36.3757542Z cargo:rustc-cfg=__clzsi2="optimized-c"
2020-01-20T11:59:36.3757809Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c
2020-01-20T11:59:36.3758204Z cargo:rustc-cfg=__clzti2="optimized-c"
2020-01-20T11:59:36.3758920Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c
2020-01-20T11:59:36.3759211Z cargo:rustc-cfg=__cmpdi2="optimized-c"
2020-01-20T11:59:36.3759517Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c
2020-01-20T11:59:36.3759795Z cargo:rustc-cfg=__cmpti2="optimized-c"
2020-01-20T11:59:36.3760111Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c
2020-01-20T11:59:36.3760392Z cargo:rustc-cfg=__ctzdi2="optimized-c"
2020-01-20T11:59:36.3760696Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c
2020-01-20T11:59:36.3760983Z cargo:rustc-cfg=__ctzsi2="optimized-c"
2020-01-20T11:59:36.3761281Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c
2020-01-20T11:59:36.3761557Z cargo:rustc-cfg=__ctzti2="optimized-c"
2020-01-20T11:59:36.3762038Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c
2020-01-20T11:59:36.3762242Z cargo:rustc-cfg=__divdc3="optimized-c"
2020-01-20T11:59:36.3762497Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divmodsi4.S
2020-01-20T11:59:36.3762704Z cargo:rustc-cfg=__divmodsi4="optimized-c"
2020-01-20T11:59:36.3762951Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c
2020-01-20T11:59:36.3763150Z cargo:rustc-cfg=__divsc3="optimized-c"
2020-01-20T11:59:36.3763410Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divsi3.S
2020-01-20T11:59:36.3763633Z cargo:rustc-cfg=__divsi3="optimized-c"
2020-01-20T11:59:36.3763899Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c
2020-01-20T11:59:36.3764134Z cargo:rustc-cfg=__divxc3="optimized-c"
2020-01-20T11:59:36.3764385Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c
2020-01-20T11:59:36.3764623Z cargo:rustc-cfg=__extendhfsf2="optimized-c"
2020-01-20T11:59:36.3764869Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c
2020-01-20T11:59:36.3765098Z cargo:rustc-cfg=__ffsti2="optimized-c"
2020-01-20T11:59:36.3765349Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S
2020-01-20T11:59:36.3765582Z cargo:rustc-cfg=__fixdfsivfp="optimized-c"
2020-01-20T11:59:36.3765837Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixsfsivfp.S
2020-01-20T11:59:36.3766080Z cargo:rustc-cfg=__fixsfsivfp="optimized-c"
2020-01-20T11:59:36.3766355Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixunsdfsivfp.S
2020-01-20T11:59:36.3766588Z cargo:rustc-cfg=__fixunsdfsivfp="optimized-c"
2020-01-20T11:59:36.3766864Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixunssfsivfp.S
2020-01-20T11:59:36.3767088Z cargo:rustc-cfg=__fixunssfsivfp="optimized-c"
2020-01-20T11:59:36.3767358Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatsidfvfp.S
2020-01-20T11:59:36.3767582Z cargo:rustc-cfg=__floatsidfvfp="optimized-c"
2020-01-20T11:59:36.3767856Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatsisfvfp.S
2020-01-20T11:59:36.3768273Z cargo:rustc-cfg=__floatsisfvfp="optimized-c"
2020-01-20T11:59:36.3768922Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatunssidfvfp.S
2020-01-20T11:59:36.3769618Z cargo:rustc-cfg=__floatunssidfvfp="optimized-c"
2020-01-20T11:59:36.3769993Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/floatunssisfvfp.S
2020-01-20T11:59:36.3770388Z cargo:rustc-cfg=__floatunssisfvfp="optimized-c"
2020-01-20T11:59:36.3770698Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/int_util.c
2020-01-20T11:59:36.3770980Z cargo:rustc-cfg=__int_util="optimized-c"
2020-01-20T11:59:36.3771306Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/modsi3.S
2020-01-20T11:59:36.3771572Z cargo:rustc-cfg=__modsi3="optimized-c"
2020-01-20T11:59:36.3771889Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/muldc3.c
2020-01-20T11:59:36.3772310Z cargo:rustc-cfg=__muldc3="optimized-c"
2020-01-20T11:59:36.3772558Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulsc3.c
2020-01-20T11:59:36.3772757Z cargo:rustc-cfg=__mulsc3="optimized-c"
2020-01-20T11:59:36.3773014Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvdi3.c
2020-01-20T11:59:36.3773217Z cargo:rustc-cfg=__mulvdi3="optimized-c"
2020-01-20T11:59:36.3773475Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvsi3.c
2020-01-20T11:59:36.3773693Z cargo:rustc-cfg=__mulvsi3="optimized-c"
2020-01-20T11:59:36.3773926Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulvti3.c
2020-01-20T11:59:36.3774140Z cargo:rustc-cfg=__mulvti3="optimized-c"
2020-01-20T11:59:36.3774371Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/mulxc3.c
2020-01-20T11:59:36.3774588Z cargo:rustc-cfg=__mulxc3="optimized-c"
2020-01-20T11:59:36.3774818Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdf2.c
2020-01-20T11:59:36.3775033Z cargo:rustc-cfg=__negdf2="optimized-c"
2020-01-20T11:59:36.3775272Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/negdf2vfp.S
2020-01-20T11:59:36.3775500Z cargo:rustc-cfg=__negdf2vfp="optimized-c"
2020-01-20T11:59:36.3775753Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negdi2.c
2020-01-20T11:59:36.3775959Z cargo:rustc-cfg=__negdi2="optimized-c"
2020-01-20T11:59:36.3776203Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negsf2.c
2020-01-20T11:59:36.3776404Z cargo:rustc-cfg=__negsf2="optimized-c"
2020-01-20T11:59:36.3776658Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/negsf2vfp.S
2020-01-20T11:59:36.3776862Z cargo:rustc-cfg=__negsf2vfp="optimized-c"
2020-01-20T11:59:36.3777109Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negti2.c
2020-01-20T11:59:36.3777321Z cargo:rustc-cfg=__negti2="optimized-c"
2020-01-20T11:59:36.3777554Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvdi2.c
2020-01-20T11:59:36.3777768Z cargo:rustc-cfg=__negvdi2="optimized-c"
2020-01-20T11:59:36.3778184Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvsi2.c
2020-01-20T11:59:36.3778784Z cargo:rustc-cfg=__negvsi2="optimized-c"
2020-01-20T11:59:36.3779123Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/negvti2.c
2020-01-20T11:59:36.3779400Z cargo:rustc-cfg=__negvti2="optimized-c"
2020-01-20T11:59:36.3779704Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritydi2.c
2020-01-20T11:59:36.3779986Z cargo:rustc-cfg=__paritydi2="optimized-c"
2020-01-20T11:59:36.3780309Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/paritysi2.c
2020-01-20T11:59:36.3780577Z cargo:rustc-cfg=__paritysi2="optimized-c"
2020-01-20T11:59:36.3780896Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/parityti2.c
2020-01-20T11:59:36.3781160Z cargo:rustc-cfg=__parityti2="optimized-c"
2020-01-20T11:59:36.3781485Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountdi2.c
2020-01-20T11:59:36.3781851Z cargo:rustc-cfg=__popcountdi2="optimized-c"
2020-01-20T11:59:36.3792623Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountsi2.c
2020-01-20T11:59:36.3793022Z cargo:rustc-cfg=__popcountsi2="optimized-c"
2020-01-20T11:59:36.3793292Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/popcountti2.c
2020-01-20T11:59:36.3793519Z cargo:rustc-cfg=__popcountti2="optimized-c"
2020-01-20T11:59:36.3793759Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/powixf2.c
2020-01-20T11:59:36.3793979Z cargo:rustc-cfg=__powixf2="optimized-c"
2020-01-20T11:59:36.3794236Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/restore_vfp_d8_d15_regs.S
2020-01-20T11:59:36.3794482Z cargo:rustc-cfg=__restore_vfp_d8_d15_regs="optimized-c"
2020-01-20T11:59:36.3794735Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/save_vfp_d8_d15_regs.S
2020-01-20T11:59:36.3794977Z cargo:rustc-cfg=__save_vfp_d8_d15_regs="optimized-c"
2020-01-20T11:59:36.3795241Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvdi3.c
2020-01-20T11:59:36.3795456Z cargo:rustc-cfg=__subvdi3="optimized-c"
2020-01-20T11:59:36.3795706Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvsi3.c
2020-01-20T11:59:36.3795909Z cargo:rustc-cfg=__subvsi3="optimized-c"
2020-01-20T11:59:36.3796158Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/subvti3.c
2020-01-20T11:59:36.3796360Z cargo:rustc-cfg=__subvti3="optimized-c"
2020-01-20T11:59:36.3796618Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch16.S
2020-01-20T11:59:36.3796823Z cargo:rustc-cfg=__switch16="optimized-c"
2020-01-20T11:59:36.3797079Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch32.S
2020-01-20T11:59:36.3797298Z cargo:rustc-cfg=__switch32="optimized-c"
2020-01-20T11:59:36.3797542Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switch8.S
2020-01-20T11:59:36.3797761Z cargo:rustc-cfg=__switch8="optimized-c"
2020-01-20T11:59:36.3798170Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/switchu8.S
2020-01-20T11:59:36.3799018Z cargo:rustc-cfg=__switchu8="optimized-c"
2020-01-20T11:59:36.3799378Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/sync_synchronize.S
2020-01-20T11:59:36.3799679Z cargo:rustc-cfg=__sync_synchronize="optimized-c"
2020-01-20T11:59:36.3800005Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfhf2.c
2020-01-20T11:59:36.3800306Z cargo:rustc-cfg=__truncdfhf2="optimized-c"
2020-01-20T11:59:36.3800661Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncdfsf2.c
2020-01-20T11:59:36.3800948Z cargo:rustc-cfg=__truncdfsf2="optimized-c"
2020-01-20T11:59:36.3801299Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/truncsfhf2.c
2020-01-20T11:59:36.3801598Z cargo:rustc-cfg=__truncsfhf2="optimized-c"
2020-01-20T11:59:36.3801950Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpdi2.c
2020-01-20T11:59:36.3802360Z cargo:rustc-cfg=__ucmpdi2="optimized-c"
2020-01-20T11:59:36.3802613Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/ucmpti2.c
2020-01-20T11:59:36.3802831Z cargo:rustc-cfg=__ucmpti2="optimized-c"
2020-01-20T11:59:36.3803072Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/udivmodsi4.S
2020-01-20T11:59:36.3803293Z cargo:rustc-cfg=__udivmodsi4="optimized-c"
2020-01-20T11:59:36.3803531Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/udivsi3.S
2020-01-20T11:59:36.3804105Z cargo:rustc-cfg=__udivsi3="optimized-c"
2020-01-20T11:59:36.3804537Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/umodsi3.S
2020-01-20T11:59:36.3805003Z cargo:rustc-cfg=__umodsi3="optimized-c"
2020-01-20T11:59:36.3805579Z cargo:rerun-if-changed=/checkout/src/llvm-project/compiler-rt/lib/builtins/apple_versioning.c
2020-01-20T11:59:36.3805869Z cargo:rustc-cfg=apple_versioning="optimized-c"
2020-01-20T11:59:36.3806201Z TARGET = Some("armv7a-none-eabihf")
2020-01-20T11:59:36.3806271Z OPT_LEVEL = Some("2")
2020-01-20T11:59:36.3806510Z HOST = Some("x86_64-unknown-linux-gnu")
2020-01-20T11:59:36.3806756Z CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
2020-01-20T11:59:36.3807043Z CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
2020-01-20T11:59:36.3807122Z CRATE_CC_NO_DEFAULTS = None
2020-01-20T11:59:36.3807202Z DEBUG = Some("true")
2020-01-20T11:59:36.3807274Z CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2")
2020-01-20T11:59:36.3807547Z CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
2020-01-20T11:59:36.3807834Z CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
2020-01-20T11:59:36.3807914Z CRATE_CC_NO_DEFAULTS = None
2020-01-20T11:59:36.3808012Z CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2")
2020-01-20T11:59:36.3809138Z CC_armv7a-none-eabihf = Some("sccache arm-none-eabi-gcc")
2020-01-20T11:59:36.3809515Z CFLAGS_armv7a-none-eabihf = Some("-ffunction-sections -fdata-sections -fPIC")
2020-01-20T11:59:36.3809604Z CRATE_CC_NO_DEFAULTS = None
2020-01-20T11:59:36.3809701Z CARGO_CFG_TARGET_FEATURE = Some("aclass,dsp,v5te,v6,v6k,v6t2,v7,vfp2")
2020-01-20T11:59:36.3810540Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/absvdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvdi2.c"
2020-01-20T11:59:36.3810771Z exit code: 0
2020-01-20T11:59:36.3811598Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/absvsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvsi2.c"
2020-01-20T11:59:36.3811853Z exit code: 0
2020-01-20T11:59:36.3812959Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/absvti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/absvti2.c"
2020-01-20T11:59:36.3813148Z exit code: 0
2020-01-20T11:59:36.3813839Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/addvdi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvdi3.c"
2020-01-20T11:59:36.3814048Z exit code: 0
2020-01-20T11:59:36.3814737Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/addvsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvsi3.c"
2020-01-20T11:59:36.3815031Z exit code: 0
2020-01-20T11:59:36.3815740Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/addvti3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/addvti3.c"
2020-01-20T11:59:36.3816055Z exit code: 0
2020-01-20T11:59:36.3816796Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_cdcmp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmp.S"
2020-01-20T11:59:36.3817003Z exit code: 0
2020-01-20T11:59:36.3817728Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_cdcmpeq_check_nan.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cdcmpeq_check_nan.c"
2020-01-20T11:59:36.3817944Z exit code: 0
2020-01-20T11:59:36.3819272Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_cfcmp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmp.S"
2020-01-20T11:59:36.3819534Z exit code: 0
2020-01-20T11:59:36.3820404Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_cfcmpeq_check_nan.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_cfcmpeq_check_nan.c"
2020-01-20T11:59:36.3820644Z exit code: 0
2020-01-20T11:59:36.3821479Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_div0.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_div0.c"
2020-01-20T11:59:36.3821732Z exit code: 0
2020-01-20T11:59:36.3823167Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_drsub.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_drsub.c"
2020-01-20T11:59:36.3823388Z exit code: 0
2020-01-20T11:59:36.3824193Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/aeabi_frsub.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/aeabi_frsub.c"
2020-01-20T11:59:36.3824469Z exit code: 0
2020-01-20T11:59:36.3825199Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/bswapdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapdi2.S"
2020-01-20T11:59:36.3825405Z exit code: 0
2020-01-20T11:59:36.3826088Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/bswapsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/bswapsi2.S"
2020-01-20T11:59:36.3826303Z exit code: 0
2020-01-20T11:59:36.3826992Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/clzdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzdi2.S"
2020-01-20T11:59:36.3827208Z exit code: 0
2020-01-20T11:59:36.3827896Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/clzsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/clzsi2.S"
2020-01-20T11:59:36.3828263Z exit code: 0
2020-01-20T11:59:36.3829514Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/clzti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/clzti2.c"
2020-01-20T11:59:36.3829759Z exit code: 0
2020-01-20T11:59:36.3830576Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/cmpdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpdi2.c"
2020-01-20T11:59:36.3830821Z exit code: 0
2020-01-20T11:59:36.3831712Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/cmpti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/cmpti2.c"
2020-01-20T11:59:36.3832124Z exit code: 0
2020-01-20T11:59:36.3833098Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/ctzdi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzdi2.c"
2020-01-20T11:59:36.3833303Z exit code: 0
2020-01-20T11:59:36.3833977Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/ctzsi2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzsi2.c"
2020-01-20T11:59:36.3834188Z exit code: 0
2020-01-20T11:59:36.3834874Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/ctzti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ctzti2.c"
2020-01-20T11:59:36.3835076Z exit code: 0
2020-01-20T11:59:36.3835751Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/divdc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divdc3.c"
2020-01-20T11:59:36.3835960Z exit code: 0
2020-01-20T11:59:36.3836655Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/divmodsi4.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divmodsi4.S"
2020-01-20T11:59:36.3836858Z exit code: 0
2020-01-20T11:59:36.3837543Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/divsc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divsc3.c"
2020-01-20T11:59:36.3837736Z exit code: 0
2020-01-20T11:59:36.3839283Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/divsi3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/divsi3.S"
2020-01-20T11:59:36.3839531Z exit code: 0
2020-01-20T11:59:36.3840470Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/divxc3.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/divxc3.c"
2020-01-20T11:59:36.3840789Z exit code: 0
2020-01-20T11:59:36.3841627Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/extendhfsf2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/extendhfsf2.c"
2020-01-20T11:59:36.3841876Z exit code: 0
2020-01-20T11:59:36.3842788Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/ffsti2.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/ffsti2.c"
2020-01-20T11:59:36.3843001Z exit code: 0
2020-01-20T11:59:36.3843683Z running: "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/fixdfsivfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S"
2020-01-20T11:59:36.3844124Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S: Assembler messages:
2020-01-20T11:59:36.3844503Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S:25: Error: selected processor does not support `vmov d7,r0,r1' in ARM mode
2020-01-20T11:59:36.3844903Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S:26: Error: selected processor does not support `vcvt.s32.f64 s15,d7' in ARM mode
2020-01-20T11:59:36.3845268Z cargo:warning=/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S:27: Error: selected processor does not support `vmov r0,s15' in ARM mode
2020-01-20T11:59:36.3845414Z 
2020-01-20T11:59:36.3845608Z --- stderr
2020-01-20T11:59:36.3845641Z 
2020-01-20T11:59:36.3845672Z 
2020-01-20T11:59:36.3845672Z 
2020-01-20T11:59:36.3846481Z error occurred: Command "sccache" "sccache" "arm-none-eabi-gcc" "-O2" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-ffunction-sections" "-fdata-sections" "-fPIC" "-fno-builtin" "-fvisibility=hidden" "-ffreestanding" "-DVISIBILITY_HIDDEN" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/armv7a-none-eabihf/release/build/compiler_builtins-e9e1398806216fc4/out/fixdfsivfp.o" "-c" "/checkout/src/llvm-project/compiler-rt/lib/builtins/arm/fixdfsivfp.S" with args "arm-none-eabi-gcc" did not execute successfully (status code exit code: 1).
2020-01-20T11:59:36.3846734Z 
2020-01-20T11:59:36.3847999Z 
2020-01-20T11:59:36.3848593Z warning: build failed, waiting for other jobs to finish...
2020-01-20T11:59:53.1755091Z [RUSTC-TIMING] core test:false 18.682
2020-01-20T11:59:53.1755091Z [RUSTC-TIMING] core test:false 18.682
2020-01-20T11:59:53.1848526Z error: build failed
2020-01-20T11:59:53.1879446Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "armv7a-none-eabihf" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "-p" "alloc" "--manifest-path" "/checkout/src/liballoc/Cargo.toml" "--features" "compiler-builtins-mem compiler-builtins-c" "--message-format" "json-render-diagnostics"
2020-01-20T11:59:53.1880889Z expected success, got: exit code: 101
2020-01-20T11:59:53.1896933Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,riscv64gc-unknown-linux-gnu,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi,armv7a-none-eabihf
2020-01-20T11:59:53.1953144Z == clock drift check ==
2020-01-20T11:59:53.1966738Z   local time: Mon Jan 20 11:59:53 UTC 2020
2020-01-20T11:59:53.2663384Z   network time: Mon, 20 Jan 2020 11:59:53 GMT
2020-01-20T11:59:53.2666170Z == end clock drift check ==
2020-01-20T11:59:53.2666170Z == end clock drift check ==
2020-01-20T11:59:54.7819924Z 
2020-01-20T11:59:54.8152220Z ##[error]Bash exited with code '1'.
2020-01-20T11:59:54.8211956Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-20T11:59:54.8214268Z ==============================================================================
2020-01-20T11:59:54.8214342Z Task         : Get sources
2020-01-20T11:59:54.8214416Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
