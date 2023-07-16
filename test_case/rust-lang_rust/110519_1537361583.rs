plain
  libtool-bin libtsan0 libubsan1 libuv1 libxml-libxml-perl
  libxml-namespacesupport-perl libxml-sax-base-perl libxml-sax-perl libxml2
  linux-libc-dev m4 make media-types ninja-build openssl patch perl
  perl-modules-5.34 pkg-config python3 python3-minimal python3.10
  python3.10-minimal readline-common rpcsvc-proto rsync sudo tex-common
0 upgraded, 128 newly installed, 0 to remove and 0 not upgraded.
Need to get 123 MB of archives.
After this operation, 463 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu jammy/main amd64 liblocale-gettext-perl amd64 1.07-4build3 [17.1 kB]
---
 ---> ad0e791c5dc0
Step 5/18 : RUN sh /scripts/crosstool-ng-git.sh
 ---> Running in e32a9da6a0cb
+ URL=https://github.com/crosstool-ng/crosstool-ng
+ REV=943364711a650d9b9e84c1b42c91cc0265b6ab5c
+ mkdir crosstool-ng
+ git init
hint: Using 'master' as the name for the initial branch. This default branch name
hint: is subject to change. To configure the initial branch name to use in all
hint: of your new repositories, which will suppress this warning, call:
---
+ git reset --hard FETCH_HEAD
HEAD is now at 9433647 uClibc-ng: Add 1.0.43
+ ./bootstrap
INFO  :: *** Generating package version descriptions
INFO  :: Master packages: autoconf automake avr-libc binutils bison cloog dtc duma elf2flt expat gcc gdb gettext glibc gmp gnuprumcu isl libelf libiconv libtool linux ltrace m4 make mingw-w64 moxiebox mpc mpfr musl ncurses newlib-nano newlib picolibc strace uClibc-ng zlib zstd
INFO  :: Generating 'config/versions/automake.in'
INFO  :: Generating 'config/versions/avr-libc.in'
INFO  :: Generating 'config/versions/binutils.in'
INFO  :: Generating 'config/versions/bison.in'
---
INFO  :: Generating comp_tools.in (menu)
INFO  :: Generating comp_libs.in (menu)
INFO  :: *** Gathering the list of data files to install
INFO  :: *** Running autoreconf
configure.ac:306: warning: gl_HOST_CPU_C_ABI_32BIT is m4_require'd but not m4_defun'd
m4/lib-prefix.m4:155: AC_LIB_PREPARE_MULTILIB is expanded from...
m4/lib-link.m4:181: AC_LIB_LINKFLAGS_BODY is expanded from...
m4/iconv.m4:10: AM_ICONV_LINKFLAGS_BODY is expanded from...
m4/gettext.m4:55: AM_GNU_GETTEXT is expanded from...
INFO  :: *** Done!
+ ./configure --prefix=/usr/local
checking for a BSD-compatible install... /usr/bin/install -c
checking whether build environment is sane... yes
---
checking for python build information... 
checking for python3.12... no
checking for python3.11... no
checking for python3.10... python3.10
checking for main in -lpython3.10... no
checking for main in -lpython3.10m... no
checking for python3.9... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.8... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.7... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.6... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.5... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.4... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.3... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.2... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.1... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python3.0... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.7... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.6... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.5... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.4... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.3... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.2... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python2.1... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
checking for python... (cached) python3.10
checking for main in -lpython3.10... (cached) no
checking for main in -lpython3.10m... (cached) no
  results of the Python check:
    Library:     no
    Include Dir: no
    Include Dir: no
