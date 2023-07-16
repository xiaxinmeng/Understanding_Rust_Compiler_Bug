plain
[00:02:28] done.
[00:02:28] Processing triggers for systemd (229-4ubuntu21.4) ...
[00:02:33]  ---> c1e30609b841
[00:02:33] Removing intermediate container 9b1307087eb4
[00:02:33] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz > /dev/null &&   tar -xzvf gdb-8.2.tar.gz > /dev/null &&   cd gdb-8.2 &&   ./configure --disable-werror --disable-binutils --disable-gas --disable-gold --disable-ld --disable-gprof > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:02:33]  ---> Running in 9e2e9f6eaa83
[00:02:34] --2018-09-07 14:46:43--  http://ftp.gnu.org/gnu/gdb/gdb-8.2.tar.gz
[00:02:34] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:02:34] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:02:34] HTTP request sent, awaiting response... 200 OK
[00:02:34] Length: 37605479 (36M) [application/x-gzip]
[00:02:34] Saving to: 'gdb-8.2.tar.gz'
[00:02:34]      0K .......... .......... .......... .......... ..........  0%  744K 49s
[00:02:34]     50K .......... .......... .......... .......... ..........  0% 1.45M 37s
[00:02:34]    100K .......... .......... .......... .......... ..........  0% 93.8M 25s
[00:02:34]    150K .......... .......... .......... .......... ..........  0% 1.48M 25s
---
[00:02:37]  36600K .......... .......... .......... .......... .......... 99% 79.1M 0s
[00:02:37]  36650K .......... .......... .......... .......... .......... 99% 3.01M 0s
[00:02:37]  36700K .......... .......... ....                            100% 5.00M=3.6s
[00:02:37] 
[00:02:37] 2018-09-07 14:46:46 (9.97 MB/s) - 'gdb-8.2.tar.gz' saved [37605479/37605479]
[00:02:37] 
[00:02:39] configure: WARNING: neither ld nor gold are enabled
[00:03:15] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:53] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:59] complete.c: In function 'fnwidth':
[00:03:59] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:59]         w = wcwidth (wc);
[00:03:59]             ^
[00:04:00] display.c: In function 'rl_redisplay':
[00:04:00] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:04:00]         temp = wcwidth (wc);
[00:04:00]                ^
[00:04:01] util.c: In function '_rl_tropen':
[00:04:01] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:04:01]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:04:01]                    ^
[00:04:03] histfile.c: In function 'history_truncate_file':
[00:04:03] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:04:03]        write (file, bp, chars_read - (bp - buffer));
[00:04:03]        ^
[00:04:03] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:04:03] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:04:03]         if (wcwidth (wc) == 0)
[00:04:03]             ^
[00:04:31] configure: WARNING: no enhanced curses library found; disabling TUI
[00:04:31] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:04:31] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:04:31] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:04:31] configure: WARNING: pkg-config not found, guile support disabled
[00:04:31] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:04:39] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:08:36] In file included from inferior.h:49:0,
[00:08:36]                  from infrun.c:26:
[00:08:36] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:08:36] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:08:36]    { set_current_program_space (m_saved_pspace); }
[00:08:36]                                                ^
[00:08:36] infrun.c:929:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:08:36]       maybe_restore_inferior;
[00:08:36]       ^
[00:11:45] ./gdb.texinfo:2021: warning: @xref node name should not contain `.'
[00:11:45] ./gdb.texinfo:8757: warning: @pxref node name should not contain `.'
[00:11:45] ./gdb.texinfo:19508: warning: @ref node name should not contain `.'
[00:11:45] ./python.texi:4060: warning: @ref node name should not contain `.'
[00:11:45] ./python.texi:4096: warning: @ref node name should not contain `.'
[00:11:47] GNU gdb (GDB) 8.2
[00:11:47] Copyright (C) 2018 Free Software Foundation, Inc.
[00:11:47] License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
[00:11:47] This is free software: you are free to change and redistribute it.
[00:11:54]  ---> ce57fa099e49
[00:11:54] Removing intermediate container 9e2e9f6eaa83
[00:11:54] Step 4/7 : COPY scripts/sccache.sh /scripts/
[00:11:54]  ---> 74c1b715b4f9
---
[00:12:07] Successfully tagged rust-ci:latest
[00:12:07] Built container sha256:f91a1a2d60927b5ff5e8b309be9c97f8d27a2e198c942b64a3d5ee6b3610bdf1
[00:12:07] Uploading finished image to s3://rust-lang-ci-sccache2/docker/c8913d125b5fb7acb1a4d3302adfbd089f5a1e9387e98c52458bf75b0035b4c2165716b8dc59a885f198cfb9c7b79fa7a8d7eba94e6b281acd38c2c8d24c4730
[00:12:07] 
[00:12:07] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:12:19] xargs: docker: terminated by signal 13

[00:12:43] travis_time:end:0620243a:start=1536331543186682147,finish=1536332188722144193,duration=645535462046
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:12:43] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:59:08] ....................................................................................................
[00:59:11] ....................................................................................................
[00:59:14] .........................................i..........................................................
[00:59:17] ....................................................................................................
[00:59:20] ..........................................................................................iiiiiiiii.
[00:59:25] ............ii......................................................................................
[00:59:28] ....................................................................................................
[00:59:32] .......................................................................i............................
[00:59:34] ....................................................................................................
