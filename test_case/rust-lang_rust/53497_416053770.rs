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
0 upgraded, 7 newly installed, 0 to remove and 173 not upgraded.
Need to get 2,315 kB of archives.
After this operation, 9,657 kB of additional disk space will be used.
Get:1 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libintl-perl all 1.23-1build1 [1,204 kB]
Get:3 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-namespacesupport-perl all 1.11-1 [13.2 kB]
Get:4 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-sax-base-perl all 1.07-1 [21.5 kB]
Get:5 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-sax-perl all 0.99+dfsg-2ubuntu1 [64.6 kB]
Get:6 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty-updates/main amd64 libxml-libxml-perl amd64 2.0108+dfsg-1ubuntu0.2 [336 kB]
---
travis_time:end:16fd6cc9:start=1535283950716654806,finish=1535283952255816922,duration=1539162116
travis_fold:end:install.1
travis_fold:start:install.2
travis_time:start:00e79825
$ wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
--2018-08-26 11:45:52--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
HTTP request sent, awaiting response... 200 OK
Length: 37338817 (36M) [application/x-gzip]
Saving to: ‘gdb-8.1.1.tar.gz’
 0% [                                       ] 0           --.-K/s              
 1% [                                       ] 448,598     1.89MB/s             
 2% [>                                      ] 1,070,678   2.25MB/s             
 4% [>                                      ] 1,707,350   2.39MB/s             
 6% [=>                                     ] 2,373,334   2.49MB/s             
 8% [==>                                    ] 3,058,774   2.57MB/s             
10% [==>                                    ] 3,768,406   2.64MB/s             
12% [===>                                   ] 4,497,750   2.70MB/s             
14% [====>                                  ] 5,241,174   2.75MB/s             
16% [=====>                                 ] 6,007,126   2.81MB/s             
18% [======>                                ] 6,798,422   2.86MB/s             
20% [======>                                ] 7,615,062   2.91MB/s             
22% [=======>                               ] 8,454,230   2.96MB/s             
24% [========>                              ] 9,313,366   3.01MB/s             
27% [=========>                             ] 10,198,998  3.06MB/s  eta 8s     
29% [==========>                            ] 11,109,718  3.19MB/s  eta 8s     
32% [===========>                           ] 12,044,630  3.27MB/s  eta 8s     
34% [============>                          ] 12,999,510  3.35MB/s  eta 8s     
37% [=============>                         ] 13,979,478  3.47MB/s  eta 8s     
40% [==============>                        ] 14,987,350  3.55MB/s  eta 6s     
42% [===============>                       ] 16,018,006  3.67MB/s  eta 6s     
45% [================>                      ] 17,075,414  3.75MB/s  eta 6s     
48% [=================>                     ] 18,138,710  3.83MB/s  eta 6s     
51% [===================>                   ] 19,235,542  3.95MB/s  eta 6s     
54% [====================>                  ] 20,354,902  4.04MB/s  eta 5s     
57% [=====================>                 ] 21,488,342  4.16MB/s  eta 5s     
60% [======================>                ] 22,651,094  4.24MB/s  eta 5s     
63% [=======================>               ] 23,833,814  4.32MB/s  eta 5s     
67% [=========================>             ] 25,046,102  4.44MB/s  eta 5s     
70% [==========================>            ] 26,271,062  4.52MB/s  eta 3s     
73% [===========================>           ] 27,534,038  4.64MB/s  eta 3s     
77% [=============================>         ] 28,864,854  4.74MB/s  eta 3s     
81% [==============================>        ] 30,285,270  4.85MB/s  eta 3s     
85% [================================>      ] 31,783,382  5.02MB/s  eta 3s     
89% [=================================>     ] 33,377,238  5.17MB/s  eta 1s     
93% [===================================>   ] 35,072,470  5.39MB/s  eta 1s     
98% [=====================================> ] 36,881,750  5.58MB/s  eta 1s     
100%[======================================>] 37,338,817  5.66MB/s   in 8.2s   
2018-08-26 11:46:00 (4.34 MB/s) - ‘gdb-8.1.1.tar.gz’ saved [37338817/37338817]
travis_fold:end:install.2
travis_fold:start:install.3
travis_time:start:1b364da0
travis_time:start:1b364da0
$ tar -xzvf gdb-8.1.1.tar.gz > /dev/null
travis_fold:end:install.3
travis_fold:start:install.4
travis_time:start:128020ab
travis_time:start:128020ab
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
travis_time:start:0d7aa57e
$ cd ..
$ cd ..
travis_time:end:0d7aa57e:start=1535284522225538656,finish=1535284522231115266,duration=5576610
travis_fold:end:install.5
travis_fold:start:install.6
travis_time:start:0e01ad52
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
travis_time:start:08894f74
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
412156 ./.git/objects
412148 ./.git/objects/pack
378644 ./gdb-8.1.1/gdb
96260 ./src
81340 ./gdb-8.1.1/bfd
63668 ./src/test
60360 ./gdb-8.1.1/sim
43276 ./gdb-8.1.1/opcodes
37924 ./src/test/ui
26932 ./gdb-8.1.1/gdb/gdbserver
23944 ./gdb-8.1.1/gdb/testsuite
23456 ./gdb-8.1.1/sim/testsuite
22568 ./gdb-8.1.1/sim/testsuite/sim
22428 ./gdb-8.1.1/gdb/python
15856 ./gdb-8.1.1/bfd/.libs
15120 ./src/test/run-pass
8980 ./gdb-8.1.1/sim/testsuite/sim/bfin
8468 ./gdb-8.1.1/gdb/mi
7872 ./gdb-8.1.1/gdb/doc
7216 ./gdb-8.1.1/libiberty
7084 ./gdb-8.1.1/gdb/tui
6696 ./src/test/ui/issues
6236 ./gdb-8.1.1/sim/frv
5812 ./gdb-8.1.1/readline
5564 ./gdb-8.1.1/gdb/testsuite/gdb.base
5408 ./gdb-8.1.1/zlib
5396 ./gdb-8.1.1/bfd/po
5136 ./gdb-8.1.1/sim/testsuite/sim/frv
4656 ./gdb-8.1.1/include
4476 ./src/librustc
4428 ./gdb-8.1.1/gdb/gdbserver/build-libiberty-gdbserver
4076 ./gdb-8.1.1/gdb/cli
3652 ./gdb-8.1.1/gdb/compile
3184 ./src/libcore
3184 ./src/libcore
3180 ./gdb-8.1.1/gdb/gnulib
3164 ./src/test/run-make-fulldeps
2720 ./gdb-8.1.1/opcodes/.libs
2692 ./gdb-8.1.1/libdecnumber
2564 ./gdb-8.1.1/sim/ppc
2540 ./gdb-8.1.1/cpu
2496 ./gdb-8.1.1/gdb/testsuite/gdb.ada
2460 ./gdb-8.1.1/sim/cris
2444 ./src/test/ui/borrowck
2424 ./gdb-8.1.1/gdb/gnulib/import
2416 ./src/librustc_mir
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
1312 ./gdb-8.1.1/opcodes/po
1308 ./src/test/rustdoc
1308 ./src/test/rustdoc
1296 ./gdb-8.1.1/sim/or1k
1180 ./src/librustc_codegen_llvm
1180 ./src/librustc_codegen_llvm
1172 ./gdb-8.1.1/sim/arm
1152 ./src/test/ui/feature-gates
1120 ./gdb-8.1.1/sim/testsuite/sim/mips
1048 ./src/librustdoc/html
1048 ./src/librustdoc/html
1024 ./gdb-8.1.1/sim/mn10300
980 ./src/test/ui/lint
960 ./src/ci
948 ./src/librustc/ty
948 ./src/librustc/ty
948 ./gdb-8.1.1/zlib/contrib/vstudio
940 ./src/ci/docker
940 ./gdb-8.1.1/gdb/testsuite/gdb.mi
932 ./gdb-8.1.1/sim/testsuite/sim/cris/asm
932 ./gdb-8.1.1/sim/aarch64
travis_time:end:08894f74:start=1535284522294350980,finish=1535284522379727400,duration=85376420
travis_fold:end:before_script.1
travis_fold:start:before_script.2
travis_time:start:08b6d2c5
---