checking for python version greater than 3.4... yes: 3.10
checking for bison >= 2.7... yes
checking for bison >= 3.0.4... yes
checking for dtc... no
checking for cvs... no
---
checking for alloca... yes
checking for ld used by gcc... /usr/bin/ld
checking if the linker (/usr/bin/ld) is GNU ld... yes
checking for shared library run path origin... done
./configure: line 13432: gl_HOST_CPU_C_ABI_32BIT: command not found
checking for ELF binary format... yes
checking for the common suffixes of directories in the library search path... ./configure: line 13533: test: !=: unary operator expected
./configure: line 13539: test: !=: unary operator expected
./configure: line 13564: test: =: unary operator expected
./configure: line 13568: test: =: unary operator expected
lib,lib,lib
checking for CFPreferencesCopyAppValue... no
checking for CFLocaleCopyPreferredLanguages... no
checking for GNU gettext in libc... yes
checking whether to use NLS... yes
---
 /usr/bin/install -c -m 644  packages/gdb/10.2/0000-uclibc-no-gettimeofday-clobber.patch packages/gdb/10.2/0001-xtensa-make-sure-ar_base-is-initialized.patch packages/gdb/10.2/0002-WIP-end-of-prologue-detection-hack.patch packages/gdb/10.2/0003-allow-android.patch packages/gdb/10.2/0004-xtensa-Fix-compilation-of-gdbserver.patch packages/gdb/10.2/0005-arc-Add-support-for-signal-handlers.patch packages/gdb/10.2/0006-arc-Add-support-for-signal-frames-for-Linux-targets.patch packages/gdb/10.2/0007-arc-Take-into-account-the-REGNUM-in-supply-collect-g.patch packages/gdb/10.2/0008-gdb-Add-native-support-for-ARC-in-GNU-Linux.patch packages/gdb/10.2/0009-arc-Make-variable-name-in-comments-uppercase.patch packages/gdb/10.2/0010-gdb-Log-pc-value-in-arc_skip_prologue.patch packages/gdb/10.2/0011-gdb-Use-correct-feature-in-tdesc-regs-for-ARC.patch packages/gdb/10.2/0012-gdb-Add-default-reggroups-for-ARC.patch packages/gdb/10.2/0013-arc-Construct-disassembler-options-dynamically.patch packages/gdb/10.2/0014-arc-Add-set-disassembler-options-support.patch packages/gdb/10.2/0015-gdb-Fix-numerical-field-extraction-for-target-descri.patch packages/gdb/10.2/0016-gdb-Make-the-builtin-boolean-type-an-unsigned-type.patch packages/gdb/10.2/0017-opcodes-Fix-the-auxiliary-register-numbers-for-ARC-H.patch packages/gdb/10.2/chksum packages/gdb/10.2/version.desc '/usr/local/share/crosstool-ng/packages/gdb/10.2'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/aarch64-rpi4-linux-gnu'
 /usr/bin/install -c -m 644  samples/aarch64-rpi4-linux-gnu/crosstool.config samples/aarch64-rpi4-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-rpi4-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/elf2flt/git-453398f9'
 /usr/bin/install -c -m 644  packages/elf2flt/git-453398f9/0000-support-binutils-2.34.patch packages/elf2flt/git-453398f9/0001-elf2flt-Use-PRIx64-instead-of-BFD_VMA_FMT-x.patch '/usr/local/share/crosstool-ng/packages/elf2flt/git-453398f9'
 /usr/bin/install -c -m 644  packages/linux/5.13.14/chksum packages/linux/5.13.14/version.desc '/usr/local/share/crosstool-ng/packages/linux/5.13.14'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc'
 /usr/bin/install -c -m 644  samples/x86_64-multilib-linux-uclibc/crosstool.config samples/x86_64-multilib-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.17.15'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.17.15'
 /usr/bin/install -c -m 644  packages/linux/5.17.15/chksum packages/linux/5.17.15/version.desc '/usr/local/share/crosstool-ng/packages/linux/5.17.15'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/libc'
 /usr/bin/install -c -m 644  scripts/build/libc/avr-libc.sh scripts/build/libc/glibc.sh scripts/build/libc/mingw-w64.sh scripts/build/libc/moxiebox.sh scripts/build/libc/musl.sh scripts/build/libc/newlib.sh scripts/build/libc/none.sh scripts/build/libc/picolibc.sh scripts/build/libc/uClibc-ng.sh '/usr/local/share/crosstool-ng/scripts/build/libc'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/6.5.0'
 /usr/bin/install -c -m 644  packages/gcc/6.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/6.5.0/0001-uclibc-conf.patch packages/gcc/6.5.0/0002-missing-execinfo_h.patch packages/gcc/6.5.0/0003-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/6.5.0/0004-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/6.5.0/0005-arm-softfloat-libgcc.patch packages/gcc/6.5.0/0006-arm_unbreak_armv4t.patch packages/gcc/6.5.0/0007-ARM-PR-target-70473-Reduce-size-of-Cortex-A8-automat.patch packages/gcc/6.5.0/0008-cilk-wchar.patch packages/gcc/6.5.0/0009-fix-m68k-compile.patch packages/gcc/6.5.0/0010-fix-m68k-uclinux.patch packages/gcc/6.5.0/0011-libgcc-mkmap-symver-support-skip_underscore.patch packages/gcc/6.5.0/0012-libgcc-config-bfin-use-the-generic-linker-version-in.patch packages/gcc/6.5.0/0013-libgcc-fix-DWARF-compilation-with-FDPIC-targets.patch packages/gcc/6.5.0/0014-bfin-define-REENTRANT.patch packages/gcc/6.5.0/0015-libgfortran-missing-include.patch packages/gcc/6.5.0/0016-nios2-bad-multilib-default.patch packages/gcc/6.5.0/0017-libgcc-disable-split-stack-nothreads.patch packages/gcc/6.5.0/0018-uclinux-enable-threads.patch packages/gcc/6.5.0/0019-bionic-ndk.patch packages/gcc/6.5.0/0020-bionic-errno.patch packages/gcc/6.5.0/0021-crystax.patch packages/gcc/6.5.0/0022-crystax.patch packages/gcc/6.5.0/0023-crystax.patch packages/gcc/6.5.0/0024-crystax.patch packages/gcc/6.5.0/0025-crystax.patch packages/gcc/6.5.0/0026-crystax.patch packages/gcc/6.5.0/0027-crystax.patch packages/gcc/6.5.0/0028-crystax.patch packages/gcc/6.5.0/0029-msp430-fix.patch packages/gcc/6.5.0/0030-isl-0.20.patch packages/gcc/6.5.0/0031-darwin-aarch64-config.patch packages/gcc/6.5.0/0032-darwin-aarch64-self-host-driver.patch packages/gcc/6.5.0/0033-darwin-align-pch_address_space-to-16k.patch packages/gcc/6.5.0/chksum packages/gcc/6.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/6.5.0'
 /usr/bin/install -c -m 644  packages/mingw-w64/v9.0.0/0000-mingw64-malloc.patch packages/mingw-w64/v9.0.0/chksum packages/mingw-w64/v9.0.0/version.desc '/usr/local/share/crosstool-ng/packages/mingw-w64/v9.0.0'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-unknown-eabi'
 /usr/bin/install -c -m 644  samples/arm-unknown-eabi/crosstool.config samples/arm-unknown-eabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-eabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/pru'
---
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.18.15'
 /usr/bin/install -c -m 644  packages/linux/5.18.15/chksum packages/linux/5.18.15/version.desc '/usr/local/share/crosstool-ng/packages/linux/5.18.15'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mips-malta-linux-gnu'
 /usr/bin/install -c -m 644  samples/mips-malta-linux-gnu/crosstool.config samples/mips-malta-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mips-malta-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-0b79182e'
 /usr/bin/install -c -m 644  packages/binutils-oracle/git-0b79182e/0001-binutils-gdb-oracle-backport.patch '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-0b79182e'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-ol8u7-linux-gnu'
 /usr/bin/install -c -m 644  samples/i686-ol8u7-linux-gnu/crosstool.config samples/i686-ol8u7-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/i686-ol8u7-linux-gnu'
 /usr/bin/install -c -m 644  packages/binutils/2.30/0000-sh-conf.patch packages/binutils/2.30/0001-ld_makefile_patch.patch packages/binutils/2.30/0002-check_ldrunpath_length.patch packages/binutils/2.30/0003-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.30/0004-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.30/0005-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.30/0006-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.30/0007-sysroot.patch packages/binutils/2.30/0008-poison-system-directories.patch packages/binutils/2.30/0009-Fix-a-missing-include-of-string.patch packages/binutils/2.30/chksum packages/binutils/2.30/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.30'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/config/binutils'
 /usr/bin/install -c -m 644  config/binutils/binutils.in '/usr/local/share/crosstool-ng/config/binutils'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.32'
---
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/zlib/1.2.12'
 /usr/bin/install -c -m 644  packages/zlib/1.2.12/0000-mingw-static-only.patch packages/zlib/1.2.12/0001-crossbuild-macos-libtool.patch packages/zlib/1.2.12/chksum packages/zlib/1.2.12/version.desc '/usr/local/share/crosstool-ng/packages/zlib/1.2.12'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/zlib/1.2.13'
 /usr/bin/install -c -m 644  packages/zlib/1.2.13/0000-mingw-static-only.patch packages/zlib/1.2.13/0001-crossbuild-macos-libtool.patch packages/zlib/1.2.13/chksum packages/zlib/1.2.13/version.desc '/usr/local/share/crosstool-ng/packages/zlib/1.2.13'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-ol8u6-linux-gnu'
 /usr/bin/install -c -m 644  samples/x86_64-ol8u6-linux-gnu/crosstool.config samples/x86_64-ol8u6-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-ol8u6-linux-gnu'
 /usr/bin/install -c -m 644  packages/linux/4.13.16/chksum packages/linux/4.13.16/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.13.16'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/6.3'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/6.3'
 /usr/bin/install -c -m 644  packages/linux/6.3/chksum packages/linux/6.3/version.desc '/usr/local/share/crosstool-ng/packages/linux/6.3'
 /usr/bin/install -c -m 644  packages/glibc-oracle/package.desc '/usr/local/share/crosstool-ng/packages/glibc-oracle'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
 /usr/bin/install -c -m 644  samples/mipsel-multilib-linux-gnu/crosstool.config samples/mipsel-multilib-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/mipsel-multilib-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/duma/2_5_15'
