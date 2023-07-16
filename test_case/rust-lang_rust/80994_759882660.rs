plain
Get:20 http://archive.ubuntu.com/ubuntu bionic-backports/universe Sources [5759 B]
Fetched 14.7 MB in 2s (6616 kB/s)
Reading package lists...
Reading package lists...
Picking 'llvm-defaults' as source package instead of 'clang'
Picking 'llvm-defaults' as source package instead of 'llvm'
Reading state information...
The following NEW packages will be installed:
The following NEW packages will be installed:
  autopoint bsdmainutils build-essential debhelper dh-autoreconf
  dh-strip-nondeterminism distro-info-data gettext gettext-base groff-base
  intltool-debian libarchive-zip-perl libbsd0 libcroco3
  libfile-stripnondeterminism-perl libpipeline1 libtimedate-perl lsb-release
  man-db po-debconf
Need to get 5669 kB of archives.
After this operation, 17.6 MB of additional disk space will be used.
Get:1 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libbsd0 amd64 0.8.7-1ubuntu0.1 [41.6 kB]
Get:2 http://archive.ubuntu.com/ubuntu bionic/main amd64 bsdmainutils amd64 11.1.2ubuntu1 [181 kB]
---
Unpacking libfile-stripnondeterminism-perl (0.040-1.1~build1) ...
Selecting previously unselected package libtimedate-perl.
Preparing to unpack .../13-libtimedate-perl_2.3000-2_all.deb ...
Unpacking libtimedate-perl (2.3000-2) ...
Selecting previously unselected package dh-strip-nondeterminism.
Preparing to unpack .../14-dh-strip-nondeterminism_0.040-1.1~build1_all.deb ...
Unpacking dh-strip-nondeterminism (0.040-1.1~build1) ...
Selecting previously unselected package libcroco3:amd64.
Preparing to unpack .../15-libcroco3_0.6.12-2_amd64.deb ...
Unpacking libcroco3:amd64 (0.6.12-2) ...
Preparing to unpack .../16-gettext_0.19.8.1-6ubuntu0.3_amd64.deb ...
Unpacking gettext (0.19.8.1-6ubuntu0.3) ...
Selecting previously unselected package intltool-debian.
Preparing to unpack .../17-intltool-debian_0.35.0+20060710.4_all.deb ...
---
Setting up gettext-base (0.19.8.1-6ubuntu0.3) ...
Setting up libpipeline1:amd64 (1.5.0-1) ...
Setting up lsb-release (9.20170808ubuntu1) ...
Setting up libbsd0:amd64 (0.8.7-1ubuntu0.1) ...
Setting up libcroco3:amd64 (0.6.12-2) ...
Setting up bsdmainutils (11.1.2ubuntu1) ...
update-alternatives: using /usr/bin/bsd-write to provide /usr/bin/write (write) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/write.1.gz because associated file /usr/share/man/man1/bsd-write.1.gz (of link group write) doesn't exist
update-alternatives: using /usr/bin/bsd-from to provide /usr/bin/from (from) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/from.1.gz because associated file /usr/share/man/man1/bsd-from.1.gz (of link group from) doesn't exist
Setting up autopoint (0.19.8.1-6ubuntu0.3) ...
Setting up libfile-stripnondeterminism-perl (0.040-1.1~build1) ...
Setting up gettext (0.19.8.1-6ubuntu0.3) ...
Setting up intltool-debian (0.35.0+20060710.4) ...
Setting up man-db (2.8.3-2ubuntu0.1) ...
debconf: (TERM is not set, so the dialog frontend is not usable.)
debconf: falling back to frontend: Readline
Building database of manual pages ...
Setting up po-debconf (1.0.20) ...
---
  linux-libc-dev-armel-cross pinentry-curses python-apt-common python3-apt
  python3-dbus python3-distutils python3-gi python3-lib2to3
  python3-software-properties python3.6-dev
