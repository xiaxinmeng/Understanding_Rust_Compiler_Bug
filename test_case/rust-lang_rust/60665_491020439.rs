plain
[00:00:28] Status: Downloaded newer image for ubuntu:19.04
[00:00:28]  ---> f723e3b6f1bd
[00:00:28] Step 2/6 : RUN apt-get update && apt-get install -y --no-install-recommends   g++   make   file   curl   ca-certificates   python2.7   git   cmake   sudo   gdb   xz-utils
[00:01:00]  ---> Running in 60aae73375ef
[00:01:01] Get:1 http://security.ubuntu.com/ubuntu disco-security InRelease [97.5 kB]
[00:01:01] Get:2 http://archive.ubuntu.com/ubuntu disco InRelease [257 kB]
[00:01:01] Get:3 http://security.ubuntu.com/ubuntu disco-security/universe amd64 Packages [16.5 kB]
[00:01:01] Get:4 http://archive.ubuntu.com/ubuntu disco-updates InRelease [97.5 kB]
[00:01:01] Get:5 http://security.ubuntu.com/ubuntu disco-security/main amd64 Packages [37.7 kB]
[00:01:01] Get:6 http://archive.ubuntu.com/ubuntu disco-backports InRelease [88.8 kB]
[00:01:01] Get:7 http://archive.ubuntu.com/ubuntu disco/universe amd64 Packages [12.0 MB]
[00:01:02] Get:8 http://archive.ubuntu.com/ubuntu disco/main amd64 Packages [1316 kB]
[00:01:02] Get:9 http://archive.ubuntu.com/ubuntu disco/restricted amd64 Packages [20.2 kB]
[00:01:02] Get:10 http://archive.ubuntu.com/ubuntu disco/multiverse amd64 Packages [193 kB]
[00:01:02] Get:11 http://archive.ubuntu.com/ubuntu disco-updates/main amd64 Packages [48.9 kB]
[00:01:02] Get:12 http://archive.ubuntu.com/ubuntu disco-updates/universe amd64 Packages [24.2 kB]
[00:01:02] Get:13 http://archive.ubuntu.com/ubuntu disco-backports/universe amd64 Packages [3042 B]
[00:01:04] Reading package lists...
[00:01:05] Reading package lists...
[00:01:05] Building dependency tree...
[00:01:05] Reading state information...
---
[00:01:06]   libubsan1-dbg libmpx2-dbg libquadmath0-dbg gdb-doc gettext-base
[00:01:06]   git-daemon-run | git-daemon-sysvinit git-doc git-el git-email git-gui gitk
[00:01:06]   gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc gdbm-l10n krb5-doc
[00:01:06]   krb5-user libstdc++-8-doc make-doc perl-doc libterm-readline-gnu-perl
[00:01:06]   | libterm-readline-perl-perl libb-debug-perl liblocale-codes-perl
[00:01:06] Recommended packages:
[00:01:06]   libc-dbg gdbserver patch less ssh-client manpages manpages-dev
[00:01:06]   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales publicsuffix
[00:01:06]   libsasl2-modules netbase
---
[00:01:06]   sudo xz-utils
[00:01:06] 0 upgraded, 98 newly installed, 0 to remove and 0 not upgraded.
[00:01:06] Need to get 86.5 MB of archives.
[00:01:06] After this operation, 381 MB of additional disk space will be used.
[00:01:06] Get:1 http://archive.ubuntu.com/ubuntu disco/main amd64 perl-modules-5.28 all 5.28.1-6 [2818 kB]
[00:01:07] Get:2 http://archive.ubuntu.com/ubuntu disco/main amd64 libgdbm6 amd64 1.18.1-4 [27.4 kB]
[00:01:07] Get:3 http://archive.ubuntu.com/ubuntu disco/main amd64 libgdbm-compat4 amd64 1.18.1-4 [6152 B]
[00:01:07] Get:4 http://archive.ubuntu.com/ubuntu disco/main amd64 libperl5.28 amd64 5.28.1-6 [3827 kB]
[00:01:07] Get:5 http://archive.ubuntu.com/ubuntu disco/main amd64 perl amd64 5.28.1-6 [204 kB]
[00:01:07] Get:6 http://archive.ubuntu.com/ubuntu disco/main amd64 libpython2.7-minimal amd64 2.7.16-2 [335 kB]
[00:01:07] Get:7 http://archive.ubuntu.com/ubuntu disco/main amd64 python2.7-minimal amd64 2.7.16-2 [1310 kB]
[00:01:07] Get:8 http://archive.ubuntu.com/ubuntu disco/main amd64 libssl1.1 amd64 1.1.1b-1ubuntu2 [1301 kB]
[00:01:07] Get:9 http://archive.ubuntu.com/ubuntu disco/main amd64 openssl amd64 1.1.1b-1ubuntu2 [621 kB]
[00:01:07] Get:10 http://archive.ubuntu.com/ubuntu disco/main amd64 ca-certificates all 20190110 [146 kB]
[00:01:07] Get:11 http://archive.ubuntu.com/ubuntu disco/main amd64 libmagic-mgc amd64 1:5.35-4 [200 kB]
[00:01:07] Get:12 http://archive.ubuntu.com/ubuntu disco/main amd64 libmagic1 amd64 1:5.35-4 [74.0 kB]
[00:01:07] Get:13 http://archive.ubuntu.com/ubuntu disco/main amd64 file amd64 1:5.35-4 [22.9 kB]
[00:01:07] Get:14 http://archive.ubuntu.com/ubuntu disco/main amd64 libelf1 amd64 0.176-1 [44.2 kB]
[00:01:07] Get:15 http://archive.ubuntu.com/ubuntu disco/main amd64 libexpat1 amd64 2.2.6-1 [87.2 kB]
[00:01:07] Get:16 http://archive.ubuntu.com/ubuntu disco/main amd64 libglib2.0-0 amd64 2.60.0-1 [1241 kB]
[00:01:07] Get:17 http://archive.ubuntu.com/ubuntu disco/main amd64 libicu63 amd64 63.1-6 [8287 kB]
[00:01:08] Get:18 http://archive.ubuntu.com/ubuntu disco/main amd64 libmpdec2 amd64 2.4.2-2 [84.1 kB]
[00:01:08] Get:19 http://archive.ubuntu.com/ubuntu disco/main amd64 libpython3.7-minimal amd64 3.7.3-2 [544 kB]
[00:01:08] Get:20 http://archive.ubuntu.com/ubuntu disco/main amd64 mime-support all 3.60ubuntu1 [30.1 kB]
[00:01:08] Get:21 http://archive.ubuntu.com/ubuntu disco/main amd64 readline-common all 8.0-1 [53.4 kB]
[00:01:08] Get:22 http://archive.ubuntu.com/ubuntu disco/main amd64 libreadline8 amd64 8.0-1 [130 kB]
[00:01:08] Get:23 http://archive.ubuntu.com/ubuntu disco/main amd64 libsqlite3-0 amd64 3.27.2-2 [533 kB]
[00:01:08] Get:24 http://archive.ubuntu.com/ubuntu disco/main amd64 libpython3.7-stdlib amd64 3.7.3-2 [1733 kB]
[00:01:08] Get:25 http://archive.ubuntu.com/ubuntu disco/main amd64 libxml2 amd64 2.9.4+dfsg1-7ubuntu3 [625 kB]
[00:01:08] Get:26 http://archive.ubuntu.com/ubuntu disco/main amd64 sudo amd64 1.8.27-1ubuntu1 [502 kB]
[00:01:08] Get:27 http://archive.ubuntu.com/ubuntu disco/main amd64 xz-utils amd64 5.2.4-1 [82.5 kB]
[00:01:08] Get:28 http://archive.ubuntu.com/ubuntu disco/main amd64 libkrb5support0 amd64 1.17-1 [33.3 kB]
[00:01:08] Get:29 http://archive.ubuntu.com/ubuntu disco/main amd64 libk5crypto3 amd64 1.17-1 [85.8 kB]
[00:01:08] Get:30 http://archive.ubuntu.com/ubuntu disco/main amd64 libkeyutils1 amd64 1.6-6 [10.1 kB]
[00:01:08] Get:31 http://archive.ubuntu.com/ubuntu disco/main amd64 libkrb5-3 amd64 1.17-1 [329 kB]
[00:01:08] Get:32 http://archive.ubuntu.com/ubuntu disco/main amd64 libgssapi-krb5-2 amd64 1.17-1 [123 kB]
[00:01:08] Get:33 http://archive.ubuntu.com/ubuntu disco/main amd64 libpcre2-8-0 amd64 10.32-5 [180 kB]
[00:01:08] Get:34 http://archive.ubuntu.com/ubuntu disco/main amd64 libpsl5 amd64 0.20.2-2 [50.0 kB]
[00:01:08] Get:35 http://archive.ubuntu.com/ubuntu disco/main amd64 binutils-common amd64 2.32-7ubuntu4 [200 kB]
[00:01:08] Get:36 http://archive.ubuntu.com/ubuntu disco/main amd64 libbinutils amd64 2.32-7ubuntu4 [468 kB]
[00:01:08] Get:37 http://archive.ubuntu.com/ubuntu disco/main amd64 binutils-x86-64-linux-gnu amd64 2.32-7ubuntu4 [1852 kB]
[00:01:08] Get:38 http://archive.ubuntu.com/ubuntu disco/main amd64 binutils amd64 2.32-7ubuntu4 [3384 B]
[00:01:08] Get:39 http://archive.ubuntu.com/ubuntu disco/main amd64 cmake-data all 3.13.4-1 [1458 kB]
[00:01:08] Get:40 http://archive.ubuntu.com/ubuntu disco/main amd64 libarchive13 amd64 3.3.3-4 [305 kB]
[00:01:08] Get:41 http://archive.ubuntu.com/ubuntu disco/main amd64 libroken18-heimdal amd64 7.5.0+dfsg-2.1 [41.4 kB]
[00:01:08] Get:42 http://archive.ubuntu.com/ubuntu disco/main amd64 libasn1-8-heimdal amd64 7.5.0+dfsg-2.1 [182 kB]
[00:01:08] Get:43 http://archive.ubuntu.com/ubuntu disco/main amd64 libheimbase1-heimdal amd64 7.5.0+dfsg-2.1 [29.4 kB]
[00:01:08] Get:44 http://archive.ubuntu.com/ubuntu disco/main amd64 libhcrypto4-heimdal amd64 7.5.0+dfsg-2.1 [86.7 kB]
[00:01:08] Get:45 http://archive.ubuntu.com/ubuntu disco/main amd64 libwind0-heimdal amd64 7.5.0+dfsg-2.1 [47.8 kB]
[00:01:08] Get:46 http://archive.ubuntu.com/ubuntu disco/main amd64 libhx509-5-heimdal amd64 7.5.0+dfsg-2.1 [107 kB]
[00:01:08] Get:47 http://archive.ubuntu.com/ubuntu disco/main amd64 libkrb5-26-heimdal amd64 7.5.0+dfsg-2.1 [206 kB]
[00:01:08] Get:48 http://archive.ubuntu.com/ubuntu disco/main amd64 libheimntlm0-heimdal amd64 7.5.0+dfsg-2.1 [14.8 kB]
[00:01:08] Get:49 http://archive.ubuntu.com/ubuntu disco/main amd64 libgssapi3-heimdal amd64 7.5.0+dfsg-2.1 [96.4 kB]
[00:01:08] Get:50 http://archive.ubuntu.com/ubuntu disco/main amd64 libsasl2-modules-db amd64 2.1.27+dfsg-1 [14.9 kB]
[00:01:08] Get:51 http://archive.ubuntu.com/ubuntu disco/main amd64 libsasl2-2 amd64 2.1.27+dfsg-1 [49.4 kB]
[00:01:08] Get:52 http://archive.ubuntu.com/ubuntu disco/main amd64 libldap-common all 2.4.47+dfsg-3ubuntu2 [16.6 kB]
[00:01:08] Get:53 http://archive.ubuntu.com/ubuntu disco/main amd64 libldap-2.4-2 amd64 2.4.47+dfsg-3ubuntu2 [155 kB]
[00:01:08] Get:54 http://archive.ubuntu.com/ubuntu disco/main amd64 libnghttp2-14 amd64 1.36.0-2 [79.0 kB]
[00:01:08] Get:55 http://archive.ubuntu.com/ubuntu disco/main amd64 librtmp1 amd64 2.4+20151223.gitfa8646d.1-2 [54.5 kB]
[00:01:08] Get:56 http://archive.ubuntu.com/ubuntu disco/main amd64 libssh-4 amd64 0.8.6-3 [185 kB]
[00:01:08] Get:57 http://archive.ubuntu.com/ubuntu disco/main amd64 libcurl4 amd64 7.64.0-2ubuntu1 [232 kB]
[00:01:09] Get:58 http://archive.ubuntu.com/ubuntu disco/main amd64 libjsoncpp1 amd64 1.7.4-3 [73.6 kB]
[00:01:09] Get:59 http://archive.ubuntu.com/ubuntu disco/main amd64 librhash0 amd64 1.3.8-1 [113 kB]
[00:01:09] Get:60 http://archive.ubuntu.com/ubuntu disco/main amd64 libuv1 amd64 1.24.1-1 [69.5 kB]
[00:01:09] Get:61 http://archive.ubuntu.com/ubuntu disco/main amd64 cmake amd64 3.13.4-1 [3471 kB]
[00:01:09] Get:62 http://archive.ubuntu.com/ubuntu disco/main amd64 gcc-8-base amd64 8.3.0-6ubuntu1 [18.6 kB]
[00:01:09] Get:63 http://archive.ubuntu.com/ubuntu disco/main amd64 libisl19 amd64 0.20-2 [565 kB]
[00:01:09] Get:64 http://archive.ubuntu.com/ubuntu disco/main amd64 libmpfr6 amd64 4.0.2-1 [240 kB]
[00:01:09] Get:65 http://archive.ubuntu.com/ubuntu disco/main amd64 libmpc3 amd64 1.1.0-1 [40.8 kB]
[00:01:09] Get:66 http://archive.ubuntu.com/ubuntu disco/main amd64 cpp-8 amd64 8.3.0-6ubuntu1 [8935 kB]
[00:01:09] Get:67 http://archive.ubuntu.com/ubuntu disco/main amd64 cpp amd64 4:8.3.0-1ubuntu3 [27.6 kB]
[00:01:09] Get:68 http://archive.ubuntu.com/ubuntu disco/main amd64 curl amd64 7.64.0-2ubuntu1 [166 kB]
[00:01:09] Get:69 http://archive.ubuntu.com/ubuntu disco/main amd64 libcc1-0 amd64 9-20190402-1ubuntu1 [48.5 kB]
[00:01:09] Get:70 http://archive.ubuntu.com/ubuntu disco/main amd64 libgomp1 amd64 9-20190402-1ubuntu1 [88.9 kB]
[00:01:09] Get:71 http://archive.ubuntu.com/ubuntu disco/main amd64 libitm1 amd64 9-20190402-1ubuntu1 [27.9 kB]
[00:01:09] Get:72 http://archive.ubuntu.com/ubuntu disco/main amd64 libatomic1 amd64 9-20190402-1ubuntu1 [9216 B]
[00:01:09] Get:73 http://archive.ubuntu.com/ubuntu disco/main amd64 libasan5 amd64 9-20190402-1ubuntu1 [394 kB]
[00:01:09] Get:74 http://archive.ubuntu.com/ubuntu disco/main amd64 liblsan0 amd64 9-20190402-1ubuntu1 [139 kB]
[00:01:09] Get:75 http://archive.ubuntu.com/ubuntu disco/main amd64 libtsan0 amd64 9-20190402-1ubuntu1 [300 kB]
[00:01:09] Get:76 http://archive.ubuntu.com/ubuntu disco/main amd64 libubsan1 amd64 9-20190402-1ubuntu1 [130 kB]
[00:01:09] Get:77 http://archive.ubuntu.com/ubuntu disco/main amd64 libmpx2 amd64 8.3.0-6ubuntu1 [11.6 kB]
[00:01:09] Get:78 http://archive.ubuntu.com/ubuntu disco/main amd64 libquadmath0 amd64 9-20190402-1ubuntu1 [146 kB]
[00:01:09] Get:79 http://archive.ubuntu.com/ubuntu disco/main amd64 libgcc-8-dev amd64 8.3.0-6ubuntu1 [2305 kB]
[00:01:09] Get:80 http://archive.ubuntu.com/ubuntu disco/main amd64 gcc-8 amd64 8.3.0-6ubuntu1 [9802 kB]
[00:01:10] Get:81 http://archive.ubuntu.com/ubuntu disco/main amd64 gcc amd64 4:8.3.0-1ubuntu3 [5212 B]
[00:01:10] Get:82 http://archive.ubuntu.com/ubuntu disco/main amd64 libc-dev-bin amd64 2.29-0ubuntu2 [71.2 kB]
[00:01:10] Get:83 http://archive.ubuntu.com/ubuntu disco/main amd64 linux-libc-dev amd64 5.0.0-13.14 [1051 kB]
[00:01:10] Get:84 http://archive.ubuntu.com/ubuntu disco/main amd64 libc6-dev amd64 2.29-0ubuntu2 [2541 kB]
[00:01:10] Get:85 http://archive.ubuntu.com/ubuntu disco/main amd64 libstdc++-8-dev amd64 8.3.0-6ubuntu1 [1542 kB]
[00:01:10] Get:86 http://archive.ubuntu.com/ubuntu disco/main amd64 g++-8 amd64 8.3.0-6ubuntu1 [10.1 MB]
[00:01:10] Get:87 http://archive.ubuntu.com/ubuntu disco/main amd64 g++ amd64 4:8.3.0-1ubuntu3 [1600 B]
[00:01:10] Get:88 http://archive.ubuntu.com/ubuntu disco/main amd64 libdw1 amd64 0.176-1 [219 kB]
[00:01:10] Get:89 http://archive.ubuntu.com/ubuntu disco/main amd64 libbabeltrace1 amd64 1.5.6-2 [154 kB]
[00:01:10] Get:90 http://archive.ubuntu.com/ubuntu disco/main amd64 libpython3.7 amd64 3.7.3-2 [1497 kB]
[00:01:10] Get:91 http://archive.ubuntu.com/ubuntu disco/main amd64 gdb amd64 8.2.91.20190405-0ubuntu3 [3114 kB]
[00:01:11] Get:92 http://archive.ubuntu.com/ubuntu disco/main amd64 libcurl3-gnutls amd64 7.64.0-2ubuntu1 [230 kB]
[00:01:11] Get:93 http://archive.ubuntu.com/ubuntu disco/main amd64 liberror-perl all 0.17027-2 [26.6 kB]
[00:01:11] Get:94 http://archive.ubuntu.com/ubuntu disco/main amd64 git-man all 1:2.20.1-2ubuntu1 [835 kB]
[00:01:11] Get:95 http://archive.ubuntu.com/ubuntu disco/main amd64 git amd64 1:2.20.1-2ubuntu1 [4107 kB]
[00:01:11] Get:96 http://archive.ubuntu.com/ubuntu disco/main amd64 libpython2.7-stdlib amd64 2.7.16-2 [1909 kB]
[00:01:11] Get:97 http://archive.ubuntu.com/ubuntu disco/main amd64 make amd64 4.2.1-1.2 [162 kB]
[00:01:11] Get:98 http://archive.ubuntu.com/ubuntu disco/main amd64 python2.7 amd64 2.7.16-2 [244 kB]
[00:01:11] Fetched 86.5 MB in 5s (18.3 MB/s)
[00:01:11] Selecting previously unselected package perl-modules-5.28.
[00:01:11] (Reading database ... 
(Reading database ... 5%
---
[02:57:43]    Compiling parking_lot v0.7.1
[02:57:44] [RUSTC-TIMING] parking_lot test:false 1.371
[02:57:45] [RUSTC-TIMING] pulldown_cmark test:false 13.466
[02:57:45]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
The job exceeded the maximum time limit for jobs, and has been terminated.