---
 /usr/bin/install -c -m 644  packages/expat/2.5.0/chksum packages/expat/2.5.0/version.desc '/usr/local/share/crosstool-ng/packages/expat/2.5.0'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.15.18'
 /usr/bin/install -c -m 644  packages/linux/4.15.18/chksum packages/linux/4.15.18/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.15.18'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/10.4.0'
 /usr/bin/install -c -m 644  packages/gcc/10.4.0/0000-libtool-leave-framework-alone.patch packages/gcc/10.4.0/0001-uclibc-conf.patch packages/gcc/10.4.0/0002-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/10.4.0/0003-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/10.4.0/0004-arm-softfloat-libgcc.patch packages/gcc/10.4.0/0005-fix-m68k-uclinux.patch packages/gcc/10.4.0/0006-libgfortran-missing-include.patch packages/gcc/10.4.0/0007-nios2-bad-multilib-default.patch packages/gcc/10.4.0/0008-libgcc-disable-split-stack-nothreads.patch packages/gcc/10.4.0/0009-bionic-ndk.patch packages/gcc/10.4.0/0010-crystax.patch packages/gcc/10.4.0/0010-fixinc-don-t-fix-machine-names-in-__has_include-.PR.patch packages/gcc/10.4.0/0011-crystax.patch packages/gcc/10.4.0/0012-crystax.patch packages/gcc/10.4.0/0013-crystax.patch packages/gcc/10.4.0/0014-crystax.patch packages/gcc/10.4.0/0015-crystax.patch packages/gcc/10.4.0/0016-crystax.patch packages/gcc/10.4.0/0017-crystax.patch packages/gcc/10.4.0/0018-isl-0.20.patch packages/gcc/10.4.0/0019-AArch64-Fix-build-issue-with-aarch64-builtins.c-on-M.patch packages/gcc/10.4.0/0020-libstdcxx-pure-stdio.patch packages/gcc/10.4.0/0021-Darwin-Arm64-Initial-support-for-the-self-host-drive.patch packages/gcc/10.4.0/0022-Remove-use-of-include_next-from-c-headers.patch packages/gcc/10.4.0/chksum packages/gcc/10.4.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/10.4.0'
 /usr/bin/install -c -m 644  packages/moxiebox/package.desc packages/moxiebox/sha.h packages/moxiebox/sha256_wrap.c '/usr/local/share/crosstool-ng/packages/moxiebox'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/newlib/2.5.0.20171222'
 /usr/bin/install -c -m 644  packages/newlib/2.5.0.20171222/0000-fix-unaligned-access-memcpy-m68k.patch packages/newlib/2.5.0.20171222/0001-fix-mt-cflags.patch packages/newlib/2.5.0.20171222/chksum packages/newlib/2.5.0.20171222/version.desc '/usr/local/share/crosstool-ng/packages/newlib/2.5.0.20171222'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/6.0.12'
---
 /usr/bin/install -c -m 644  samples/i586-geode-linux-uclibc/crosstool.config samples/i586-geode-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/i586-geode-linux-uclibc'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.14.314'
 /usr/bin/install -c -m 644  packages/linux/4.14.314/chksum packages/linux/4.14.314/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.14.314'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/11.3.0'
 /usr/bin/install -c -m 644  packages/gcc/11.3.0/0000-libtool-leave-framework-alone.patch packages/gcc/11.3.0/0001-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/11.3.0/0002-arm-softfloat-libgcc.patch packages/gcc/11.3.0/0003-libgcc-disable-split-stack-nothreads.patch packages/gcc/11.3.0/0004-Remove-use-of-include_next-from-c-headers.patch packages/gcc/11.3.0/0005-arc-Update-ZOL-pattern.patch packages/gcc/11.3.0/0006-arc-Update-u-maddhisi4-patterns.patch packages/gcc/11.3.0/0007-arc-Fix-maddhisi-patterns.patch packages/gcc/11.3.0/0008-Darwin-aarch64-Initial-support-for-the-self-host-dri.patch packages/gcc/11.3.0/0009-libstdc-Check-for-TLS-support-on-mingw-cross-compile.patch packages/gcc/11.3.0/0010-fixinc-don-t-fix-machine-names-in-__has_include-.PR.patch packages/gcc/11.3.0/chksum packages/gcc/11.3.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/11.3.0'
 /usr/bin/install -c -m 644  samples/arm-unknown-linux-musleabi/crosstool.config samples/arm-unknown-linux-musleabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-musleabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/ltrace'
 /usr/bin/install -c -m 644  packages/ltrace/package.desc '/usr/local/share/crosstool-ng/packages/ltrace'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.36.1'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.36.1'
 /usr/bin/install -c -m 644  packages/binutils/2.36.1/0000-sh-conf.patch packages/binutils/2.36.1/0001-ld_makefile_patch.patch packages/binutils/2.36.1/0002-check_ldrunpath_length.patch packages/binutils/2.36.1/0003-MinGW-w64-winpthreads-doesnt-have-pthread_mutexattr_settype.patch packages/binutils/2.36.1/0004-Dont-link-to-libfl-as-its-unnecessary.patch packages/binutils/2.36.1/0005-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.36.1/0006-Darwin-Two-fixes-from-Android-NDK-PTHREAD_ONCE_INIT-wcsncasecmp.patch packages/binutils/2.36.1/0007-sysroot.patch packages/binutils/2.36.1/0008-poison-system-directories.patch packages/binutils/2.36.1/chksum packages/binutils/2.36.1/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.36.1'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils-oracle/2.27'
 /usr/bin/install -c -m 644  packages/binutils-oracle/2.27/version.desc '/usr/local/share/crosstool-ng/packages/binutils-oracle/2.27'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils'
 /usr/bin/install -c -m 644  packages/binutils/binutils-ld.in packages/binutils/package.desc '/usr/local/share/crosstool-ng/packages/binutils'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-oracle/git-dbf713b5'
 /usr/bin/install -c -m 644  packages/gcc-oracle/git-dbf713b5/0001-Fix-compiler-error.patch '/usr/local/share/crosstool-ng/packages/gcc-oracle/git-dbf713b5'
 /usr/bin/install -c -m 644  samples/sh-multilib-linux-gnu/crosstool.config samples/sh-multilib-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/sh-multilib-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-uclibcgnueabi'
 /usr/bin/install -c -m 644  samples/arm-unknown-linux-uclibcgnueabi/crosstool.config samples/arm-unknown-linux-uclibcgnueabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-unknown-linux-uclibcgnueabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/config/versions'