Suggested packages:
  binutils-doc gcc-7-locales dbus-user-session libpam-systemd pinentry-gnome3
  tor g++-7-multilib-arm-linux-gnueabi gcc-7-doc libstdc++6-7-dbg-armel-cross
  libgomp1-dbg-armel-cross libitm1-dbg-armel-cross libatomic1-dbg-armel-cross
  libasan4-dbg-armel-cross liblsan0-dbg-armel-cross libtsan0-dbg-armel-cross
  libubsan0-dbg-armel-cross libcilkrts5-dbg-armel-cross
  libubsan0-dbg-armel-cross libcilkrts5-dbg-armel-cross
  libmpx2-dbg-armel-cross libquadmath0-dbg-armel-cross scdaemon isoquery
  gmp-doc libgmp10-doc libmpfr-doc pinentry-doc python3-apt-dbg python-apt-doc
  python-dbus-doc python3-dbus-dbg
Recommended packages:
  gnupg gcc-multilib dbus nodejs-doc unattended-upgrades
  binutils-arm-linux-gnueabi cpp-7-arm-linux-gnueabi dh-python dirmngr
  g++-7-arm-linux-gnueabi gcc-7-arm-linux-gnueabi gcc-7-arm-linux-gnueabi-base
  gcc-7-cross-base gcc-7-multilib gcc-8-cross-base gir1.2-glib-2.0 gpg
  gpg-agent gpgconf iso-codes lib32asan4 lib32atomic1 lib32cilkrts5
---
Setting up libx32asan4 (7.5.0-3ubuntu1~18.04) ...
Setting up python3-dbus (1.2.6-1) ...
Setting up libgcc1-armel-cross (1:8.4.0-1ubuntu1~18.04cross2) ...
Setting up libx32cilkrts5 (7.5.0-3ubuntu1~18.04) ...
Setting up nodejs (8.10.0~dfsg-2ubuntu0.4) ...
update-alternatives: using /usr/bin/nodejs to provide /usr/bin/js (js) in auto mode
update-alternatives: warning: skip creation of /usr/share/man/man1/js.1.gz because associated file /usr/share/man/man1/nodejs.1.gz (of link group js) doesn't exist
Setting up libx32ubsan0 (7.5.0-3ubuntu1~18.04) ...
Setting up gpgconf (2.2.4-1ubuntu1.3) ...
Setting up lib32asan4 (7.5.0-3ubuntu1~18.04) ...
Setting up lib32mpx2 (8.4.0-1ubuntu1~18.04) ...
---
Processing triggers for libc-bin (2.27-3ubuntu1.3) ...
Removing intermediate container 8ff4919e07aa
 ---> 6d9b35cbc77f
Step 6/47 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
 ---> Running in 8843d61accad
Warning: apt-key output should not be parsed (stdout is not a terminal)
Executing: /tmp/apt-key-gpghome.qc4GeqR7aA/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
gpg: key 74DA7924C5513486: public key "Igor Kozhukhov <igor@dilos.org>" imported
gpg:               imported: 1
Removing intermediate container 8843d61accad
 ---> 751190a6c411
Step 7/47 : RUN add-apt-repository -y 'deb http://apt.dilos.org/dilos dilos2 main'
---
+ ZIRCON=e9a26dbc70d631029f8ee9763103910b7e3a2fe1
+ mkdir -p zircon
+ pushd zircon
+ git init
Initialized empty Git repository in /tmp/zircon/.git/
+ git remote add origin https://github.com/rust-lang-nursery/mirror-google-fuchsia-zircon
+ git fetch --depth=1 origin e9a26dbc70d631029f8ee9763103910b7e3a2fe1
From https://github.com/rust-lang-nursery/mirror-google-fuchsia-zircon
 * branch            e9a26dbc70d631029f8ee9763103910b7e3a2fe1 -> FETCH_HEAD
+ git reset --hard FETCH_HEAD
HEAD is now at e9a26db [dev][usb-bus] Handle stalls while reading string descriptors
+ ./scripts/download-toolchain
Downloading https://fuchsia-build.storage.googleapis.com/magenta/toolchain/x86_64-elf/Linux-x86_64/509cd48809f5252ffc8ef0c72dbf208151750f66

                                                                           0.0%
