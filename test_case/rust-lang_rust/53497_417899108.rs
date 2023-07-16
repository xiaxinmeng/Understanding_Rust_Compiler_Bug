plain
The following additional packages will be installed:
  libintl-perl libtext-unidecode-perl libxml-libxml-perl
  libxml-namespacesupport-perl libxml-sax-base-perl libxml-sax-perl
Suggested packages:
  libintl-xs-perl texlive-base texlive-latex-base texlive-generic-recommended
Recommended packages:
  libxml-sax-expat-perl
The following NEW packages will be installed:
  libintl-perl libtext-unidecode-perl libxml-libxml-perl
  libintl-perl libtext-unidecode-perl libxml-libxml-perl
  libxml-namespacesupport-perl libxml-sax-base-perl libxml-sax-perl texinfo
0 upgraded, 7 newly installed, 0 to remove and 178 not upgraded.
Need to get 2,315 kB of archives.
After this operation, 9,657 kB of additional disk space will be used.
Get:1 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libintl-perl all 1.23-1build1 [1,204 kB]
Get:3 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-namespacesupport-perl all 1.11-1 [13.2 kB]
Get:4 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-sax-base-perl all 1.07-1 [21.5 kB]
Get:5 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-sax-perl all 0.99+dfsg-2ubuntu1 [64.6 kB]
Get:6 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty-updates/main amd64 libxml-libxml-perl amd64 2.0108+dfsg-1ubuntu0.2 [336 kB]
---
travis_time:end:170c21a6:start=1535851633708193315,finish=1535851635582667384,duration=1874474069
travis_fold:end:install.1
travis_fold:start:install.2
travis_time:start:04ab1238
$ wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
--2018-09-02 01:27:15--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
HTTP request sent, awaiting response... 200 OK
Length: 37338817 (36M) [application/x-gzip]
Saving to: ‘gdb-8.1.1.tar.gz’
 0% [                                       ] 0           --.-K/s              
 2% [                                       ] 799,190     3.80MB/s             
37% [=============>                         ] 13,827,158  32.4MB/s             
80% [==============================>        ] 30,017,622  47.2MB/s             
100%[======================================>] 37,338,817  51.0MB/s   in 0.7s   
2018-09-02 01:27:16 (51.0 MB/s) - ‘gdb-8.1.1.tar.gz’ saved [37338817/37338817]
travis_fold:end:install.2
travis_fold:start:install.3
travis_time:start:2c6aec36
travis_time:start:2c6aec36
$ tar -xzvf gdb-8.1.1.tar.gz > /dev/null
travis_fold:end:install.3
travis_fold:start:install.4
travis_time:start:1ed0a4ec
travis_time:start:1ed0a4ec
$ cd gdb-8.1.1 && ./configure > /dev/null && make > /dev/null && sudo make install > /dev/null
./simple-object-elf.c: In function ‘simple_object_elf_copy_lto_debug_sections’:
./simple-object-elf.c:1161:3: warning: ISO C90 forbids mixed declarations and code [-Wpedantic]
   int changed;
   ^
util.c: In function ‘_rl_tropen’:
util.c:520:3: warning: format ‘%ld’ expects argument of type ‘long int’, but argument 3 has type ‘__pid_t’ [-Wformat=]
   sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
   ^
histfile.c: In function ‘history_truncate_file’:
histfile.c:410:13: warning: ignoring return value of ‘write’, declared with attribute warn_unused_result [-Wunused-result]
       write (file, bp, chars_read - (bp - buffer));
             ^
configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
configure: WARNING: linux/perf_event.h missing or too old; some features may be unavailable.
configure: WARNING: libipt is missing or unusable; some features may be unavailable.
configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
Creating observer.htmp
Creating observer.itmp
cli/cli-cmds.c: In function ‘void complete_command(const char*, int)’:
cli/cli-cmds.c:304:48: warning: ‘word’ may be used uninitialized in this function [-Wmaybe-uninitialized]
         get_max_completions_reached_message ());
                                                ^
cli/cli-cmds.c:277:71: warning: ‘tracker’ may be used uninitialized in this function [-Wmaybe-uninitialized]
     = tracker->build_completion_result (word, word - arg, strlen (arg));
                                                                       ^
breakpoint.c: In function ‘void check_status_watchpoint(bpstat)’:
breakpoint.c:5056:20: warning: ‘e’ may be used uninitialized in this function [-Wmaybe-uninitialized]
    wp_check_result e;
                    ^