---
 /usr/bin/install -c -m 644  samples/aarch64-ol8u7-linux-gnu/crosstool.config samples/aarch64-ol8u7-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-ol8u7-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gettext'
 /usr/bin/install -c -m 644  packages/gettext/package.desc '/usr/local/share/crosstool-ng/packages/gettext'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils/2.40'
 /usr/bin/install -c -m 644  packages/binutils/2.40/0000-sh-conf.patch packages/binutils/2.40/0001-check_ldrunpath_length.patch packages/binutils/2.40/0002-MinGW-w64-winpthreads-doesn-t-have-pthread_mutexattr.patch packages/binutils/2.40/0003-Don-t-link-to-libfl-as-it-s-unnecessary.patch packages/binutils/2.40/0004-Darwin-gold-binary-cc-include-string-not-cstring.patch packages/binutils/2.40/0005-Fix-darwin-build.patch packages/binutils/2.40/0006-sysroot.patch packages/binutils/2.40/0007-poison-system-directories.patch packages/binutils/2.40/chksum packages/binutils/2.40/version.desc '/usr/local/share/crosstool-ng/packages/binutils/2.40'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.23'
 /usr/bin/install -c -m 644  packages/glibc/2.23/0000-Suppress-GCC-6-warning-about-ambiguous-else-with-Wpa.patch packages/glibc/2.23/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.23/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.23/0003-typedef-caddr.patch packages/glibc/2.23/0004-fix-rpc_parse-format.patch packages/glibc/2.23/0005-explicit-boolean.patch packages/glibc/2.23/0006-nis-bogus-conditional.patch packages/glibc/2.23/0007-regexp-common.patch packages/glibc/2.23/0008-strftime-multiple-stmts.patch packages/glibc/2.23/0009-if_nametoindex-size-check.patch packages/glibc/2.23/0010-utmp-nonstring.patch packages/glibc/2.23/0011-getlogin_r-use-strnlen.patch packages/glibc/2.23/0012-zic.c-use-memcpy.patch packages/glibc/2.23/0013-Fix-cmpli-usage-in-power6-memset.patch packages/glibc/2.23/0014-MIPS-SPARC-fix-wrong-vfork-aliases-in-libpthread.so.patch packages/glibc/2.23/chksum packages/glibc/2.23/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.23'
 /usr/bin/install -c -m 644  packages/glibc/2.24/0000-sh-fix-gcc6.patch packages/glibc/2.24/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.24/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.24/0003-typedef-caddr.patch packages/glibc/2.24/0004-fix-rpc_parse-format.patch packages/glibc/2.24/0005-explicit-boolean.patch packages/glibc/2.24/0006-nis-bogus-conditional.patch packages/glibc/2.24/0007-regexp-common.patch packages/glibc/2.24/0008-strftime-multiple-stmts.patch packages/glibc/2.24/0009-if_nametoindex-size-check.patch packages/glibc/2.24/0010-utmp-nonstring.patch packages/glibc/2.24/0011-getlogin_r-use-strnlen.patch packages/glibc/2.24/0012-zic.c-use-memcpy.patch packages/glibc/2.24/0013-fix-cmpli-usage-in-power6-memset.patch packages/glibc/2.24/chksum packages/glibc/2.24/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.24'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.25'
 /usr/bin/install -c -m 644  packages/glibc/2.25/0000-sh-fix-gcc6.patch packages/glibc/2.25/0001-Fix-build-with-enable-static-nss.patch packages/glibc/2.25/0002-Fix-combreloc-test-BSD-grep.patch packages/glibc/2.25/0003-typedef-caddr.patch packages/glibc/2.25/0004-sh4-trap-divdi3.patch packages/glibc/2.25/0005-sparc-extra-plt-call.patch packages/glibc/2.25/0006-regexp-common.patch packages/glibc/2.25/0007-strftime-multiple-stmts.patch packages/glibc/2.25/0008-if_nametoindex-size-check.patch packages/glibc/2.25/0009-utmp-nonstring.patch packages/glibc/2.25/0010-getlogin_r-use-strnlen.patch packages/glibc/2.25/0011-zic.c-use-memcpy.patch packages/glibc/2.25/chksum packages/glibc/2.25/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.25'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.26'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.26'
 /usr/bin/install -c -m 644  packages/glibc/2.26/0000-typedef-caddr.patch packages/glibc/2.26/0001-aarch64-rewrite-elf_machine_load_address.patch packages/glibc/2.26/0002-if_nametoindex-size-check.patch packages/glibc/2.26/0003-utmp-nonstring.patch packages/glibc/2.26/0004-getlogin_r-use-strnlen.patch packages/glibc/2.26/0005-zic.c-use-memcpy.patch packages/glibc/2.26/0006-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.26/chksum packages/glibc/2.26/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.26'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.27'
 /usr/bin/install -c -m 644  packages/glibc/2.27/0000-typedef-caddr.patch packages/glibc/2.27/0001-Fix-build-with-GCC-10-when-long-double-double.patch packages/glibc/2.27/0002-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.27/chksum packages/glibc/2.27/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.27'
 /usr/bin/install -c -m 644  samples/i686-ubuntu14.04-linux-gnu/crosstool.config samples/i686-ubuntu14.04-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/i686-ubuntu14.04-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/picolibc'
 /usr/bin/install -c -m 644  packages/picolibc/package.desc '/usr/local/share/crosstool-ng/packages/picolibc'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.28'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.28'
 /usr/bin/install -c -m 644  packages/glibc/2.28/0000-typedef-caddr.patch packages/glibc/2.28/0001-x86-Assume-enable-cet-if-GCC-defaults-to-CET-BZ-2522.patch packages/glibc/2.28/0002-Fix-build-with-GCC-10-when-long-double-double.patch packages/glibc/2.28/0003-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.28/chksum packages/glibc/2.28/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.28'
 /usr/bin/install -c -m 644  samples/x86_64-multilib-linux-gnu/crosstool.config samples/x86_64-multilib-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.29'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.29'
 /usr/bin/install -c -m 644  packages/glibc/2.29/0000-typedef-caddr.patch packages/glibc/2.29/0001-Add-ARC-architecture.patch packages/glibc/2.29/0002-x86-Assume-enable-cet-if-GCC-defaults-to-CET-BZ-2522.patch packages/glibc/2.29/0003-Fix-build-with-GCC-10-when-long-double-double.patch packages/glibc/2.29/0004-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.29/chksum packages/glibc/2.29/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.29'
 /usr/bin/install -c -m 644  packages/binutils-oracle/git-889339ab/0000-Fix-a-missing-include-of-string.patch '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-889339ab'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/8.5.0'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/8.5.0'
 /usr/bin/install -c -m 644  packages/gcc/8.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/8.5.0/0001-uclibc-conf.patch packages/gcc/8.5.0/0002-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/8.5.0/0003-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/8.5.0/0004-arm-softfloat-libgcc.patch packages/gcc/8.5.0/0005-fix-m68k-uclinux.patch packages/gcc/8.5.0/0006-libgfortran-missing-include.patch packages/gcc/8.5.0/0007-nios2-bad-multilib-default.patch packages/gcc/8.5.0/0008-libgcc-disable-split-stack-nothreads.patch packages/gcc/8.5.0/0009-bionic-ndk.patch packages/gcc/8.5.0/0010-crystax.patch packages/gcc/8.5.0/0011-crystax.patch packages/gcc/8.5.0/0012-crystax.patch packages/gcc/8.5.0/0013-crystax.patch packages/gcc/8.5.0/0014-crystax.patch packages/gcc/8.5.0/0015-crystax.patch packages/gcc/8.5.0/0016-crystax.patch packages/gcc/8.5.0/0017-crystax.patch packages/gcc/8.5.0/0018-ARC-Add-multilib-support-for-linux-targets.patch packages/gcc/8.5.0/0019-isl-0.20.patch packages/gcc/8.5.0/0020-ARM-fix-cmse.patch packages/gcc/8.5.0/0021-arm-Make-arm_cmse.h-C99-compatible.patch packages/gcc/8.5.0/0022-ARC-Update-fma-expansions.patch packages/gcc/8.5.0/0023-darwin-aarch64-config.patch packages/gcc/8.5.0/0024-darwin-aarch64-self-host-driver.patch packages/gcc/8.5.0/0025-darwin-align-pch_address_space-to-16k.patch packages/gcc/8.5.0/chksum packages/gcc/8.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/8.5.0'
 /usr/bin/install -c -m 644  packages/gnuprumcu/package.desc '/usr/local/share/crosstool-ng/packages/gnuprumcu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/config/comp_tools'
 /usr/bin/install -c -m 644  config/comp_tools/autoconf.in config/comp_tools/automake.in config/comp_tools/bison.in config/comp_tools/dtc.in config/comp_tools/libtool.in config/comp_tools/m4.in config/comp_tools/make.in '/usr/local/share/crosstool-ng/config/comp_tools'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/12.2.0'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/12.2.0'
 /usr/bin/install -c -m 644  packages/gcc/12.2.0/0000-libtool-leave-framework-alone.patch packages/gcc/12.2.0/0001-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/12.2.0/0002-arm-softfloat-libgcc.patch packages/gcc/12.2.0/0003-libgcc-disable-split-stack-nothreads.patch packages/gcc/12.2.0/0004-Remove-use-of-include_next-from-c-headers.patch packages/gcc/12.2.0/0005-Allow-default-libc-to-be-specified-to-configure.patch packages/gcc/12.2.0/0006-driver-Extend-getenv-function-to-allow-default-value.patch packages/gcc/12.2.0/0007-Add-newlib-and-picolibc-as-default-C-library-choices.patch packages/gcc/12.2.0/0008-Support-picolibc-targets.patch packages/gcc/12.2.0/0009-gcc-Allow-g-to-work-differently-from-gcc.patch packages/gcc/12.2.0/chksum packages/gcc/12.2.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/12.2.0'
 /usr/bin/install -c -m 644  packages/isl/0.16.1/chksum packages/isl/0.16.1/version.desc '/usr/local/share/crosstool-ng/packages/isl/0.16.1'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/override'
 /usr/bin/install -c -m 644  scripts/override/install '/usr/local/share/crosstool-ng/scripts/override'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-linaro/7.4-2019.02'
