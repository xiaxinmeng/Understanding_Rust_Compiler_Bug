diff
--- build-ok.txt	2017-11-02 01:19:54.152117397 +0100
+++ build-fail.txt	2017-11-02 01:18:13.342764166 +0100
@@ -4,12 +4,12 @@
 autotools-dev 20161112.1+nmu1
 binfmt-support 2.1.8-1
 bsdmainutils 9.0.14
-cargo 0.20.0-2
+cargo 0.22.0-1~exp1
 cmake 3.9.3-1
 cmake-data 3.9.3-1
-debhelper 10.10.4
+debhelper 10.10.5
 dh-autoreconf 14
-dh-strip-nondeterminism 0.039-1
+dh-strip-nondeterminism 0.040-1
 file 1:5.32-1
 gdb 7.12-6+b1
 gettext 0.19.8.1-4
@@ -33,17 +33,17 @@
 liberror-perl 0.17025-1
 libexpat1 2.2.3-1
 libffi-dev 3.2.1-6
-libfile-stripnondeterminism-perl 0.039-1
+libfile-stripnondeterminism-perl 0.040-1
 libgit2-26 0.26.0+dfsg.1-1.1
-libglib2.0-0 2.54.1-1
-libgssapi-krb5-2 1.15.2-1
+libglib2.0-0 2.54.2-1
+libgssapi-krb5-2 1.15.2-2
 libhttp-parser2.1 2.1-2+b1
 libicu57 57.1-8
 libjsoncpp1 1.7.4-3
-libk5crypto3 1.15.2-1
+libk5crypto3 1.15.2-2
 libkeyutils1 1.5.9-9.1
-libkrb5-3 1.15.2-1
-libkrb5support0 1.15.2-1
+libkrb5-3 1.15.2-2
+libkrb5support0 1.15.2-2
 libldap-2.4-2 2.4.45+dfsg-1
 libldap-common 2.4.45+dfsg-1
 libllvm4.0 1:4.0.1-8
@@ -53,7 +53,7 @@
 libmagic-mgc 1:5.32-1
 libmpdec2 2.4.2-1
 libncurses5 6.0+20170902-1
-libnghttp2-14 1.26.0-1
+libnghttp2-14 1.27.0-1
 libpcre2-8-0 10.22-3
 libpipeline1 1.4.2-1
 libpopt0 1.16-10+b2
@@ -75,8 +75,8 @@
 libssh2-1 1.8.0-1
 libssl1.0.2 1.0.2l-2
 libssl1.1 1.1.0f-5
-libstd-rust-1.20 1.20.0+dfsg1-3
-libstd-rust-dev 1.20.0+dfsg1-3
+libstd-rust-1.21 1.21.0+dfsg1-2
+libstd-rust-dev 1.21.0+dfsg1-2
 libtimedate-perl 2.3000-2
 libtinfo-dev 6.0+20170902-1
 libtool 2.4.6-2
@@ -86,6 +86,7 @@
 llvm-4.0-dev 1:4.0.1-8
 llvm-4.0-runtime 1:4.0.1-8
 llvm-4.0-tools 1:4.0.1-8
+lsb-base 9.20170808
 m4 1.4.18-1
 man-db 2.7.6.1-2
 mime-support 3.60
@@ -96,5 +97,5 @@
 python2.7-minimal 2.7.14-2
 python-minimal 2.7.14-1
 readline-common 7.0-3
-rustc 1.20.0+dfsg1-3
+rustc 1.21.0+dfsg1-2
 zlib1g-dev 1:1.2.8.dfsg-5