.././../../libiberty/simple-object-elf.c: In function ‘simple_object_elf_copy_lto_debug_sections’:
.././../../libiberty/simple-object-elf.c:1161:3: warning: ISO C90 forbids mixed declarations and code [-Wpedantic]
   int changed;
   ^
This is not dpkg install-info anymore, but GNU install-info
See the man page for ginstall-info for command line arguments
./gdb.texinfo:2498: warning: `.' or `,' must follow @xref
./gdb.texinfo:2517: warning: `.' or `,' must follow @xref
This is not dpkg install-info anymore, but GNU install-info
See the man page for ginstall-info for command line arguments
This is not dpkg install-info anymore, but GNU install-info
See the man page for ginstall-info for command line arguments
This is not dpkg install-info anymore, but GNU install-info
See the man page for ginstall-info for command line arguments
travis_fold:end:install.4
travis_fold:start:install.5
travis_time:start:36e10d1f
$ cd ..
$ cd ..
travis_time:end:36e10d1f:start=1535852198611099767,finish=1535852198620981225,duration=9881458
travis_fold:end:install.5
travis_fold:start:install.6
travis_time:start:099da61c
$ gdb --version
GNU gdb (GDB) 8.1.1
Copyright (C) 2018 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<http://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word".
travis_fold:end:install.6
travis_fold:start:before_script.1
travis_time:start:150b12c0
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
412504 ./.git/objects
412496 ./.git/objects/pack
378644 ./gdb-8.1.1/gdb
96616 ./src
81340 ./gdb-8.1.1/bfd
63928 ./src/test
60360 ./gdb-8.1.1/sim
43276 ./gdb-8.1.1/opcodes
38124 ./src/test/ui
26932 ./gdb-8.1.1/gdb/gdbserver
23944 ./gdb-8.1.1/gdb/testsuite
23456 ./gdb-8.1.1/sim/testsuite
22568 ./gdb-8.1.1/sim/testsuite/sim
22428 ./gdb-8.1.1/gdb/python
15856 ./gdb-8.1.1/bfd/.libs
15132 ./src/test/run-pass
8980 ./gdb-8.1.1/sim/testsuite/sim/bfin
8468 ./gdb-8.1.1/gdb/mi
7872 ./gdb-8.1.1/gdb/doc
7216 ./gdb-8.1.1/libiberty
7084 ./gdb-8.1.1/gdb/tui
6704 ./src/test/ui/issues
6236 ./gdb-8.1.1/sim/frv
5812 ./gdb-8.1.1/readline
5564 ./gdb-8.1.1/gdb/testsuite/gdb.base
5408 ./gdb-8.1.1/zlib
5396 ./gdb-8.1.1/bfd/po
5136 ./gdb-8.1.1/sim/testsuite/sim/frv
4656 ./gdb-8.1.1/include
4492 ./src/librustc
4428 ./gdb-8.1.1/gdb/gdbserver/build-libiberty-gdbserver
4076 ./gdb-8.1.1/gdb/cli
3652 ./gdb-8.1.1/gdb/compile
3192 ./src/libcore
3180 ./src/test/run-make-fulldeps
3180 ./src/test/run-make-fulldeps
3180 ./gdb-8.1.1/gdb/gnulib
2720 ./gdb-8.1.1/opcodes/.libs
2692 ./gdb-8.1.1/libdecnumber
2564 ./gdb-8.1.1/sim/ppc
2540 ./gdb-8.1.1/cpu
2496 ./gdb-8.1.1/gdb/testsuite/gdb.ada
2460 ./gdb-8.1.1/sim/cris
2444 ./src/librustc_mir
2444 ./src/librustc_mir
2424 ./gdb-8.1.1/gdb/gnulib/import
2404 ./gdb-8.1.1/gdb/build-gnulib
2356 ./gdb-8.1.1/gdb/gdbserver/build-gnulib-gdbserver
2236 ./gdb-8.1.1/zlib/contrib
1980 ./src/test/ui/error-codes
1980 ./src/test/ui/error-codes
1956 ./gdb-8.1.1/sim/sh64
1928 ./gdb-8.1.1/include/opcode
1920 ./gdb-8.1.1/sim/m32r
1876 ./gdb-8.1.1/gdb/gdbserver/build-gnulib-gdbserver/import
1876 ./gdb-8.1.1/gdb/build-gnulib/import
1740 ./gdb-8.1.1/gdb/testsuite/gdb.cp
1724 ./gdb-8.1.1/bfd/.deps
1716 ./gdb-8.1.1/gdb/testsuite/gdb.dwarf2
1680 ./gdb-8.1.1/sim/common
1628 ./gdb-8.1.1/gdb/.deps
1624 ./src/libsyntax
1604 ./gdb-8.1.1/gdb/testsuite/gdb.arch
1600 ./gdb-8.1.1/sim/testsuite/sim/cris
1576 ./gdb-8.1.1/sim/testsuite/sim/sh64
1564 ./gdb-8.1.1/sim/bfin
1540 ./src/etc/installer
1540 ./src/etc/installer
1524 ./gdb-8.1.1/gdb/features
1500 ./gdb-8.1.1/gdb/testsuite/gdb.disasm
1492 ./src/libstd/sys
1476 ./gdb-8.1.1/bfd/doc
1396 ./src/librustc_typeck
1396 ./src/librustc_typeck
1324 ./gdb-8.1.1/sim/mips
1312 ./src/test/rustdoc
1312 ./gdb-8.1.1/opcodes/po
1308 ./src/test/ui/regions
1296 ./gdb-8.1.1/sim/or1k
1180 ./src/librustc_codegen_llvm
1180 ./src/librustc_codegen_llvm
1172 ./gdb-8.1.1/sim/arm
1168 ./src/test/ui/feature-gates
1120 ./gdb-8.1.1/sim/testsuite/sim/mips
1048 ./src/librustdoc/html
1048 ./src/librustdoc/html
1024 ./gdb-8.1.1/sim/mn10300
980 ./src/test/ui/lint
952 ./src/ci
948 ./src/librustc/ty
948 ./src/librustc/ty
948 ./gdb-8.1.1/zlib/contrib/vstudio
940 ./gdb-8.1.1/gdb/testsuite/gdb.mi
932 ./src/ci/docker
932 ./gdb-8.1.1/sim/testsuite/sim/cris/asm
932 ./gdb-8.1.1/sim/aarch64
travis_time:end:150b12c0:start=1535852198697187151,finish=1535852198783274345,duration=86087194
travis_fold:end:before_script.1
travis_fold:start:before_script.2
travis_time:start:025b4646
---
[00:01:51] done.
[00:01:51] Processing triggers for systemd (229-4ubuntu21.4) ...
[00:01:58]  ---> b334687b62a8
[00:01:59] Removing intermediate container b8ab3d459b80
[00:01:59] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:01:59]  ---> Running in a37765d6fbec
[00:01:59] /bin/sh: 1: wget: not found
[00:01:59] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 127
[00:02:00] Sending build context to Docker daemon  500.7kB
[00:02:00] Step 1/7 : FROM ubuntu:16.04
[00:02:00]  ---> 52b10959e8aa
[00:02:00] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:00] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:00]  ---> Using cache
[00:02:00]  ---> b334687b62a8
[00:02:00] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:00]  ---> Running in 2172c7b12358
[00:02:01] /bin/sh: 1: wget: not found
[00:02:01] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 127
[00:02:03] Sending build context to Docker daemon  500.7kB
[00:02:03] Step 1/7 : FROM ubuntu:16.04
[00:02:03]  ---> 52b10959e8aa
[00:02:03] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:03] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:03]  ---> Using cache
[00:02:03]  ---> b334687b62a8
[00:02:03] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:03]  ---> Running in 85f29fa4902c
[00:02:03] /bin/sh: 1: wget: not found
[00:02:03] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 127
[00:02:06] Sending build context to Docker daemon  500.7kB
[00:02:06] Step 1/7 : FROM ubuntu:16.04
[00:02:06]  ---> 52b10959e8aa
[00:02:06] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:06] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:06]  ---> Using cache
[00:02:06]  ---> b334687b62a8
[00:02:06] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:06]  ---> Running in 8156b5b19228
[00:02:07] /bin/sh: 1: wget: not found
[00:02:07] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 127
[00:02:11] Sending build context to Docker daemon  500.7kB
[00:02:11] Step 1/7 : FROM ubuntu:16.04
[00:02:11]  ---> 52b10959e8aa
[00:02:11] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:11] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:02:11]  ---> Using cache
[00:02:11]  ---> b334687b62a8
[00:02:11] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:11]  ---> Running in c0bd193007c9
[00:02:11] /bin/sh: 1: wget: not found
[00:02:12] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 127
travis_time:end:02628e3a:start=1535852198890619633,finish=1535852330944252659,duration=132053633026

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:01be0940
---
travis_time:end:08433634:start=1535852331393536933,finish=1535852331401904008,duration=8367075
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06b0c3d6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1e3a88ce
travis_time:start:1e3a88ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0609854d
$ dmesg | grep -i kill