---
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/make'
 /usr/bin/install -c -m 644  packages/make/package.desc '/usr/local/share/crosstool-ng/packages/make'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc'
 /usr/bin/install -c -m 644  packages/gcc/package.desc '/usr/local/share/crosstool-ng/packages/gcc'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-43eccdca'
 /usr/bin/install -c -m 644  packages/binutils-oracle/git-43eccdca/0000-Fix-a-missing-include-of-string.patch packages/binutils-oracle/git-43eccdca/0001-revert-rpm-dd-changes.patch packages/binutils-oracle/git-43eccdca/0002-fix_to_patch_92.patch '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-43eccdca'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/bpf-unknown-none'
 /usr/bin/install -c -m 644  samples/bpf-unknown-none/crosstool.config samples/bpf-unknown-none/reported.by '/usr/local/share/crosstool-ng/samples/bpf-unknown-none'
 /usr/bin/install -c -m 644  packages/autoconf/2.65/chksum packages/autoconf/2.65/version.desc '/usr/local/share/crosstool-ng/packages/autoconf/2.65'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/autoconf/2.69'
 /usr/bin/install -c -m 644  packages/autoconf/2.69/chksum packages/autoconf/2.69/version.desc '/usr/local/share/crosstool-ng/packages/autoconf/2.69'
 /usr/bin/install -c -m 644  packages/autoconf/2.69/chksum packages/autoconf/2.69/version.desc '/usr/local/share/crosstool-ng/packages/autoconf/2.69'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/zstd/1.5.2'
 /usr/bin/install -c -m 644  packages/zstd/1.5.2/chksum packages/zstd/1.5.2/version.desc '/usr/local/share/crosstool-ng/packages/zstd/1.5.2'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/zstd/1.5.5'
 /usr/bin/install -c -m 644  packages/zstd/1.5.5/chksum packages/zstd/1.5.5/version.desc '/usr/local/share/crosstool-ng/packages/zstd/1.5.5'
 /usr/bin/install -c -m 644  packages/linux/3.18.139/chksum packages/linux/3.18.139/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.18.139'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/scripts/build/debug'
 /usr/bin/install -c -m 644  scripts/build/debug/200-duma.sh scripts/build/debug/300-gdb.sh scripts/build/debug/400-ltrace.sh scripts/build/debug/500-strace.sh scripts/build/debug/duma.in scripts/build/debug/gdbinit.in '/usr/local/share/crosstool-ng/scripts/build/debug'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.0.19'
---
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/4.1.49'
 /usr/bin/install -c -m 644  packages/linux/4.1.49/chksum packages/linux/4.1.49/version.desc '/usr/local/share/crosstool-ng/packages/linux/4.1.49'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/bison/3.5'
 /usr/bin/install -c -m 644  packages/bison/3.5/chksum packages/bison/3.5/version.desc '/usr/local/share/crosstool-ng/packages/bison/3.5'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/12.2.0/experimental'
 /usr/bin/install -c -m 644  packages/gcc/12.2.0/experimental/0001-picolibc-Add-custom-spec-file-fragments-for-using-pi.patch '/usr/local/share/crosstool-ng/packages/gcc/12.2.0/experimental'
 /usr/bin/install -c -m 644  packages/linux/3.2.101/chksum packages/linux/3.2.101/version.desc '/usr/local/share/crosstool-ng/packages/linux/3.2.101'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.5.19'
 /usr/bin/install -c -m 644  packages/linux/5.5.19/chksum packages/linux/5.5.19/version.desc '/usr/local/share/crosstool-ng/packages/linux/5.5.19'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/5.16'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace/5.16'
 /usr/bin/install -c -m 644  packages/strace/5.16/chksum packages/strace/5.16/version.desc '/usr/local/share/crosstool-ng/packages/strace/5.16'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc-e500v2-linux-gnuspe'
 /usr/bin/install -c -m 644  samples/powerpc-e500v2-linux-gnuspe/crosstool.config samples/powerpc-e500v2-linux-gnuspe/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-e500v2-linux-gnuspe'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/6.1.25'
 /usr/bin/install -c -m 644  packages/linux/6.1.25/chksum packages/linux/6.1.25/version.desc '/usr/local/share/crosstool-ng/packages/linux/6.1.25'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/strace'
 /usr/bin/install -c -m 644  packages/strace/package.desc '/usr/local/share/crosstool-ng/packages/strace'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc-oracle/git-ea029c9b'
 /usr/bin/install -c -m 644  packages/glibc-oracle/git-ea029c9b/0000-backport-missing-header-file.patch '/usr/local/share/crosstool-ng/packages/glibc-oracle/git-ea029c9b'
 /usr/bin/install -c -m 644  samples/xtensa-fsf-elf/crosstool.config samples/xtensa-fsf-elf/reported.by '/usr/local/share/crosstool-ng/samples/xtensa-fsf-elf'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.2.17'
 /usr/bin/install -c -m 644  packages/linux/5.2.17/chksum packages/linux/5.2.17/version.desc '/usr/local/share/crosstool-ng/packages/linux/5.2.17'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/avr-libc/2.1.0'