#############                                                             18.9%
#############################################                             62.6%
#############################################                             62.6%
######################################################################## 100.0%
Unpacking /tmp/zircon/prebuilt/downloads/x86_64-elf-Linux-x86_64.tar.bz2
Downloading https://fuchsia-build.storage.googleapis.com/magenta/toolchain/aarch64-elf/Linux-x86_64/509cd48809f5252ffc8ef0c72dbf208151750f66

##############                                                            20.5%
#############################                                             41.2%
###############################################                           66.5%
###############################################                           66.5%
######################################################################## 100.0%
Unpacking /tmp/zircon/prebuilt/downloads/aarch64-elf-Linux-x86_64.tar.bz2
Downloading https://storage.googleapis.com/fuchsia/clang/linux-amd64/86d57720455e2d6aff2b48f9cea3ef77ab2af06a

                                                                           0.0%
###                                                                        5.3%
#######                                                                   10.6%
---
#################################################                         68.8%
###########################################################               82.1%
################################################################          90.0%
######################################################################## 100.0%
Unpacking /tmp/zircon/prebuilt/downloads/clang+llvm-x86_64-linux.zip
+ chmod -R a+rx prebuilt/downloads/clang+llvm-x86_64-linux
+ cp -a prebuilt/downloads/clang+llvm-x86_64-linux/. /usr/local
+ for arch in x86_64 aarch64
+ build x86_64
+ local arch=x86_64
+ case "${arch}" in
+ tgt=zircon-pc-x86-64
+ hide_output make -j16 zircon-pc-x86-64
Thu Jan 14 02:13:52 UTC 2021 - building ...
+ dst=/usr/local/x86_64-fuchsia
+ mkdir -p /usr/local/x86_64-fuchsia
+ mkdir -p /usr/local/x86_64-fuchsia
/tmp/build-fuchsia-toolchain.sh: line 22:    56 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ cp -a build-zircon-pc-x86-64/sysroot/include /usr/local/x86_64-fuchsia/
+ cp -a build-zircon-pc-x86-64/sysroot/lib /usr/local/x86_64-fuchsia/
+ for arch in x86_64 aarch64
+ build aarch64
+ local arch=aarch64
+ case "${arch}" in
+ tgt=zircon-qemu-arm64
+ hide_output make -j16 zircon-qemu-arm64
Thu Jan 14 02:14:25 UTC 2021 - building ...
+ dst=/usr/local/aarch64-fuchsia
+ mkdir -p /usr/local/aarch64-fuchsia
---
Hit:7 http://archive.ubuntu.com/ubuntu bionic-backports InRelease
Get:9 http://apt.dilos.org/dilos dilos2/main solaris-i386 Packages [1915 kB]
Fetched 1915 kB in 3s (592 kB/s)
Reading package lists...
++ apt-cache depends --recurse --no-replaces libc-dev:solaris-i386 libm-dev:solaris-i386 libpthread-dev:solaris-i386 libresolv-dev:solaris-i386 librt-dev:solaris-i386 libsocket-dev:solaris-i386 system-crt:solaris-i386 system-header:solaris-i386
++ grep '^\w'
+ apt-get download libc-dev:solaris-i386 libm-dev:solaris-i386 libpthread-dev:solaris-i386 libresolv-dev:solaris-i386 librt-dev:solaris-i386 libsocket-dev:solaris-i386 system-crt:solaris-i386 system-header:solaris-i386 libc:solaris-i386 libm:solaris-i386 libmvec:solaris-i386 libpthread:solaris-i386 libresolv:solaris-i386 librt:solaris-i386 libsocket:solaris-i386 libelf:solaris-i386 libld:solaris-i386 libmd:solaris-i386 libnsl:solaris-i386 libmp:solaris-i386
Get:1 http://apt.dilos.org/dilos dilos2/main solaris-i386 libc solaris-i386 2.0.2.48 [1638 kB]
Get:3 http://apt.dilos.org/dilos dilos2/main solaris-i386 libelf solaris-i386 2.0.2.48 [103 kB]
Get:4 http://apt.dilos.org/dilos dilos2/main solaris-i386 libld solaris-i386 2.0.2.48 [622 kB]
Get:5 http://apt.dilos.org/dilos dilos2/main solaris-i386 libm solaris-i386 2.0.2.48 [312 kB]
Get:6 http://apt.dilos.org/dilos dilos2/main solaris-i386 libm-dev solaris-i386 2.0.2.48 [230 kB]
---
Get:18 http://apt.dilos.org/dilos dilos2/main solaris-i386 libsocket-dev solaris-i386 2.0.2.48 [1580 B]
Get:19 http://apt.dilos.org/dilos dilos2/main solaris-i386 system-crt solaris-i386 2.0.2.48 [4372 B]
Get:20 http://apt.dilos.org/dilos dilos2/main solaris-i386 system-header solaris-i386 2.0.2.48 [2259 kB]
Fetched 6278 kB in 4s (1452 kB/s)
W: Download is performed unsandboxed as root as file '/tmp/solaris/libc_2.0.2.48_solaris-i386.deb' couldn't be accessed by user '_apt'. - pkgAcquire::Run (13: Permission denied)
+ dpkg -x libc-dev_2.0.2.48_solaris-i386.deb .
+ for deb in *$APT_ARCH.deb
+ dpkg -x libc_2.0.2.48_solaris-i386.deb .
+ for deb in *$APT_ARCH.deb
---
+ dpkg -x libsocket_2.0.2.48_solaris-i386.deb .
+ for deb in *$APT_ARCH.deb
+ dpkg -x system-crt_2.0.2.48_solaris-i386.deb .
+ for deb in *$APT_ARCH.deb
+ dpkg -x system-header_2.0.2.48_solaris-i386.deb .
+ rm usr/include/link.h
+ patch -p0
patching file usr/include/string.h
+ mkdir /usr/local/x86_64-sun-solaris2.10/usr
+ mv usr/include /usr/local/x86_64-sun-solaris2.10/usr/include
+ mv usr/lib/amd64/crt1.o usr/lib/amd64/crti.o usr/lib/amd64/crtn.o usr/lib/amd64/gcrt1.o usr/lib/amd64/ld.so.1 usr/lib/amd64/libc.so usr/lib/amd64/libc.so.1 usr/lib/amd64/libc_db.so usr/lib/amd64/libc_db.so.1 usr/lib/amd64/libelf.so.1 usr/lib/amd64/liblddbg.so.4 usr/lib/amd64/libm.so usr/lib/amd64/libm.so.1 usr/lib/amd64/libm.so.2 usr/lib/amd64/libmd.so.1 usr/lib/amd64/libmp.so.2 usr/lib/amd64/libmvec.so usr/lib/amd64/libmvec.so.1 usr/lib/amd64/libnsl.so.1 usr/lib/amd64/libposix4.so usr/lib/amd64/libposix4.so.1 usr/lib/amd64/libpthread.so usr/lib/amd64/libpthread.so.1 usr/lib/amd64/libresolv.so usr/lib/amd64/libresolv.so.2 usr/lib/amd64/librt.so usr/lib/amd64/librt.so.1 usr/lib/amd64/librtld.so.1 usr/lib/amd64/librtld_db.so usr/lib/amd64/librtld_db.so.1 usr/lib/amd64/libsocket.so usr/lib/amd64/libsocket.so.1 usr/lib/amd64/values-Xa.o usr/lib/amd64/values-Xc.o usr/lib/amd64/values-Xs.o usr/lib/amd64/values-Xt.o usr/lib/amd64/values-xpg4.o usr/lib/amd64/values-xpg6.o /usr/local/x86_64-sun-solaris2.10/lib
+ mv lib/amd64/ld.so.1 lib/amd64/libc.so lib/amd64/libc.so.1 lib/amd64/libc_db.so lib/amd64/libc_db.so.1 lib/amd64/libelf.so.1 lib/amd64/libld.so.4 lib/amd64/liblddbg.so.4 lib/amd64/libm.so lib/amd64/libm.so.1 lib/amd64/libm.so.2 lib/amd64/libmd.so.1 lib/amd64/libmp.so.2 lib/amd64/libmvec.so lib/amd64/libmvec.so.1 lib/amd64/libnsl.so.1 lib/amd64/libposix4.so lib/amd64/libposix4.so.1 lib/amd64/libpthread.so lib/amd64/libpthread.so.1 lib/amd64/libresolv.so lib/amd64/libresolv.so.2 lib/amd64/librt.so lib/amd64/librt.so.1 lib/amd64/librtld.so.1 lib/amd64/librtld_db.so lib/amd64/librtld_db.so.1 lib/amd64/libsocket.so lib/amd64/libsocket.so.1 /usr/local/x86_64-sun-solaris2.10/lib
+ ln -s usr/include /usr/local/x86_64-sun-solaris2.10/sys-include
+ ln -s usr/include /usr/local/x86_64-sun-solaris2.10/include
+ rm -rf solaris
+ mkdir gcc
+ cd gcc
+ curl https://ftp.gnu.org/gnu/gcc/gcc-6.4.0/gcc-6.4.0.tar.xz
---
100 72.6M  100 72.6M    0     0  7838k      0  0:00:09  0:00:09 --:--:-- 7834k
+ cd gcc-6.4.0
+ mkdir ../gcc-build
+ cd ../gcc-build
+ hide_output ../gcc-6.4.0/configure --enable-languages=c,c++ --target=x86_64-sun-solaris2.10 --with-gnu-as --with-gnu-ld --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrts --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
Thu Jan 14 02:16:18 UTC 2021 - building ...
Thu Jan 14 02:16:48 UTC 2021 - building ...
Thu Jan 14 02:17:18 UTC 2021 - building ...
Thu Jan 14 02:17:48 UTC 2021 - building ...
---
Hit:7 http://archive.ubuntu.com/ubuntu bionic-backports InRelease
Get:9 http://apt.dilos.org/dilos dilos2/main solaris-sparc Packages [1873 kB]
Fetched 1873 kB in 3s (587 kB/s)
Reading package lists...
++ apt-cache depends --recurse --no-replaces libc-dev:solaris-sparc libm-dev:solaris-sparc libpthread-dev:solaris-sparc libresolv-dev:solaris-sparc librt-dev:solaris-sparc libsocket-dev:solaris-sparc system-crt:solaris-sparc system-header:solaris-sparc
++ grep '^\w'
+ apt-get download libc-dev:solaris-sparc libm-dev:solaris-sparc libpthread-dev:solaris-sparc libresolv-dev:solaris-sparc librt-dev:solaris-sparc libsocket-dev:solaris-sparc system-crt:solaris-sparc system-header:solaris-sparc libc:solaris-sparc libm:solaris-sparc libmvec:solaris-sparc libpthread:solaris-sparc libresolv:solaris-sparc librt:solaris-sparc libsocket:solaris-sparc libelf:solaris-sparc libld:solaris-sparc libmd:solaris-sparc libnsl:solaris-sparc libmp:solaris-sparc
Get:1 http://apt.dilos.org/dilos dilos2/main solaris-sparc libc solaris-sparc 2.0.2.48 [1129 kB]
Get:3 http://apt.dilos.org/dilos dilos2/main solaris-sparc libelf solaris-sparc 2.0.2.48 [88.7 kB]
Get:4 http://apt.dilos.org/dilos dilos2/main solaris-sparc libld solaris-sparc 2.0.2.48 [544 kB]
Get:5 http://apt.dilos.org/dilos dilos2/main solaris-sparc libm solaris-sparc 2.0.2.48 [395 kB]
Get:6 http://apt.dilos.org/dilos dilos2/main solaris-sparc libm-dev solaris-sparc 2.0.2.48 [230 kB]
---
Get:18 http://apt.dilos.org/dilos dilos2/main solaris-sparc libsocket-dev solaris-sparc 2.0.2.48 [1584 B]
Get:19 http://apt.dilos.org/dilos dilos2/main solaris-sparc system-crt solaris-sparc 2.0.2.48 [3724 B]
Get:20 http://apt.dilos.org/dilos dilos2/main solaris-sparc system-header solaris-sparc 2.0.2.48 [2316 kB]
Fetched 5755 kB in 4s (1444 kB/s)
W: Download is performed unsandboxed as root as file '/tmp/solaris/libc_2.0.2.48_solaris-sparc.deb' couldn't be accessed by user '_apt'. - pkgAcquire::Run (13: Permission denied)
+ dpkg -x libc-dev_2.0.2.48_solaris-sparc.deb .
+ for deb in *$APT_ARCH.deb
+ dpkg -x libc_2.0.2.48_solaris-sparc.deb .
+ for deb in *$APT_ARCH.deb
---
+ dpkg -x libsocket_2.0.2.48_solaris-sparc.deb .
+ for deb in *$APT_ARCH.deb
+ dpkg -x system-crt_2.0.2.48_solaris-sparc.deb .
+ for deb in *$APT_ARCH.deb
+ dpkg -x system-header_2.0.2.48_solaris-sparc.deb .
+ rm usr/include/link.h
+ patch -p0
patching file usr/include/string.h
+ mkdir /usr/local/sparcv9-sun-solaris2.10/usr
+ mv usr/include /usr/local/sparcv9-sun-solaris2.10/usr/include
+ mv usr/lib/sparcv9/crt1.o usr/lib/sparcv9/crti.o usr/lib/sparcv9/crtn.o usr/lib/sparcv9/ld.so.1 usr/lib/sparcv9/libc.so usr/lib/sparcv9/libc.so.1 usr/lib/sparcv9/libc_db.so usr/lib/sparcv9/libc_db.so.1 usr/lib/sparcv9/libelf.so.1 usr/lib/sparcv9/liblddbg.so.4 usr/lib/sparcv9/libm.so usr/lib/sparcv9/libm.so.1 usr/lib/sparcv9/libm.so.2 usr/lib/sparcv9/libmd.so.1 usr/lib/sparcv9/libmp.so.2 usr/lib/sparcv9/libmvec.so usr/lib/sparcv9/libmvec.so.1 usr/lib/sparcv9/libnsl.so.1 usr/lib/sparcv9/libposix4.so usr/lib/sparcv9/libposix4.so.1 usr/lib/sparcv9/libpthread.so usr/lib/sparcv9/libpthread.so.1 usr/lib/sparcv9/libresolv.so usr/lib/sparcv9/libresolv.so.2 usr/lib/sparcv9/librt.so usr/lib/sparcv9/librt.so.1 usr/lib/sparcv9/librtld.so.1 usr/lib/sparcv9/librtld_db.so usr/lib/sparcv9/librtld_db.so.1 usr/lib/sparcv9/libsocket.so usr/lib/sparcv9/libsocket.so.1 usr/lib/sparcv9/values-Xa.o usr/lib/sparcv9/values-Xc.o usr/lib/sparcv9/values-Xs.o usr/lib/sparcv9/values-Xt.o usr/lib/sparcv9/values-xpg4.o usr/lib/sparcv9/values-xpg6.o /usr/local/sparcv9-sun-solaris2.10/lib
+ mv lib/sparcv9/ld.so.1 lib/sparcv9/libc.so lib/sparcv9/libc.so.1 lib/sparcv9/libc_db.so lib/sparcv9/libc_db.so.1 lib/sparcv9/libelf.so.1 lib/sparcv9/libld.so.4 lib/sparcv9/liblddbg.so.4 lib/sparcv9/libm.so lib/sparcv9/libm.so.1 lib/sparcv9/libm.so.2 lib/sparcv9/libmd.so.1 lib/sparcv9/libmp.so.2 lib/sparcv9/libmvec.so lib/sparcv9/libmvec.so.1 lib/sparcv9/libnsl.so.1 lib/sparcv9/libposix4.so lib/sparcv9/libposix4.so.1 lib/sparcv9/libpthread.so lib/sparcv9/libpthread.so.1 lib/sparcv9/libresolv.so lib/sparcv9/libresolv.so.2 lib/sparcv9/librt.so lib/sparcv9/librt.so.1 lib/sparcv9/librtld.so.1 lib/sparcv9/librtld_db.so lib/sparcv9/librtld_db.so.1 lib/sparcv9/libsocket.so lib/sparcv9/libsocket.so.1 /usr/local/sparcv9-sun-solaris2.10/lib
+ ln -s usr/include /usr/local/sparcv9-sun-solaris2.10/sys-include
+ ln -s usr/include /usr/local/sparcv9-sun-solaris2.10/include
+ rm -rf solaris
+ mkdir gcc
+ cd gcc
+ curl https://ftp.gnu.org/gnu/gcc/gcc-6.4.0/gcc-6.4.0.tar.xz
---
100 72.6M  100 72.6M    0     0  7658k      0  0:00:09  0:00:09 --:--:-- 8379k
+ cd gcc-6.4.0
+ mkdir ../gcc-build
+ cd ../gcc-build
+ hide_output ../gcc-6.4.0/configure --enable-languages=c,c++ --target=sparcv9-sun-solaris2.10 --with-gnu-as --with-gnu-ld --disable-multilib --disable-nls --disable-libgomp --disable-libquadmath --disable-libssp --disable-libvtv --disable-libcilkrts --disable-libada --disable-libsanitizer --disable-libquadmath-support --disable-lto
Thu Jan 14 02:20:55 UTC 2021 - building ...
Thu Jan 14 02:21:25 UTC 2021 - building ...
Thu Jan 14 02:21:55 UTC 2021 - building ...
Thu Jan 14 02:22:25 UTC 2021 - building ...
---
Step 23/47 : COPY host-x86_64/dist-various-2/build-wasi-toolchain.sh /tmp/
 ---> ced0c0370a42