[00:44:36] travis_fold:start:test_ui
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/export2.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/required-lang-item.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_share-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_send-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_send-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
[00:44:36] "/checkout/src/test/ui/no_send-enum.rs" is
[00:44:36] false
[00:44:36] false
[00:44:36] false
---
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
0 upgraded, 7 newly installed, 0 to remove and 173 not upgraded.
Need to get 2,315 kB of archives.
After this operation, 9,657 kB of additional disk space will be used.
Get:1 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libintl-perl all 1.23-1build1 [1,204 kB]
Get:3 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-namespacesupport-perl all 1.11-1 [13.2 kB]
Get:4 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-sax-base-perl all 1.07-1 [21.5 kB]
Get:5 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty/main amd64 libxml-sax-perl all 0.99+dfsg-2ubuntu1 [64.6 kB]
Get:6 http://us-central1.gce.archive.ubuntu.com/ubuntu trusty-updates/main amd64 libxml-libxml-perl amd64 2.0108+dfsg-1ubuntu0.2 [336 kB]
---
travis_time:end:01b90d0b:start=1535288746565880955,finish=1535288748195210024,duration=1629329069
travis_fold:end:install.1
travis_fold:start:install.2
travis_time:start:0b189183
$ wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
--2018-08-26 13:05:48--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
HTTP request sent, awaiting response... 200 OK
Length: 37338817 (36M) [application/x-gzip]
Saving to: ‘gdb-8.1.1.tar.gz’
 0% [                                       ] 0           --.-K/s              
 2% [                                       ] 777,814     3.70MB/s             
