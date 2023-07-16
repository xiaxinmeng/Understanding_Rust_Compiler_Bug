plain
2019-09-26T02:34:02.7830272Z  ---> Running in 5c3dc4882af0
2019-09-26T02:34:04.5868175Z Removing intermediate container 5c3dc4882af0
2019-09-26T02:34:04.5869958Z  ---> acfddc30ecfb
2019-09-26T02:34:04.5871686Z Step 6/41 : RUN yum upgrade -y && yum install -y       curl       bzip2       gcc       gcc-c++       make       glibc-devel       perl       zlib-devel       file       xz       which       pkgconfig       wget       autoconf       gettext
2019-09-26T02:34:04.7693934Z  ---> Running in be382e7fbe42
2019-09-26T02:34:08.6957640Z Reducing CentOS-5 - libselinux to included packages only
2019-09-26T02:34:08.6982441Z Setting up Upgrade Process
2019-09-26T02:34:09.2201122Z Resolving Dependencies
2019-09-26T02:34:09.2205700Z --> Running transaction check
2019-09-26T02:34:09.2205700Z --> Running transaction check
2019-09-26T02:34:09.2252436Z ---> Package bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-09-26T02:34:09.2354143Z ---> Package bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-09-26T02:34:09.2443935Z ---> Package nspr.x86_64 0:4.11.0-1.el5_11 set to be updated
2019-09-26T02:34:09.2527292Z ---> Package nss.x86_64 0:3.21.3-2.el5_11 set to be updated
2019-09-26T02:34:09.2617122Z ---> Package openssl.x86_64 0:0.9.8e-40.el5_11 set to be updated
2019-09-26T02:34:09.2906771Z ---> Package tzdata.x86_64 0:2017b-1.el5 set to be updated
2019-09-26T02:34:09.4056279Z 
2019-09-26T02:34:09.4056688Z Dependencies Resolved
2019-09-26T02:34:09.4138021Z 
2019-09-26T02:34:09.4138843Z ================================================================================
2019-09-26T02:34:09.4138843Z ================================================================================
2019-09-26T02:34:09.4138951Z  Package         Arch        Version                         Repository    Size
2019-09-26T02:34:09.4139096Z ================================================================================
2019-09-26T02:34:09.4139777Z Updating:
2019-09-26T02:34:09.4140613Z  bind-libs       x86_64      30:9.3.6-25.P1.el5_11.12        updates      902 k
2019-09-26T02:34:09.4140979Z  bind-utils      x86_64      30:9.3.6-25.P1.el5_11.12        updates      181 k
2019-09-26T02:34:09.4141757Z  nspr            x86_64      4.11.0-1.el5_11                 updates      123 k
2019-09-26T02:34:09.4142457Z  nss             x86_64      3.21.3-2.el5_11                 updates      1.3 M
2019-09-26T02:34:09.4142781Z  openssl         x86_64      0.9.8e-40.el5_11                updates      1.7 M
2019-09-26T02:34:09.4143648Z  tzdata          x86_64      2017b-1.el5                     updates      757 k
2019-09-26T02:34:09.4144084Z Transaction Summary
2019-09-26T02:34:09.4144185Z ================================================================================
2019-09-26T02:34:09.4144324Z Install       0 Package(s)
2019-09-26T02:34:09.4144610Z Upgrade       6 Package(s)
2019-09-26T02:34:09.4144610Z Upgrade       6 Package(s)
2019-09-26T02:34:09.4144652Z 
2019-09-26T02:34:09.4144706Z Total download size: 4.9 M
2019-09-26T02:34:09.4144811Z Downloading Packages:
2019-09-26T02:34:09.6083203Z --------------------------------------------------------------------------------
2019-09-26T02:34:09.6085438Z Total                                            25 MB/s | 4.9 MB     00:00     
2019-09-26T02:34:09.6093905Z warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
2019-09-26T02:34:09.6143221Z Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
2019-09-26T02:34:09.7745862Z Running rpm_check_debug
2019-09-26T02:34:09.9072384Z Finished Transaction Test
2019-09-26T02:34:09.9072591Z Transaction Test Succeeded
2019-09-26T02:34:09.9597036Z Running Transaction
2019-09-26T02:34:10.1102445Z 
---
2019-09-26T02:34:13.3298861Z   Updating       : bind-utils                                              5/12 
2019-09-26T02:34:13.3299138Z 
2019-09-26T02:34:14.0553663Z   Updating       : tzdata                                                  6/12 
2019-09-26T02:34:14.0553886Z 
2019-09-26T02:34:14.1302425Z   Cleanup        : bind-utils                                              7/12 
2019-09-26T02:34:14.1302588Z 
2019-09-26T02:34:14.7959205Z   Cleanup        : nss                                                     8/12 
2019-09-26T02:34:15.4482863Z   Cleanup        : bind-libs                                               9/12 
2019-09-26T02:34:15.4482999Z 
2019-09-26T02:34:16.1223383Z   Cleanup        : openssl                                                10/12 
2019-09-26T02:34:16.1225984Z 
2019-09-26T02:34:16.1225984Z 
2019-09-26T02:34:16.7703319Z   Cleanup        : nspr                                                   11/12 
2019-09-26T02:34:16.8888560Z   Cleanup        : tzdata                                                 12/12 
2019-09-26T02:34:16.8888687Z 
2019-09-26T02:34:16.8888781Z Updated:
2019-09-26T02:34:16.8888781Z Updated:
2019-09-26T02:34:16.8889703Z   bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12                                     
2019-09-26T02:34:16.8890186Z   bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12                                    
2019-09-26T02:34:16.8890589Z   nspr.x86_64 0:4.11.0-1.el5_11                                                 
2019-09-26T02:34:16.8890890Z   nss.x86_64 0:3.21.3-2.el5_11                                                  
2019-09-26T02:34:16.8893351Z   openssl.x86_64 0:0.9.8e-40.el5_11                                             
2019-09-26T02:34:16.8954854Z   tzdata.x86_64 0:2017b-1.el5                                                   
2019-09-26T02:34:16.9003542Z Complete!
2019-09-26T02:34:16.9003542Z Complete!
2019-09-26T02:34:17.0790364Z Reducing CentOS-5 - libselinux to included packages only
2019-09-26T02:34:17.0804247Z Setting up Install Process
2019-09-26T02:34:18.0564604Z Resolving Dependencies
2019-09-26T02:34:18.0571098Z --> Running transaction check
2019-09-26T02:34:18.0571098Z --> Running transaction check
2019-09-26T02:34:18.0571387Z ---> Package autoconf.noarch 0:2.59-12 set to be updated
2019-09-26T02:34:18.0979343Z --> Processing Dependency: imake for package: autoconf
2019-09-26T02:34:18.1003190Z --> Processing Dependency: m4 for package: autoconf
2019-09-26T02:34:18.1026952Z ---> Package bzip2.x86_64 0:1.0.3-6.el5_5 set to be updated
2019-09-26T02:34:18.1150422Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be updated
2019-09-26T02:34:18.1352372Z --> Processing Dependency: libc.so.6(GLIBC_2.4) for package: curl
2019-09-26T02:34:18.1530656Z --> Processing Dependency: libgssapi_krb5.so.2 for package: curl
2019-09-26T02:34:18.1570770Z --> Processing Dependency: libdl.so.2(GLIBC_2.1) for package: curl
2019-09-26T02:34:18.1620469Z --> Processing Dependency: libc.so.6(GLIBC_2.1.3) for package: curl
2019-09-26T02:34:18.1621028Z --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
2019-09-26T02:34:18.1621259Z --> Processing Dependency: libidn.so.11 for package: curl
2019-09-26T02:34:18.1668612Z --> Processing Dependency: libz.so.1 for package: curl
2019-09-26T02:34:18.1691399Z --> Processing Dependency: libc.so.6 for package: curl
2019-09-26T02:34:18.1738915Z --> Processing Dependency: libdl.so.2 for package: curl
2019-09-26T02:34:18.1739404Z --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
2019-09-26T02:34:18.1758498Z --> Processing Dependency: libc.so.6(GLIBC_2.3) for package: curl
2019-09-26T02:34:18.1807590Z --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
2019-09-26T02:34:18.1808140Z --> Processing Dependency: libk5crypto.so.3 for package: curl
2019-09-26T02:34:18.1827349Z --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
2019-09-26T02:34:18.1874726Z --> Processing Dependency: libssl.so.6 for package: curl
2019-09-26T02:34:18.1954729Z --> Processing Dependency: libcom_err.so.2 for package: curl
2019-09-26T02:34:19.1498747Z --> Processing Dependency: libcrypto.so.6 for package: curl
2019-09-26T02:34:19.1500151Z --> Processing Dependency: libc.so.6(GLIBC_2.0) for package: curl
2019-09-26T02:34:19.1500621Z --> Processing Dependency: libdl.so.2(GLIBC_2.0) for package: curl
2019-09-26T02:34:19.1501104Z --> Processing Dependency: libkrb5.so.3 for package: curl
2019-09-26T02:34:19.1501566Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be updated
2019-09-26T02:34:19.1502579Z --> Processing Dependency: libidn.so.11()(64bit) for package: curl
2019-09-26T02:34:19.1503236Z ---> Package file.x86_64 0:4.17-28 set to be updated
2019-09-26T02:34:19.1504091Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-26T02:34:19.1504769Z --> Processing Dependency: cpp = 4.1.2-55.el5 for package: gcc
2019-09-26T02:34:19.1505494Z --> Processing Dependency: libgomp >= 4.1.2-55.el5 for package: gcc
2019-09-26T02:34:19.1506286Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-26T02:34:19.1506803Z --> Processing Dependency: libstdc++-devel = 4.1.2-55.el5 for package: gcc-c++
2019-09-26T02:34:19.1507287Z ---> Package gettext.i386 0:0.17-1.el5 set to be updated
2019-09-26T02:34:19.1507751Z --> Processing Dependency: libgomp.so.1 for package: gettext
2019-09-26T02:34:19.1508148Z --> Processing Dependency: libgomp.so.1(GOMP_1.0) for package: gettext
2019-09-26T02:34:19.1508803Z --> Processing Dependency: libgcc_s.so.1 for package: gettext
2019-09-26T02:34:19.1509262Z --> Processing Dependency: libgcc_s.so.1(GCC_3.3.1) for package: gettext
2019-09-26T02:34:19.1509877Z ---> Package gettext.x86_64 0:0.17-1.el5 set to be updated
2019-09-26T02:34:19.1510299Z ---> Package glibc-devel.i386 0:2.5-123.el5_11.3 set to be updated
2019-09-26T02:34:19.1510852Z --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
2019-09-26T02:34:19.1511275Z --> Processing Dependency: glibc-headers for package: glibc-devel
2019-09-26T02:34:19.1512221Z ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-09-26T02:34:19.1512909Z ---> Package make.x86_64 1:3.81-3.el5 set to be updated
2019-09-26T02:34:19.1513410Z ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
2019-09-26T02:34:19.1513888Z --> Processing Dependency: libgdbm.so.2 for package: perl
2019-09-26T02:34:19.1514415Z --> Processing Dependency: libdb-4.3.so for package: perl
2019-09-26T02:34:19.1514949Z ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
2019-09-26T02:34:19.1515613Z ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
2019-09-26T02:34:19.1516097Z ---> Package wget.x86_64 0:1.11.4-3.el5_8.2 set to be updated
2019-09-26T02:34:19.1516530Z ---> Package which.x86_64 0:2.16-7 set to be updated
2019-09-26T02:34:19.1516941Z ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-09-26T02:34:19.1517479Z --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
2019-09-26T02:34:19.1517958Z --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
2019-09-26T02:34:19.1518746Z ---> Package zlib-devel.i386 0:1.2.3-7.el5 set to be updated
2019-09-26T02:34:19.1519203Z ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
2019-09-26T02:34:19.1519615Z --> Running transaction check
2019-09-26T02:34:19.1519994Z ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-26T02:34:19.1520379Z ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
2019-09-26T02:34:19.1520881Z --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
2019-09-26T02:34:19.1521492Z --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
2019-09-26T02:34:19.1522396Z --> Processing Dependency: libstdc++.so.6 for package: db4
2019-09-26T02:34:19.1522942Z ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
2019-09-26T02:34:19.1523503Z --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
2019-09-26T02:34:19.1524009Z ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
2019-09-26T02:34:19.1524559Z ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
2019-09-26T02:34:19.1525112Z ---> Package glibc-headers.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-09-26T02:34:19.1525885Z --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
2019-09-26T02:34:19.1526646Z --> Processing Dependency: kernel-headers for package: glibc-headers
2019-09-26T02:34:19.1527043Z ---> Package imake.x86_64 0:1.0.2-3 set to be updated
2019-09-26T02:34:19.1527418Z ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
2019-09-26T02:34:19.1527807Z --> Processing Dependency: libkeyutils.so.1 for package: krb5-libs
2019-09-26T02:34:19.1528422Z --> Processing Dependency: libselinux.so.1 for package: krb5-libs
2019-09-26T02:34:19.1528830Z --> Processing Dependency: libkeyutils.so.1(KEYUTILS_0.3) for package: krb5-libs
2019-09-26T02:34:19.1529234Z ---> Package libgcc.i386 0:4.1.2-55.el5 set to be updated
2019-09-26T02:34:19.1529731Z ---> Package libgomp.i386 0:4.4.7-1.el5 set to be updated
2019-09-26T02:34:19.1530225Z ---> Package libgomp.x86_64 0:4.4.7-1.el5 set to be updated
2019-09-26T02:34:19.1530678Z ---> Package libidn.i386 0:0.6.5-1.1 set to be updated
2019-09-26T02:34:19.1531059Z ---> Package libidn.x86_64 0:0.6.5-1.1 set to be updated
2019-09-26T02:34:19.1531662Z ---> Package libstdc++-devel.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-26T02:34:23.6202749Z ---> Package m4.x86_64 0:1.4.5-3.el5.1 set to be updated
2019-09-26T02:34:23.6214888Z ---> Package openssl.i686 0:0.9.8e-40.el5_11 set to be updated
2019-09-26T02:34:23.6262667Z ---> Package xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-09-26T02:34:23.6275147Z ---> Package zlib.i386 0:1.2.3-7.el5 set to be updated
2019-09-26T02:34:23.6291610Z --> Running transaction check
2019-09-26T02:34:23.6291957Z ---> Package device-mapper.i386 0:1.02.67-2.el5_11.1 set to be updated
2019-09-26T02:34:23.6317511Z --> Processing Dependency: libsepol.so.1 for package: device-mapper
2019-09-26T02:34:23.6346117Z ---> Package kernel-headers.x86_64 0:2.6.18-419.el5 set to be updated
2019-09-26T02:34:23.6346490Z ---> Package keyutils-libs.i386 0:1.2-1.el5 set to be updated
2019-09-26T02:34:23.6358742Z ---> Package libselinux.i386 0:1.33.4-5.7.el5.centos set to be updated
2019-09-26T02:34:23.6408416Z ---> Package libstdc++.i386 0:4.1.2-55.el5 set to be updated
2019-09-26T02:34:23.6456709Z --> Running transaction check
2019-09-26T02:34:23.6457025Z ---> Package libsepol.i386 0:1.15.2-3.el5 set to be updated
2019-09-26T02:34:23.7629440Z 
2019-09-26T02:34:23.7629626Z Dependencies Resolved
2019-09-26T02:34:23.8086823Z 
2019-09-26T02:34:23.8087037Z ================================================================================
2019-09-26T02:34:23.8087037Z ================================================================================
2019-09-26T02:34:23.8087127Z  Package          Arch    Version                             Repository   Size
2019-09-26T02:34:23.8087281Z ================================================================================
2019-09-26T02:34:23.8087379Z Installing:
2019-09-26T02:34:23.8088120Z  autoconf         noarch  2.59-12                             base        647 k
2019-09-26T02:34:23.8088588Z  bzip2            x86_64  1.0.3-6.el5_5                       base         50 k
2019-09-26T02:34:23.8089005Z  curl             i386    7.15.5-17.el5_9                     base        235 k
2019-09-26T02:34:23.8089448Z  curl             x86_64  7.15.5-17.el5_9                     base        232 k
2019-09-26T02:34:23.8089703Z  file             x86_64  4.17-28                             base        321 k
2019-09-26T02:34:23.8090016Z  gcc              x86_64  4.1.2-55.el5                        base        5.3 M
2019-09-26T02:34:23.8090490Z  gcc-c++          x86_64  4.1.2-55.el5                        base        3.8 M
2019-09-26T02:34:23.8090760Z  gettext          i386    0.17-1.el5                          base        2.4 M
2019-09-26T02:34:23.8091080Z  gettext          x86_64  0.17-1.el5                          base        2.4 M
2019-09-26T02:34:23.8091351Z  glibc-devel      i386    2.5-123.el5_11.3                    updates     2.1 M
2019-09-26T02:34:23.8091667Z  glibc-devel      x86_64  2.5-123.el5_11.3                    updates     2.4 M
2019-09-26T02:34:23.8091961Z  make             x86_64  1:3.81-3.el5                        base        470 k
2019-09-26T02:34:23.8092716Z  perl             i386    4:5.8.8-43.el5_11                   updates      12 M
2019-09-26T02:34:23.8093097Z  perl             x86_64  4:5.8.8-43.el5_11                   updates      12 M
2019-09-26T02:34:23.8093434Z  pkgconfig        x86_64  1:0.21-2.el5                        base         61 k
2019-09-26T02:34:23.8093768Z  wget             x86_64  1.11.4-3.el5_8.2                    base        583 k
2019-09-26T02:34:23.8094079Z  which            x86_64  2.16-7                              base         24 k
2019-09-26T02:34:23.8094413Z  xz               x86_64  4.999.9-0.3.beta.20091007git.el5    base        146 k
2019-09-26T02:34:23.8095005Z  zlib-devel       i386    1.2.3-7.el5                         base        102 k
2019-09-26T02:34:23.8095387Z  zlib-devel       x86_64  1.2.3-7.el5                         base        103 k
2019-09-26T02:34:23.8095511Z Installing for dependencies:
2019-09-26T02:34:23.8096012Z  cpp              x86_64  4.1.2-55.el5                        base        2.9 M
2019-09-26T02:34:23.8096277Z  db4              i386    4.3.29-10.el5_5.2                   base        910 k
2019-09-26T02:34:23.8096541Z  device-mapper    i386    1.02.67-2.el5_11.1                  updates     804 k
2019-09-26T02:34:23.8096918Z  e2fsprogs-libs   i386    1.39-37.el5                         base        120 k
2019-09-26T02:34:23.8097183Z  gdbm             i386    1.8.0-28.el5                        base         28 k
2019-09-26T02:34:23.8097425Z  glibc            i686    2.5-123.el5_11.3                    updates     5.4 M
2019-09-26T02:34:23.8097696Z  glibc-headers    x86_64  2.5-123.el5_11.3                    updates     602 k
2019-09-26T02:34:23.8097937Z  imake            x86_64  1.0.2-3                             base        319 k
2019-09-26T02:34:23.8098196Z  kernel-headers   x86_64  2.6.18-419.el5                      updates     1.5 M
2019-09-26T02:34:23.8098619Z  keyutils-libs    i386    1.2-1.el5                           base         18 k
2019-09-26T02:34:23.8098849Z  krb5-libs        i386    1.6.1-80.el5_11                     updates     670 k
2019-09-26T02:34:23.8099100Z  libgcc           i386    4.1.2-55.el5                        base         97 k
2019-09-26T02:34:23.8099329Z  libgomp          i386    4.4.7-1.el5                         base         74 k
2019-09-26T02:34:23.8099756Z  libgomp          x86_64  4.4.7-1.el5                         base         71 k
2019-09-26T02:34:23.8100014Z  libidn           i386    0.6.5-1.1                           base        194 k
2019-09-26T02:34:23.8100623Z  libidn           x86_64  0.6.5-1.1                           base        195 k
2019-09-26T02:34:23.8100913Z  libselinux       i386    1.33.4-5.7.el5.centos               libselinux   77 k
2019-09-26T02:34:23.8101178Z  libsepol         i386    1.15.2-3.el5                        base        128 k
2019-09-26T02:34:23.8101460Z  libstdc++        i386    4.1.2-55.el5                        base        364 k
2019-09-26T02:34:23.8101743Z  libstdc++-devel  x86_64  4.1.2-55.el5                        base        2.8 M
2019-09-26T02:34:23.8103523Z  m4               x86_64  1.4.5-3.el5.1                       base        171 k
2019-09-26T02:34:23.8103885Z  openssl          i686    0.9.8e-40.el5_11                    updates     1.7 M
2019-09-26T02:34:23.8104229Z  xz-libs          x86_64  4.999.9-0.3.beta.20091007git.el5    base         95 k
2019-09-26T02:34:23.8104562Z  zlib             i386    1.2.3-7.el5                         base         51 k
2019-09-26T02:34:23.8104711Z Transaction Summary
2019-09-26T02:34:23.8104799Z ================================================================================
2019-09-26T02:34:23.8104922Z Install      44 Package(s)
2019-09-26T02:34:23.8104992Z Upgrade       0 Package(s)
2019-09-26T02:34:23.8104992Z Upgrade       0 Package(s)
2019-09-26T02:34:23.8105055Z 
2019-09-26T02:34:23.8109767Z Total download size: 65 M
2019-09-26T02:34:23.8109862Z Downloading Packages:
2019-09-26T02:34:25.2890092Z --------------------------------------------------------------------------------
2019-09-26T02:34:25.2890322Z Total                                            44 MB/s |  65 MB     00:01     
2019-09-26T02:34:25.3350494Z Running rpm_check_debug
2019-09-26T02:34:25.7100951Z Finished Transaction Test
2019-09-26T02:34:25.7101088Z Transaction Test Succeeded
2019-09-26T02:34:25.7954792Z Running Transaction
2019-09-26T02:34:26.0345463Z 
---
2019-09-26T02:34:31.0977319Z   Installing     : cpp                                                     8/44 
2019-09-26T02:34:31.0977686Z 
2019-09-26T02:34:31.9474248Z   Installing     : m4                                                      9/44 
2019-09-26T02:34:31.9474452Z 
2019-09-26T02:34:32.6286755Z   Installing     : xz-libs                                                10/44 
2019-09-26T02:34:32.8054829Z   Installing     : xz                                                     11/44 
2019-09-26T02:34:32.8055106Z 
2019-09-26T02:34:33.5286397Z   Installing     : gettext                                                12/44 
2019-09-26T02:34:33.5286575Z 
---
2019-09-26T02:34:37.2408115Z   Installing     : zlib                                                   19/44 
2019-09-26T02:34:37.2408489Z 
2019-09-26T02:34:37.9248507Z   Installing     : libstdc++                                              20/44 
2019-09-26T02:34:37.9248721Z 
2019-09-26T02:34:38.5956256Z   Installing     : libsepol                                               21/44 
2019-09-26T02:34:39.2932100Z   Installing     : libselinux                                             22/44 
2019-09-26T02:34:39.2932261Z 
2019-09-26T02:34:39.9858937Z   Installing     : device-mapper                                          23/44 
2019-09-26T02:34:39.9859057Z 
---
2019-09-26T02:34:43.1072631Z   Installing     : libidn                                                 27/44 
2019-09-26T02:34:43.1072956Z 
2019-09-26T02:34:43.8298738Z   Installing     : keyutils-libs                                          28/44 
2019-09-26T02:34:43.8299070Z 
2019-09-26T02:34:44.5919936Z   Installing     : krb5-libs                                              29/44 
2019-09-26T02:34:45.3494299Z   Installing     : openssl                                                30/44 
2019-09-26T02:34:45.3494573Z 
2019-09-26T02:34:46.7990906Z   Installing     : gdbm                                                   31/44 
2019-09-26T02:34:46.7992047Z 
---
2019-09-26T02:34:54.5670115Z 
2019-09-26T02:34:54.6918968Z   Installing     : gcc-c++                                                44/44 
2019-09-26T02:34:54.6919064Z 
2019-09-26T02:34:54.6919269Z Installed:
2019-09-26T02:34:54.6919561Z   autoconf.noarch 0:2.59-12                                                     
2019-09-26T02:34:54.6919856Z   bzip2.x86_64 0:1.0.3-6.el5_5                                                  
2019-09-26T02:34:54.6920245Z   curl.i386 0:7.15.5-17.el5_9                                                   
2019-09-26T02:34:54.6920534Z   curl.x86_64 0:7.15.5-17.el5_9                                                 
2019-09-26T02:34:54.6920784Z   file.x86_64 0:4.17-28                                                         
2019-09-26T02:34:54.6921343Z   gcc.x86_64 0:4.1.2-55.el5                                                     
2019-09-26T02:34:54.6921612Z   gcc-c++.x86_64 0:4.1.2-55.el5                                                 
2019-09-26T02:34:54.6921870Z   gettext.i386 0:0.17-1.el5                                                     
2019-09-26T02:34:54.6922127Z   gettext.x86_64 0:0.17-1.el5                                                   
2019-09-26T02:34:54.6922510Z   glibc-devel.i386 0:2.5-123.el5_11.3                                           
2019-09-26T02:34:54.6922774Z   glibc-devel.x86_64 0:2.5-123.el5_11.3                                         
2019-09-26T02:34:54.6923042Z   make.x86_64 1:3.81-3.el5                                                      
2019-09-26T02:34:54.6923423Z   perl.i386 4:5.8.8-43.el5_11                                                   
2019-09-26T02:34:54.6923689Z   perl.x86_64 4:5.8.8-43.el5_11                                                 
2019-09-26T02:34:54.6924105Z   pkgconfig.x86_64 1:0.21-2.el5                                                 
2019-09-26T02:34:54.6925095Z   wget.x86_64 0:1.11.4-3.el5_8.2                                                
2019-09-26T02:34:54.6925423Z   which.x86_64 0:2.16-7                                                         
2019-09-26T02:34:54.6925928Z   xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5                                  
2019-09-26T02:34:54.6926293Z   zlib-devel.i386 0:1.2.3-7.el5                                                 
2019-09-26T02:34:54.6926609Z   zlib-devel.x86_64 0:1.2.3-7.el5                                               
2019-09-26T02:34:54.6927132Z Dependency Installed:
2019-09-26T02:34:54.6927132Z Dependency Installed:
2019-09-26T02:34:54.6927557Z   cpp.x86_64 0:4.1.2-55.el5                                                     
2019-09-26T02:34:54.6928187Z   db4.i386 0:4.3.29-10.el5_5.2                                                  
2019-09-26T02:34:54.6928487Z   device-mapper.i386 0:1.02.67-2.el5_11.1                                       
2019-09-26T02:34:54.6928727Z   e2fsprogs-libs.i386 0:1.39-37.el5                                             
2019-09-26T02:34:54.6929105Z   gdbm.i386 0:1.8.0-28.el5                                                      
2019-09-26T02:34:54.6929554Z   glibc.i686 0:2.5-123.el5_11.3                                                 
2019-09-26T02:34:54.6929800Z   glibc-headers.x86_64 0:2.5-123.el5_11.3                                       
2019-09-26T02:34:54.6930055Z   imake.x86_64 0:1.0.2-3                                                        
2019-09-26T02:34:54.6930294Z   kernel-headers.x86_64 0:2.6.18-419.el5                                        
2019-09-26T02:34:54.6930559Z   keyutils-libs.i386 0:1.2-1.el5                                                
2019-09-26T02:34:54.6930816Z   krb5-libs.i386 0:1.6.1-80.el5_11                                              
2019-09-26T02:34:54.6931058Z   libgcc.i386 0:4.1.2-55.el5                                                    
2019-09-26T02:34:54.6931314Z   libgomp.i386 0:4.4.7-1.el5                                                    
2019-09-26T02:34:54.6931718Z   libgomp.x86_64 0:4.4.7-1.el5                                                  
2019-09-26T02:34:54.6931965Z   libidn.i386 0:0.6.5-1.1                                                       
2019-09-26T02:34:54.6932217Z   libidn.x86_64 0:0.6.5-1.1                                                     
2019-09-26T02:34:54.6932444Z   libselinux.i386 0:1.33.4-5.7.el5.centos                                       
2019-09-26T02:34:54.6932687Z   libsepol.i386 0:1.15.2-3.el5                                                  
2019-09-26T02:34:54.6932922Z   libstdc++.i386 0:4.1.2-55.el5                                                 
2019-09-26T02:34:54.6933349Z   libstdc++-devel.x86_64 0:4.1.2-55.el5                                         
2019-09-26T02:34:54.6933588Z   m4.x86_64 0:1.4.5-3.el5.1                                                     
2019-09-26T02:34:54.6934856Z   openssl.i686 0:0.9.8e-40.el5_11                                               
2019-09-26T02:34:54.6935264Z   xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5                             
2019-09-26T02:34:54.6935581Z   zlib.i386 0:1.2.3-7.el5                                                       
2019-09-26T02:34:54.6940983Z Complete!
2019-09-26T02:35:10.9975149Z Removing intermediate container be382e7fbe42
2019-09-26T02:35:10.9977544Z  ---> 7bec15472535
2019-09-26T02:35:10.9977978Z Step 7/41 : ENV PATH=/rustroot/bin:$PATH
---
2019-09-26T02:35:20.0555930Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-09-26T02:35:20.0556480Z 
2019-09-26T02:35:20.6355786Z   0 5184k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-09-26T02:35:20.6356838Z 100 5184k  100 5184k    0     0  5637k      0 --:--:-- --:--:-- --:--:-- 8939k
2019-09-26T02:35:20.6466031Z + cd openssl-1.0.2k
2019-09-26T02:35:20.6467002Z + hide_output ./config --prefix=/rustroot shared -fPIC
2019-09-26T02:35:22.0020748Z + hide_output make -j10
2019-09-26T02:35:22.0021413Z + set +x
2019-09-26T02:35:52.0069182Z Thu Sep 26 02:35:52 UTC 2019 - building ...
2019-09-26T02:36:22.0323171Z Thu Sep 26 02:36:22 UTC 2019 - building ...
2019-09-26T02:36:22.0323171Z Thu Sep 26 02:36:22 UTC 2019 - building ...
2019-09-26T02:36:30.0803214Z shared.sh: line 1:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-26T02:36:30.0803744Z + hide_output make install
2019-09-26T02:36:30.0803858Z + set +x
2019-09-26T02:36:50.1175230Z shared.sh: line 1:   350 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-26T02:36:50.1175652Z + cd ..
2019-09-26T02:36:50.1175915Z + rm -rf openssl-1.0.2k
2019-09-26T02:36:50.6923536Z ./build-openssl.sh: line 16:  4113 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
2019-09-26T02:36:50.6924660Z + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
2019-09-26T02:36:53.0101284Z Removing intermediate container c2421a31567d
2019-09-26T02:36:53.0102101Z Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
2019-09-26T02:36:54.6925372Z  ---> 57475d998ef3
2019-09-26T02:36:54.6926554Z Step 15/41 : RUN ./build-curl.sh
2019-09-26T02:36:54.6927442Z  ---> Running in fe1dc43dbc90
2019-09-26T02:36:54.6927442Z  ---> Running in fe1dc43dbc90
2019-09-26T02:36:55.0372452Z + source shared.sh
2019-09-26T02:36:55.0372667Z + VERSION=7.51.0
2019-09-26T02:36:55.0373550Z + tar xjf -
2019-09-26T02:36:55.0373859Z + curl https://cool.haxx.se/download/curl-7.51.0.tar.bz2
2019-09-26T02:36:56.7844509Z 
2019-09-26T02:36:56.7845602Z curl: (51) SSL: certificate subject name 'anja.haxx.se' does not match target host name 'cool.haxx.se'
2019-09-26T02:36:56.7852378Z 
2019-09-26T02:36:56.7852485Z bzip2: Compressed file ends unexpectedly;
2019-09-26T02:36:56.7852573Z  perhaps it is corrupted?  *Possible* reason follows.
2019-09-26T02:36:56.7852636Z bzip2: Inappropriate ioctl for device
2019-09-26T02:36:56.7852716Z  Input file = (stdin), output file = (stdout)
2019-09-26T02:36:56.7853389Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:36:56.7853389Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:36:56.7853751Z You can use the -tvv option to test integrity of such files.
2019-09-26T02:36:56.7853811Z 
2019-09-26T02:36:56.7854123Z You can use the `bzip2recover' program to attempt to recover
2019-09-26T02:36:56.7854232Z data from undamaged sections of corrupted files.
2019-09-26T02:36:56.7854531Z tar: Child returned status 2
2019-09-26T02:36:56.7854631Z tar: Error exit delayed from previous errors
2019-09-26T02:36:56.7854631Z tar: Error exit delayed from previous errors
2019-09-26T02:36:57.1459683Z The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 2
2019-09-26T02:36:58.2498799Z Sending build context to Docker daemon  526.3kB
2019-09-26T02:36:58.2499886Z 
2019-09-26T02:36:58.2674145Z Step 1/41 : FROM centos:5
2019-09-26T02:36:58.2678055Z  ---> 1ae98b2c895d
---
2019-09-26T02:36:58.2773578Z  ---> 57475d998ef3
2019-09-26T02:36:58.2773888Z Step 15/41 : RUN ./build-curl.sh
2019-09-26T02:36:58.4848120Z  ---> Running in a1561e4f53d3
2019-09-26T02:36:58.8911799Z + source shared.sh
2019-09-26T02:36:58.8912726Z + VERSION=7.51.0
2019-09-26T02:36:58.8913822Z + curl https://cool.haxx.se/download/curl-7.51.0.tar.bz2
2019-09-26T02:37:00.3658957Z 
2019-09-26T02:37:00.3658957Z 
2019-09-26T02:37:00.3660079Z curl: (51) SSL: certificate subject name 'anja.haxx.se' does not match target host name 'cool.haxx.se'
2019-09-26T02:37:00.3669905Z 
2019-09-26T02:37:00.3670323Z bzip2: Compressed file ends unexpectedly;
2019-09-26T02:37:00.3670436Z  perhaps it is corrupted?  *Possible* reason follows.
2019-09-26T02:37:00.3670532Z bzip2: Inappropriate ioctl for device
2019-09-26T02:37:00.3670678Z  Input file = (stdin), output file = (stdout)
2019-09-26T02:37:00.3670976Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:00.3670976Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:00.3671750Z You can use the -tvv option to test integrity of such files.
2019-09-26T02:37:00.3672053Z 
2019-09-26T02:37:00.3672464Z You can use the `bzip2recover' program to attempt to recover
2019-09-26T02:37:00.3672695Z data from undamaged sections of corrupted files.
2019-09-26T02:37:00.3673965Z tar: Child returned status 2
2019-09-26T02:37:00.3674223Z tar: Error exit delayed from previous errors
2019-09-26T02:37:00.3674223Z tar: Error exit delayed from previous errors
2019-09-26T02:37:00.7582719Z The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 2
2019-09-26T02:37:02.8872788Z Sending build context to Docker daemon  526.3kB
2019-09-26T02:37:02.8874537Z 
2019-09-26T02:37:02.9033144Z Step 1/41 : FROM centos:5
2019-09-26T02:37:02.9038056Z  ---> 1ae98b2c895d
---
2019-09-26T02:37:03.0833995Z  ---> Running in 67f92cd33f7f
2019-09-26T02:37:03.5073739Z + source shared.sh
2019-09-26T02:37:03.5075074Z + VERSION=7.51.0
2019-09-26T02:37:03.5076280Z + tar xjf -
2019-09-26T02:37:03.5076717Z + curl https://cool.haxx.se/download/curl-7.51.0.tar.bz2
2019-09-26T02:37:04.9805710Z 
2019-09-26T02:37:04.9806414Z curl: (51) SSL: certificate subject name 'anja.haxx.se' does not match target host name 'cool.haxx.se'
2019-09-26T02:37:04.9823953Z 
2019-09-26T02:37:04.9825455Z bzip2: Compressed file ends unexpectedly;
2019-09-26T02:37:04.9826473Z  perhaps it is corrupted?  *Possible* reason follows.
2019-09-26T02:37:04.9826763Z bzip2: Inappropriate ioctl for device
2019-09-26T02:37:04.9827236Z  Input file = (stdin), output file = (stdout)
2019-09-26T02:37:04.9827393Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:04.9827393Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:04.9828233Z You can use the -tvv option to test integrity of such files.
2019-09-26T02:37:04.9828302Z 
2019-09-26T02:37:04.9828632Z You can use the `bzip2recover' program to attempt to recover
2019-09-26T02:37:04.9828725Z data from undamaged sections of corrupted files.
2019-09-26T02:37:04.9828971Z tar: Child returned status 2
2019-09-26T02:37:04.9829056Z tar: Error exit delayed from previous errors
2019-09-26T02:37:04.9829056Z tar: Error exit delayed from previous errors
2019-09-26T02:37:05.3464895Z The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 2
2019-09-26T02:37:08.4429597Z Sending build context to Docker daemon  526.3kB
2019-09-26T02:37:08.4430683Z 
2019-09-26T02:37:08.4716849Z Step 1/41 : FROM centos:5
2019-09-26T02:37:08.4719083Z  ---> 1ae98b2c895d
---
2019-09-26T02:37:08.4854933Z  ---> 57475d998ef3
2019-09-26T02:37:08.4855194Z Step 15/41 : RUN ./build-curl.sh
2019-09-26T02:37:08.6922736Z  ---> Running in 2faa188fc8a6
2019-09-26T02:37:09.1305700Z + source shared.sh
2019-09-26T02:37:09.1306752Z + VERSION=7.51.0
2019-09-26T02:37:09.1307828Z + curl https://cool.haxx.se/download/curl-7.51.0.tar.bz2
2019-09-26T02:37:10.6131730Z 
2019-09-26T02:37:10.6131730Z 
2019-09-26T02:37:10.6133000Z curl: (51) SSL: certificate subject name 'anja.haxx.se' does not match target host name 'cool.haxx.se'
2019-09-26T02:37:10.6146956Z 
2019-09-26T02:37:10.6147660Z bzip2: Compressed file ends unexpectedly;
2019-09-26T02:37:10.6147790Z  perhaps it is corrupted?  *Possible* reason follows.
2019-09-26T02:37:10.6148611Z bzip2: Inappropriate ioctl for device
2019-09-26T02:37:10.6148950Z  Input file = (stdin), output file = (stdout)
2019-09-26T02:37:10.6149074Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:10.6149074Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:10.6149612Z You can use the -tvv option to test integrity of such files.
2019-09-26T02:37:10.6149684Z 
2019-09-26T02:37:10.6149925Z You can use the `bzip2recover' program to attempt to recover
2019-09-26T02:37:10.6150018Z data from undamaged sections of corrupted files.
2019-09-26T02:37:10.6150487Z tar: Child returned status 2
2019-09-26T02:37:10.6150557Z tar: Error exit delayed from previous errors
2019-09-26T02:37:10.6150557Z tar: Error exit delayed from previous errors
2019-09-26T02:37:11.6975475Z The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 2
2019-09-26T02:37:15.0734482Z Sending build context to Docker daemon  526.3kB
2019-09-26T02:37:15.0735961Z 
2019-09-26T02:37:15.0954906Z Step 1/41 : FROM centos:5
2019-09-26T02:37:15.0959212Z  ---> 1ae98b2c895d
---
2019-09-26T02:37:15.3848969Z  ---> Running in 5789ddc8199b
2019-09-26T02:37:15.7779114Z + source shared.sh
2019-09-26T02:37:15.7821033Z + VERSION=7.51.0
2019-09-26T02:37:15.7821297Z + tar xjf -
2019-09-26T02:37:15.7821525Z + curl https://cool.haxx.se/download/curl-7.51.0.tar.bz2
2019-09-26T02:37:17.2639209Z 
2019-09-26T02:37:17.2640520Z curl: (51) SSL: certificate subject name 'anja.haxx.se' does not match target host name 'cool.haxx.se'
2019-09-26T02:37:17.2646175Z 
2019-09-26T02:37:17.2646907Z bzip2: Compressed file ends unexpectedly;
2019-09-26T02:37:17.2647644Z  perhaps it is corrupted?  *Possible* reason follows.
2019-09-26T02:37:17.2647730Z bzip2: Inappropriate ioctl for device
2019-09-26T02:37:17.2647834Z  Input file = (stdin), output file = (stdout)
2019-09-26T02:37:17.2647977Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:17.2647977Z It is possible that the compressed file(s) have become corrupted.
2019-09-26T02:37:17.2648864Z You can use the -tvv option to test integrity of such files.
2019-09-26T02:37:17.2649119Z 
2019-09-26T02:37:17.2649373Z You can use the `bzip2recover' program to attempt to recover
2019-09-26T02:37:17.2649463Z data from undamaged sections of corrupted files.
2019-09-26T02:37:17.2653141Z tar: Child returned status 2
2019-09-26T02:37:17.2653500Z tar: Error exit delayed from previous errors
2019-09-26T02:37:17.2653500Z tar: Error exit delayed from previous errors
2019-09-26T02:37:17.5983840Z The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 2
2019-09-26T02:37:17.5991642Z The command has failed after 5 attempts.
2019-09-26T02:37:17.6139429Z ##[error]Bash exited with code '1'.
2019-09-26T02:37:17.6168751Z ##[section]Starting: Upload CPU usage statistics
2019-09-26T02:37:17.6171284Z ==============================================================================
2019-09-26T02:37:17.6171374Z Task         : Bash
2019-09-26T02:37:17.6171431Z Description  : Run a Bash script on macOS, Linux, or Windows
