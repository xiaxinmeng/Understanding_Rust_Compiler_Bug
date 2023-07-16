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
travis_time:end:016c2983:start=1535275699519770916,finish=1535275701004075631,duration=1484304715
travis_fold:end:install.1
travis_fold:start:install.2
travis_time:start:180524ca
$ wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
--2018-08-26 09:28:21--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
HTTP request sent, awaiting response... 200 OK
Length: 37338817 (36M) [application/x-gzip]
Saving to: ‘gdb-8.1.1.tar.gz’
 0% [                                       ] 0           --.-K/s              
 1% [                                       ] 669,398     2.82MB/s             
 8% [==>                                    ] 3,126,230   6.99MB/s             
12% [===>                                   ] 4,513,238   6.63MB/s             
15% [=====>                                 ] 5,894,486   6.42MB/s             
19% [======>                                ] 7,310,934   6.32MB/s             
23% [========>                              ] 8,765,398   6.29MB/s             
27% [=========>                             ] 10,245,206  6.28MB/s             
31% [===========>                           ] 11,622,486  6.20MB/s             
34% [============>                          ] 12,834,774  6.09MB/s             
37% [=============>                         ] 13,948,502  6.02MB/s             
40% [==============>                        ] 14,940,886  5.86MB/s             
43% [===============>                       ] 16,115,158  5.78MB/s             
46% [=================>                     ] 17,310,550  5.72MB/s             
49% [==================>                    ] 18,472,406  5.66MB/s  eta 3s     
51% [===================>                   ] 19,366,230  5.53MB/s  eta 3s     
54% [====================>                  ] 20,280,022  5.34MB/s  eta 3s     
56% [=====================>                 ] 21,220,566  5.10MB/s  eta 3s     
59% [======================>                ] 22,183,638  5.00MB/s  eta 3s     
62% [=======================>               ] 23,166,422  4.85MB/s  eta 3s     
64% [========================>              ] 24,163,286  4.75MB/s  eta 3s     
67% [=========================>             ] 25,168,598  4.58MB/s  eta 3s     
70% [==========================>            ] 26,196,438  4.47MB/s  eta 3s     
72% [===========================>           ] 27,246,806  4.42MB/s  eta 3s     
75% [============================>          ] 28,311,254  4.36MB/s  eta 2s     
78% [=============================>         ] 29,340,502  4.34MB/s  eta 2s     
80% [==============================>        ] 30,228,950  4.25MB/s  eta 2s     
82% [===============================>       ] 30,952,662  4.15MB/s  eta 2s     
84% [================================>      ] 31,645,654  4.05MB/s  eta 2s     
87% [================================>      ] 32,503,126  3.99MB/s  eta 1s     
89% [=================================>     ] 33,392,982  4.01MB/s  eta 1s     
91% [==================================>    ] 34,300,886  4.02MB/s  eta 1s     
94% [===================================>   ] 35,249,878  4.04MB/s  eta 1s     
96% [====================================>  ] 36,183,638  4.05MB/s  eta 1s     
99% [=====================================> ] 37,150,422  4.08MB/s  eta 0s     
100%[======================================>] 37,338,817  4.09MB/s   in 7.6s   
2018-08-26 09:28:28 (4.71 MB/s) - ‘gdb-8.1.1.tar.gz’ saved [37338817/37338817]
travis_fold:end:install.2
travis_fold:start:install.3
travis_time:start:17f52560
travis_time:start:17f52560
$ tar -xzvf gdb-8.1.1.tar.gz > /dev/null
travis_fold:end:install.3
travis_fold:start:install.4
travis_time:start:111183de
travis_time:start:111183de
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
travis_time:start:014df525
$ cd ..
$ cd ..
travis_time:end:014df525:start=1535276261593782169,finish=1535276261600944019,duration=7161850
travis_fold:end:install.5
travis_fold:start:install.6
travis_time:start:0cf9b3cc
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
travis_time:start:01c3dc1a
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
412132 ./.git/objects
412124 ./.git/objects/pack
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
travis_time:end:01c3dc1a:start=1535276261668687617,finish=1535276261754219183,duration=85531566
travis_fold:end:before_script.1
travis_fold:start:before_script.2
travis_time:start:071cfc70
---
[00:45:16]    Compiling compiletest v0.0.0 (file:///checkout/src/tools/compiletest)
[00:45:16] error[E0425]: cannot find value `test_file` in this scope
[00:45:16]   --> tools/compiletest/src/header.rs:49:33
[00:45:16]    |
[00:45:16] 49 |             println!("{:?} is", test_file);
[00:45:16]    |                                 ^^^^^^^^^ did you mean `testfile`?
[00:45:16] error[E0425]: cannot find value `test_file` in this scope
[00:45:16]   --> tools/compiletest/src/header.rs:65:46
[00:45:16]    |
[00:45:16]    |
[00:45:16] 65 |             println!("{:?} will be ignored", test_file);
[00:45:16]    |                                              ^^^^^^^^^ did you mean `testfile`?
[00:45:17] error: aborting due to 2 previous errors
[00:45:17] 
[00:45:17] For more information about this error, try `rustc --explain E0425`.
[00:45:17] error: Could not compile `compiletest`.
[00:45:17] error: Could not compile `compiletest`.
[00:45:17] 
[00:45:17] Caused by:
[00:45:17]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name compiletest tools/compiletest/src/main.rs --color always --error-format json --crate-type bin --emit=dep-info,link -C opt-level=2 -C metadata=e7c792fe814f5406 -C extra-filename=-e7c792fe814f5406 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps --extern diff=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libdiff-d2a650d80e73503e.rlib --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libenv_logger-e803a21f3defa65f.rlib --extern filetime=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libfiletime-9bcd1601bd218a76.rlib --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libgetopts-bde56a42402a0852.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/liblibc-1482fac8994892d4.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/liblog-deb7b90968dd7ba0.rlib --extern regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7a6a486060e3b2e9.rlib --extern rustfix=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/librustfix-74cf7700a32ca457.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde-2435d05837154024.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release/deps/libserde_derive-852a8953aff62411.so --extern serde_json=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps/libserde_json-5f48fd0bb4ac789f.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/build/backtrace-sys-ee16f6821aef40e9/out` (exit code: 1)
[00:45:17] expected success, got: exit code: 101
[00:45:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:45:17] Build completed unsuccessfully in 
travis_time:end:1b948240:start=1535276261838974649,finish=1535278980003458353,duration=2718164483704
---
travis_time:end:004c2ac4:start=1535278980964344129,finish=1535278980975838132,duration=11494003
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06a1959e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15bc2d4c
$ dmesg | grep -i kill