26% [=========>                             ] 9,863,638   23.5MB/s             
55% [====================>                  ] 20,735,062  32.9MB/s             
86% [================================>      ] 32,393,046  38.4MB/s             
100%[======================================>] 37,338,817  40.5MB/s   in 0.9s   
2018-08-26 13:05:49 (40.5 MB/s) - ‘gdb-8.1.1.tar.gz’ saved [37338817/37338817]
travis_fold:end:install.2
travis_fold:start:install.3
travis_time:start:019d17fa
travis_time:start:019d17fa
$ tar -xzvf gdb-8.1.1.tar.gz > /dev/null
travis_fold:end:install.3
travis_fold:start:install.4
travis_time:start:0d78746c
travis_time:start:0d78746c
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
travis_time:start:079141f8
$ cd ..
$ cd ..
travis_time:end:079141f8:start=1535289321427223665,finish=1535289321433271795,duration=6048130
travis_fold:end:install.5
travis_fold:start:install.6
travis_time:start:003adb8f
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
travis_time:start:1af145c4
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
412236 ./.git/objects
412228 ./.git/objects/pack
378644 ./gdb-8.1.1/gdb
96260 ./src
81340 ./gdb-8.1.1/bfd
63668 ./src/test
60360 ./gdb-8.1.1/sim
43276 ./gdb-8.1.1/opcodes
37924 ./src/test/ui
26932 ./gdb-8.1.1/gdb/gdbserver
23944 ./gdb-8.1.1/gdb/testsuite
23456 ./gdb-8.1.1/sim/testsuite
22568 ./gdb-8.1.1/sim/testsuite/sim
22428 ./gdb-8.1.1/gdb/python
15856 ./gdb-8.1.1/bfd/.libs
15120 ./src/test/run-pass
8980 ./gdb-8.1.1/sim/testsuite/sim/bfin
8468 ./gdb-8.1.1/gdb/mi
7872 ./gdb-8.1.1/gdb/doc
7216 ./gdb-8.1.1/libiberty
7084 ./gdb-8.1.1/gdb/tui
6696 ./src/test/ui/issues
6236 ./gdb-8.1.1/sim/frv
5812 ./gdb-8.1.1/readline
5564 ./gdb-8.1.1/gdb/testsuite/gdb.base
5408 ./gdb-8.1.1/zlib
5396 ./gdb-8.1.1/bfd/po
5136 ./gdb-8.1.1/sim/testsuite/sim/frv
4656 ./gdb-8.1.1/include
4476 ./src/librustc
4428 ./gdb-8.1.1/gdb/gdbserver/build-libiberty-gdbserver
4076 ./gdb-8.1.1/gdb/cli
3652 ./gdb-8.1.1/gdb/compile
3184 ./src/libcore
3184 ./src/libcore
3180 ./gdb-8.1.1/gdb/gnulib
3164 ./src/test/run-make-fulldeps
2720 ./gdb-8.1.1/opcodes/.libs
2692 ./gdb-8.1.1/libdecnumber
2564 ./gdb-8.1.1/sim/ppc
2540 ./gdb-8.1.1/cpu
2496 ./gdb-8.1.1/gdb/testsuite/gdb.ada
2460 ./gdb-8.1.1/sim/cris
2444 ./src/test/ui/borrowck
2424 ./gdb-8.1.1/gdb/gnulib/import
2416 ./src/librustc_mir
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
1312 ./gdb-8.1.1/opcodes/po
1308 ./src/test/rustdoc
1308 ./src/test/rustdoc
1296 ./gdb-8.1.1/sim/or1k
1180 ./src/librustc_codegen_llvm
1180 ./src/librustc_codegen_llvm
1172 ./gdb-8.1.1/sim/arm
1152 ./src/test/ui/feature-gates
1120 ./gdb-8.1.1/sim/testsuite/sim/mips
1048 ./src/librustdoc/html
1048 ./src/librustdoc/html
1024 ./gdb-8.1.1/sim/mn10300
980 ./src/test/ui/lint
960 ./src/ci
948 ./src/librustc/ty
948 ./src/librustc/ty
948 ./gdb-8.1.1/zlib/contrib/vstudio
940 ./src/ci/docker
940 ./gdb-8.1.1/gdb/testsuite/gdb.mi
932 ./gdb-8.1.1/sim/testsuite/sim/cris/asm
932 ./gdb-8.1.1/sim/aarch64
travis_time:end:1af145c4:start=1535289321499358311,finish=1535289321581030879,duration=81672568
travis_fold:end:before_script.1
travis_fold:start:before_script.2
travis_time:start:2a5daa65
---

[00:49:47] travis_fold:start:test_ui
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/export2.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/parse-error-correct.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/extoption_env-too-many-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/required-lang-item.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/writing-to-immutable-vec.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/cfg-attr-unknown-attribute-macro-expansion.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/stmt_expr_attrs_no_feature.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/implicit-method-bind.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/associated-path-shl.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_share-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/constructor-lifetime-args.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_send-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_send-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
[00:49:47] "/checkout/src/test/ui/no_send-enum.rs" is
[00:49:47] false
[00:49:47] false
[00:49:47] false