Step 24/47 : RUN /tmp/build-wasi-toolchain.sh
 ---> Running in 241329fc637f
+ curl https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-11.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   243    0   243    0     0    290      0 --:--:-- --:--:-- --:--:--   289
100   243    0   243    0     0    290      0 --:--:-- --:--:-- --:--:--   289
xz: (stdin): File format not recognized
tar: Child returned status 1
tar: Error is not recoverable: exiting now
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon  488.4kB

Step 1/47 : FROM ubuntu:18.04
 ---> 2c047404e52d
---
 ---> Using cache
 ---> ced0c0370a42
Step 24/47 : RUN /tmp/build-wasi-toolchain.sh
 ---> Running in 76b0e0817106
+ curl https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-11.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   243    0   243    0     0    421      0 --:--:-- --:--:-- --:--:--   420
xz: (stdin): File format not recognized
tar: Child returned status 1
tar: Error is not recoverable: exiting now
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon  488.4kB

Step 1/47 : FROM ubuntu:18.04
 ---> 2c047404e52d
---
 ---> Using cache
 ---> ced0c0370a42
Step 24/47 : RUN /tmp/build-wasi-toolchain.sh
 ---> Running in e732561fd37f
+ curl https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-11.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   243    0   243    0     0    549      0 --:--:-- --:--:-- --:--:--   549
xz: (stdin): File format not recognized
tar: Child returned status 1
tar: Error is not recoverable: exiting now
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon  488.4kB

