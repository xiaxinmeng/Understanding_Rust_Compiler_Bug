 bash
/home/zazdxscf $ sudo paxctl -v /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
Password: 
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu>

- PaX flags: -------x-e-- [/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc]
    RANDEXEC is disabled
    EMUTRAMP is disabled
-----------
zazdxscf@gobaby 2015/10/26 01:12:24 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12306 8 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl -vr /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu>

/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc: Text file busy
-----------
zazdxscf@gobaby 2015/10/26 01:12:46 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12307 9 1  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl -v /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu>

- PaX flags: -------x-e-- [/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc]
    RANDEXEC is disabled
    EMUTRAMP is disabled
-----------
zazdxscf@gobaby 2015/10/26 01:12:53 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12308 10 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl -vr /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu>

/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc: Text file busy
-----------
zazdxscf@gobaby 2015/10/26 01:12:56 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12309 11 1  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl -v /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
PaX control v0.9
Copyright 2004,2005,2006,2007,2009,2010,2011,2012,2014 PaX Team <pageexec@freemail.hu>

- PaX flags: -------x-e-- [/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc]
    RANDEXEC is disabled
    EMUTRAMP is disabled
-----------
zazdxscf@gobaby 2015/10/26 01:13:11 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12310 12 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -v /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : not found

-----------
zazdxscf@gobaby 2015/10/26 01:13:21 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12311 13 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -v -l -r /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e-r-

-----------
zazdxscf@gobaby 2015/10/26 01:13:55 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12312 14 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -v -rl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e-r-

-----------
zazdxscf@gobaby 2015/10/26 01:14:23 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12313 15 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vrl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e-r-

-----------
zazdxscf@gobaby 2015/10/26 01:14:27 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12314 16 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vrl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e-r-

-----------
zazdxscf@gobaby 2015/10/26 01:14:41 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12315 17 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ time RUST_BACKTRACE= gdb --args /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
GNU gdb (Gentoo 7.10 vanilla) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://bugs.gentoo.org/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
(gdb) r
Starting program: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
During startup program terminated with signal SIGSEGV, Segmentation fault.
(gdb) quit

real    0m6.921s
user    0m0.090s
sys 0m0.077s
-----------
zazdxscf@gobaby 2015/10/26 01:14:58 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12316 18 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vRl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e-R-

-----------
zazdxscf@gobaby 2015/10/26 01:15:03 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12317 19 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ time RUST_BACKTRACE= gdb --args /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
GNU gdb (Gentoo 7.10 vanilla) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://bugs.gentoo.org/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
(gdb) r
Starting program: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
warning: Cannot call inferior functions, Linux kernel PaX protection forbids return to non-executable pages!
[New LWP 20518]
error: Option 'version' given more than once.
^C
Program received signal SIGINT, Interrupt.
0x000003bd5507b9ad in ?? ()
(gdb) bt full
#0  0x000003bd5507b9ad in ?? ()
No symbol table info available.
#1  0x000000005d12c3a8 in ?? ()
No symbol table info available.
#2  0x0000000000000010 in ?? ()
No symbol table info available.
#3  0x000003bd5507b8d0 in ?? ()
No symbol table info available.
#4  0x000003bd53512d28 in ?? ()
No symbol table info available.
#5  0x0000000000000000 in ?? ()
No symbol table info available.
(gdb) quit
A debugging session is active.

    Inferior 1 [process 20514] will be killed.

Quit anyway? (y or n) EOF [assumed Y]

real    0m10.945s
user    0m1.257s
sys 0m0.860s
-----------
zazdxscf@gobaby 2015/10/26 01:15:16 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12318 20 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vrsmpl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : pemrs

-----------
zazdxscf@gobaby 2015/10/26 01:15:38 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12319 21 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ time RUST_BACKTRACE= gdb --args /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
GNU gdb (Gentoo 7.10 vanilla) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://bugs.gentoo.org/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
(gdb) r
Starting program: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
During startup program terminated with signal SIGSEGV, Segmentation fault.
(gdb) quit

real    0m3.143s
user    0m0.093s
sys 0m0.070s
-----------
zazdxscf@gobaby 2015/10/26 01:15:44 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12320 22 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vRsmpl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : pemRs

-----------
zazdxscf@gobaby 2015/10/26 01:15:50 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12321 23 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ time RUST_BACKTRACE= gdb --args /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
GNU gdb (Gentoo 7.10 vanilla) 7.10
Copyright (C) 2015 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.  Type "show copying"
and "show warranty" for details.
This GDB was configured as "x86_64-pc-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://bugs.gentoo.org/>.
Find the GDB manual and other documentation resources online at:
<http://www.gnu.org/software/gdb/documentation/>.
For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc...done.
(gdb) r
Starting program: /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc -VV
During startup program terminated with signal SIGSEGV, Segmentation fault.
(gdb) quit

real    0m5.499s
user    0m0.107s
sys 0m0.053s
-----------
zazdxscf@gobaby 2015/10/26 01:15:58 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12322 24 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vzl /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -----

-----------
zazdxscf@gobaby 2015/10/26 01:16:10 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12323 25 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vzle /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc

Package Name : elfix 0.9.1
Bug Reports  : http://bugs.gentoo.org/
Program Name : paxctl-ng
Description  : Get or set pax flags on an ELF object

Usage        : paxctl-ng -PpEeMmRrSs|-Z|-z [-L|-l] [-v] ELF
             : paxctl-ng -C|-c|-d [-v] ELF
             : paxctl-ng -F|-f [-v] ELF
             : paxctl-ng -v ELF
             : paxctl-ng -L|-l
             : paxctl-ng [-h]

Options      : -P enable PAGEEXEC   -p disable  PAGEEXEC
             : -E enable EMUTRAMP   -e disable  EMUTRAMP
             : -M enable MPROTECT   -m disable  MPROTECT
             : -R enable RANDMMAP   -r disable  RANDMMAP
             : -S enable SEGMEXEC   -s disable  SEGMEXEC
             : -Z all secure settings   -z all default settings
             : -L set only PT_PAX flags -l set only XATTR_PAX flags
             :
             : -C create XATTR_PAX with most secure setting
             : -c create XATTR_PAX all default settings
             : -d delete XATTR_PAX field
             : -F copy PT_PAX to XATTR_PAX
             : -f copy XATTR_PAX to PT_PAX
             : -L when given alone, EXIT_SUCCESS (PT_PAX is supported)
             : -l when given alone, EXIT_SUCCESS (XATTR_PAX is supported)
             : -v view the flags, along with any accompanying operation
             : -h print out this help

Note         :  If both enabling and disabling flags are set, the default - is used

-----------
zazdxscf@gobaby 2015/10/26 01:16:14 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12324 26 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vel /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e---

-----------
zazdxscf@gobaby 2015/10/26 01:16:19 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12325 27 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vc /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : -e---

-----------
zazdxscf@gobaby 2015/10/26 01:16:26 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12326 28 1  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ sudo paxctl-ng -vd /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc
/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage1/bin/rustc:
    open(O_RDWR) failed: cannot change PT_PAX flags
    PT_PAX    : -e---
    XATTR_PAX : not found

-----------
zazdxscf@gobaby 2015/10/26 01:16:28 -bash4.3.42 t:14 j:0 d:5 pp:9467 p:30555
!12327 29 0  4.2.3-hardened-r4-g45b4b78 #2 SMP Tue Oct 20 14:39:20 CEST 2015
/home/zazdxscf
/home/zazdxscf $ 
