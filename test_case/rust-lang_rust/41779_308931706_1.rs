
cd /home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/build-amd64 && exec /usr/bin/env -i RUSTFLAGS="-L /home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/modgcc-libs" CC=cc PYTHONUSERBASE=/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0 LIBTOOL="/usr/local/bin/libtool"  PATH='/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin:/usr/bin:/bin:/usr/sbin:/sbin:/usr/local/bin:/usr/X11R6/bin' PREFIX='/usr/local'  LOCALBASE='/usr/local' X11BASE='/usr/X11R6'  CFLAGS='-O2 -pipe'  TRUEPREFIX='/usr/local' DESTDIR=''  HOME='/rust-1.18.0_writes_to_HOME' COMPILER_VERSION=gcc4  PICFLAG="-fpic" ASPICFLAG=  BINGRP=bin BINOWN=root BINMODE=755 NONBINMODE=644  DIRMODE=755  INSTALL_COPY=-c INSTALL_STRIP=-s  MANGRP=bin MANOWN=root
MANMODE=644 BSD_INSTALL_PROGRAM="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -c -s  -m 755"  BSD_INSTALL_SCRIPT="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -c  -m 755"  BSD_INSTALL_DATA="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -c  -m 644"  BSD_INSTALL_MAN="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -c  -m 644"  BSD_INSTALL_PROGRAM_DIR="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -d  -m 755"  BSD_INSTALL_SCRIPT_DIR="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -d  -m 755"  BSD_INSTALL_DATA_DIR="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -d  -m 755"  BSD_INSTALL_MAN_DIR="/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/bin/install -d  -m 755"  /usr/local/bin/python2.7 /home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/rustc-1.18.0-src/src/bootstrap/bootstrap.py dist --verbose --jobs=1
   Compiling libc v0.2.21
   Compiling rustc-serialize v0.3.23
   Compiling getopts v0.2.14
   Compiling gcc v0.3.45
   Compiling num_cpus v0.2.13
   Compiling filetime v0.1.10
   Compiling build_helper v0.1.0 (file:///home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/rustc-1.18.0-src/src/build_helper)
   Compiling cmake v0.1.22
   Compiling toml v0.1.30
   Compiling bootstrap v0.0.0 (file:///home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/rustc-1.18.0-src/src/bootstrap)
    Finished dev [unoptimized] target(s) in 39.19 secs


failed to execute command: "/data/semarie/repos/openbsd/ports/pobj/rust-1.18.0/build-amd64/build/x86_64-unknown-openbsd/stage0/bin/cargo" "metadata" "--manifest-path" "/home/semarie/repos/openbsd/ports/pobj/rust-1.18.0/rustc-1.18.0-src/src/libst/Cargo.toml"
error: No such file or directory (os error 2)


Build completed unsuccessfully in 0:00:40