---
 /usr/bin/install -c -m 644  packages/picolibc/1.8.1/chksum packages/picolibc/1.8.1/version.desc '/usr/local/share/crosstool-ng/packages/picolibc/1.8.1'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/expat'
 /usr/bin/install -c -m 644  packages/expat/package.desc '/usr/local/share/crosstool-ng/packages/expat'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.30'
 /usr/bin/install -c -m 644  packages/glibc/2.30/0000-typedef-caddr.patch packages/glibc/2.30/0001-Add-ARC-architecture.patch packages/glibc/2.30/0002-x86-Assume-enable-cet-if-GCC-defaults-to-CET-BZ-2522.patch packages/glibc/2.30/0003-Fix-build-with-GCC-10-when-long-double-double.patch packages/glibc/2.30/0004-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.30/chksum packages/glibc/2.30/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.30'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.31'
 /usr/bin/install -c -m 644  packages/glibc/2.31/0000-typedef-caddr.patch packages/glibc/2.31/0001-Add-ARC-architecture.patch packages/glibc/2.31/0002-Fix-build-with-GCC-10-when-long-double-double.patch packages/glibc/2.31/0003-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.31/chksum packages/glibc/2.31/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.31'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.32'
 /usr/bin/install -c -m 644  packages/glibc/2.32/0000-typedef-caddr.patch packages/glibc/2.32/0001-Set-version.h-RELEASE-to-stable-Bug-26700.patch packages/glibc/2.32/0002-Add-ARC700-support.patch packages/glibc/2.32/0003-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.32/chksum packages/glibc/2.32/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.32'
 /usr/bin/install -c -m 644  samples/x86_64-unknown-linux-uclibc/crosstool.config samples/x86_64-unknown-linux-uclibc/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-unknown-linux-uclibc'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.33'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.33'
 /usr/bin/install -c -m 644  packages/glibc/2.33/0000-typedef-caddr.patch packages/glibc/2.33/0001-Set-version.h-RELEASE-to-stable-Bug-26700.patch packages/glibc/2.33/0002-Add-ARC700-support.patch packages/glibc/2.33/0003-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.33/chksum packages/glibc/2.33/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.33'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.34'
 /usr/bin/install -c -m 644  packages/glibc/2.34/0000-typedef-caddr.patch packages/glibc/2.34/0001-Add-ARC700-support.patch packages/glibc/2.34/0002-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.34/chksum packages/glibc/2.34/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.34'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.35'
 /usr/bin/install -c -m 644  packages/glibc/2.35/0000-typedef-caddr.patch packages/glibc/2.35/0001-Add-ARC700-support.patch packages/glibc/2.35/0002-linux-Fix-missing-__convert_scm_timestamps-BZ-28860.patch packages/glibc/2.35/0003-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.35/chksum packages/glibc/2.35/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.35'
 /usr/bin/install -c -m 644  packages/gnuprumcu/0.6.0/chksum packages/gnuprumcu/0.6.0/version.desc '/usr/local/share/crosstool-ng/packages/gnuprumcu/0.6.0'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.36'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc/2.36'
 /usr/bin/install -c -m 644  packages/glibc/2.36/0000-typedef-caddr.patch packages/glibc/2.36/0001-Add-ARC700-support.patch packages/glibc/2.36/0002-Mimic-kernel-defition-for-BLOCK_SIZE.patch packages/glibc/2.36/0003-Use-compile_c_snippet-to-check-linux_mount.h.patch packages/glibc/2.36/0004-Fix_sys_mount.h_usage_with_kernel_headers.patch packages/glibc/2.36/0005-Fix-enum-fsconfig_command-detection-in-sys_mount.h.patch packages/glibc/2.36/0006-Makerules-fix-MAKEFLAGS-assignment-for-upcoming-make.patch packages/glibc/2.36/chksum packages/glibc/2.36/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.36'
 /usr/bin/install -c -m 644  packages/glibc/2.37/0001-Add-ARC700-support.patch packages/glibc/2.37/chksum packages/glibc/2.37/version.desc '/usr/local/share/crosstool-ng/packages/glibc/2.37'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux/5.8.18'
 /usr/bin/install -c -m 644  packages/linux/5.8.18/chksum packages/linux/5.8.18/version.desc '/usr/local/share/crosstool-ng/packages/linux/5.8.18'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/armv6-unknown-linux-gnueabi'
