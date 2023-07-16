plain
[00:01:48] done.
[00:01:48] Processing triggers for systemd (229-4ubuntu21.4) ...
[00:01:55]  ---> 397a5134ba67
[00:01:55] Removing intermediate container ce8268672f01
[00:01:55] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:01:56]  ---> Running in 9c5bb7b791d2
[00:01:56] --2018-09-03 13:56:09--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
[00:01:56] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:01:56] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:01:56] HTTP request sent, awaiting response... 200 OK
[00:01:56] Length: 37338817 (36M) [application/x-gzip]
[00:01:56] Saving to: 'gdb-8.1.1.tar.gz'
[00:01:56]      0K .......... .......... .......... .......... ..........  0%  761K 48s
[00:01:56]     50K .......... .......... .......... .......... ..........  0% 1.47M 36s
[00:01:56]    100K .......... .......... .......... .......... ..........  0%  109M 24s
[00:01:56]    150K .......... .......... .......... .......... ..........  0% 1.51M 24s
---
[00:02:04]  36350K .......... .......... .......... .......... .......... 99% 52.8M 0s
[00:02:04]  36400K .......... .......... .......... .......... .......... 99%  172M 0s
[00:02:04]  36450K .......... ...                                        100% 44.2M=8.2s
[00:02:04] 
[00:02:04] 2018-09-03 13:56:18 (4.32 MB/s) - 'gdb-8.1.1.tar.gz' saved [37338817/37338817]
[00:02:04] 
[00:02:07] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:02:07] configure: WARNING:
[00:02:07] *** Makeinfo is missing. Info documentation will not be built.
[00:02:41] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:20] ar: `u' modifier ignored since `D' is the default (see `U')
[00:03:26] complete.c: In function 'fnwidth':
[00:03:26] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:26]         w = wcwidth (wc);
[00:03:26]             ^
[00:03:27] display.c: In function 'rl_redisplay':
[00:03:27] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:27]         temp = wcwidth (wc);
[00:03:27]                ^
[00:03:28] util.c: In function '_rl_tropen':
[00:03:28] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:03:28]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:03:28]                    ^
[00:03:30] histfile.c: In function 'history_truncate_file':
[00:03:30] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:03:30]        write (file, bp, chars_read - (bp - buffer));
[00:03:30]        ^
[00:03:30] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:03:30] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:03:30]         if (wcwidth (wc) == 0)
[00:03:30]             ^
[00:03:55] configure: WARNING: no enhanced curses library found; disabling TUI
[00:03:55] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:03:55] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:03:56] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:03:56] configure: WARNING: pkg-config not found, guile support disabled
[00:03:56] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:04:03] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:04:26] configure: WARNING:
[00:04:26] *** Makeinfo is missing. Info documentation will not be built.
[00:04:35] Creating observer.htmp
[00:04:35] Creating observer.itmp
[00:05:06] cli/cli-cmds.c: In function 'void complete_command(const char*, int)':
[00:05:06] cli/cli-cmds.c:304:48: warning: 'word' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:05:06]          get_max_completions_reached_message ());
[00:05:06]                                                 ^
[00:05:06] cli/cli-cmds.c:277:71: warning: 'tracker' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:05:06]      = tracker->build_completion_result (word, word - arg, strlen (arg));
[00:05:06]                                                                        ^
[00:06:14] breakpoint.c: In function 'void check_status_watchpoint(bpstat)':
[00:06:14] breakpoint.c:5079:4: warning: 'e' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:06:14]     switch (e)
[00:06:14]     ^
[00:06:14] breakpoint.c:5056:20: note: 'e' was declared here
[00:06:14]     wp_check_result e;
[00:07:31] In file included from inferior.h:48:0,
[00:07:31] In file included from inferior.h:48:0,
[00:07:31]                  from infrun.c:26:
[00:07:31] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:07:31] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:07:31]    { set_current_program_space (m_saved_pspace); }
[00:07:31]                                                ^
[00:07:31] infrun.c:927:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:07:31]       maybe_restore_inferior;
[00:07:31]       ^
[00:10:09] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:10:09] WARNING: 'makeinfo' is missing on your system.
[00:10:09]          You should only need it if you modified a '.texi' file, or
[00:10:09]          any other file indirectly affecting the aspect of the manual.
[00:10:09]          You might want to install the Texinfo package:
[00:10:09]          <http://www.gnu.org/software/texinfo/>
[00:10:09]          The spurious makeinfo call might also be the consequence of
[00:10:09]          using a buggy 'make' (AIX, DU, IRIX), in which case you might
[00:10:09]          want to install GNU make:
[00:10:09]          <http://www.gnu.org/software/make/>
[00:10:09] make[5]: *** [gdb.info] Error 127
[00:10:09] make[4]: *** [subdir_do] Error 1
[00:10:09] make[3]: *** [install-only] Error 2
[00:10:09] make[2]: *** [install] Error 2
[00:10:09] make[1]: *** [install-gdb] Error 2
[00:10:09] make: *** [install] Error 2
[00:10:09] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 2
[00:10:10] Sending build context to Docker daemon  500.7kB
[00:10:10] Step 1/7 : FROM ubuntu:16.04
[00:10:10]  ---> 52b10959e8aa
[00:10:10] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:10:10] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:10:10]  ---> Using cache
[00:10:10]  ---> 397a5134ba67
[00:10:10] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:10:10]  ---> Running in 0d25d0900117
[00:10:10] --2018-09-03 14:04:24--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
[00:10:10] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:10:10] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:10:10] HTTP request sent, awaiting response... 200 OK
[00:10:10] Length: 37338817 (36M) [application/x-gzip]
[00:10:10] Saving to: 'gdb-8.1.1.tar.gz'
[00:10:11]      0K .......... .......... .......... .......... ..........  0%  759K 48s
[00:10:11]     50K .......... .......... .......... .......... ..........  0% 1.50M 36s
[00:10:11]    100K .......... .......... .......... .......... ..........  0% 71.1M 24s
[00:10:11]    150K .......... .......... .......... .......... ..........  0%  160M 18s
---
[00:10:13]  36350K .......... .......... .......... .......... .......... 99% 27.1M 0s
[00:10:13]  36400K .......... .......... .......... .......... .......... 99% 28.9M 0s
[00:10:13]  36450K .......... ...                                        100% 8.77M=2.6s
[00:10:13] 
[00:10:13] 2018-09-03 14:04:27 (13.7 MB/s) - 'gdb-8.1.1.tar.gz' saved [37338817/37338817]
[00:10:13] 
[00:10:15] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:10:16] configure: WARNING:
[00:10:16] *** Makeinfo is missing. Info documentation will not be built.
[00:10:49] ar: `u' modifier ignored since `D' is the default (see `U')
[00:11:27] ar: `u' modifier ignored since `D' is the default (see `U')
[00:11:33] complete.c: In function 'fnwidth':
[00:11:33] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:11:33]         w = wcwidth (wc);
[00:11:33]             ^
[00:11:34] display.c: In function 'rl_redisplay':
[00:11:34] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:11:34]         temp = wcwidth (wc);
[00:11:34]                ^
[00:11:35] util.c: In function '_rl_tropen':
[00:11:35] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:11:35]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:11:35]                    ^
[00:11:37] histfile.c: In function 'history_truncate_file':
[00:11:37] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:11:37]        write (file, bp, chars_read - (bp - buffer));
[00:11:37]        ^
[00:11:37] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:11:37] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:11:37]         if (wcwidth (wc) == 0)
[00:11:37]             ^
[00:12:03] configure: WARNING: no enhanced curses library found; disabling TUI
[00:12:03] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:12:03] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:12:03] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:12:03] configure: WARNING: pkg-config not found, guile support disabled
[00:12:03] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:12:11] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:12:34] configure: WARNING:
[00:12:34] *** Makeinfo is missing. Info documentation will not be built.
[00:12:43] Creating observer.htmp
[00:12:43] Creating observer.itmp
[00:13:14] cli/cli-cmds.c: In function 'void complete_command(const char*, int)':
[00:13:14] cli/cli-cmds.c:304:48: warning: 'word' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:13:14]          get_max_completions_reached_message ());
[00:13:14]                                                 ^
[00:13:14] cli/cli-cmds.c:277:71: warning: 'tracker' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:13:14]      = tracker->build_completion_result (word, word - arg, strlen (arg));
[00:13:14]                                                                        ^
[00:14:24] breakpoint.c: In function 'void check_status_watchpoint(bpstat)':
[00:14:24] breakpoint.c:5079:4: warning: 'e' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:14:24]     switch (e)
[00:14:24]     ^
[00:14:24] breakpoint.c:5056:20: note: 'e' was declared here
[00:14:24]     wp_check_result e;
[00:15:41] In file included from inferior.h:48:0,
[00:15:41] In file included from inferior.h:48:0,
[00:15:41]                  from infrun.c:26:
[00:15:41] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:15:41] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:15:41]    { set_current_program_space (m_saved_pspace); }
[00:15:41]                                                ^
[00:15:41] infrun.c:927:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:15:41]       maybe_restore_inferior;
[00:15:41]       ^
[00:18:19] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:18:19] WARNING: 'makeinfo' is missing on your system.
[00:18:19]          You should only need it if you modified a '.texi' file, or
[00:18:19]          any other file indirectly affecting the aspect of the manual.
[00:18:19]          You might want to install the Texinfo package:
[00:18:19]          <http://www.gnu.org/software/texinfo/>
[00:18:19]          The spurious makeinfo call might also be the consequence of
[00:18:19]          using a buggy 'make' (AIX, DU, IRIX), in which case you might
[00:18:19]          want to install GNU make:
[00:18:19]          <http://www.gnu.org/software/make/>
[00:18:19] make[5]: *** [gdb.info] Error 127
[00:18:19] make[4]: *** [subdir_do] Error 1
[00:18:19] make[3]: *** [install-only] Error 2
[00:18:19] make[2]: *** [install] Error 2
[00:18:19] make[1]: *** [install-gdb] Error 2
[00:18:19] make: *** [install] Error 2
[00:18:20] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 2
[00:18:22] Sending build context to Docker daemon  500.7kB
[00:18:22] Step 1/7 : FROM ubuntu:16.04
[00:18:22]  ---> 52b10959e8aa
[00:18:22] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:18:22] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:18:22]  ---> Using cache
[00:18:22]  ---> 397a5134ba67
[00:18:22] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:18:22]  ---> Running in c292f06698eb
[00:18:22] --2018-09-03 14:12:36--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
[00:18:22] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:18:22] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:18:22] HTTP request sent, awaiting response... 200 OK
[00:18:22] Length: 37338817 (36M) [application/x-gzip]
[00:18:22] Saving to: 'gdb-8.1.1.tar.gz'
[00:18:22]      0K .......... .......... .......... .......... ..........  0%  738K 49s
[00:18:22]     50K .......... .......... .......... .......... ..........  0% 1.47M 37s
[00:18:22]    100K .......... .......... .......... .......... ..........  0% 4.08M 27s
[00:18:22]    150K .......... .......... .......... .......... ..........  0%  164M 21s
---
[00:18:24]  36350K .......... .......... .......... .......... .......... 99% 96.8M 0s
[00:18:24]  36400K .......... .......... .......... .......... .......... 99% 71.7M 0s
[00:18:24]  36450K .......... ...                                        100% 34.1M=1.9s
[00:18:24] 
[00:18:24] 2018-09-03 14:12:38 (18.6 MB/s) - 'gdb-8.1.1.tar.gz' saved [37338817/37338817]
[00:18:24] 
[00:18:27] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:18:27] configure: WARNING:
[00:18:27] *** Makeinfo is missing. Info documentation will not be built.
[00:19:02] ar: `u' modifier ignored since `D' is the default (see `U')
[00:19:39] ar: `u' modifier ignored since `D' is the default (see `U')
[00:19:45] complete.c: In function 'fnwidth':
[00:19:45] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:19:45]         w = wcwidth (wc);
[00:19:45]             ^
[00:19:46] display.c: In function 'rl_redisplay':
[00:19:46] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:19:46]         temp = wcwidth (wc);
[00:19:46]                ^
[00:19:47] util.c: In function '_rl_tropen':
[00:19:47] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:19:47]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:19:47]                    ^
[00:19:49] histfile.c: In function 'history_truncate_file':
[00:19:49] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:19:49]        write (file, bp, chars_read - (bp - buffer));
[00:19:49]        ^
[00:19:49] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:19:49] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:19:49]         if (wcwidth (wc) == 0)
[00:19:49]             ^
[00:20:15] configure: WARNING: no enhanced curses library found; disabling TUI
[00:20:15] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:20:15] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:20:16] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:20:16] configure: WARNING: pkg-config not found, guile support disabled
[00:20:16] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:20:23] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:20:48] configure: WARNING:
[00:20:48] *** Makeinfo is missing. Info documentation will not be built.
[00:20:56] Creating observer.htmp
[00:20:56] Creating observer.itmp
[00:21:28] cli/cli-cmds.c: In function 'void complete_command(const char*, int)':
[00:21:28] cli/cli-cmds.c:304:48: warning: 'word' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:21:28]          get_max_completions_reached_message ());
[00:21:28]                                                 ^
[00:21:28] cli/cli-cmds.c:277:71: warning: 'tracker' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:21:28]      = tracker->build_completion_result (word, word - arg, strlen (arg));
[00:21:28]                                                                        ^
[00:22:37] breakpoint.c: In function 'void check_status_watchpoint(bpstat)':
[00:22:37] breakpoint.c:5079:4: warning: 'e' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:22:37]     switch (e)
[00:22:37]     ^
[00:22:37] breakpoint.c:5056:20: note: 'e' was declared here
[00:22:37]     wp_check_result e;
[00:23:57] In file included from inferior.h:48:0,
[00:23:57] In file included from inferior.h:48:0,
[00:23:57]                  from infrun.c:26:
[00:23:57] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:23:57] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:23:57]    { set_current_program_space (m_saved_pspace); }
[00:23:57]                                                ^
[00:23:57] infrun.c:927:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:23:57]       maybe_restore_inferior;
[00:23:57]       ^
[00:26:34] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:26:34] WARNING: 'makeinfo' is missing on your system.
[00:26:34]          You should only need it if you modified a '.texi' file, or
[00:26:34]          any other file indirectly affecting the aspect of the manual.
[00:26:34]          You might want to install the Texinfo package:
[00:26:34]          <http://www.gnu.org/software/texinfo/>
[00:26:34]          The spurious makeinfo call might also be the consequence of
[00:26:34]          using a buggy 'make' (AIX, DU, IRIX), in which case you might
[00:26:34]          want to install GNU make:
[00:26:34]          <http://www.gnu.org/software/make/>
[00:26:34] make[5]: *** [gdb.info] Error 127
[00:26:34] make[4]: *** [subdir_do] Error 1
[00:26:34] make[3]: *** [install-only] Error 2
[00:26:34] make[2]: *** [install] Error 2
[00:26:34] make[1]: *** [install-gdb] Error 2
[00:26:34] make: *** [install] Error 2
[00:26:34] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 2
[00:26:38] Sending build context to Docker daemon  500.7kB
[00:26:38] Step 1/7 : FROM ubuntu:16.04
[00:26:38]  ---> 52b10959e8aa
[00:26:38] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:26:38] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:26:38]  ---> Using cache
[00:26:38]  ---> 397a5134ba67
[00:26:38] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:26:38]  ---> Running in e1334c13cda6
[00:26:38] --2018-09-03 14:20:51--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
[00:26:38] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:26:38] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:26:38] HTTP request sent, awaiting response... 200 OK
[00:26:38] Length: 37338817 (36M) [application/x-gzip]
[00:26:38] Saving to: 'gdb-8.1.1.tar.gz'
[00:26:38]      0K .......... .......... .......... .......... ..........  0%  753K 48s
[00:26:38]     50K .......... .......... .......... .......... ..........  0% 1.48M 36s
[00:26:38]    100K .......... .......... .......... .......... ..........  0% 56.3M 24s
[00:26:38]    150K .......... .......... .......... .......... ..........  0% 1.51M 24s
---
[00:26:39]  36350K .......... .......... .......... .......... .......... 99%  351M 0s
[00:26:39]  36400K .......... .......... .......... .......... .......... 99%  323M 0s
[00:26:39]  36450K .......... ...                                        100%  248M=1.0s
[00:26:39] 
[00:26:39] 2018-09-03 14:20:53 (36.3 MB/s) - 'gdb-8.1.1.tar.gz' saved [37338817/37338817]
[00:26:39] 
[00:26:41] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:26:42] configure: WARNING:
[00:26:42] *** Makeinfo is missing. Info documentation will not be built.
[00:27:15] ar: `u' modifier ignored since `D' is the default (see `U')
[00:27:53] ar: `u' modifier ignored since `D' is the default (see `U')
[00:27:59] complete.c: In function 'fnwidth':
[00:27:59] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:27:59]         w = wcwidth (wc);
[00:27:59]             ^
[00:28:00] display.c: In function 'rl_redisplay':
[00:28:00] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:28:00]         temp = wcwidth (wc);
[00:28:00]                ^
[00:28:01] util.c: In function '_rl_tropen':
[00:28:01] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:28:01]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:28:01]                    ^
[00:28:03] histfile.c: In function 'history_truncate_file':
[00:28:03] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:28:03]        write (file, bp, chars_read - (bp - buffer));
[00:28:03]        ^
[00:28:03] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:28:03] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:28:03]         if (wcwidth (wc) == 0)
[00:28:03]             ^
[00:28:29] configure: WARNING: no enhanced curses library found; disabling TUI
[00:28:29] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:28:29] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:28:29] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:28:29] configure: WARNING: pkg-config not found, guile support disabled
[00:28:29] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:28:37] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:29:02] configure: WARNING:
[00:29:02] *** Makeinfo is missing. Info documentation will not be built.
[00:29:10] Creating observer.htmp
[00:29:10] Creating observer.itmp
[00:29:42] cli/cli-cmds.c: In function 'void complete_command(const char*, int)':
[00:29:42] cli/cli-cmds.c:304:48: warning: 'word' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:29:42]          get_max_completions_reached_message ());
[00:29:42]                                                 ^
[00:29:42] cli/cli-cmds.c:277:71: warning: 'tracker' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:29:42]      = tracker->build_completion_result (word, word - arg, strlen (arg));
[00:29:42]                                                                        ^
[00:30:50] breakpoint.c: In function 'void check_status_watchpoint(bpstat)':
[00:30:50] breakpoint.c:5079:4: warning: 'e' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:30:50]     switch (e)
[00:30:50]     ^
[00:30:50] breakpoint.c:5056:20: note: 'e' was declared here
[00:30:50]     wp_check_result e;
[00:32:09] In file included from inferior.h:48:0,
[00:32:09] In file included from inferior.h:48:0,
[00:32:09]                  from infrun.c:26:
[00:32:09] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:32:09] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:32:09]    { set_current_program_space (m_saved_pspace); }
[00:32:09]                                                ^
[00:32:09] infrun.c:927:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:32:09]       maybe_restore_inferior;
[00:32:09]       ^
[00:34:48] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:34:48] WARNING: 'makeinfo' is missing on your system.
[00:34:48]          You should only need it if you modified a '.texi' file, or
[00:34:48]          any other file indirectly affecting the aspect of the manual.
[00:34:48]          You might want to install the Texinfo package:
[00:34:48]          <http://www.gnu.org/software/texinfo/>
[00:34:48]          The spurious makeinfo call might also be the consequence of
[00:34:48]          using a buggy 'make' (AIX, DU, IRIX), in which case you might
[00:34:48]          want to install GNU make:
[00:34:48]          <http://www.gnu.org/software/make/>
[00:34:48] make[5]: *** [gdb.info] Error 127
[00:34:48] make[4]: *** [subdir_do] Error 1
[00:34:48] make[3]: *** [install-only] Error 2
[00:34:48] make[2]: *** [install] Error 2
[00:34:48] make[1]: *** [install-gdb] Error 2
[00:34:48] make: *** [install] Error 2
[00:34:48] The command '/bin/sh -c wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version' returned a non-zero code: 2
[00:34:52] Sending build context to Docker daemon  500.7kB
[00:34:52] Step 1/7 : FROM ubuntu:16.04
[00:34:52]  ---> 52b10959e8aa
[00:34:52] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:34:52] Step 2/7 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   wget   llvm-5.0-tools   libedit-dev   zlib1g-dev   xz-utils
[00:34:52]  ---> Using cache
[00:34:52]  ---> 397a5134ba67
[00:34:52] Step 3/7 : RUN wget http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz > /dev/null &&   tar -xzvf gdb-8.1.1.tar.gz > /dev/null &&   cd gdb-8.1.1 &&   ./configure > /dev/null &&   make > /dev/null &&   sudo make install > /dev/null &&   cd .. &&   gdb --version
[00:34:52]  ---> Running in a1b05f49a374
[00:34:52] --2018-09-03 14:29:06--  http://ftp.gnu.org/gnu/gdb/gdb-8.1.1.tar.gz
[00:34:52] Resolving ftp.gnu.org (ftp.gnu.org)... 208.118.235.20, 2001:4830:134:3::b
[00:34:53] Connecting to ftp.gnu.org (ftp.gnu.org)|208.118.235.20|:80... connected.
[00:34:53] HTTP request sent, awaiting response... 200 OK
[00:34:53] Length: 37338817 (36M) [application/x-gzip]
[00:34:53] Saving to: 'gdb-8.1.1.tar.gz'
[00:34:53]      0K .......... .......... .......... .......... ..........  0%  772K 47s
[00:34:53]     50K .......... .......... .......... .......... ..........  0% 1.50M 35s
[00:34:53]    100K .......... .......... .......... .......... ..........  0%  164M 24s
[00:34:53]    150K .......... .......... .......... .......... ..........  0% 1.51M 24s
---
[00:34:55]  36350K .......... .......... .......... .......... .......... 99%  106M 0s
[00:34:55]  36400K .......... .......... .......... .......... .......... 99%  277M 0s
[00:34:55]  36450K .......... ...                                        100% 50.1M=2.1s
[00:34:55] 
[00:34:55] 2018-09-03 14:29:08 (16.6 MB/s) - 'gdb-8.1.1.tar.gz' saved [37338817/37338817]
[00:34:55] 
[00:34:57] /gdb-8.1.1/missing: 81: /gdb-8.1.1/missing: makeinfo: not found
[00:34:58] configure: WARNING:
[00:34:58] *** Makeinfo is missing. Info documentation will not be built.
[00:35:32] ar: `u' modifier ignored since `D' is the default (see `U')
[00:36:10] ar: `u' modifier ignored since `D' is the default (see `U')
[00:36:16] complete.c: In function 'fnwidth':
[00:36:16] complete.c:701:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:36:16]         w = wcwidth (wc);
[00:36:16]             ^
[00:36:17] display.c: In function 'rl_redisplay':
[00:36:17] display.c:768:15: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:36:17]         temp = wcwidth (wc);
[00:36:17]                ^
[00:36:18] util.c: In function '_rl_tropen':
[00:36:18] util.c:520:19: warning: format '%ld' expects argument of type 'long int', but argument 3 has type '__pid_t {aka int}' [-Wformat=]
[00:36:18]    sprintf (fnbuf, "/var/tmp/rltrace.%ld", getpid());
[00:36:18]                    ^
[00:36:19] histfile.c: In function 'history_truncate_file':
[00:36:19] histfile.c:410:7: warning: ignoring return value of 'write', declared with attribute warn_unused_result [-Wunused-result]
[00:36:19]        write (file, bp, chars_read - (bp - buffer));
[00:36:19]        ^
[00:36:20] mbutil.c: In function '_rl_find_next_mbchar_internal':
[00:36:20] mbutil.c:122:12: warning: implicit declaration of function 'wcwidth' [-Wimplicit-function-declaration]
[00:36:20]         if (wcwidth (wc) == 0)
[00:36:20]             ^
[00:36:46] configure: WARNING: no enhanced curses library found; disabling TUI
[00:36:47] configure: WARNING: expat is missing or unusable; some features may be unavailable.
[00:36:47] configure: WARNING: MPFR is missing or unusable; some features may be unavailable.
[00:36:47] configure: WARNING: python is missing or unusable; some features may be unavailable.
[00:36:47] configure: WARNING: pkg-config not found, guile support disabled
[00:36:47] configure: WARNING: libipt is missing or unusable; some features may be unavailable.
[00:36:55] configure: WARNING: babeltrace is missing or unusable; GDB is unable to read CTF data.
[00:37:19] configure: WARNING:
[00:37:19] *** Makeinfo is missing. Info documentation will not be built.
[00:37:28] Creating observer.htmp
[00:37:28] Creating observer.itmp
[00:37:59] cli/cli-cmds.c: In function 'void complete_command(const char*, int)':
[00:37:59] cli/cli-cmds.c:304:48: warning: 'word' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:37:59]          get_max_completions_reached_message ());
[00:37:59]                                                 ^
[00:37:59] cli/cli-cmds.c:277:71: warning: 'tracker' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:37:59]      = tracker->build_completion_result (word, word - arg, strlen (arg));
[00:37:59]                                                                        ^
[00:39:07] breakpoint.c: In function 'void check_status_watchpoint(bpstat)':
[00:39:07] breakpoint.c:5079:4: warning: 'e' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:39:07]     switch (e)
[00:39:07]     ^
[00:39:07] breakpoint.c:5056:20: note: 'e' was declared here
[00:39:07]     wp_check_result e;
[00:40:27] In file included from inferior.h:48:0,
[00:40:27] In file included from inferior.h:48:0,
[00:40:27]                  from infrun.c:26:
[00:40:27] progspace.h: In function 'void handle_vfork_child_exec_or_exit(int)':
[00:40:27] progspace.h:285:47: warning: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' may be used uninitialized in this function [-Wmaybe-uninitialized]
[00:40:27]    { set_current_program_space (m_saved_pspace); }
[00:40:27]                                                ^
[00:40:27] infrun.c:927:6: note: '*((void*)(& maybe_restore_inferior)+32).scoped_restore_current_program_space::m_saved_pspace' was declared here
[00:40:27]       maybe_restore_inferior;
