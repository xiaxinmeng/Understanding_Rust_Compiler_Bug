plain
2020-01-21T10:41:36.4039588Z ========================== Starting Command Output ===========================
2020-01-21T10:41:36.4041326Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7eb0fbf9-0269-404d-be62-eb6bfe5635a3.sh
2020-01-21T10:41:36.4041372Z 
2020-01-21T10:41:36.4045897Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T10:41:36.4052410Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68194/merge to s
2020-01-21T10:41:36.4054586Z Task         : Get sources
2020-01-21T10:41:36.4054625Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T10:41:36.4054711Z Version      : 1.0.0
2020-01-21T10:41:36.4054748Z Author       : Microsoft
---
2020-01-21T10:41:37.3832186Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T10:41:37.3844420Z ##[command]git config gc.auto 0
2020-01-21T10:41:37.3847048Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T10:41:37.3849239Z ##[command]git config --get-all http.proxy
2020-01-21T10:41:37.3856224Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68194/merge:refs/remotes/pull/68194/merge
---
2020-01-21T10:45:17.7228049Z Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
2020-01-21T10:45:17.7228108Z 
2020-01-21T10:45:17.7228442Z Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
2020-01-21T10:45:17.7228901Z  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
2020-01-21T10:45:18.1517321Z gpg: keyring `/tmp/tmpa4le3bqe/secring.gpg' created
2020-01-21T10:45:18.1517758Z gpg: keyring `/tmp/tmpa4le3bqe/pubring.gpg' created
2020-01-21T10:45:18.1518652Z gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
2020-01-21T10:45:18.4710151Z gpg: /tmp/tmpa4le3bqe/trustdb.gpg: trustdb created
2020-01-21T10:45:18.4723693Z gpg: no ultimately trusted keys found
2020-01-21T10:45:18.4723815Z gpg: Total number processed: 1
2020-01-21T10:45:18.4723871Z gpg:               imported: 1  (RSA: 1)
2020-01-21T10:45:18.5335392Z OK
---
2020-01-21T10:45:49.1036281Z 100 2427k  100 2427k    0     0  2545k      0 --:--:-- --:--:-- --:--:-- 2544k
2020-01-21T10:45:49.1177546Z + cd crosstool-ng-crosstool-ng-1.24.0
2020-01-21T10:45:49.1180515Z + ./bootstrap
2020-01-21T10:45:49.1239064Z INFO  :: *** Generating package version descriptions
2020-01-21T10:45:49.6168214Z INFO  :: Master packages: android-ndk autoconf automake avr-libc binutils bison cloog dtc duma elf2flt expat gcc gdb gettext glibc-ports glibc gmp isl libelf libiconv libtool linux ltrace m4 make mingw-w64 moxiebox mpc mpfr musl ncurses newlib strace uClibc zlib
2020-01-21T10:45:49.6170909Z INFO  :: Generating 'config/versions/android-ndk.in'
2020-01-21T10:45:50.4674090Z INFO  :: Generating 'config/versions/automake.in'
2020-01-21T10:45:50.7846900Z INFO  :: Generating 'config/versions/avr-libc.in'
2020-01-21T10:45:50.9671370Z INFO  :: Generating 'config/versions/binutils.in'
2020-01-21T10:45:53.7370937Z INFO  :: Generating 'config/versions/bison.in'
---
2020-01-21T10:47:23.3880371Z checking lex output file root... lex.yy
2020-01-21T10:47:23.8989014Z checking lex library... -lfl
2020-01-21T10:47:24.1870917Z checking whether yytext is a pointer... yes
2020-01-21T10:47:24.1891294Z checking for bison... bison -y
2020-01-21T10:47:24.2461562Z checking whether g++ supports C++11 features with -std=gnu++11... yes
2020-01-21T10:47:24.3206103Z checking if gcc can static link... yes
2020-01-21T10:47:24.3221819Z checking for gobjcopy... no
2020-01-21T10:47:24.3226698Z checking for objcopy... objcopy
2020-01-21T10:47:24.3267591Z checking for absolute path to objcopy... /usr/bin/objcopy
2020-01-21T10:47:24.3280917Z checking for gobjdump... no
---
2020-01-21T10:47:25.5870417Z checking for ld used by gcc... /usr/bin/ld
2020-01-21T10:47:25.5904339Z checking if the linker (/usr/bin/ld) is GNU ld... yes
2020-01-21T10:47:25.5936554Z checking for shared library run path origin... done
2020-01-21T10:47:25.6103532Z checking whether NLS is requested... yes
2020-01-21T10:47:25.6285722Z checking for CFPreferencesCopyAppValue... no
2020-01-21T10:47:25.6450160Z checking for CFLocaleCopyCurrent... no
2020-01-21T10:47:25.6898686Z checking whether to use NLS... yes
2020-01-21T10:47:25.6903496Z checking where the gettext function comes from... libc
2020-01-21T10:47:25.6916639Z checking for pkg-config... /usr/bin/pkg-config
2020-01-21T10:47:25.6935398Z checking pkg-config is at least version 0.9.0... yes
---
2020-01-21T10:47:26.3400658Z /usr/bin/make  all-recursive
2020-01-21T10:47:26.3503856Z make[1]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0'
2020-01-21T10:47:26.3545339Z Making all in kconfig
2020-01-21T10:47:26.3583385Z make[2]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-21T10:47:26.3589191Z bison -y -l -b zconf -p zconf  -ozconf.c zconf.y
2020-01-21T10:47:26.3596259Z flex -L -Pzconf  -ozconf.lex.c zconf.l
2020-01-21T10:47:26.4444872Z make[3]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-21T10:47:26.4445751Z depbase=`echo conf.o | sed 's|[^/]*$|.deps/&|;s|\.o$||'`;\
2020-01-21T10:47:26.4446500Z gcc -DHAVE_CONFIG_H -I. -I..  -include config.h -DCONFIG_=\"CT_\"   -g -O2 -MT conf.o -MD -MP -MF $depbase.Tpo -c -o conf.o conf.c &&\
2020-01-21T10:47:26.4446995Z mv -f $depbase.Tpo $depbase.Po
---
2020-01-21T10:47:29.6471470Z libtool: link: gcc -g -O2 -o mconf mconf.o zconf.o lxdialog/checklist.o lxdialog/inputbox.o lxdialog/menubox.o lxdialog/textbox.o lxdialog/util.o lxdialog/yesno.o  -lncurses
2020-01-21T10:47:29.6813000Z make[3]: Leaving directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-21T10:47:29.6818855Z make[2]: Leaving directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
2020-01-21T10:47:29.6919949Z make[2]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0'
2020-01-21T10:47:29.6920667Z ( /bin/sed -e 's,[@]docdir[@],/usr/local/share/doc/crosstool-ng,g' -e 's,[@]pkgdatadir[@],/usr/local/share/crosstool-ng,g' -e 's,[@]pkglibexecdir[@],/usr/local/libexec/crosstool-ng,g' -e 's,[@]progname[@],'`echo ct-ng | sed 's,x,x,'`',g' | /bin/bash config.status --file=- ) < ct-ng.in >ct-ng-t && chmod a-w,a+x ct-ng-t && mv -f ct-ng-t ct-ng
2020-01-21T10:47:29.6921390Z /bin/mkdir -p bash-completion && ( /bin/sed -e 's,[@]docdir[@],/usr/local/share/doc/crosstool-ng,g' -e 's,[@]pkgdatadir[@],/usr/local/share/crosstool-ng,g' -e 's,[@]pkglibexecdir[@],/usr/local/libexec/crosstool-ng,g' -e 's,[@]progname[@],'`echo ct-ng | sed 's,x,x,'`',g' | /bin/bash config.status --file=- ) < bash-completion/ct-ng.in >bash-completion/ct-ng-t && mv -f bash-completion/ct-ng-t bash-completion/ct-ng
2020-01-21T10:47:29.7726592Z /bin/mkdir -p docs && ( /bin/sed -e 's,[@]docdir[@],/usr/local/share/doc/crosstool-ng,g' -e 's,[@]pkgdatadir[@],/usr/local/share/crosstool-ng,g' -e 's,[@]pkglibexecdir[@],/usr/local/libexec/crosstool-ng,g' -e 's,[@]progname[@],'`echo ct-ng | sed 's,x,x,'`',g' | /bin/bash config.status --file=- ) < docs/ct-ng.1.in >docs/ct-ng.1-t && mv -f docs/ct-ng.1-t docs/ct-ng.1
2020-01-21T10:47:30.4630439Z make[1]: Leaving directory '/build/crosstool-ng-crosstool-ng-1.24.0'
2020-01-21T10:47:30.4630662Z + make install
2020-01-21T10:47:30.4631624Z Making install in kconfig
2020-01-21T10:47:30.4632107Z make[1]: Entering directory '/build/crosstool-ng-crosstool-ng-1.24.0/kconfig'
---
2020-01-21T10:47:30.4650682Z  /usr/bin/install -c -m 644  packages/glibc/2.14.1/0000-respect-env-CPPFLAGS.patch packages/glibc/2.14.1/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.14.1/0002-fix-signed-shift-overlow.patch packages/glibc/2.14.1/0003-dl-openat64-variadic.patch packages/glibc/2.14.1/0004-unused-variables.patch packages/glibc/2.14.1/0005-misleading-indentation.patch packages/glibc/2.14.1/0006-dl-open-array-bounds.patch packages/glibc/2.14.1/0007-i386-x86_64-revert-clone-cfi.patch packages/glibc/2.14.1/0008-disable-ldconfig.patch packages/glibc/2.14.1/0009-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.14.1/0010-queue-header-updates.patch packages/glibc/2.14.1/0011-manual-no-perl.patch packages/glibc/2.14.1/0012-localedef-fix-trampoline.patch packages/glibc/2.14.1/0013-resolv-dynamic.patch packages/glibc/2.14.1/0014-localedef-mmap.patch packages/glibc/2.14.1/0015-fadvise64_64.patch packages/glibc/2.14.1/0016-section-comments.patch packages/glibc/2.14.1/0017-no-inline-gmon.patch packages/glibc/2.14.1/0018-assume-pipe2.patch packages/glibc/2.14.1/0019-china.patch packages/glibc/2.14.1/0020-new-valencian-locale.patch packages/glibc/2.14.1/0021-macos-cross-rpcgen.patch packages/glibc/2.14.1/0022-nscd-one-fork.patch packages/glibc/2.14.1/0023-hppa-nptl-carlos.patch packages/glibc/2.14.1/0024-dl_execstack-PaX-support.patch packages/glibc/2.14.1/0025-pre20040117-pt_pax.patch packages/glibc/2.14.1/0026-tests-sandbox-libdl-paths.patch packages/glibc/2.14.1/0027-dont-build-timezone.patch packages/glibc/2.14.1/0028-alpha-xstat.patch packages/glibc/2.14.1/0029-alpha-creat.patch packages/glibc/2.14.1/0030-alpha_alpha-add-fdatasync-support.patch packages/glibc/2.14.1/0031-ppc-atomic.patch packages/glibc/2.14.1/0032-mips_shn_undef-hack.patch packages/glibc/2.14.1/0033-alpha-atfcts.patch packages/glibc/2.14.1/0034-syslog.patch packages/glibc/2.14.1/0035-debug-readlink_chk-readklinkat_chk.patch packages/glibc/2.14.1/0036-cpuid-include.patch packages/glibc/2.14.1/0037-asm-i686.patch packages/glibc/2.14.1/0038-fix-rpc_parse-format.patch packages/glibc/2.14.1/0039-nis-bogus-conditional.patch '/usr/local/share/crosstool-ng/packages/glibc/2.14.1'
2020-01-21T10:47:30.4651317Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-21T10:47:30.4653971Z  /usr/bin/install -c -m 644  packages/glibc/2.15/0000-respect-env-CPPFLAGS.patch packages/glibc/2.15/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.15/0002-fix-signed-shift-overlow.patch packages/glibc/2.15/0003-dl-openat64-variadic.patch packages/glibc/2.15/0004-unused-variables.patch packages/glibc/2.15/0005-misleading-indentation.patch packages/glibc/2.15/0006-dl-open-array-bounds.patch packages/glibc/2.15/0007-i386-x86_64-revert-clone-cfi.patch packages/glibc/2.15/0008-disable-ldconfig.patch packages/glibc/2.15/0009-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.15/0010-queue-header-updates.patch packages/glibc/2.15/0011-manual-no-perl.patch packages/glibc/2.15/0012-localedef-fix-trampoline.patch packages/glibc/2.15/0013-resolv-dynamic.patch packages/glibc/2.15/0014-fadvise64_64.patch packages/glibc/2.15/0015-section-comments.patch packages/glibc/2.15/0016-no-inline-gmon.patch packages/glibc/2.15/0017-assume-pipe2.patch packages/glibc/2.15/0018-china.patch packages/glibc/2.15/0019-new-valencian-locale.patch packages/glibc/2.15/0020-macos-cross-rpcgen.patch packages/glibc/2.15/0021-nscd-one-fork.patch packages/glibc/2.15/0022-hppa-nptl-carlos.patch packages/glibc/2.15/0023-dl_execstack-PaX-support.patch packages/glibc/2.15/0024-pre20040117-pt_pax.patch packages/glibc/2.15/0025-tests-sandbox-libdl-paths.patch packages/glibc/2.15/0026-dont-build-timezone.patch packages/glibc/2.15/0027-alpha-xstat.patch packages/glibc/2.15/0028-alpha-creat.patch packages/glibc/2.15/0029-alpha_alpha-add-fdatasync-support.patch packages/glibc/2.15/0030-ppc-atomic.patch packages/glibc/2.15/0031-mips_shn_undef-hack.patch packages/glibc/2.15/0032-alpha-atfcts.patch packages/glibc/2.15/0033-syslog.patch packages/glibc/2.15/0034-debug-readlink_chk-readklinkat_chk.patch packages/glibc/2.15/0035-cpuid-include.patch packages/glibc/2.15/0036-asm-i686.patch packages/glibc/2.15/0037-fix-rpc_parse-format.patch packages/glibc/2.15/0038-nis-bogus-conditional.patch packages/glibc/2.15/0039-try-link-static.patch '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-21T10:47:30.4654873Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.16.0'
2020-01-21T10:47:30.4657670Z  /usr/bin/install -c -m 644  packages/glibc/2.16.0/0000-respect-env-CPPFLAGS.patch packages/glibc/2.16.0/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.16.0/0002-fix-signed-shift-overlow.patch packages/glibc/2.16.0/0003-dl-openat64-variadic.patch packages/glibc/2.16.0/0004-unused-variables.patch packages/glibc/2.16.0/0005-misleading-indentation.patch packages/glibc/2.16.0/0006-dl-open-array-bounds.patch packages/glibc/2.16.0/0007-i386-x86_64-revert-clone-cfi.patch packages/glibc/2.16.0/0008-disable-ldconfig.patch packages/glibc/2.16.0/0009-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.16.0/0010-queue-header-updates.patch packages/glibc/2.16.0/0011-localedef-fix-trampoline.patch packages/glibc/2.16.0/0012-resolv-dynamic.patch packages/glibc/2.16.0/0013-fadvise64_64.patch packages/glibc/2.16.0/0014-assume-pipe2.patch packages/glibc/2.16.0/0015-china.patch packages/glibc/2.16.0/0016-new-valencian-locale.patch packages/glibc/2.16.0/0017-macos-cross-rpcgen.patch packages/glibc/2.16.0/0018-nscd-one-fork.patch packages/glibc/2.16.0/0019-hppa-nptl-carlos.patch packages/glibc/2.16.0/0020-dl_execstack-PaX-support.patch packages/glibc/2.16.0/0021-pre20040117-pt_pax.patch packages/glibc/2.16.0/0022-tests-sandbox-libdl-paths.patch packages/glibc/2.16.0/0023-dont-build-timezone.patch packages/glibc/2.16.0/0024-alpha-xstat.patch packages/glibc/2.16.0/0025-alpha-creat.patch packages/glibc/2.16.0/0026-alpha_alpha-add-fdatasync-support.patch packages/glibc/2.16.0/0027-fix-parsing-of-numeric-hosts-in-gethostbyname_r.patch packages/glibc/2.16.0/0028-ppc-atomic.patch packages/glibc/2.16.0/0029-mips_shn_undef-hack.patch packages/glibc/2.16.0/0030-alpha-atfcts.patch packages/glibc/2.16.0/0031-syslog.patch packages/glibc/2.16.0/0032-debug-readlink_chk-readklinkat_chk.patch packages/glibc/2.16.0/0033-fix-rpc_parse-format.patch packages/glibc/2.16.0/0034-nis-bogus-conditional.patch packages/glibc/2.16.0/0035-obstack-common.patch packages/glibc/2.16.0/0036-new-tools.patch packages/glibc/2.16.0/0037-strftime-multiple-stmts.patch packages/glibc/2.16.0/0038-if_nametoindex-size-check.patch packages/glibc/2.16.0/0039-utmp-nonstring.patch '/usr/local/share/crosstool-ng/packages/glibc/2.16.0'
2020-01-21T10:47:30.4658294Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/libiconv/1.14'
2020-01-21T10:47:30.4658737Z  /usr/bin/install -c -m 644  packages/libiconv/1.14/0000-srclib_stdio.in.h-remove-gets-declarations.patch packages/libiconv/1.14/chksum packages/libiconv/1.14/version.desc '/usr/local/share/crosstool-ng/packages/libiconv/1.14'
2020-01-21T10:47:30.4659356Z  /usr/bin/install -c -m 644  packages/libiconv/1.15/chksum packages/libiconv/1.15/version.desc '/usr/local/share/crosstool-ng/packages/libiconv/1.15'
2020-01-21T10:47:30.4659614Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/cloog/0.18.1'
2020-01-21T10:47:30.4664044Z  /usr/bin/install -c -m 644  packages/cloog/0.18.1/chksum packages/cloog/0.18.1/version.desc '/usr/local/share/crosstool-ng/packages/cloog/0.18.1'
2020-01-21T10:47:30.4664343Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc'
---
2020-01-21T10:47:30.4676949Z  /usr/bin/install -c -m 644  packages/mpfr/package.desc '/usr/local/share/crosstool-ng/packages/mpfr'
2020-01-21T10:47:30.4677385Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/avr-libc/2.0.0'
2020-01-21T10:47:30.4677746Z  /usr/bin/install -c -m 644  packages/avr-libc/2.0.0/chksum packages/avr-libc/2.0.0/version.desc '/usr/local/share/crosstool-ng/packages/avr-libc/2.0.0'
2020-01-21T10:47:30.4677998Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mpfr/2.4.2'
2020-01-21T10:47:30.4678781Z  /usr/bin/install -c -m 644  packages/mpfr/2.4.2/0000-sin_cos_underflow.patch packages/mpfr/2.4.2/0001-longlong.h.patch packages/mpfr/2.4.2/0002-gmp5.patch packages/mpfr/2.4.2/chksum packages/mpfr/2.4.2/version.desc '/usr/local/share/crosstool-ng/packages/mpfr/2.4.2'
2020-01-21T10:47:30.4679478Z  /usr/bin/install -c -m 644  packages/bison/3.0.5/chksum packages/bison/3.0.5/version.desc '/usr/local/share/crosstool-ng/packages/bison/3.0.5'
2020-01-21T10:47:30.4679774Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-linaro/6.4-2018.05'
2020-01-21T10:47:30.4683073Z  /usr/bin/install -c -m 644  packages/gcc-linaro/6.4-2018.05/0000-libtool-leave-framework-alone.patch packages/gcc-linaro/6.4-2018.05/0001-uclibc-conf.patch packages/gcc-linaro/6.4-2018.05/0002-missing-execinfo_h.patch packages/gcc-linaro/6.4-2018.05/0003-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc-linaro/6.4-2018.05/0004-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc-linaro/6.4-2018.05/0005-arm-softfloat-libgcc.patch packages/gcc-linaro/6.4-2018.05/0006-arm_unbreak_armv4t.patch packages/gcc-linaro/6.4-2018.05/0007-cilk-wchar.patch packages/gcc-linaro/6.4-2018.05/0008-fix-m68k-compile.patch packages/gcc-linaro/6.4-2018.05/0009-fix-m68k-uclinux.patch packages/gcc-linaro/6.4-2018.05/0010-libgcc-mkmap-symver-support-skip_underscore.patch packages/gcc-linaro/6.4-2018.05/0011-libgcc-config-bfin-use-the-generic-linker-version-in.patch packages/gcc-linaro/6.4-2018.05/0012-libgcc-fix-DWARF-compilation-with-FDPIC-targets.patch packages/gcc-linaro/6.4-2018.05/0013-bfin-define-REENTRANT.patch packages/gcc-linaro/6.4-2018.05/0014-libgfortran-missing-include.patch packages/gcc-linaro/6.4-2018.05/0015-nios2-bad-multilib-default.patch packages/gcc-linaro/6.4-2018.05/0016-libgcc-disable-split-stack-nothreads.patch packages/gcc-linaro/6.4-2018.05/0017-uclinux-enable-threads.patch packages/gcc-linaro/6.4-2018.05/0018-bionic-ndk.patch packages/gcc-linaro/6.4-2018.05/0019-bionic-errno.patch packages/gcc-linaro/6.4-2018.05/0020-crystax.patch packages/gcc-linaro/6.4-2018.05/0021-crystax.patch packages/gcc-linaro/6.4-2018.05/0022-crystax.patch packages/gcc-linaro/6.4-2018.05/0023-crystax.patch packages/gcc-linaro/6.4-2018.05/0024-crystax.patch packages/gcc-linaro/6.4-2018.05/0025-crystax.patch packages/gcc-linaro/6.4-2018.05/0026-crystax.patch packages/gcc-linaro/6.4-2018.05/0027-crystax.patch packages/gcc-linaro/6.4-2018.05/0028-isl-0.20.patch packages/gcc-linaro/6.4-2018.05/chksum packages/gcc-linaro/6.4-2018.05/version.desc '/usr/local/share/crosstool-ng/packages/gcc-linaro/6.4-2018.05'
2020-01-21T10:47:30.4683767Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/avr-libc/1.8.1'
---
2020-01-21T10:47:30.4714366Z  /usr/bin/install -c -m 644  samples/x86_64-multilib-linux-uclibc/crosstool.config samples/x86_64-multilib-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc'
2020-01-21T10:47:30.4715120Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/libc'
2020-01-21T10:47:30.4715772Z  /usr/bin/install -c -m 644  scripts/build/libc/avr-libc.sh scripts/build/libc/bionic.sh scripts/build/libc/glibc.sh scripts/build/libc/mingw-w64.sh scripts/build/libc/moxiebox.sh scripts/build/libc/musl.sh scripts/build/libc/newlib.sh scripts/build/libc/none.sh scripts/build/libc/uClibc.sh '/usr/local/share/crosstool-ng/scripts/build/libc'
2020-01-21T10:47:30.4716126Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/6.5.0'
2020-01-21T10:47:30.4718444Z  /usr/bin/install -c -m 644  packages/gcc/6.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/6.5.0/0001-uclibc-conf.patch packages/gcc/6.5.0/0002-missing-execinfo_h.patch packages/gcc/6.5.0/0003-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/6.5.0/0004-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/6.5.0/0005-arm-softfloat-libgcc.patch packages/gcc/6.5.0/0006-arm_unbreak_armv4t.patch packages/gcc/6.5.0/0007-ARM-PR-target-70473-Reduce-size-of-Cortex-A8-automat.patch packages/gcc/6.5.0/0008-cilk-wchar.patch packages/gcc/6.5.0/0009-fix-m68k-compile.patch packages/gcc/6.5.0/0010-fix-m68k-uclinux.patch packages/gcc/6.5.0/0011-libgcc-mkmap-symver-support-skip_underscore.patch packages/gcc/6.5.0/0012-libgcc-config-bfin-use-the-generic-linker-version-in.patch packages/gcc/6.5.0/0013-libgcc-fix-DWARF-compilation-with-FDPIC-targets.patch packages/gcc/6.5.0/0014-bfin-define-REENTRANT.patch packages/gcc/6.5.0/0015-libgfortran-missing-include.patch packages/gcc/6.5.0/0016-nios2-bad-multilib-default.patch packages/gcc/6.5.0/0017-libgcc-disable-split-stack-nothreads.patch packages/gcc/6.5.0/0018-uclinux-enable-threads.patch packages/gcc/6.5.0/0019-bionic-ndk.patch packages/gcc/6.5.0/0020-bionic-errno.patch packages/gcc/6.5.0/0021-crystax.patch packages/gcc/6.5.0/0022-crystax.patch packages/gcc/6.5.0/0023-crystax.patch packages/gcc/6.5.0/0024-crystax.patch packages/gcc/6.5.0/0025-crystax.patch packages/gcc/6.5.0/0026-crystax.patch packages/gcc/6.5.0/0027-crystax.patch packages/gcc/6.5.0/0028-crystax.patch packages/gcc/6.5.0/0029-msp430-fix.patch packages/gcc/6.5.0/0030-isl-0.20.patch packages/gcc/6.5.0/chksum packages/gcc/6.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/6.5.0'
2020-01-21T10:47:30.4720110Z  /usr/bin/install -c -m 644  samples/arm-unknown-eabi/crosstool.config samples/arm-unknown-eabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-eabi'
2020-01-21T10:47:30.4720770Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/automake/1.11.6'
2020-01-21T10:47:30.4721182Z  /usr/bin/install -c -m 644  packages/automake/1.11.6/0000-escape-left-brace.patch packages/automake/1.11.6/chksum packages/automake/1.11.6/version.desc '/usr/local/share/crosstool-ng/packages/automake/1.11.6'
2020-01-21T10:47:30.4721470Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.11.1'
---
2020-01-21T10:47:30.4725159Z  /usr/bin/install -c -m 644  samples/x86_64-centos6-linux-gnu/crosstool.config samples/x86_64-centos6-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-centos6-linux-gnu'
2020-01-21T10:47:30.4725575Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-uclibcgnueabihf'
2020-01-21T10:47:30.4726724Z  /usr/bin/install -c -m 644  samples/arm-unknown-linux-uclibcgnueabihf/crosstool.config samples/arm-unknown-linux-uclibcgnueabihf/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-uclibcgnueabihf'
2020-01-21T10:47:30.4727032Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc-ports/2.13'
2020-01-21T10:47:30.4728214Z  /usr/bin/install -c -m 644  packages/glibc-ports/2.13/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc-ports/2.13/0001-m68k-sys-user.patch packages/glibc-ports/2.13/0002-alpha-SETPIPE-GETPIPE.patch packages/glibc-ports/2.13/0003-alpha-statfs.patch packages/glibc-ports/2.13/0004-alpha-cache-shape.patch packages/glibc-ports/2.13/0005-alpha-DEFAULT_STACK_PERMS.patch packages/glibc-ports/2.13/0006-alpha-fix-gcc-4.1-warnings.patch packages/glibc-ports/2.13/0007-alpha-feupdateenv.patch packages/glibc-ports/2.13/0008-alpha-fix-rtld-fPIC.patch packages/glibc-ports/2.13/0009-arm-cirrus-ep93xx-maverick-crunch-fpu.patch packages/glibc-ports/2.13/0010-nptl-lowlevellock.patch packages/glibc-ports/2.13/0011-fpu-cw-mips.patch packages/glibc-ports/2.13/0012-support-hard-float-eabi.patch packages/glibc-ports/2.13/chksum packages/glibc-ports/2.13/version.desc '/usr/local/share/crosstool-ng/packages/glibc-ports/2.13'
2020-01-21T10:47:30.4729473Z  /usr/bin/install -c -m 644  packages/glibc-ports/2.15/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc-ports/2.15/0001-libmemusage-link-failure.patch packages/glibc-ports/2.15/0002-m68k-sys-user.patch packages/glibc-ports/2.15/0003-alpha-cache-shape.patch packages/glibc-ports/2.15/0004-alpha-fix-gcc-4.1-warnings.patch packages/glibc-ports/2.15/0005-alpha-fix-rtld-fPIC.patch packages/glibc-ports/2.15/0006-arm-cirrus-ep93xx-maverick-crunch-fpu.patch packages/glibc-ports/2.15/0007-nptl-lowlevellock.patch packages/glibc-ports/2.15/0008-fpu-cw-mips.patch packages/glibc-ports/2.15/chksum packages/glibc-ports/2.15/version.desc '/usr/local/share/crosstool-ng/packages/glibc-ports/2.15'
2020-01-21T10:47:30.4729810Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mips-malta-linux-gnu'
2020-01-21T10:47:30.4730181Z  /usr/bin/install -c -m 644  samples/mips-malta-linux-gnu/crosstool.config samples/mips-malta-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mips-malta-linux-gnu'
2020-01-21T10:47:30.4730455Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-linaro/4.8-2015.06'
---
2020-01-21T10:47:30.4741608Z  /usr/bin/install -c -m 644  packages/glibc/2.13/0040-nis-bogus-conditional.patch packages/glibc/2.13/0041-initfini-ppc64.patch packages/glibc/2.13/0042-obstack-common.patch packages/glibc/2.13/0043-new-tools.patch packages/glibc/2.13/0044-strftime-multiple-stmts.patch packages/glibc/2.13/0045-if_nametoindex-size-check.patch packages/glibc/2.13/0046-utmp-nonstring.patch packages/glibc/2.13/0047-getlogin_r-use-strnlen.patch packages/glibc/2.13/0048-zic.c-use-memcpy.patch packages/glibc/2.13/chksum packages/glibc/2.13/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.13'
2020-01-21T10:47:30.4741923Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.32'
2020-01-21T10:47:30.4743515Z  /usr/bin/install -c -m 644  packages/binutils/2.32/0000-sh-conf.patch packages/binutils/2.32/0001-ld_makefile_patch.patch packages/binutils/2.32/0002-check_ldrunpath_length.patch packages/binutils/2.32/0003-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.32/0004-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.32/0005-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.32/0006-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.32/0007-sysroot.patch packages/binutils/2.32/0008-poison-system-directories.patch packages/binutils/2.32/chksum packages/binutils/2.32/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.32'
2020-01-21T10:47:30.4744262Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-21T10:47:30.4745619Z  /usr/bin/install -c -m 644  packages/glibc/2.15/0040-builtin_expect.patch packages/glibc/2.15/0041-gcc_s-suffix.patch packages/glibc/2.15/0042-obsolete-rpc.patch packages/glibc/2.15/0043-obstack-common.patch packages/glibc/2.15/0044-new-tools.patch packages/glibc/2.15/0045-strftime-multiple-stmts.patch packages/glibc/2.15/0046-if_nametoindex-size-check.patch packages/glibc/2.15/0047-utmp-nonstring.patch packages/glibc/2.15/0048-getlogin_r-use-strnlen.patch packages/glibc/2.15/0049-zic.c-use-memcpy.patch packages/glibc/2.15/0050-fdivp-order.patch packages/glibc/2.15/chksum packages/glibc/2.15/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.15'
2020-01-21T10:47:30.4746804Z  /usr/bin/install -c -m 644  packages/linux/3.9.11/chksum packages/linux/3.9.11/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.9.11'
2020-01-21T10:47:30.4747167Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.17'
2020-01-21T10:47:30.4748526Z  /usr/bin/install -c -m 644  packages/glibc/2.17/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc/2.17/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.17/0002-fix-signed-shift-overlow.patch packages/glibc/2.17/0003-dl-openat64-variadic.patch packages/glibc/2.17/0004-unused-variables.patch packages/glibc/2.17/0005-misleading-indentation.patch packages/glibc/2.17/0006-dl-open-array-bounds.patch packages/glibc/2.17/0007-support-make4.patch packages/glibc/2.17/0008-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.17/0009-macos-cross-rpcgen.patch packages/glibc/2.17/0010-fix-rpc_parse-format.patch packages/glibc/2.17/0011-nis-bogus-conditional.patch packages/glibc/2.17/0012-obstack-common.patch packages/glibc/2.17/0013-strftime-multiple-stmts.patch packages/glibc/2.17/0014-if_nametoindex-size-check.patch packages/glibc/2.17/0015-utmp-nonstring.patch packages/glibc/2.17/0016-getlogin_r-use-strnlen.patch packages/glibc/2.17/0017-zic.c-use-memcpy.patch packages/glibc/2.17/chksum packages/glibc/2.17/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.17'
2020-01-21T10:47:30.4748897Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.18'
2020-01-21T10:47:30.4748897Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.18'
2020-01-21T10:47:30.4751066Z  /usr/bin/install -c -m 644  packages/glibc/2.18/0000-4f2bcda-ARM-Fix-clone-code-when-built-for-Thumb.patch packages/glibc/2.18/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.18/0002-fix-signed-shift-overlow.patch packages/glibc/2.18/0003-dl-openat64-variadic.patch packages/glibc/2.18/0004-unused-variables.patch packages/glibc/2.18/0005-misleading-indentation.patch packages/glibc/2.18/0006-dl-open-array-bounds.patch packages/glibc/2.18/0007-2770d15-Fix-PI-mutex-check-in-pthread_cond_broadcast-and-pthread_cond_signal.patch packages/glibc/2.18/0008-support-make4.patch packages/glibc/2.18/0009-arm-unwind.patch packages/glibc/2.18/0010-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.18/0011-macos-cross-rpcgen.patch packages/glibc/2.18/0012-fix-rpc_parse-format.patch packages/glibc/2.18/0013-nis-bogus-conditional.patch packages/glibc/2.18/0014-strftime-multiple-stmts.patch packages/glibc/2.18/0015-if_nametoindex-size-check.patch packages/glibc/2.18/0016-utmp-nonstring.patch packages/glibc/2.18/0017-getlogin_r-use-strnlen.patch packages/glibc/2.18/0018-zic.c-use-memcpy.patch packages/glibc/2.18/chksum packages/glibc/2.18/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.18'
2020-01-21T10:47:30.4751919Z  /usr/bin/install -c -m 644  samples/powerpc64-unknown-linux-gnu/crosstool.config samples/powerpc64-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc64-unknown-linux-gnu'
2020-01-21T10:47:30.4752189Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.19'
2020-01-21T10:47:30.4753601Z  /usr/bin/install -c -m 644  packages/glibc/2.19/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.19/0001-fix-signed-shift-overlow.patch packages/glibc/2.19/0002-dl-openat64-variadic.patch packages/glibc/2.19/0003-unused-variables.patch packages/glibc/2.19/0004-misleading-indentation.patch packages/glibc/2.19/0005-dl-open-array-bounds.patch packages/glibc/2.19/0006-arm-unwind.patch packages/glibc/2.19/0007-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.19/0008-typedef-caddr.patch packages/glibc/2.19/0009-fix-rpc_parse-format.patch packages/glibc/2.19/0010-explicit-boolean.patch packages/glibc/2.19/0011-nis-bogus-conditional.patch packages/glibc/2.19/0012-strftime-multiple-stmts.patch packages/glibc/2.19/0013-if_nametoindex-size-check.patch packages/glibc/2.19/0014-utmp-nonstring.patch packages/glibc/2.19/0015-getlogin_r-use-strnlen.patch packages/glibc/2.19/0016-zic.c-use-memcpy.patch packages/glibc/2.19/chksum packages/glibc/2.19/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.19'
2020-01-21T10:47:30.4754058Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/musl'
---
2020-01-21T10:47:30.4765926Z  /usr/bin/install -c -m 644  samples/msp430-unknown-elf/crosstool.config samples/msp430-unknown-elf/reported.by '/usr/local/share/crosstool-ng/samples/msp430-unknown-elf'
2020-01-21T10:47:30.4766196Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.3.0.20160226'
2020-01-21T10:47:30.4767517Z  /usr/bin/install -c -m 644  packages/newlib/2.3.0.20160226/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.3.0.20160226/0001-fix-eabihf.patch packages/newlib/2.3.0.20160226/0002-fix-mt-cflags.patch packages/newlib/2.3.0.20160226/chksum packages/newlib/2.3.0.20160226/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.3.0.20160226'
2020-01-21T10:47:30.4767845Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/zlib/1.2.11'
2020-01-21T10:47:30.4768365Z  /usr/bin/install -c -m 644  packages/zlib/1.2.11/0000-make-check-fail.patch packages/zlib/1.2.11/0001-no-_wopen-cygwin.patch packages/zlib/1.2.11/0002-mingw-static-only.patch packages/zlib/1.2.11/chksum packages/zlib/1.2.11/version.desc '/usr/local/share/crosstool-ng/packages/zlib/1.2.11'
2020-01-21T10:47:30.4768980Z  /usr/bin/install -c -m 644  packages/gdb/7.6.1/chksum packages/gdb/7.6.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.6.1'
2020-01-21T10:47:30.4769253Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc-ports/2.16.0'
2020-01-21T10:47:30.4770051Z  /usr/bin/install -c -m 644  packages/glibc-ports/2.16.0/0000-Fix-ARM-build-with-GCC-trunk.patch packages/glibc-ports/2.16.0/0001-m68k-sys-user.patch packages/glibc-ports/2.16.0/0002-alpha-cache-shape.patch packages/glibc-ports/2.16.0/0003-alpha-fix-gcc-4.1-warnings.patch packages/glibc-ports/2.16.0/0004-alpha-fix-rtld-fPIC.patch packages/glibc-ports/2.16.0/0005-nptl-lowlevellock.patch packages/glibc-ports/2.16.0/0006-fpu-cw-mips.patch packages/glibc-ports/2.16.0/chksum packages/glibc-ports/2.16.0/version.desc '/usr/local/share/crosstool-ng/packages/glibc-ports/2.16.0'
2020-01-21T10:47:30.4770350Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
2020-01-21T10:47:30.4770350Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
2020-01-21T10:47:30.4770708Z  /usr/bin/install -c -m 644  packages/linux/4.13.16/chksum packages/linux/4.13.16/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
2020-01-21T10:47:30.4770975Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.6'
2020-01-21T10:47:30.4771316Z  /usr/bin/install -c -m 644  packages/strace/4.6/chksum packages/strace/4.6/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.6'
2020-01-21T10:47:30.4771569Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.7'
2020-01-21T10:47:30.4771897Z  /usr/bin/install -c -m 644  packages/strace/4.7/chksum packages/strace/4.7/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.7'
2020-01-21T10:47:30.4772167Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.8'
2020-01-21T10:47:30.4772561Z  /usr/bin/install -c -m 644  packages/strace/4.8/0000-strace-4.8-glibc_2.18_build_fix-1.patch packages/strace/4.8/chksum packages/strace/4.8/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.8'
2020-01-21T10:47:30.4773337Z  /usr/bin/install -c -m 644  packages/strace/4.9/chksum packages/strace/4.9/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.9'
2020-01-21T10:47:30.4773620Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils-linaro/2.25.0-2015.01-2'
2020-01-21T10:47:30.4774112Z  /usr/bin/install -c -m 644  packages/binutils-linaro/2.25.0-2015.01-2/chksum packages/binutils-linaro/2.25.0-2015.01-2/version.desc '/usr/local/share/crosstool-ng/packages/binutils-linaro/2.25.0-2015.01-2'
2020-01-21T10:47:30.4774427Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
2020-01-21T10:47:30.4774427Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
2020-01-21T10:47:30.4775324Z  /usr/bin/install -c -m 644  samples/mipsel-multilib-linux-gnu/crosstool.config samples/mipsel-multilib-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
2020-01-21T10:47:30.4775673Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/duma/2_5_15'
2020-01-21T10:47:30.4776155Z  /usr/bin/install -c -m 644  packages/duma/2_5_15/0000-cross-compile.patch packages/duma/2_5_15/0001-separate_cpp.patch packages/duma/2_5_15/0002-cpp11-new-operator.patch packages/duma/2_5_15/chksum packages/duma/2_5_15/version.desc '/usr/local/share/crosstool-ng/packages/duma/2_5_15'
2020-01-21T10:47:30.4776740Z  /usr/bin/install -c -m 644  packages/cloog/package.desc '/usr/local/share/crosstool-ng/packages/cloog'
2020-01-21T10:47:30.4777021Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/4.9.4'
2020-01-21T10:47:30.4780509Z  /usr/bin/install -c -m 644  packages/gcc/4.9.4/0000-Use-ucontext_t-not-struct-ucontext-in-linux-unwind.h.patch packages/gcc/4.9.4/0001-gcc_bug_62231.patch packages/gcc/4.9.4/0002-gcc_bug_62231.patch packages/gcc/4.9.4/0003-libtool-leave-framework-alone.patch packages/gcc/4.9.4/0004-uclibc-conf.patch packages/gcc/4.9.4/0005-msp430-string-literals.patch packages/gcc/4.9.4/0006-alpha-bad-eh_frame.patch packages/gcc/4.9.4/0007-pr65730.patch packages/gcc/4.9.4/0008-gcc-config.gcc-fix-typo-for-powerpc-e6500-cpu_is_64b.patch packages/gcc/4.9.4/0009-pr43538.patch packages/gcc/4.9.4/0010-mt-ospace-preserve-FLAGS_FOR_TARGET.patch packages/gcc/4.9.4/0011-sanitizer-Fix-build-with-_FILE_OFFSET_BITS-64.patch packages/gcc/4.9.4/0012-missing-execinfo_h.patch packages/gcc/4.9.4/0013-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/4.9.4/0014-arm-softfloat-libgcc.patch packages/gcc/4.9.4/0015-arm_unbreak_armv4t.patch packages/gcc/4.9.4/0016-microblaze-enable-dwarf-eh-support.patch packages/gcc/4.9.4/0017-libstdcxx-uclibc-c99.patch packages/gcc/4.9.4/0018-cilk-wchar.patch packages/gcc/4.9.4/0019-xtensa-add-mauto-litpools-option.patch packages/gcc/4.9.4/0020-xtensa-reimplement-register-spilling.patch packages/gcc/4.9.4/0021-xtensa-use-unwind-dw2-fde-dip-instead-of-unwind-dw2-.patch packages/gcc/4.9.4/0022-xtensa-fix-_Unwind_GetCFA.patch packages/gcc/4.9.4/0023-xtensa-add-uclinux-support.patch packages/gcc/4.9.4/0024-gcc-xtensa-fix-fprintf-format-specifiers.patch packages/gcc/4.9.4/0025-xtensa-fix-PR-target-82181.patch packages/gcc/4.9.4/0026-nios2_legitimize_address.patch packages/gcc/4.9.4/0027-fix-m68k-compile.patch packages/gcc/4.9.4/0028-fix-m68k-uclinux.patch packages/gcc/4.9.4/0029-musl-support.patch packages/gcc/4.9.4/0030-microblaze-uclibc.patch packages/gcc/4.9.4/0031-libgcc-disable-split-stack-nothreads.patch packages/gcc/4.9.4/0032-uclinux-enable-threads.patch packages/gcc/4.9.4/1000-powerpc-link-with-math-lib.patch.conditional packages/gcc/4.9.4/chksum packages/gcc/4.9.4/version.desc '/usr/local/share/crosstool-ng/packages/gcc/4.9.4'
2020-01-21T10:47:30.4781249Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-sde-elf'
2020-01-21T10:47:30.4781249Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-sde-elf'
2020-01-21T10:47:30.4781860Z  /usr/bin/install -c -m 644  samples/mipsel-sde-elf/crosstool.config samples/mipsel-sde-elf/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-sde-elf'
2020-01-21T10:47:30.4782138Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/8.3.0'
2020-01-21T10:47:30.4784504Z  /usr/bin/install -c -m 644  packages/gcc/8.3.0/0000-libtool-leave-framework-alone.patch packages/gcc/8.3.0/0001-uclibc-conf.patch packages/gcc/8.3.0/0002-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/8.3.0/0003-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/8.3.0/0004-arm-softfloat-libgcc.patch packages/gcc/8.3.0/0005-fix-m68k-uclinux.patch packages/gcc/8.3.0/0006-libgfortran-missing-include.patch packages/gcc/8.3.0/0007-nios2-bad-multilib-default.patch packages/gcc/8.3.0/0008-libgcc-disable-split-stack-nothreads.patch packages/gcc/8.3.0/0009-bionic-ndk.patch packages/gcc/8.3.0/0010-crystax.patch packages/gcc/8.3.0/0011-crystax.patch packages/gcc/8.3.0/0012-crystax.patch packages/gcc/8.3.0/0013-crystax.patch packages/gcc/8.3.0/0014-crystax.patch packages/gcc/8.3.0/0015-crystax.patch packages/gcc/8.3.0/0016-crystax.patch packages/gcc/8.3.0/0017-crystax.patch packages/gcc/8.3.0/0018-ARC-Add-multilib-support-for-linux-targets.patch packages/gcc/8.3.0/0019-isl-0.20.patch packages/gcc/8.3.0/0020-ARM-fix-cmse.patch packages/gcc/8.3.0/0021-arm-Make-arm_cmse.h-C99-compatible.patch packages/gcc/8.3.0/chksum packages/gcc/8.3.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/8.3.0'
2020-01-21T10:47:30.4786329Z  /usr/bin/install -c -m 644  packages/newlib/3.1.0.20181231/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/3.1.0.20181231/0001-fix-mt-cflags.patch packages/newlib/3.1.0.20181231/chksum packages/newlib/3.1.0.20181231/version.desc '/usr/local/share/crosstool-ng/packages/newlib/3.1.0.20181231'
2020-01-21T10:47:30.4786626Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/2.6.38.8'
2020-01-21T10:47:30.4786982Z  /usr/bin/install -c -m 644  packages/linux/2.6.38.8/chksum packages/linux/2.6.38.8/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.38.8'
2020-01-21T10:47:30.4787243Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.0.1a'
---
2020-01-21T10:47:30.5119498Z  /usr/bin/install -c -m 644  packages/isl/0.14.1/chksum packages/isl/0.14.1/version.desc '/usr/local/share/crosstool-ng/packages/isl/0.14.1'
2020-01-21T10:47:30.5134227Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arc-multilib-elf32'
2020-01-21T10:47:30.5146295Z  /usr/bin/install -c -m 644  samples/arc-multilib-elf32/crosstool.config samples/arc-multilib-elf32/reported.by '/usr/local/share/crosstool-ng/samples/arc-multilib-elf32'
2020-01-21T10:47:30.5167312Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/arch'
2020-01-21T10:47:30.5182623Z  /usr/bin/install -c -m 644  config/arch/alpha.in config/arch/arc.in config/arch/arm.in config/arch/avr.in config/arch/m68k.in config/arch/microblaze.in config/arch/mips.in config/arch/moxie.in config/arch/msp430.in config/arch/nios2.in config/arch/powerpc.in config/arch/riscv.in config/arch/s390.in config/arch/sh.in config/arch/sparc.in config/arch/x86.in config/arch/xtensa.in '/usr/local/share/crosstool-ng/config/arch'
2020-01-21T10:47:30.5219582Z  /usr/bin/install -c -m 644  packages/linux/3.0.101/chksum packages/linux/3.0.101/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.0.101'
2020-01-21T10:47:30.5246695Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-ubuntu16.04-linux-gnu'
2020-01-21T10:47:30.5257841Z  /usr/bin/install -c -m 644  samples/i686-ubuntu16.04-linux-gnu/crosstool.config samples/i686-ubuntu16.04-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/i686-ubuntu16.04-linux-gnu'
2020-01-21T10:47:30.5275010Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/1.20.0'
---
2020-01-21T10:47:30.5988661Z  /usr/bin/install -c -m 644  samples/powerpc-e300c3-linux-gnu/crosstool.config samples/powerpc-e300c3-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-e300c3-linux-gnu'
2020-01-21T10:47:30.6007062Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.17.19'
2020-01-21T10:47:30.6021262Z  /usr/bin/install -c -m 644  packages/linux/4.17.19/chksum packages/linux/4.17.19/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.17.19'
2020-01-21T10:47:30.6043154Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mpfr/3.0.1'
2020-01-21T10:47:30.6061369Z  /usr/bin/install -c -m 644  packages/mpfr/3.0.1/0000-asin_exprange.patch packages/mpfr/3.0.1/0001-rec_sqrt-carry.patch packages/mpfr/3.0.1/0002-atan-expo-range.patch packages/mpfr/3.0.1/0003-texp-zero.patch packages/mpfr/3.0.1/chksum packages/mpfr/3.0.1/version.desc '/usr/local/share/crosstool-ng/packages/mpfr/3.0.1'
2020-01-21T10:47:30.6095244Z  /usr/bin/install -c -m 644  packages/linux/2.6.33.7/chksum packages/linux/2.6.33.7/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.33.7'
2020-01-21T10:47:30.6126843Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.26.1'
2020-01-21T10:47:30.6142143Z  /usr/bin/install -c -m 644  packages/binutils/2.26.1/0000-sh-conf.patch packages/binutils/2.26.1/0001-ld_makefile_patch.patch packages/binutils/2.26.1/0002-check_ldrunpath_length.patch packages/binutils/2.26.1/0003-fix-gold-pthreads-typo.patch packages/binutils/2.26.1/0004-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.26.1/0005-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.26.1/0006-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.26.1/0007-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.26.1/0008-sysroot.patch packages/binutils/2.26.1/0009-poison-system-directories.patch packages/binutils/2.26.1/0010-Fix-library-paths-on-PowerPC.patch packages/binutils/2.26.1/0011-xtensa-fix-signedness-of-gas-relocations.patch packages/binutils/2.26.1/0012-xtensa-fix-.init-.fini-literals-moving.patch packages/binutils/2.26.1/chksum packages/binutils/2.26.1/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.26.1'
2020-01-21T10:47:30.6177675Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/gen'
---
2020-01-21T10:47:30.6785319Z  /usr/bin/install -c -m 644  packages/glibc/2.21/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.21/0001-fix-signed-shift-overlow.patch packages/glibc/2.21/0002-dl-openat64-variadic.patch packages/glibc/2.21/0003-unused-variables.patch packages/glibc/2.21/0004-misleading-indentation.patch packages/glibc/2.21/0005-dl-open-array-bounds.patch packages/glibc/2.21/0006-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.21/0007-typedef-caddr.patch packages/glibc/2.21/0008-fix-rpc_parse-format.patch packages/glibc/2.21/0009-explicit-boolean.patch packages/glibc/2.21/0010-nis-bogus-conditional.patch packages/glibc/2.21/0011-dlclose-assert.patch packages/glibc/2.21/0012-strftime-multiple-stmts.patch packages/glibc/2.21/0013-if_nametoindex-size-check.patch packages/glibc/2.21/0014-utmp-nonstring.patch packages/glibc/2.21/0015-getlogin_r-use-strnlen.patch packages/glibc/2.21/0016-zic.c-use-memcpy.patch packages/glibc/2.21/chksum packages/glibc/2.21/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.21'
2020-01-21T10:47:30.6811543Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gettext'
2020-01-21T10:47:30.6829430Z  /usr/bin/install -c -m 644  packages/gettext/package.desc '/usr/local/share/crosstool-ng/packages/gettext'
2020-01-21T10:47:30.6859328Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.22'
2020-01-21T10:47:30.6872499Z  /usr/bin/install -c -m 644  packages/glibc/2.22/0000-sparc32-sem_open-missing-include.patch packages/glibc/2.22/0001-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.22/0002-fix-signed-shift-overlow.patch packages/glibc/2.22/0003-dl-openat64-variadic.patch packages/glibc/2.22/0004-unused-variables.patch packages/glibc/2.22/0005-misleading-indentation.patch packages/glibc/2.22/0006-cve-2105-7547-getaddrinfo-stack.patch packages/glibc/2.22/0007-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.22/0008-typedef-caddr.patch packages/glibc/2.22/0009-fix-rpc_parse-format.patch packages/glibc/2.22/0010-explicit-boolean.patch packages/glibc/2.22/0011-nis-bogus-conditional.patch packages/glibc/2.22/0012-strftime-multiple-stmts.patch packages/glibc/2.22/0013-if_nametoindex-size-check.patch packages/glibc/2.22/0014-utmp-nonstring.patch packages/glibc/2.22/0015-getlogin_r-use-strnlen.patch packages/glibc/2.22/0016-zic.c-use-memcpy.patch packages/glibc/2.22/chksum packages/glibc/2.22/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.22'
2020-01-21T10:47:30.6921322Z  /usr/bin/install -c -m 644  packages/glibc/2.23/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.23/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.23/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.23/0003-typedef-caddr.patch packages/glibc/2.23/0004-fix-rpc_parse-format.patch packages/glibc/2.23/0005-explicit-boolean.patch packages/glibc/2.23/0006-nis-bogus-conditional.patch packages/glibc/2.23/0007-regexp-common.patch packages/glibc/2.23/0008-strftime-multiple-stmts.patch packages/glibc/2.23/0009-if_nametoindex-size-check.patch packages/glibc/2.23/0010-utmp-nonstring.patch packages/glibc/2.23/0011-getlogin_r-use-strnlen.patch packages/glibc/2.23/0012-zic.c-use-memcpy.patch packages/glibc/2.23/chksum packages/glibc/2.23/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.23'
2020-01-21T10:47:30.6952858Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.24'
2020-01-21T10:47:30.6968331Z  /usr/bin/install -c -m 644  packages/glibc/2.24/0000-sh-fix-gcc6.patch packages/glibc/2.24/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.24/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.24/0003-typedef-caddr.patch packages/glibc/2.24/0004-fix-rpc_parse-format.patch packages/glibc/2.24/0005-explicit-boolean.patch packages/glibc/2.24/0006-nis-bogus-conditional.patch packages/glibc/2.24/0007-regexp-common.patch packages/glibc/2.24/0008-strftime-multiple-stmts.patch packages/glibc/2.24/0009-if_nametoindex-size-check.patch packages/glibc/2.24/0010-utmp-nonstring.patch packages/glibc/2.24/0011-getlogin_r-use-strnlen.patch packages/glibc/2.24/0012-zic.c-use-memcpy.patch packages/glibc/2.24/chksum packages/glibc/2.24/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.24'
2020-01-21T10:47:30.6997383Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.25'
2020-01-21T10:47:30.6997383Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.25'
2020-01-21T10:47:30.7014208Z  /usr/bin/install -c -m 644  packages/glibc/2.25/0000-sh-fix-gcc6.patch packages/glibc/2.25/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.25/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.25/0003-typedef-caddr.patch packages/glibc/2.25/0004-sh4-trap-divdi3.patch packages/glibc/2.25/0005-sparc-extra-plt-call.patch packages/glibc/2.25/0006-regexp-common.patch packages/glibc/2.25/0007-strftime-multiple-stmts.patch packages/glibc/2.25/0008-if_nametoindex-size-check.patch packages/glibc/2.25/0009-utmp-nonstring.patch packages/glibc/2.25/0010-getlogin_r-use-strnlen.patch packages/glibc/2.25/0011-zic.c-use-memcpy.patch packages/glibc/2.25/chksum packages/glibc/2.25/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.25'
2020-01-21T10:47:30.7054338Z  /usr/bin/install -c -m 644  packages/glibc/2.26/0000-typedef-caddr.patch packages/glibc/2.26/0001-aarch64-rewrite-elf_machine_load_address.patch packages/glibc/2.26/0002-if_nametoindex-size-check.patch packages/glibc/2.26/0003-utmp-nonstring.patch packages/glibc/2.26/0004-getlogin_r-use-strnlen.patch packages/glibc/2.26/0005-zic.c-use-memcpy.patch packages/glibc/2.26/chksum packages/glibc/2.26/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.26'
2020-01-21T10:47:30.7073880Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.27'
2020-01-21T10:47:30.7087820Z  /usr/bin/install -c -m 644  packages/glibc/2.27/0000-typedef-caddr.patch packages/glibc/2.27/chksum packages/glibc/2.27/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.27'
2020-01-21T10:47:30.7104625Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-ubuntu14.04-linux-gnu'
---
2020-01-21T10:47:30.7602055Z  /usr/bin/install -c -m 644  packages/linux/3.5.7/chksum packages/linux/3.5.7/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.5.7'
2020-01-21T10:47:30.7619032Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/binutils'
2020-01-21T10:47:30.7632924Z  /usr/bin/install -c -m 644  scripts/build/binutils/binutils.sh '/usr/local/share/crosstool-ng/scripts/build/binutils'
2020-01-21T10:47:30.7657834Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.31.1'
2020-01-21T10:47:30.7672881Z  /usr/bin/install -c -m 644  packages/binutils/2.31.1/0000-sh-conf.patch packages/binutils/2.31.1/0001-ld_makefile_patch.patch packages/binutils/2.31.1/0002-check_ldrunpath_length.patch packages/binutils/2.31.1/0003-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.31.1/0004-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.31.1/0005-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.31.1/0006-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.31.1/0007-sysroot.patch packages/binutils/2.31.1/0008-poison-system-directories.patch packages/binutils/2.31.1/0009-xtensa-fix-relaxation-of-undefined-weak-references-i.patch packages/binutils/2.31.1/0010-xtensa-move-dynamic-relocations-sections-consistency.patch packages/binutils/2.31.1/0011-Restore-build-on-x86_64-w64-mingw32.patch packages/binutils/2.31.1/chksum packages/binutils/2.31.1/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.31.1'
2020-01-21T10:47:30.7717121Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/5.5.0'
2020-01-21T10:47:30.7734369Z  /usr/bin/install -c -m 644  packages/gcc/5.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/5.5.0/0001-uclibc-conf.patch packages/gcc/5.5.0/0002-msp430-string-literals.patch packages/gcc/5.5.0/0003-xtensa-implement-trap-pattern.patch packages/gcc/5.5.0/0004-alpha-bad-eh_frame.patch packages/gcc/5.5.0/0005-gcc-config.gcc-fix-typo-for-powerpc-e6500-cpu_is_64b.patch packages/gcc/5.5.0/0006-missing-execinfo_h.patch packages/gcc/5.5.0/0007-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/5.5.0/0008-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/5.5.0/0009-arm-softfloat-libgcc.patch packages/gcc/5.5.0/0010-arm_unbreak_armv4t.patch packages/gcc/5.5.0/0011-microblaze-enable-dwarf-eh-support.patch packages/gcc/5.5.0/0012-libstdcxx-uclibc-c99.patch packages/gcc/5.5.0/0013-cilk-wchar.patch packages/gcc/5.5.0/0014-xtensa-add-mauto-litpools-option.patch packages/gcc/5.5.0/0015-xtensa-reimplement-register-spilling.patch packages/gcc/5.5.0/0016-xtensa-add-uclinux-support.patch packages/gcc/5.5.0/0017-fix-m68k-compile.patch packages/gcc/5.5.0/0018-fix-m68k-uclinux.patch packages/gcc/5.5.0/0019-microblaze-uclibc.patch packages/gcc/5.5.0/0020-libitm-fixes-for-musl-support.patch packages/gcc/5.5.0/0021-fixincludes-update-for-musl-support.patch packages/gcc/5.5.0/0022-unwind-fix-for-musl.patch packages/gcc/5.5.0/0023-libstdc++-libgfortran-gthr-workaround-for-musl.patch packages/gcc/5.5.0/0024-musl-libc-config.patch packages/gcc/5.5.0/0025-add-musl-support-to-gcc.patch packages/gcc/5.5.0/0026-mips-musl-support.patch packages/gcc/5.5.0/0027-x86-musl-support.patch packages/gcc/5.5.0/0028-arm-musl-support.patch packages/gcc/5.5.0/0029-aarch64-musl-support.patch packages/gcc/5.5.0/0030-nios2-bad-multilib-default.patch packages/gcc/5.5.0/0031-libgcc-disable-split-stack-nothreads.patch packages/gcc/5.5.0/0032-uclinux-enable-threads.patch packages/gcc/5.5.0/0033-msp430-fix.patch packages/gcc/5.5.0/0034-xtensa-fix-PR-target-65416.patch packages/gcc/5.5.0/chksum packages/gcc/5.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/5.5.0'
2020-01-21T10:47:30.7775925Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts'
2020-01-21T10:47:30.7789532Z  /usr/bin/install -c -m 644  scripts/config.guess scripts/config.rpath scripts/config.sub scripts/crosstool-NG.sh scripts/functions scripts/populate.in scripts/saveSample.sh scripts/scripts.mk scripts/show-config.sh scripts/show-tuple.sh scripts/toolchain-config.in scripts/version-check.sh scripts/xldd.in '/usr/local/share/crosstool-ng/scripts'
2020-01-21T10:47:30.7827976Z  /usr/bin/install -c -m 644  packages/gdb/7.3.1/chksum packages/gdb/7.3.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.3.1'
2020-01-21T10:47:30.7845132Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/aarch64-unknown-linux-gnu'
2020-01-21T10:47:30.7858461Z  /usr/bin/install -c -m 644  samples/aarch64-unknown-linux-gnu/crosstool.config samples/aarch64-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-unknown-linux-gnu'
2020-01-21T10:47:30.7875199Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib'
---
2020-01-21T10:47:30.8218539Z  /usr/bin/install -c -m 644  packages/musl/1.1.21/chksum packages/musl/1.1.21/version.desc '/usr/local/share/crosstool-ng/packages/musl/1.1.21'
2020-01-21T10:47:30.8234229Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc-860-linux-gnu'
2020-01-21T10:47:30.8257782Z  /usr/bin/install -c -m 644  samples/powerpc-860-linux-gnu/crosstool.config samples/powerpc-860-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-860-linux-gnu'
2020-01-21T10:47:30.8276892Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/6.8a'
2020-01-21T10:47:30.8292355Z  /usr/bin/install -c -m 644  packages/gdb/6.8a/0000-dwarf-stack-overflow.patch packages/gdb/6.8a/0001-security-errata-20050610.patch packages/gdb/6.8a/0002-tdep-opcode-include-workaround.patch packages/gdb/6.8a/0003-reg-no-longer-active.patch packages/gdb/6.8a/0004-sim-ppc-have-config-h.patch packages/gdb/6.8a/0005-handle-stpcpy-define.patch packages/gdb/6.8a/chksum packages/gdb/6.8a/version.desc '/usr/local/share/crosstool-ng/packages/gdb/6.8a'
2020-01-21T10:47:30.8325311Z  /usr/bin/install -c -m 644  scripts/build/test_suite/gcc.sh '/usr/local/share/crosstool-ng/scripts/build/test_suite'
2020-01-21T10:47:30.8341660Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.0.0'
2020-01-21T10:47:30.8354117Z  /usr/bin/install -c -m 644  packages/newlib/2.0.0/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.0.0/0001-fix-eabihf.patch packages/newlib/2.0.0/0002-fix-mt-cflags.patch packages/newlib/2.0.0/chksum packages/newlib/2.0.0/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.0.0'
2020-01-21T10:47:30.8372790Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/isl/0.15'
---
2020-01-21T10:47:30.8789928Z  /usr/bin/install -c -m 644  samples/aarch64-unknown-linux-uclibc/crosstool.config samples/aarch64-unknown-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-unknown-linux-uclibc'
2020-01-21T10:47:30.8807551Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-bare_newlib_cortex_m3_nommu-eabi'
2020-01-21T10:47:30.8820543Z  /usr/bin/install -c -m 644  samples/arm-bare_newlib_cortex_m3_nommu-eabi/crosstool.config samples/arm-bare_newlib_cortex_m3_nommu-eabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-bare_newlib_cortex_m3_nommu-eabi'
2020-01-21T10:47:30.8847251Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/arch'
2020-01-21T10:47:30.8860870Z  /usr/bin/install -c -m 644  scripts/build/arch/alpha.sh scripts/build/arch/arc.sh scripts/build/arch/arm.sh scripts/build/arch/avr.sh scripts/build/arch/m68k.sh scripts/build/arch/microblaze.sh scripts/build/arch/mips.sh scripts/build/arch/moxie.sh scripts/build/arch/msp430.sh scripts/build/arch/nios2.sh scripts/build/arch/powerpc.sh scripts/build/arch/riscv.sh scripts/build/arch/s390.sh scripts/build/arch/sh.sh scripts/build/arch/sparc.sh scripts/build/arch/x86.sh scripts/build/arch/xtensa.sh '/usr/local/share/crosstool-ng/scripts/build/arch'
2020-01-21T10:47:30.8898265Z  /usr/bin/install -c -m 644  packages/gdb-linaro/7.8-2014.09/chksum packages/gdb-linaro/7.8-2014.09/version.desc '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.8-2014.09'
2020-01-21T10:47:30.8914165Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc64le-unknown-linux-gnu'
2020-01-21T10:47:30.8929150Z  /usr/bin/install -c -m 644  samples/powerpc64le-unknown-linux-gnu/crosstool.config samples/powerpc64le-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc64le-unknown-linux-gnu'
2020-01-21T10:47:30.8946081Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/duma'
---
2020-01-21T10:47:30.9316661Z  /usr/bin/install -c -m 644  packages/linux/3.17.8/chksum packages/linux/3.17.8/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.17.8'
2020-01-21T10:47:30.9337544Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/8.2.1'
2020-01-21T10:47:30.9351593Z  /usr/bin/install -c -m 644  packages/gdb/8.2.1/0000-musl_fix.patch packages/gdb/8.2.1/0001-uclibc-no-gettimeofday-clobber.patch packages/gdb/8.2.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/8.2.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/8.2.1/0004-allow-android.patch packages/gdb/8.2.1/chksum packages/gdb/8.2.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/8.2.1'
2020-01-21T10:47:30.9375375Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.10.1'
2020-01-21T10:47:30.9389814Z  /usr/bin/install -c -m 644  packages/gdb/7.10.1/0000-musl_fix.patch packages/gdb/7.10.1/0001-xtensa-initialize-call_abi-in-xtensa_tdep.patch packages/gdb/7.10.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/7.10.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/7.10.1/chksum packages/gdb/7.10.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.10.1'
2020-01-21T10:47:30.9426106Z  /usr/bin/install -c -m 644  packages/expat/2.2.6/chksum packages/expat/2.2.6/version.desc '/usr/local/share/crosstool-ng/packages/expat/2.2.6'
2020-01-21T10:47:30.9449650Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.10'
2020-01-21T10:47:30.9449650Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.10'
2020-01-21T10:47:30.9464365Z  /usr/bin/install -c -m 644  packages/strace/4.10/0000-aarch64_rt_sigreturn.patch packages/strace/4.10/0001-arm_mmap2.patch packages/strace/4.10/0002-aarch64_arch_regs.patch packages/strace/4.10/0003-stat64-v.test.patch packages/strace/4.10/0004-select_test.patch packages/strace/4.10/0005-fix_aarch64_ioctl_decoding.patch packages/strace/4.10/0006-fix_bexecve64_test.patch packages/strace/4.10/0007-decode_mips_indirect_syscall.patch packages/strace/4.10/0008-upstream-musl_includes.patch packages/strace/4.10/0009-use-host-ioctl.patch packages/strace/4.10/chksum packages/strace/4.10/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.10'
2020-01-21T10:47:30.9502216Z  /usr/bin/install -c -m 644  packages/strace/4.11/0000-use-host-ioctl.patch packages/strace/4.11/chksum packages/strace/4.11/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.11'
2020-01-21T10:47:30.9521483Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.12'
2020-01-21T10:47:30.9534014Z  /usr/bin/install -c -m 644  packages/strace/4.12/0000-use-host-ioctl.patch packages/strace/4.12/chksum packages/strace/4.12/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.12'
2020-01-21T10:47:30.9553471Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/debug'
2020-01-21T10:47:30.9553471Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/debug'
2020-01-21T10:47:30.9566255Z  /usr/bin/install -c -m 644  scripts/build/debug/200-duma.sh scripts/build/debug/300-gdb.sh scripts/build/debug/400-ltrace.sh scripts/build/debug/500-strace.sh scripts/build/debug/duma.in scripts/build/debug/gdbinit.in '/usr/local/share/crosstool-ng/scripts/build/debug'
2020-01-21T10:47:30.9599061Z  /usr/bin/install -c -m 644  packages/strace/4.13/0000-use-host-ioctl.patch packages/strace/4.13/chksum packages/strace/4.13/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.13'
2020-01-21T10:47:30.9617841Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.14'
2020-01-21T10:47:30.9631577Z  /usr/bin/install -c -m 644  packages/strace/4.14/0000-use-host-ioctl.patch packages/strace/4.14/chksum packages/strace/4.14/version.desc '/usr/local/share/crosstool-ng/packages/strace/4.14'
2020-01-21T10:47:30.9650440Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/4.15'
---
2020-01-21T10:47:31.0198525Z  /usr/bin/install -c -m 644  samples/riscv32-hifive1-elf/crosstool.config samples/riscv32-hifive1-elf/reported.by '/usr/local/share/crosstool-ng/samples/riscv32-hifive1-elf'
2020-01-21T10:47:31.0215356Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/armv7-rpi2-linux-gnueabihf'
2020-01-21T10:47:31.0226835Z  /usr/bin/install -c -m 644  samples/armv7-rpi2-linux-gnueabihf/crosstool.config samples/armv7-rpi2-linux-gnueabihf/reported.by '/usr/local/share/crosstool-ng/samples/armv7-rpi2-linux-gnueabihf'
2020-01-21T10:47:31.0253252Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages'
2020-01-21T10:47:31.0266798Z  /usr/bin/install -c -m 644  packages/GNU.help packages/Linaro.help '/usr/local/share/crosstool-ng/packages'
2020-01-21T10:47:31.0298852Z  /usr/bin/install -c -m 644  config/cc/gcc.in config/cc/gcc.in.mips '/usr/local/share/crosstool-ng/config/cc'
2020-01-21T10:47:31.0316443Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-unknown-linux-gnu'
2020-01-21T10:47:31.0330084Z  /usr/bin/install -c -m 644  samples/mipsel-unknown-linux-gnu/crosstool.config samples/mipsel-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-unknown-linux-gnu'
2020-01-21T10:47:31.0347740Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/nios2-altera-linux-gnu'
---
2020-01-21T10:47:31.0830278Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.25/0000-gdb8.patch packages/uClibc-ng/1.0.25/0001-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.25/chksum packages/uClibc-ng/1.0.25/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.25'
2020-01-21T10:47:31.0849201Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.26'
2020-01-21T10:47:31.0861754Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.26/0000-gdb8.patch packages/uClibc-ng/1.0.26/0001-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.26/chksum packages/uClibc-ng/1.0.26/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.26'
2020-01-21T10:47:31.0883548Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.27'
2020-01-21T10:47:31.0898263Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.27/0000-bessel-link-failure.patch packages/uClibc-ng/1.0.27/0001-feraiseexcept-2.patch packages/uClibc-ng/1.0.27/0002-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.27/0003-feraiseexcept.patch packages/uClibc-ng/1.0.27/chksum packages/uClibc-ng/1.0.27/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.27'
2020-01-21T10:47:31.0928525Z  /usr/bin/install -c -m 644  packages/mpc/1.1.0/chksum packages/mpc/1.1.0/version.desc '/usr/local/share/crosstool-ng/packages/mpc/1.1.0'
2020-01-21T10:47:31.0945575Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.28'
2020-01-21T10:47:31.0959278Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.28/0000-uClibc-ng-does-not-implement-name_to_handle_at.patch packages/uClibc-ng/1.0.28/chksum packages/uClibc-ng/1.0.28/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.28'
2020-01-21T10:47:31.0981141Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.29'
2020-01-21T10:47:31.0981141Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.29'
2020-01-21T10:47:31.0995637Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.29/chksum packages/uClibc-ng/1.0.29/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.29'
2020-01-21T10:47:31.1012476Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/licenses.d'
2020-01-21T10:47:31.1025731Z  /usr/bin/install -c -m 644  licenses.d/gpl.txt licenses.d/lgpl.txt '/usr/local/share/crosstool-ng/licenses.d'
2020-01-21T10:47:31.1057896Z  /usr/bin/install -c -m 644  packages/linux/2.6.37.6/chksum packages/linux/2.6.37.6/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.37.6'
2020-01-21T10:47:31.1092839Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/4.8.5'
2020-01-21T10:47:31.1110919Z  /usr/bin/install -c -m 644  packages/gcc/4.8.5/0000-gcc_bug_62231.patch packages/gcc/4.8.5/0001-gcc_bug_62231.patch packages/gcc/4.8.5/0002-libtool-leave-framework-alone.patch packages/gcc/4.8.5/0003-uclibc-conf.patch packages/gcc/4.8.5/0004-alpha-bad-eh_frame.patch packages/gcc/4.8.5/0005-pr65730.patch packages/gcc/4.8.5/0006-pr43538.patch packages/gcc/4.8.5/0007-mt-ospace-preserve-FLAGS_FOR_TARGET.patch packages/gcc/4.8.5/0008-build_gcc-5_with_gcc-6.patch packages/gcc/4.8.5/0009-missing-execinfo_h.patch packages/gcc/4.8.5/0010-libmudflap-susv3-legacy.patch packages/gcc/4.8.5/0011-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/4.8.5/0012-arm-softfloat-libgcc.patch packages/gcc/4.8.5/0013-arm_unbreak_armv4t.patch packages/gcc/4.8.5/0014-PR57717-E500v2.patch packages/gcc/4.8.5/0015-PR60155.patch packages/gcc/4.8.5/0016-aarch64-vmlaq_lane_s32-typo.patch packages/gcc/4.8.5/0017-libstdcxx-uclibc-c99.patch packages/gcc/4.8.5/0018-PR-other-56780.patch packages/gcc/4.8.5/0019-xtensa-add-mauto-litpools-option.patch packages/gcc/4.8.5/0020-xtensa-reimplement-register-spilling.patch packages/gcc/4.8.5/0021-xtensa-use-unwind-dw2-fde-dip-instead-of-unwind-dw2-.patch packages/gcc/4.8.5/0022-xtensa-fix-_Unwind_GetCFA.patch packages/gcc/4.8.5/0023-gcc-xtensa-fix-fprintf-format-specifiers.patch packages/gcc/4.8.5/0024-xtensa-fix-PR-target-82181.patch packages/gcc/4.8.5/0025-musl-support.patch packages/gcc/4.8.5/0026-cygwin64.patch packages/gcc/4.8.5/1000-powerpc-link-with-math-lib.patch.conditional packages/gcc/4.8.5/chksum packages/gcc/4.8.5/version.desc '/usr/local/share/crosstool-ng/packages/gcc/4.8.5'
2020-01-21T10:47:31.1147448Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/alphaev67-unknown-linux-gnu'
---
2020-01-21T10:47:31.1560188Z  /usr/bin/install -c -m 644  packages/glibc-ports/package.desc '/usr/local/share/crosstool-ng/packages/glibc-ports'
2020-01-21T10:47:31.1582422Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.12.1'
2020-01-21T10:47:31.1597144Z  /usr/bin/install -c -m 644  packages/gdb/7.12.1/0000-musl_fix.patch packages/gdb/7.12.1/0001-uclibc-no-gettimeofday-clobber.patch packages/gdb/7.12.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/7.12.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/7.12.1/0004-allow-android.patch packages/gdb/7.12.1/0005-include-order.patch packages/gdb/7.12.1/0006-duplicate-typedef.patch packages/gdb/7.12.1/chksum packages/gdb/7.12.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.12.1'
2020-01-21T10:47:31.1618029Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/debug'
2020-01-21T10:47:31.1633563Z  /usr/bin/install -c -m 644  config/debug/duma.in config/debug/gdb.in config/debug/gdb.in.cross config/debug/gdb.in.native config/debug/ltrace.in config/debug/strace.in '/usr/local/share/crosstool-ng/config/debug'
2020-01-21T10:47:31.1678536Z  /usr/bin/install -c -m 644  packages/gdb-linaro/7.3-2011.12/chksum packages/gdb-linaro/7.3-2011.12/version.desc '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.3-2011.12'
2020-01-21T10:47:31.1698580Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.2.0.20151023'
2020-01-21T10:47:31.1712536Z  /usr/bin/install -c -m 644  packages/newlib/2.2.0.20151023/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.2.0.20151023/0001-fix-eabihf.patch packages/newlib/2.2.0.20151023/0002-fix-mt-cflags.patch packages/newlib/2.2.0.20151023/chksum packages/newlib/2.2.0.20151023/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.2.0.20151023'
2020-01-21T10:47:31.1733598Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/android-ndk'
---
2020-01-21T10:47:31.3166612Z  /usr/bin/install -c -m 644  packages/ncurses/6.1/0000-ncurses-6.1-20180129.patch packages/ncurses/6.1/chksum packages/ncurses/6.1/version.desc '/usr/local/share/crosstool-ng/packages/ncurses/6.1'
2020-01-21T10:47:31.3187774Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples'
2020-01-21T10:47:31.3195570Z  /usr/bin/install -c -m 644  samples/samples.mk '/usr/local/share/crosstool-ng/samples'
2020-01-21T10:47:31.3217243Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/ltrace/0.5.3'
2020-01-21T10:47:31.3228417Z  /usr/bin/install -c -m 644  packages/ltrace/0.5.3/0000-allow-cross-compile.patch packages/ltrace/0.5.3/0001-alpha-support.patch packages/ltrace/0.5.3/0002-debian-ltrace_0.5.3-2.patch packages/ltrace/0.5.3/0003-add-sysdep.patch packages/ltrace/0.5.3/0004-mips.patch packages/ltrace/0.5.3/0005-mips-remove-CP.patch packages/ltrace/0.5.3/0006-allow-configurable-arch.patch packages/ltrace/0.5.3/0007-fix-missing-ptrace-defines.patch packages/ltrace/0.5.3/0008-lib-supcc.patch packages/ltrace/0.5.3/0009-libltrace-genindex.patch packages/ltrace/0.5.3/0010-ar-configurable.patch packages/ltrace/0.5.3/0011-configure-hostos.patch packages/ltrace/0.5.3/chksum packages/ltrace/0.5.3/version.desc '/usr/local/share/crosstool-ng/packages/ltrace/0.5.3'
2020-01-21T10:47:31.3268186Z  /usr/bin/install -c -m 644  samples/arm-cortex_a15-linux-gnueabihf/crosstool.config samples/arm-cortex_a15-linux-gnueabihf/reported.by '/usr/local/share/crosstool-ng/samples/arm-cortex_a15-linux-gnueabihf'
2020-01-21T10:47:31.3283336Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/moxiebox/git-9a79ac54'
2020-01-21T10:47:31.3294445Z  /usr/bin/install -c -m 644  packages/moxiebox/git-9a79ac54/0001-Remove-PKG_CONFIG-check.patch '/usr/local/share/crosstool-ng/packages/moxiebox/git-9a79ac54'
2020-01-21T10:47:31.3308440Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/2.6.39.4'
2020-01-21T10:47:31.3308440Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/2.6.39.4'
2020-01-21T10:47:31.3319673Z  /usr/bin/install -c -m 644  packages/linux/2.6.39.4/chksum packages/linux/2.6.39.4/version.desc '/usr/local/share/crosstool-ng/packages/linux/2.6.39.4'
2020-01-21T10:47:31.3335184Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/make/3.81'
2020-01-21T10:47:31.3346676Z  /usr/bin/install -c -m 644  packages/make/3.81/chksum packages/make/3.81/version.desc '/usr/local/share/crosstool-ng/packages/make/3.81'
2020-01-21T10:47:31.3361325Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc-unknown-linux-uclibc'
2020-01-21T10:47:31.3372770Z  /usr/bin/install -c -m 644  samples/powerpc-unknown-linux-uclibc/crosstool.config samples/powerpc-unknown-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-unknown-linux-uclibc'
2020-01-21T10:47:31.3391613Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/companion_libs'
2020-01-21T10:47:31.3408339Z  /usr/bin/install -c -m 644  scripts/build/companion_libs/050-zlib.sh scripts/build/companion_libs/100-gmp.sh scripts/build/companion_libs/110-mpfr.sh scripts/build/companion_libs/121-isl.sh scripts/build/companion_libs/130-cloog.sh scripts/build/companion_libs/140-mpc.sh scripts/build/companion_libs/200-libelf.sh scripts/build/companion_libs/210-expat.sh scripts/build/companion_libs/220-ncurses.sh scripts/build/companion_libs/320-libiconv.sh scripts/build/companion_libs/330-gettext.sh '/usr/local/share/crosstool-ng/scripts/build/companion_libs'
2020-01-21T10:47:31.3441819Z  /usr/bin/install -c -m 644  packages/gmp/4.3.2/chksum packages/gmp/4.3.2/version.desc '/usr/local/share/crosstool-ng/packages/gmp/4.3.2'
2020-01-21T10:47:31.3458531Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/contrib'
2020-01-21T10:47:31.3458531Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/contrib'
2020-01-21T10:47:31.3471776Z  /usr/bin/install -c -m 644  contrib/openrisc-or32.patch.lzma '/usr/local/share/crosstool-ng/contrib'
2020-01-21T10:47:31.3505358Z  /usr/bin/install -c -m 644  samples/x86_64-w64-mingw32/crosstool.config samples/x86_64-w64-mingw32/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-w64-mingw32'
2020-01-21T10:47:31.3524733Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/libc'
2020-01-21T10:47:31.3537574Z  /usr/bin/install -c -m 644  config/libc/avr-libc.in config/libc/bionic.in config/libc/glibc.in config/libc/mingw-w64.in config/libc/moxiebox.in config/libc/musl.in config/libc/newlib.in config/libc/none.in config/libc/uClibc.in '/usr/local/share/crosstool-ng/config/libc'
2020-01-21T10:47:31.3557326Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc64-multilib-linux-gnu'
---
2020-01-21T10:47:31.3660747Z  /usr/bin/install -c -m 644  config/binutils.in config/cc.in config/comp_libs.in config/comp_tools.in config/config.in config/debug.in config/global.in config/kernel.in config/libc.in config/target.in config/test_suite.in config/toolchain.in '/usr/local/share/crosstool-ng/config'
2020-01-21T10:47:31.3687881Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/automake/1.14.1'
2020-01-21T10:47:31.3700852Z  /usr/bin/install -c -m 644  packages/automake/1.14.1/0000-escape-left-brace.patch packages/automake/1.14.1/chksum packages/automake/1.14.1/version.desc '/usr/local/share/crosstool-ng/packages/automake/1.14.1'
2020-01-21T10:47:31.3721447Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc/0.9.33.2'
2020-01-21T10:47:31.3744482Z  /usr/bin/install -c -m 644  packages/uClibc/0.9.33.2/0000-m68k-ice.patch packages/uClibc/0.9.33.2/0001-fix-kernel-3.4plus-build.patch packages/uClibc/0.9.33.2/0002-fix-darwin-build.patch packages/uClibc/0.9.33.2/0003-arm-unwind.patch packages/uClibc/0.9.33.2/0004-no-install-D.patch packages/uClibc/0.9.33.2/0005-prefer-multilib.patch packages/uClibc/0.9.33.2/0006-dlopen-static.patch packages/uClibc/0.9.33.2/0007-make-olddefconfig.patch packages/uClibc/0.9.33.2/chksum packages/uClibc/0.9.33.2/version.desc '/usr/local/share/crosstool-ng/packages/uClibc/0.9.33.2'
2020-01-21T10:47:31.3780464Z  /usr/bin/install -c -m 644  samples/armeb-unknown-linux-uclibcgnueabi/crosstool.config samples/armeb-unknown-linux-uclibcgnueabi/reported.by '/usr/local/share/crosstool-ng/samples/armeb-unknown-linux-uclibcgnueabi'
2020-01-21T10:47:31.3795909Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/alphaev56-unknown-linux-gnu'
2020-01-21T10:47:31.3811183Z  /usr/bin/install -c -m 644  samples/alphaev56-unknown-linux-gnu/crosstool.config samples/alphaev56-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/alphaev56-unknown-linux-gnu'
2020-01-21T10:47:31.3825654Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-ubuntu12.04-linux-gnu'
---
2020-01-21T10:47:31.4097144Z  /usr/bin/install -c -m 644  samples/armeb-unknown-eabi/crosstool.config samples/armeb-unknown-eabi/reported.by '/usr/local/share/crosstool-ng/samples/armeb-unknown-eabi'
2020-01-21T10:47:31.4116363Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gmp/6.0.0a'
2020-01-21T10:47:31.4130514Z  /usr/bin/install -c -m 644  packages/gmp/6.0.0a/chksum packages/gmp/6.0.0a/version.desc '/usr/local/share/crosstool-ng/packages/gmp/6.0.0a'
2020-01-21T10:47:31.4152716Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/config/global'
2020-01-21T10:47:31.4165755Z  /usr/bin/install -c -m 644  config/global/build-behave.in config/global/ct-behave.in config/global/download.in config/global/extract.in config/global/logging.in config/global/paths.in '/usr/local/share/crosstool-ng/config/global'
2020-01-21T10:47:31.4202103Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.24'
2020-01-21T10:47:31.4219567Z  /usr/bin/install -c -m 644  packages/binutils/2.24/0000-fix-enable-install-libiberty-flag.patch packages/binutils/2.24/0001-dont-segv-on-initial-instructions-overflow.patch packages/binutils/2.24/0002-sh-conf.patch packages/binutils/2.24/0003-ld_makefile_patch.patch packages/binutils/2.24/0004-check_ldrunpath_length.patch packages/binutils/2.24/0005-fix-gold-pthreads-typo.patch packages/binutils/2.24/0006-sysroot.patch packages/binutils/2.24/0007-poison-system-directories.patch packages/binutils/2.24/0008-Fix-library-paths-on-PowerPC.patch packages/binutils/2.24/0009-xtensa-trampolines.patch packages/binutils/2.24/0010-xtensa-gas-first-frag-alignment.patch packages/binutils/2.24/0011-xtensa-gas-ld-diff-relocation-signed.patch packages/binutils/2.24/0012-xtensa-fix-ld-segfault-when-linking-linux-modules.patch packages/binutils/2.24/0013-Fix-call8-call-target-out-of-range-xtensa-ld-relaxation.patch packages/binutils/2.24/0014-Fix-trampolines-search-code-for-conditional-branches.patch packages/binutils/2.24/0015-xtensa-optimize-check_section_ebb_pcrels_fit.patch packages/binutils/2.24/0016-xtensa-optimize-removed_by_actions.patch packages/binutils/2.24/0017-xtensa-optimize-find_removed_literal.patch packages/binutils/2.24/0018-xtensa-replace-action-list-with-splay-tree.patch packages/binutils/2.24/0019-xtensa-optimize-trampolines-relaxation.patch packages/binutils/2.24/0020-xtensa-fix-localized-symbol-refcounting-with-gc-sect.patch packages/binutils/2.24/0021-xtensa-fix-gas-segfault-with-text-section-literals.patch packages/binutils/2.24/0022-xtensa-add-auto-litpools-option.patch packages/binutils/2.24/0023-xtensa-fix-signedness-of-gas-relocations.patch packages/binutils/2.24/0024-xtensa-fix-.init-.fini-literals-moving.patch packages/binutils/2.24/chksum packages/binutils/2.24/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.24'
2020-01-21T10:47:31.4273174Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.30/chksum packages/uClibc-ng/1.0.30/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.30'
2020-01-21T10:47:31.4297679Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.27'
2020-01-21T10:47:31.4320466Z  /usr/bin/install -c -m 644  packages/binutils/2.27/0000-missing-break.patch packages/binutils/2.27/0001-sh-conf.patch packages/binutils/2.27/0002-ld_makefile_patch.patch packages/binutils/2.27/0003-check_ldrunpath_length.patch packages/binutils/2.27/0004-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.27/0005-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.27/0006-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.27/0007-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.27/0008-sysroot.patch packages/binutils/2.27/0009-poison-system-directories.patch packages/binutils/2.27/0010-Fix-library-paths-on-PowerPC.patch packages/binutils/2.27/0011-xtensa-reverse-shift-count.patch packages/binutils/2.27/chksum packages/binutils/2.27/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.27'
2020-01-21T10:47:31.4347468Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/companion_tools'
2020-01-21T10:47:31.4347468Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/companion_tools'
2020-01-21T10:47:31.4362115Z  /usr/bin/install -c -m 644  scripts/build/companion_tools/050-make.sh scripts/build/companion_tools/100-m4.sh scripts/build/companion_tools/200-autoconf.sh scripts/build/companion_tools/300-automake.sh scripts/build/companion_tools/400-libtool.sh scripts/build/companion_tools/500-dtc.sh scripts/build/companion_tools/510-bison.sh '/usr/local/share/crosstool-ng/scripts/build/companion_tools'
2020-01-21T10:47:31.4398597Z  /usr/bin/install -c -m 644  packages/mingw-w64/v3.1.0/0000-mingw64-malloc.patch packages/mingw-w64/v3.1.0/0001-gendef-explicit-fallthrough.patch packages/mingw-w64/v3.1.0/0002-genpeimg-explicit-fallthrough.patch packages/mingw-w64/v3.1.0/chksum packages/mingw-w64/v3.1.0/version.desc '/usr/local/share/crosstool-ng/packages/mingw-w64/v3.1.0'
2020-01-21T10:47:31.4417417Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.31'
2020-01-21T10:47:31.4431982Z  /usr/bin/install -c -m 644  packages/uClibc-ng/1.0.31/chksum packages/uClibc-ng/1.0.31/version.desc '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.31'
2020-01-21T10:47:31.4447899Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/cc'
---
2020-01-21T10:47:31.5063567Z  /usr/bin/install -c -m 644  packages/expat/2.1.1/chksum packages/expat/2.1.1/version.desc '/usr/local/share/crosstool-ng/packages/expat/2.1.1'
2020-01-21T10:47:31.5080410Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.7.1-2014.06-1'
2020-01-21T10:47:31.5093945Z  /usr/bin/install -c -m 644  packages/gdb-linaro/7.7.1-2014.06-1/chksum packages/gdb-linaro/7.7.1-2014.06-1/version.desc '/usr/local/share/crosstool-ng/packages/gdb-linaro/7.7.1-2014.06-1'
2020-01-21T10:47:31.5111150Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/make/4.2.1'
2020-01-21T10:47:31.5125352Z  /usr/bin/install -c -m 644  packages/make/4.2.1/0000-glob-v2.patch packages/make/4.2.1/0001-glob-v2-gl_lstat.patch packages/make/4.2.1/chksum packages/make/4.2.1/version.desc '/usr/local/share/crosstool-ng/packages/make/4.2.1'
2020-01-21T10:47:31.5159002Z  /usr/bin/install -c -m 644  packages/gdb/8.1.1/0000-musl_fix.patch packages/gdb/8.1.1/0001-uclibc-no-gettimeofday-clobber.patch packages/gdb/8.1.1/0002-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/8.1.1/0003-WIP-end-of-prologue-detection-hack.patch packages/gdb/8.1.1/0004-allow-android.patch packages/gdb/8.1.1/chksum packages/gdb/8.1.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/8.1.1'
2020-01-21T10:47:31.5181733Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc'
2020-01-21T10:47:31.5193104Z  /usr/bin/install -c -m 644  packages/uClibc/config packages/uClibc/package.desc '/usr/local/share/crosstool-ng/packages/uClibc'
2020-01-21T10:47:31.5209550Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.9.1'
2020-01-21T10:47:31.5209550Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb/7.9.1'
2020-01-21T10:47:31.5221514Z  /usr/bin/install -c -m 644  packages/gdb/7.9.1/chksum packages/gdb/7.9.1/version.desc '/usr/local/share/crosstool-ng/packages/gdb/7.9.1'
2020-01-21T10:47:31.5245883Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/s390x-ibm-linux-gnu'
2020-01-21T10:47:31.5260626Z  /usr/bin/install -c -m 644  samples/s390x-ibm-linux-gnu/crosstool.config samples/s390x-ibm-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/s390x-ibm-linux-gnu'
2020-01-21T10:47:31.5278849Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/samples/moxie-unknown-elf'
2020-01-21T10:47:31.5290995Z  /usr/bin/install -c -m 644  samples/moxie-unknown-elf/crosstool.config samples/moxie-unknown-elf/reported.by '/usr/local/share/crosstool-ng/samples/moxie-unknown-elf'
2020-01-21T10:47:31.5313372Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/ltrace/0.7.3'
2020-01-21T10:47:31.5325573Z  /usr/bin/install -c -m 644  packages/ltrace/0.7.3/0000-avoid-libstdc++.patch packages/ltrace/0.7.3/0001-printf-p.patch packages/ltrace/0.7.3/0002-alpha-debug.h.patch packages/ltrace/0.7.3/0003-compile-warning.patch packages/ltrace/0.7.3/0004-sparc-ftbfs.patch packages/ltrace/0.7.3/0005-unexpected-breakpoint.patch packages/ltrace/0.7.3/0006-gcc-5.patch packages/ltrace/0.7.3/0007-glibc-2.24.patch packages/ltrace/0.7.3/chksum packages/ltrace/0.7.3/version.desc '/usr/local/share/crosstool-ng/packages/ltrace/0.7.3'
2020-01-21T10:47:31.5361296Z  /usr/bin/install -c -m 644  scripts/build/kernel/bare-metal.sh scripts/build/kernel/linux.sh scripts/build/kernel/windows.sh '/usr/local/share/crosstool-ng/scripts/build/kernel'
2020-01-21T10:47:31.5380722Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mingw-w64/v4.0.6'
2020-01-21T10:47:31.5394190Z  /usr/bin/install -c -m 644  packages/mingw-w64/v4.0.6/0000-mingw64-malloc.patch packages/mingw-w64/v4.0.6/0001-gendef-explicit-fallthrough.patch packages/mingw-w64/v4.0.6/0002-genpeimg-explicit-fallthrough.patch packages/mingw-w64/v4.0.6/chksum packages/mingw-w64/v4.0.6/version.desc '/usr/local/share/crosstool-ng/packages/mingw-w64/v4.0.6'
2020-01-21T10:47:31.5414588Z  /bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/3.14.79'
---
2020-01-21T10:47:38.4636838Z Removing intermediate container 5eb037d872dc
2020-01-21T10:47:38.4638037Z  ---> 46892d08cfbe
2020-01-21T10:47:38.4638347Z Step 9/68 : RUN ./build-riscv-toolchain.sh
2020-01-21T10:47:38.5158947Z  ---> Running in cfced5371d83
2020-01-21T10:47:38.8594178Z + mkdir -p /tmp/build-riscv
2020-01-21T10:47:38.8633276Z + cp riscv64-unknown-linux-gnu.config /tmp/build-riscv/.config
2020-01-21T10:47:38.8684217Z + cd /tmp/build-riscv
2020-01-21T10:47:38.8684528Z + set +x
2020-01-21T10:48:08.8680127Z Tue Jan 21 10:48:08 UTC 2020 - building ...
2020-01-21T10:48:38.9718791Z Tue Jan 21 10:48:38 UTC 2020 - building ...
2020-01-21T10:49:08.8807294Z Tue Jan 21 10:49:08 UTC 2020 - building ...
---
2020-01-21T11:45:23.2813294Z Successfully built ad07b3acf730
2020-01-21T11:45:23.2867433Z Successfully tagged rust-ci:latest
2020-01-21T11:45:23.3361751Z Built container sha256:ad07b3acf7304d634e8c2348e950c8a2e783b12c95ded55552f3487d4edd443f
2020-01-21T11:45:23.3378566Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/37e3b631ef273a28256a75d56b56604370975a77d4f3109db6cb2d5e679e997f389b20ba010ae0859fd81a52ff207473a4ce6661d680134e2d63b1a1b307fd3c
2020-01-21T11:49:26.7933953Z upload failed: - to s3://rust-lang-ci-sccache2/docker/37e3b631ef273a28256a75d56b56604370975a77d4f3109db6cb2d5e679e997f389b20ba010ae0859fd81a52ff207473a4ce6661d680134e2d63b1a1b307fd3c An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-01-21T11:49:27.6123287Z [CI_JOB_NAME=dist-various-1]
2020-01-21T11:49:27.6169052Z == clock drift check ==
2020-01-21T11:49:27.6232732Z   local time: Tue Jan 21 11:49:27 UTC 2020
2020-01-21T11:49:27.7787805Z   network time: Tue, 21 Jan 2020 11:49:27 GMT
---
2020-01-21T12:54:14.4898201Z     Finished release [optimized] target(s) in 1m 43s
2020-01-21T12:54:14.8817138Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv6m-none-eabi)
2020-01-21T12:54:14.8921181Z 
2020-01-21T12:54:14.8921602Z running 13 tests
2020-01-21T12:54:43.7281648Z ...iiiii.iiithread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:387:22
2020-01-21T12:54:43.7283563Z F
2020-01-21T12:54:43.7283800Z failures:
2020-01-21T12:54:43.7283945Z 
2020-01-21T12:54:43.7284436Z ---- [run-make] run-make/thumb-none-qemu stdout ----
2020-01-21T12:54:43.7284436Z ---- [run-make] run-make/thumb-none-qemu stdout ----
2020-01-21T12:54:43.7284628Z 
2020-01-21T12:54:43.7284793Z error: make failed
2020-01-21T12:54:43.7284975Z status: exit code: 2
2020-01-21T12:54:43.7285139Z command: "make"
2020-01-21T12:54:43.7285294Z stdout:
2020-01-21T12:54:43.7286035Z ------------------------------------------
2020-01-21T12:54:43.7286322Z bash script.sh
2020-01-21T12:54:43.7286855Z AR_riscv64gc_unknown_linux_gnu=riscv64-unknown-linux-gnu-ar
2020-01-21T12:54:43.7287375Z AR_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-ar
2020-01-21T12:54:43.7287591Z AWS_ACCESS_KEY_ID=$(SCCACHE_AWS_ACCESS_KEY_ID)
2020-01-21T12:54:43.7287789Z AWS_SECRET_ACCESS_KEY=$(SCCACHE_AWS_SECRET_ACCESS_KEY)
2020-01-21T12:54:43.7287976Z BOOTSTRAP_PARENT_ID=1508
2020-01-21T12:54:43.7288141Z BOOTSTRAP_PYTHON=/usr/bin/python2.7
2020-01-21T12:54:43.7288799Z BUILD_DIR=/checkout/obj/build
2020-01-21T12:54:43.7288965Z BUILD_SOURCEBRANCHNAME=merge
2020-01-21T12:54:43.7289436Z CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
2020-01-21T12:54:43.7289648Z CARGO_HOME=/cargo
---
2020-01-21T12:54:43.7298514Z CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
2020-01-21T12:54:43.7300207Z DEPLOY=1
2020-01-21T12:54:43.7300835Z HERE=/checkout/src/test/run-make/thumb-none-qemu
2020-01-21T12:54:43.7301092Z HOME=/home/user
2020-01-21T12:54:43.7301252Z HOSTNAME=d90cf90f45c4
2020-01-21T12:54:43.7301739Z HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
2020-01-21T12:54:43.7302249Z LD_LIBRARY_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
2020-01-21T12:54:43.7302473Z LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
2020-01-21T12:54:43.7302815Z LLVM_CXXFLAGS=
2020-01-21T12:54:43.7302815Z LLVM_CXXFLAGS=
2020-01-21T12:54:43.7303283Z LLVM_FILECHECK=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck
2020-01-21T12:54:43.7303511Z MAKEFLAGS=
2020-01-21T12:54:43.7303670Z MAKELEVEL=1
2020-01-21T12:54:43.7303820Z MFLAGS=
2020-01-21T12:54:43.7304794Z PATH=/x-tools/riscv64-unknown-linux-gnu/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
2020-01-21T12:54:43.7305295Z PWD=/checkout/src/test/run-make/thumb-none-qemu
2020-01-21T12:54:43.7305514Z PYTHON=/usr/bin/python2.7
2020-01-21T12:54:43.7306478Z RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf
2020-01-21T12:54:43.7306478Z RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf
2020-01-21T12:54:43.7307095Z RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
2020-01-21T12:54:43.7307612Z RUSTFMT=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt
2020-01-21T12:54:43.7308789Z RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7hf=/musl-armv7hf       --musl-root-aarch64=/musl-aarch64       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --musl-root-mips64=/musl-mips64       --musl-root-mips64el=/musl-mips64el       --disable-docs --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --debuginfo-level-std=1 --enable-missing-tools
2020-01-21T12:54:43.7309081Z RUST_RELEASE_CHANNEL=nightly
2020-01-21T12:54:43.7309249Z S=/checkout
2020-01-21T12:54:43.7311540Z SCRIPT=python2.7 ../x.py test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf src/test/run-make &&       python2.7 ../x.py dist --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,riscv64gc-unknown-linux-gnu,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
2020-01-21T12:54:43.7312065Z SHLVL=3
2020-01-21T12:54:43.7312332Z SRC=/checkout
2020-01-21T12:54:43.7312561Z STAGING_DIR=/tmp
2020-01-21T12:54:43.7312561Z STAGING_DIR=/tmp
2020-01-21T12:54:43.7313012Z TARGET=thumbv6m-none-eabi
2020-01-21T12:54:43.7314410Z TARGETS=asmjs-unknown-emscripten,wasm32-unknown-emscripten,x86_64-rumprun-netbsd,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-linux-musl,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,riscv64gc-unknown-linux-gnu,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf
2020-01-21T12:54:43.7315059Z TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
2020-01-21T12:54:43.7315275Z TERM=xterm
2020-01-21T12:54:43.7315436Z TF_BUILD=True
2020-01-21T12:54:43.7316264Z TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
2020-01-21T12:54:43.7316829Z TOOLSTATE_REPO=https://github.com/rust-lang-nursery/rust-toolstate.git
2020-01-21T12:54:43.7317078Z TOOLSTATE_REPO_ACCESS_TOKEN=$(TOOLSTATE_REPO_ACCESS_TOKEN)
2020-01-21T12:54:43.7317591Z WORK_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
2020-01-21T12:54:43.7317800Z _=/usr/bin/env
2020-01-21T12:54:43.7317986Z __COMPAT_LAYER=RunAsInvoker
2020-01-21T12:54:43.7318513Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
2020-01-21T12:54:43.7319222Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu /checkout/src/test/run-make/thumb-none-qemu
2020-01-21T12:54:43.7319712Z Makefile:27: recipe for target 'all' failed
2020-01-21T12:54:43.7320336Z ------------------------------------------
2020-01-21T12:54:43.7320538Z stderr:
2020-01-21T12:54:43.7320939Z ------------------------------------------
2020-01-21T12:54:43.7320939Z ------------------------------------------
2020-01-21T12:54:43.7321180Z + CRATE=example
2020-01-21T12:54:43.7321340Z + env
2020-01-21T12:54:43.7321507Z + sort
2020-01-21T12:54:43.7322009Z + mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
2020-01-21T12:54:43.7322841Z + pushd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu
2020-01-21T12:54:43.7323365Z + rm -rf example
2020-01-21T12:54:43.7323834Z + cp -a /checkout/src/test/run-make/thumb-none-qemu/example .
2020-01-21T12:54:43.7324038Z + pushd example
2020-01-21T12:54:43.7324645Z + env RUSTC_BOOTSTRAP=1 'RUSTFLAGS=-C linker=arm-none-eabi-ld -C link-arg=-Tlink.x' /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
2020-01-21T12:54:43.7325085Z + grep 'x = 42'
---
2020-01-21T12:54:43.7336541Z    Compiling proc-macro2 v1.0.8
2020-01-21T12:54:43.7336761Z    Compiling unicode-xid v0.2.0
2020-01-21T12:54:43.7336826Z    Compiling stable_deref_trait v1.1.1
2020-01-21T12:54:43.7336888Z    Compiling syn v1.0.14
2020-01-21T12:54:43.7337105Z    Compiling cortex-m v0.6.2
2020-01-21T12:54:43.7337150Z    Compiling vcell v0.1.2
2020-01-21T12:54:43.7337372Z    Compiling cortex-m-semihosting v0.3.5
2020-01-21T12:54:43.7337667Z    Compiling cortex-m-rt v0.6.11
2020-01-21T12:54:43.7337715Z    Compiling r0 v0.2.2
2020-01-21T12:54:43.7337921Z    Compiling panic-halt v0.2.0
2020-01-21T12:54:43.7349475Z    Compiling volatile-register v0.2.0
2020-01-21T12:54:43.7349748Z    Compiling rustc_version v0.2.3
2020-01-21T12:54:43.7350195Z    Compiling generic-array v0.13.2
2020-01-21T12:54:43.7350473Z    Compiling generic-array v0.12.3
2020-01-21T12:54:43.7350473Z    Compiling generic-array v0.12.3
2020-01-21T12:54:43.7350527Z    Compiling quote v1.0.2
2020-01-21T12:54:43.7350775Z    Compiling bare-metal v0.2.5
2020-01-21T12:54:43.7351045Z    Compiling as-slice v0.1.2
2020-01-21T12:54:43.7351099Z    Compiling aligned v0.3.2
2020-01-21T12:54:43.7351370Z    Compiling cortex-m-rt-macros v0.1.7
2020-01-21T12:54:43.7351750Z    Compiling example v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example)
2020-01-21T12:54:43.7351820Z     Finished dev [unoptimized + debuginfo] target(s) in 27.87s
2020-01-21T12:54:43.7352241Z      Running `qemu-system-arm -cpu cortex-m0 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv6m-none-eabi/debug/example`
2020-01-21T12:54:43.7352330Z Unable to find CPU definition
2020-01-21T12:54:43.7352381Z make: *** [all] Error 1
2020-01-21T12:54:43.7352696Z ------------------------------------------
2020-01-21T12:54:43.7352732Z 
2020-01-21T12:54:43.7352760Z 
2020-01-21T12:54:43.7352788Z 
2020-01-21T12:54:43.7352788Z 
2020-01-21T12:54:43.7352850Z failures:
2020-01-21T12:54:43.7353100Z     [run-make] run-make/thumb-none-qemu
2020-01-21T12:54:43.7353135Z 
2020-01-21T12:54:43.7353431Z test result: FAILED. 4 passed; 1 failed; 8 ignored; 0 measured; 0 filtered out
2020-01-21T12:54:43.7353497Z 
2020-01-21T12:54:43.7353526Z 
2020-01-21T12:54:43.7353553Z 
2020-01-21T12:54:43.7357380Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--quiet" "--llvm-version" "9.0.1-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-21T12:54:43.7357773Z 
2020-01-21T12:54:43.7357806Z 
2020-01-21T12:54:43.7358452Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf src/test/run-make
2020-01-21T12:54:43.7358534Z Build completed unsuccessfully in 1:03:25
2020-01-21T12:54:43.7358534Z Build completed unsuccessfully in 1:03:25
2020-01-21T12:54:43.7358599Z == clock drift check ==
2020-01-21T12:54:43.7359362Z   local time: Tue Jan 21 12:54:43 UTC 2020
2020-01-21T12:54:44.0234210Z   network time: Tue, 21 Jan 2020 12:54:44 GMT
2020-01-21T12:54:44.0242927Z == end clock drift check ==
2020-01-21T12:54:46.2951185Z 
2020-01-21T12:54:46.3345305Z ##[error]Bash exited with code '1'.
2020-01-21T12:54:46.3359950Z ##[section]Finishing: Run build
2020-01-21T12:54:46.3389252Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68194/merge to s
2020-01-21T12:54:46.3391517Z Task         : Get sources
2020-01-21T12:54:46.3391570Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T12:54:46.3391623Z Version      : 1.0.0
2020-01-21T12:54:46.3391686Z Author       : Microsoft
2020-01-21T12:54:46.3391686Z Author       : Microsoft
2020-01-21T12:54:46.3391741Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T12:54:46.3391813Z ==============================================================================
2020-01-21T12:54:46.8441490Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T12:54:46.8495575Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68194/merge to s
2020-01-21T12:54:46.8625162Z Cleaning up task key
2020-01-21T12:54:46.8625936Z Start cleaning up orphan processes.
2020-01-21T12:54:46.8789345Z Terminate orphan process: pid (4078) (python)
2020-01-21T12:54:46.9098623Z ##[section]Finishing: Finalize Job