---
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/glibc-oracle/2.17'
 /usr/bin/install -c -m 644  packages/glibc-oracle/2.17/version.desc '/usr/local/share/crosstool-ng/packages/glibc-oracle/2.17'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/config/debug'
 /usr/bin/install -c -m 644  config/debug/duma.in config/debug/gdb.in config/debug/gdb.in.cross config/debug/gdb.in.native config/debug/ltrace.in config/debug/strace.in '/usr/local/share/crosstool-ng/config/debug'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-ol8u7-linux-gnu'
 /usr/bin/install -c -m 644  samples/x86_64-ol8u7-linux-gnu/crosstool.config samples/x86_64-ol8u7-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-ol8u7-linux-gnu'
 /usr/bin/install -c -m 644  samples/x86_64-multilib-linux-uclibc,powerpc-unknown-elf/crosstool.config samples/x86_64-multilib-linux-uclibc,powerpc-unknown-elf/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc,powerpc-unknown-elf'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-ubuntu14.04-linux-gnu'
 /usr/bin/install -c -m 644  samples/x86_64-ubuntu14.04-linux-gnu/crosstool.config samples/x86_64-ubuntu14.04-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-ubuntu14.04-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-multilib-linux-uclibcgnueabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-multilib-linux-uclibcgnueabi'
 /usr/bin/install -c -m 644  samples/arm-multilib-linux-uclibcgnueabi/crosstool.config samples/arm-multilib-linux-uclibcgnueabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-multilib-linux-uclibcgnueabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/powerpc-unknown-linux-gnu'
 /usr/bin/install -c -m 644  samples/powerpc-unknown-linux-gnu/crosstool.config samples/powerpc-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/powerpc-unknown-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-linaro/5.5-2017.10'
 /usr/bin/install -c -m 644  packages/gcc-linaro/5.5-2017.10/0000-libtool-leave-framework-alone.patch packages/gcc-linaro/5.5-2017.10/0001-uclibc-conf.patch packages/gcc-linaro/5.5-2017.10/0002-msp430-string-literals.patch packages/gcc-linaro/5.5-2017.10/0003-xtensa-implement-trap-pattern.patch packages/gcc-linaro/5.5-2017.10/0004-gcc-config.gcc-fix-typo-for-powerpc-e6500-cpu_is_64b.patch packages/gcc-linaro/5.5-2017.10/0005-missing-execinfo_h.patch packages/gcc-linaro/5.5-2017.10/0006-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc-linaro/5.5-2017.10/0007-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc-linaro/5.5-2017.10/0008-arm-softfloat-libgcc.patch packages/gcc-linaro/5.5-2017.10/0009-arm_unbreak_armv4t.patch packages/gcc-linaro/5.5-2017.10/0010-microblaze-enable-dwarf-eh-support.patch packages/gcc-linaro/5.5-2017.10/0011-libstdcxx-uclibc-c99.patch packages/gcc-linaro/5.5-2017.10/0012-cilk-wchar.patch packages/gcc-linaro/5.5-2017.10/0013-xtensa-add-mauto-litpools-option.patch packages/gcc-linaro/5.5-2017.10/0014-xtensa-reimplement-register-spilling.patch packages/gcc-linaro/5.5-2017.10/0015-xtensa-add-uclinux-support.patch packages/gcc-linaro/5.5-2017.10/0016-fix-m68k-compile.patch packages/gcc-linaro/5.5-2017.10/0017-fix-m68k-uclinux.patch packages/gcc-linaro/5.5-2017.10/0018-microblaze-uclibc.patch packages/gcc-linaro/5.5-2017.10/0019-unwind-fix-for-musl.patch packages/gcc-linaro/5.5-2017.10/0020-nios2-bad-multilib-default.patch packages/gcc-linaro/5.5-2017.10/0021-libgcc-disable-split-stack-nothreads.patch packages/gcc-linaro/5.5-2017.10/0022-uclinux-enable-threads.patch packages/gcc-linaro/5.5-2017.10/chksum packages/gcc-linaro/5.5-2017.10/version.desc '/usr/local/share/crosstool-ng/packages/gcc-linaro/5.5-2017.10'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/linux'
 /usr/bin/install -c -m 644  packages/linux/package.desc '/usr/local/share/crosstool-ng/packages/linux'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc-oracle/8.5.0-10.0.2'
 /usr/bin/install -c -m 644  packages/gcc-oracle/8.5.0-10.0.2/version.desc '/usr/local/share/crosstool-ng/packages/gcc-oracle/8.5.0-10.0.2'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-ol8u6-linux-gnu'
 /usr/bin/install -c -m 644  samples/i686-ol8u6-linux-gnu/crosstool.config samples/i686-ol8u6-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/i686-ol8u6-linux-gnu'
 /usr/bin/install -c -m 644  packages/isl/0.20/chksum packages/isl/0.20/version.desc '/usr/local/share/crosstool-ng/packages/isl/0.20'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/isl/0.21'
 /usr/bin/install -c -m 644  packages/isl/0.21/chksum packages/isl/0.21/version.desc '/usr/local/share/crosstool-ng/packages/isl/0.21'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/riscv32-unknown-elf'
---
 /usr/bin/install -c -m 644  samples/loongarch64-unknown-linux-gnu/crosstool.config samples/loongarch64-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/loongarch64-unknown-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/bison/3.3.2'
 /usr/bin/install -c -m 644  packages/bison/3.3.2/chksum packages/bison/3.3.2/version.desc '/usr/local/share/crosstool-ng/packages/bison/3.3.2'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gcc/7.5.0'
 /usr/bin/install -c -m 644  packages/gcc/7.5.0/0000-libtool-leave-framework-alone.patch packages/gcc/7.5.0/0001-uclibc-conf.patch packages/gcc/7.5.0/0002-gcc-plugin-Win-Dont-need-undefined-extern-var-refs-nor-fpic.patch packages/gcc/7.5.0/0003-gcc-plugin-POSIX-include-sys-select-h.patch packages/gcc/7.5.0/0004-arm-softfloat-libgcc.patch packages/gcc/7.5.0/0005-cilk-wchar.patch packages/gcc/7.5.0/0006-fix-m68k-uclinux.patch packages/gcc/7.5.0/0007-libgfortran-missing-include.patch packages/gcc/7.5.0/0008-nios2-bad-multilib-default.patch packages/gcc/7.5.0/0009-libgcc-disable-split-stack-nothreads.patch packages/gcc/7.5.0/0010-bionic-ndk.patch packages/gcc/7.5.0/0011-bionic-errno.patch packages/gcc/7.5.0/0012-crystax.patch packages/gcc/7.5.0/0013-crystax.patch packages/gcc/7.5.0/0014-crystax.patch packages/gcc/7.5.0/0015-crystax.patch packages/gcc/7.5.0/0016-crystax.patch packages/gcc/7.5.0/0017-crystax.patch packages/gcc/7.5.0/0018-crystax.patch packages/gcc/7.5.0/0019-crystax.patch packages/gcc/7.5.0/0020-isl-0.20.patch packages/gcc/7.5.0/0021-darwin-aarch64-config.patch packages/gcc/7.5.0/0022-darwin-aarch64-self-host-driver.patch packages/gcc/7.5.0/0023-darwin-align-pch_address_space-to-16k.patch packages/gcc/7.5.0/chksum packages/gcc/7.5.0/version.desc '/usr/local/share/crosstool-ng/packages/gcc/7.5.0'
 /usr/bin/install -c -m 644  samples/aarch64-ol7u9-linux-gnu/crosstool.config samples/aarch64-ol7u9-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/aarch64-ol7u9-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc,moxie-unknown-moxiebox'
 /usr/bin/install -c -m 644  samples/x86_64-multilib-linux-uclibc,moxie-unknown-moxiebox/crosstool.config samples/x86_64-multilib-linux-uclibc,moxie-unknown-moxiebox/reported.by '/usr/local/share/crosstool-ng/samples/x86_64-multilib-linux-uclibc,moxie-unknown-moxiebox'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/gdb'