Step 1/47 : FROM ubuntu:18.04
 ---> 2c047404e52d
---
 ---> Using cache
 ---> ced0c0370a42
Step 24/47 : RUN /tmp/build-wasi-toolchain.sh
 ---> Running in ef320ce85b58
+ curl https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-11.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   243    0   243    0     0    649      0 --:--:-- --:--:-- --:--:--   649
xz: (stdin): File format not recognized
tar: Child returned status 1
tar: Error is not recoverable: exiting now
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
Sending build context to Docker daemon  488.4kB

Step 1/47 : FROM ubuntu:18.04
 ---> 2c047404e52d
---
Step 23/47 : COPY host-x86_64/dist-various-2/build-wasi-toolchain.sh /tmp/
 ---> Using cache
 ---> ced0c0370a42
Step 24/47 : RUN /tmp/build-wasi-toolchain.sh
 ---> Running in b479074d50ec
+ curl https://ci-mirrors.rust-lang.org/rustc/clang%2Bllvm-11.0.0-x86_64-linux-gnu-ubuntu-16.04.tar.xz
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   243    0   243    0     0    662      0 --:--:-- --:--:-- --:--:--   660
xz: (stdin): File format not recognized
tar: Child returned status 1
tar: Error is not recoverable: exiting now
The command '/bin/sh -c /tmp/build-wasi-toolchain.sh' returned a non-zero code: 2
##[error]Process completed with exit code 1.
Post job cleanup.