---
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/alphaev56-unknown-linux-gnu'
 /usr/bin/install -c -m 644  samples/alphaev56-unknown-linux-gnu/crosstool.config samples/alphaev56-unknown-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/alphaev56-unknown-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/arm-nano-eabi'
 /usr/bin/install -c -m 644  samples/arm-nano-eabi/crosstool.config samples/arm-nano-eabi/reported.by '/usr/local/share/crosstool-ng/samples/arm-nano-eabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/riscv32-picolibc-elf'
 /usr/bin/install -c -m 644  samples/riscv32-picolibc-elf/crosstool.config samples/riscv32-picolibc-elf/reported.by '/usr/local/share/crosstool-ng/samples/riscv32-picolibc-elf'
 /usr/bin/install -c -m 644  samples/mips-unknown-elf/crosstool.config samples/mips-unknown-elf/reported.by '/usr/local/share/crosstool-ng/samples/mips-unknown-elf'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/samples/i686-centos6-linux-gnu'
 /usr/bin/install -c -m 644  samples/i686-centos6-linux-gnu/crosstool.config samples/i686-centos6-linux-gnu/reported.by '/usr/local/share/crosstool-ng/samples/i686-centos6-linux-gnu'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mpfr/4.0.2'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/mpfr/4.0.2'
 /usr/bin/install -c -m 644  packages/mpfr/4.0.2/chksum packages/mpfr/4.0.2/version.desc '/usr/local/share/crosstool-ng/packages/mpfr/4.0.2'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/picolibc/1.5.1'
 /usr/bin/install -c -m 644  packages/picolibc/1.5.1/0000-libc-Remove-include-sys-select.h-from-sys-types.h.patch packages/picolibc/1.5.1/0001-tinystdio-Fix-snprintf-buf-0-.-to-not-smash-buffer.patch packages/picolibc/1.5.1/0002-libc-Expose-wchar-stdio-prototypes-even-for-TINY_STD.patch packages/picolibc/1.5.1/chksum packages/picolibc/1.5.1/version.desc '/usr/local/share/crosstool-ng/packages/picolibc/1.5.1'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-461776a5'
 /usr/bin/install -c -m 644  packages/binutils-oracle/git-461776a5/0000-Fix-a-missing-include-of-string.patch '/usr/local/share/crosstool-ng/packages/binutils-oracle/git-461776a5'
 /usr/bin/install -c -m 644  samples/armeb-unknown-eabi/crosstool.config samples/armeb-unknown-eabi/reported.by '/usr/local/share/crosstool-ng/samples/armeb-unknown-eabi'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/config/global'
 /usr/bin/install -c -m 644  config/global/build-behave.in config/global/ct-behave.in config/global/download.in config/global/extract.in config/global/logging.in config/global/paths.in '/usr/local/share/crosstool-ng/config/global'
 /usr/bin/mkdir -p '/usr/local/share/crosstool-ng/packages/uClibc-ng/1.0.30'
---
Step 14/18 : ENV PATH=$PATH:/x-tools/loongarch64-unknown-linux-gnu/bin
 ---> Running in ab848cd83522
Removing intermediate container ab848cd83522
 ---> 37c0c7cc8885
Step 15/18 : ENV CC_loongarch64_unknown_linux_gnu=loongarch64-unknown-linux-gnu-gcc     AR_loongarch64_unknown_linux_gnu=loongarch64-unknown-linux-gnu-ar     CXX_loongarch64_unknown_linux_gnu=loongarch64-unknown-linux-gnu-g++
Removing intermediate container 53bc1aee374c
 ---> 068a06c84ffe
Step 16/18 : ENV HOSTS=loongarch64-unknown-linux-gnu
 ---> Running in d0562ab6abb5
---
Successfully tagged rust-ci:latest
Built container sha256:65fedd6e6d91cd673653d9f9f0cf5ae46d958a7696fafa21cce29779899bc878
Uploading finished image to https://ci-caches.rust-lang.org/docker/b5e0966885bd7129cb8f30054083e1d5b676f392552aab3e494bfe537f1ca2710699e53459c1d441253396049caefe3022ef881b6fb6052ac965963c6c12c9b3

<botocore.awsrequest.AWSRequest object at 0x7fbc423239d0>
gzip: stdout: Broken pipe
xargs: docker: terminated by signal 13
[CI_JOB_NAME=dist-loongarch64-linux]
[CI_JOB_NAME=dist-loongarch64-linux]
---
CMAKE_loongarch64-unknown-linux-gnu = None
CMAKE_loongarch64_unknown_linux_gnu = None
TARGET_CMAKE = None
CMAKE = None
running: cd "/checkout/obj/build/loongarch64-unknown-linux-gnu/llvm/build" && CMAKE_PREFIX_PATH="" DESTDIR="" "cmake" "/checkout/src/llvm-project/llvm" "-G" "Ninja" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_UNREACHABLE_OPTIMIZE=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;LoongArch;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=8" "-DLLVM_TARGET_ARCH=loongarch64" "-DLLVM_DEFAULT_TARGET_TRIPLE=loongarch64-unknown-linux-gnu" "-DLLVM_ENABLE_WARNINGS=OFF" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZSTD=OFF" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_TABLEGEN=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-tblgen" "-DLLVM_NM=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-nm" "-DLLVM_CONFIG_PATH=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-config" "-DLLVM_VERSION_SUFFIX=-rust-1.71.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_CROSSCOMPILING=True" "-DCMAKE_SYSTEM_NAME=Linux" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=loongarch64-unknown-linux-gnu-gcc" "-DCMAKE_CXX_COMPILER=loongarch64-unknown-linux-gnu-g++" "-DCMAKE_ASM_COMPILER=loongarch64-unknown-linux-gnu-gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/loongarch64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 12.2.0
-- The ASM compiler identification is GNU
-- Found assembler: /x-tools/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc
-- Detecting C compiler ABI info
---
CMAKE_loongarch64-unknown-linux-gnu = None
CMAKE_loongarch64_unknown_linux_gnu = None
TARGET_CMAKE = None
CMAKE = None
running: cd "/checkout/obj/build/loongarch64-unknown-linux-gnu/lld/build" && CMAKE_PREFIX_PATH="" DESTDIR="" "cmake" "/checkout/src/llvm-project/lld" "-G" "Ninja" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_CROSSCOMPILING=True" "-DCMAKE_SYSTEM_NAME=Linux" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=loongarch64-unknown-linux-gnu-gcc" "-DCMAKE_CXX_COMPILER=loongarch64-unknown-linux-gnu-g++" "-DCMAKE_ASM_COMPILER=loongarch64-unknown-linux-gnu-gcc" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-Bsymbolic -static-libstdc++" "-DLLVM_CMAKE_DIR=/checkout/obj/build/loongarch64-unknown-linux-gnu/llvm/lib/cmake/llvm" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_TABLEGEN_EXE=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/llvm-tblgen" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/loongarch64-unknown-linux-gnu/lld" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is GNU 12.2.0
-- Detecting C compiler ABI info
-- Detecting C compiler ABI info - done
-- Check for working C compiler: /x-tools/loongarch64-unknown-linux-gnu/bin/loongarch64-unknown-linux-gnu-gcc - skipped
---
  IMAGE: dist-loongarch64-linux
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
##[endgroup]
cp: cannot stat 'obj/build/metrics.json': No such file or directory
##[error]Process completed with exit code 1.
