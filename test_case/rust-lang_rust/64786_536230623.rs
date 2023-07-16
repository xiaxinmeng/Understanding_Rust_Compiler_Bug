plain
2019-09-28T22:32:27.5885342Z  ---> Running in 4c3aa7b815fd
2019-09-28T22:32:29.2388224Z Removing intermediate container 4c3aa7b815fd
2019-09-28T22:32:29.2389738Z  ---> 2f55727dcb24
2019-09-28T22:32:29.2391273Z Step 6/41 : RUN yum upgrade -y && yum install -y       curl       bzip2       gcc       gcc-c++       make       glibc-devel       perl       zlib-devel       file       xz       which       pkgconfig       wget       autoconf       gettext
2019-09-28T22:32:29.4688090Z  ---> Running in 0bce3380585c
2019-09-28T22:32:38.7013272Z Reducing CentOS-5 - libselinux to included packages only
2019-09-28T22:32:38.7036126Z Setting up Upgrade Process
2019-09-28T22:32:39.2014203Z Resolving Dependencies
2019-09-28T22:32:39.2020056Z --> Running transaction check
2019-09-28T22:32:39.2020056Z --> Running transaction check
2019-09-28T22:32:39.2056387Z ---> Package bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-09-28T22:32:39.2155846Z ---> Package bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-09-28T22:32:39.2238018Z ---> Package nspr.x86_64 0:4.11.0-1.el5_11 set to be updated
2019-09-28T22:32:39.2319313Z ---> Package nss.x86_64 0:3.21.3-2.el5_11 set to be updated
2019-09-28T22:32:39.2405097Z ---> Package openssl.x86_64 0:0.9.8e-40.el5_11 set to be updated
2019-09-28T22:32:39.2696227Z ---> Package tzdata.x86_64 0:2017b-1.el5 set to be updated
2019-09-28T22:32:39.3807825Z 
2019-09-28T22:32:39.3808163Z Dependencies Resolved
2019-09-28T22:32:39.3889745Z 
2019-09-28T22:32:39.3889995Z ================================================================================
2019-09-28T22:32:39.3889995Z ================================================================================
2019-09-28T22:32:39.3890096Z  Package         Arch        Version                         Repository    Size
2019-09-28T22:32:39.3890228Z ================================================================================
2019-09-28T22:32:39.3890344Z Updating:
2019-09-28T22:32:39.3891472Z  bind-libs       x86_64      30:9.3.6-25.P1.el5_11.12        updates      902 k
2019-09-28T22:32:39.3891875Z  bind-utils      x86_64      30:9.3.6-25.P1.el5_11.12        updates      181 k
2019-09-28T22:32:39.3892239Z  nspr            x86_64      4.11.0-1.el5_11                 updates      123 k
2019-09-28T22:32:39.3892622Z  nss             x86_64      3.21.3-2.el5_11                 updates      1.3 M
2019-09-28T22:32:39.3892935Z  openssl         x86_64      0.9.8e-40.el5_11                updates      1.7 M
2019-09-28T22:32:39.3893305Z  tzdata          x86_64      2017b-1.el5                     updates      757 k
2019-09-28T22:32:39.3893485Z Transaction Summary
2019-09-28T22:32:39.3893615Z ================================================================================
2019-09-28T22:32:39.3893704Z Install       0 Package(s)
2019-09-28T22:32:39.3893817Z Upgrade       6 Package(s)
2019-09-28T22:32:39.3893817Z Upgrade       6 Package(s)
2019-09-28T22:32:39.3893862Z 
2019-09-28T22:32:39.3893925Z Total download size: 4.9 M
2019-09-28T22:32:39.3894036Z Downloading Packages:
2019-09-28T22:32:39.9821005Z --------------------------------------------------------------------------------
2019-09-28T22:32:39.9821366Z Total                                           8.3 MB/s | 4.9 MB     00:00     
2019-09-28T22:32:39.9829543Z warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
2019-09-28T22:32:39.9876104Z Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
2019-09-28T22:32:40.1128204Z Running rpm_check_debug
2019-09-28T22:32:40.2324105Z Finished Transaction Test
2019-09-28T22:32:40.2324521Z Transaction Test Succeeded
2019-09-28T22:32:40.2802955Z Running Transaction
2019-09-28T22:32:40.4094516Z 
---
2019-09-28T22:32:43.2086003Z   Updating       : tzdata                                                  6/12 
2019-09-28T22:32:43.2087510Z 
2019-09-28T22:32:43.2882800Z   Cleanup        : bind-utils                                              7/12 
2019-09-28T22:32:43.2882957Z 
2019-09-28T22:32:43.7363220Z   Cleanup        : nss                                                     8/12 
2019-09-28T22:32:44.1772586Z   Cleanup        : bind-libs                                               9/12 
2019-09-28T22:32:44.1773408Z 
2019-09-28T22:32:44.6131575Z   Cleanup        : openssl                                                10/12 
2019-09-28T22:32:44.6132810Z 
2019-09-28T22:32:44.6132810Z 
2019-09-28T22:32:45.0572590Z   Cleanup        : nspr                                                   11/12 
2019-09-28T22:32:45.1769759Z   Cleanup        : tzdata                                                 12/12 
2019-09-28T22:32:45.1770089Z 
2019-09-28T22:32:45.1770346Z Updated:
2019-09-28T22:32:45.1770346Z Updated:
2019-09-28T22:32:45.1771912Z   bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12                                     
2019-09-28T22:32:45.1772531Z   bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12                                    
2019-09-28T22:32:45.1773110Z   nspr.x86_64 0:4.11.0-1.el5_11                                                 
2019-09-28T22:32:45.1773656Z   nss.x86_64 0:3.21.3-2.el5_11                                                  
2019-09-28T22:32:45.1774159Z   openssl.x86_64 0:0.9.8e-40.el5_11                                             
2019-09-28T22:32:45.1774829Z   tzdata.x86_64 0:2017b-1.el5                                                   
2019-09-28T22:32:45.1775539Z Complete!
2019-09-28T22:32:45.1775539Z Complete!
2019-09-28T22:32:45.3875124Z Reducing CentOS-5 - libselinux to included packages only
2019-09-28T22:32:45.3875461Z Setting up Install Process
2019-09-28T22:32:46.4228772Z Resolving Dependencies
2019-09-28T22:32:46.4229587Z --> Running transaction check
2019-09-28T22:32:46.4229587Z --> Running transaction check
2019-09-28T22:32:46.4229902Z ---> Package autoconf.noarch 0:2.59-12 set to be updated
2019-09-28T22:32:46.4641582Z --> Processing Dependency: imake for package: autoconf
2019-09-28T22:32:46.4669564Z --> Processing Dependency: m4 for package: autoconf
2019-09-28T22:32:46.4695539Z ---> Package bzip2.x86_64 0:1.0.3-6.el5_5 set to be updated
2019-09-28T22:32:46.4817202Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be updated
2019-09-28T22:32:46.5027432Z --> Processing Dependency: libc.so.6(GLIBC_2.4) for package: curl
2019-09-28T22:32:46.5216157Z --> Processing Dependency: libgssapi_krb5.so.2 for package: curl
2019-09-28T22:32:46.5257396Z --> Processing Dependency: libdl.so.2(GLIBC_2.1) for package: curl
2019-09-28T22:32:46.5316770Z --> Processing Dependency: libc.so.6(GLIBC_2.1.3) for package: curl
2019-09-28T22:32:46.5317064Z --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
2019-09-28T22:32:46.5317348Z --> Processing Dependency: libidn.so.11 for package: curl
2019-09-28T22:32:46.5367469Z --> Processing Dependency: libz.so.1 for package: curl
2019-09-28T22:32:46.5388559Z --> Processing Dependency: libc.so.6 for package: curl
2019-09-28T22:32:46.5439652Z --> Processing Dependency: libdl.so.2 for package: curl
2019-09-28T22:32:46.5439975Z --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
2019-09-28T22:32:46.5465001Z --> Processing Dependency: libc.so.6(GLIBC_2.3) for package: curl
2019-09-28T22:32:46.5523478Z --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
2019-09-28T22:32:46.5523845Z --> Processing Dependency: libk5crypto.so.3 for package: curl
2019-09-28T22:32:46.5544942Z --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
2019-09-28T22:32:46.5590492Z --> Processing Dependency: libssl.so.6 for package: curl
2019-09-28T22:32:46.5669854Z --> Processing Dependency: libcom_err.so.2 for package: curl
2019-09-28T22:32:46.5694391Z --> Processing Dependency: libcrypto.so.6 for package: curl
2019-09-28T22:32:46.5747939Z --> Processing Dependency: libc.so.6(GLIBC_2.0) for package: curl
2019-09-28T22:32:46.5796791Z --> Processing Dependency: libdl.so.2(GLIBC_2.0) for package: curl
2019-09-28T22:32:46.5797157Z --> Processing Dependency: libkrb5.so.3 for package: curl
2019-09-28T22:32:46.5817798Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be updated
2019-09-28T22:32:46.5963009Z --> Processing Dependency: libidn.so.11()(64bit) for package: curl
2019-09-28T22:32:46.6002746Z ---> Package file.x86_64 0:4.17-28 set to be updated
2019-09-28T22:32:46.6039033Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-28T22:32:46.6123957Z --> Processing Dependency: cpp = 4.1.2-55.el5 for package: gcc
2019-09-28T22:32:46.6148784Z --> Processing Dependency: libgomp >= 4.1.2-55.el5 for package: gcc
2019-09-28T22:32:46.6249609Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-28T22:32:46.6314486Z --> Processing Dependency: libstdc++-devel = 4.1.2-55.el5 for package: gcc-c++
2019-09-28T22:32:46.6400498Z ---> Package gettext.i386 0:0.17-1.el5 set to be updated
2019-09-28T22:32:46.6544992Z --> Processing Dependency: libgomp.so.1 for package: gettext
2019-09-28T22:32:46.6583974Z --> Processing Dependency: libgomp.so.1(GOMP_1.0) for package: gettext
2019-09-28T22:32:46.6584433Z --> Processing Dependency: libgcc_s.so.1 for package: gettext
2019-09-28T22:32:46.6608214Z --> Processing Dependency: libgcc_s.so.1(GCC_3.3.1) for package: gettext
2019-09-28T22:32:46.6608499Z ---> Package gettext.x86_64 0:0.17-1.el5 set to be updated
2019-09-28T22:32:46.6704160Z ---> Package glibc-devel.i386 0:2.5-123.el5_11.3 set to be updated
2019-09-28T22:32:46.6885294Z --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
2019-09-28T22:32:46.6917256Z --> Processing Dependency: glibc-headers for package: glibc-devel
2019-09-28T22:32:46.6917667Z ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-09-28T22:32:46.7113128Z ---> Package make.x86_64 1:3.81-3.el5 set to be updated
2019-09-28T22:32:46.7126551Z ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
2019-09-28T22:32:46.8755775Z --> Processing Dependency: libgdbm.so.2 for package: perl
2019-09-28T22:32:46.8774069Z --> Processing Dependency: libdb-4.3.so for package: perl
2019-09-28T22:32:46.8795380Z ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
2019-09-28T22:32:46.9083691Z ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
2019-09-28T22:32:46.9094485Z ---> Package wget.x86_64 0:1.11.4-3.el5_8.2 set to be updated
2019-09-28T22:32:46.9135017Z ---> Package which.x86_64 0:2.16-7 set to be updated
2019-09-28T22:32:46.9142376Z ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-09-28T22:32:46.9175970Z --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
2019-09-28T22:32:46.9269999Z --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
2019-09-28T22:32:46.9270301Z ---> Package zlib-devel.i386 0:1.2.3-7.el5 set to be updated
2019-09-28T22:32:46.9285784Z ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
2019-09-28T22:32:46.9293940Z --> Running transaction check
2019-09-28T22:32:46.9294289Z ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-28T22:32:46.9306026Z ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
2019-09-28T22:32:46.9364852Z --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
2019-09-28T22:32:46.9393920Z --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
2019-09-28T22:32:46.9394417Z --> Processing Dependency: libstdc++.so.6 for package: db4
2019-09-28T22:32:46.9394736Z ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
2019-09-28T22:32:46.9442834Z --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
2019-09-28T22:32:46.9469901Z ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
2019-09-28T22:32:46.9479440Z ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
2019-09-28T22:32:46.9566608Z ---> Package glibc-headers.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-09-28T22:32:46.9608175Z --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
2019-09-28T22:32:46.9699373Z --> Processing Dependency: kernel-headers for package: glibc-headers
2019-09-28T22:32:46.9699756Z ---> Package imake.x86_64 0:1.0.2-3 set to be updated
2019-09-28T22:32:46.9714349Z ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
2019-09-28T22:32:46.9799330Z --> Processing Dependency: libkeyutils.so.1 for package: krb5-libs
2019-09-28T22:32:46.9820193Z --> Processing Dependency: libselinux.so.1 for package: krb5-libs
2019-09-28T22:32:46.9849261Z --> Processing Dependency: libkeyutils.so.1(KEYUTILS_0.3) for package: krb5-libs
2019-09-28T22:32:46.9888646Z ---> Package libgcc.i386 0:4.1.2-55.el5 set to be updated
2019-09-28T22:32:46.9889270Z ---> Package libgomp.i386 0:4.4.7-1.el5 set to be updated
2019-09-28T22:32:46.9921372Z ---> Package libgomp.x86_64 0:4.4.7-1.el5 set to be updated
2019-09-28T22:32:46.9954413Z ---> Package libidn.i386 0:0.6.5-1.1 set to be updated
2019-09-28T22:32:46.9971130Z ---> Package libidn.x86_64 0:0.6.5-1.1 set to be updated
2019-09-28T22:32:46.9987047Z ---> Package libstdc++-devel.x86_64 0:4.1.2-55.el5 set to be updated
2019-09-28T22:32:53.2468659Z ---> Package m4.x86_64 0:1.4.5-3.el5.1 set to be updated
2019-09-28T22:32:53.2481162Z ---> Package openssl.i686 0:0.9.8e-40.el5_11 set to be updated
2019-09-28T22:32:53.2530517Z ---> Package xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-09-28T22:32:53.2543014Z ---> Package zlib.i386 0:1.2.3-7.el5 set to be updated
2019-09-28T22:32:53.2556963Z --> Running transaction check
2019-09-28T22:32:53.2560337Z ---> Package device-mapper.i386 0:1.02.67-2.el5_11.1 set to be updated
2019-09-28T22:32:53.2586484Z --> Processing Dependency: libsepol.so.1 for package: device-mapper
2019-09-28T22:32:53.2610323Z ---> Package kernel-headers.x86_64 0:2.6.18-419.el5 set to be updated
2019-09-28T22:32:53.2614067Z ---> Package keyutils-libs.i386 0:1.2-1.el5 set to be updated
2019-09-28T22:32:53.2626163Z ---> Package libselinux.i386 0:1.33.4-5.7.el5.centos set to be updated
2019-09-28T22:32:53.2676335Z ---> Package libstdc++.i386 0:4.1.2-55.el5 set to be updated
2019-09-28T22:32:53.2725411Z --> Running transaction check
2019-09-28T22:32:53.2726072Z ---> Package libsepol.i386 0:1.15.2-3.el5 set to be updated
2019-09-28T22:32:53.3994905Z 
2019-09-28T22:32:53.3995083Z Dependencies Resolved
2019-09-28T22:32:53.4489550Z 
2019-09-28T22:32:53.4493137Z ================================================================================
2019-09-28T22:32:53.4493137Z ================================================================================
2019-09-28T22:32:53.4493647Z  Package          Arch    Version                             Repository   Size
2019-09-28T22:32:53.4494462Z ================================================================================
2019-09-28T22:32:53.4495610Z Installing:
2019-09-28T22:32:53.4498015Z  autoconf         noarch  2.59-12                             base        647 k
2019-09-28T22:32:53.4498793Z  bzip2            x86_64  1.0.3-6.el5_5                       base         50 k
2019-09-28T22:32:53.4499565Z  curl             i386    7.15.5-17.el5_9                     base        235 k
2019-09-28T22:32:53.4500128Z  curl             x86_64  7.15.5-17.el5_9                     base        232 k
2019-09-28T22:32:53.4500712Z  file             x86_64  4.17-28                             base        321 k
2019-09-28T22:32:53.4501266Z  gcc              x86_64  4.1.2-55.el5                        base        5.3 M
2019-09-28T22:32:53.4502484Z  gcc-c++          x86_64  4.1.2-55.el5                        base        3.8 M
2019-09-28T22:32:53.4503164Z  gettext          i386    0.17-1.el5                          base        2.4 M
2019-09-28T22:32:53.4503873Z  gettext          x86_64  0.17-1.el5                          base        2.4 M
2019-09-28T22:32:53.4504548Z  glibc-devel      i386    2.5-123.el5_11.3                    updates     2.1 M
2019-09-28T22:32:53.4505167Z  glibc-devel      x86_64  2.5-123.el5_11.3                    updates     2.4 M
2019-09-28T22:32:53.4506386Z  make             x86_64  1:3.81-3.el5                        base        470 k
2019-09-28T22:32:53.4506890Z  perl             i386    4:5.8.8-43.el5_11                   updates      12 M
2019-09-28T22:32:53.4507413Z  perl             x86_64  4:5.8.8-43.el5_11                   updates      12 M
2019-09-28T22:32:53.4507942Z  pkgconfig        x86_64  1:0.21-2.el5                        base         61 k
2019-09-28T22:32:53.4508450Z  wget             x86_64  1.11.4-3.el5_8.2                    base        583 k
2019-09-28T22:32:53.4509006Z  which            x86_64  2.16-7                              base         24 k
2019-09-28T22:32:53.4510082Z  xz               x86_64  4.999.9-0.3.beta.20091007git.el5    base        146 k
2019-09-28T22:32:53.4510741Z  zlib-devel       i386    1.2.3-7.el5                         base        102 k
2019-09-28T22:32:53.4511779Z  zlib-devel       x86_64  1.2.3-7.el5                         base        103 k
2019-09-28T22:32:53.4512546Z Installing for dependencies:
2019-09-28T22:32:53.4513150Z  cpp              x86_64  4.1.2-55.el5                        base        2.9 M
2019-09-28T22:32:53.4514496Z  db4              i386    4.3.29-10.el5_5.2                   base        910 k
2019-09-28T22:32:53.4514828Z  device-mapper    i386    1.02.67-2.el5_11.1                  updates     804 k
2019-09-28T22:32:53.4515172Z  e2fsprogs-libs   i386    1.39-37.el5                         base        120 k
2019-09-28T22:32:53.4515656Z  gdbm             i386    1.8.0-28.el5                        base         28 k
2019-09-28T22:32:53.4515911Z  glibc            i686    2.5-123.el5_11.3                    updates     5.4 M
2019-09-28T22:32:53.4516367Z  glibc-headers    x86_64  2.5-123.el5_11.3                    updates     602 k
2019-09-28T22:32:53.4516770Z  imake            x86_64  1.0.2-3                             base        319 k
2019-09-28T22:32:53.4517031Z  kernel-headers   x86_64  2.6.18-419.el5                      updates     1.5 M
2019-09-28T22:32:53.4517286Z  keyutils-libs    i386    1.2-1.el5                           base         18 k
2019-09-28T22:32:53.4517527Z  krb5-libs        i386    1.6.1-80.el5_11                     updates     670 k
2019-09-28T22:32:53.4517782Z  libgcc           i386    4.1.2-55.el5                        base         97 k
2019-09-28T22:32:53.4518021Z  libgomp          i386    4.4.7-1.el5                         base         74 k
2019-09-28T22:32:53.4518279Z  libgomp          x86_64  4.4.7-1.el5                         base         71 k
2019-09-28T22:32:53.4518697Z  libidn           i386    0.6.5-1.1                           base        194 k
2019-09-28T22:32:53.4518974Z  libidn           x86_64  0.6.5-1.1                           base        195 k
2019-09-28T22:32:53.4519246Z  libselinux       i386    1.33.4-5.7.el5.centos               libselinux   77 k
2019-09-28T22:32:53.4519498Z  libsepol         i386    1.15.2-3.el5                        base        128 k
2019-09-28T22:32:53.4519759Z  libstdc++        i386    4.1.2-55.el5                        base        364 k
2019-09-28T22:32:53.4520004Z  libstdc++-devel  x86_64  4.1.2-55.el5                        base        2.8 M
2019-09-28T22:32:53.4520275Z  m4               x86_64  1.4.5-3.el5.1                       base        171 k
2019-09-28T22:32:53.4520539Z  openssl          i686    0.9.8e-40.el5_11                    updates     1.7 M
2019-09-28T22:32:53.4520790Z  xz-libs          x86_64  4.999.9-0.3.beta.20091007git.el5    base         95 k
2019-09-28T22:32:53.4521050Z  zlib             i386    1.2.3-7.el5                         base         51 k
2019-09-28T22:32:53.4521178Z Transaction Summary
2019-09-28T22:32:53.4521254Z ================================================================================
2019-09-28T22:32:53.4521347Z Install      44 Package(s)
2019-09-28T22:32:53.4521406Z Upgrade       0 Package(s)
2019-09-28T22:32:53.4521406Z Upgrade       0 Package(s)
2019-09-28T22:32:53.4521630Z 
2019-09-28T22:32:53.4521682Z Total download size: 65 M
2019-09-28T22:32:53.4521758Z Downloading Packages:
2019-09-28T22:32:57.5971240Z --------------------------------------------------------------------------------
2019-09-28T22:32:57.5971481Z Total                                            16 MB/s |  65 MB     00:04     
2019-09-28T22:32:57.6515706Z Running rpm_check_debug
2019-09-28T22:32:58.0613814Z Finished Transaction Test
2019-09-28T22:32:58.0614033Z Transaction Test Succeeded
2019-09-28T22:32:58.1412926Z Running Transaction
2019-09-28T22:32:58.3737100Z 
---
2019-09-28T22:33:02.3521285Z   Installing     : cpp                                                     8/44 
2019-09-28T22:33:02.3521509Z 
2019-09-28T22:33:02.8206831Z   Installing     : m4                                                      9/44 
2019-09-28T22:33:02.8207358Z 
2019-09-28T22:33:03.2999047Z   Installing     : xz-libs                                                10/44 
2019-09-28T22:33:03.4847464Z   Installing     : xz                                                     11/44 
2019-09-28T22:33:03.4847683Z 
2019-09-28T22:33:04.0005917Z   Installing     : gettext                                                12/44 
2019-09-28T22:33:04.0007072Z 
---
2019-09-28T22:33:06.6111929Z   Installing     : zlib                                                   19/44 
2019-09-28T22:33:07.0654703Z 
2019-09-28T22:33:07.0655284Z   Installing     : libstdc++                                              20/44 
2019-09-28T22:33:07.0655415Z 
2019-09-28T22:33:07.5235659Z   Installing     : libsepol                                               21/44 
2019-09-28T22:33:08.0138099Z   Installing     : libselinux                                             22/44 
2019-09-28T22:33:08.0138238Z 
2019-09-28T22:33:08.4989232Z   Installing     : device-mapper                                          23/44 
2019-09-28T22:33:08.4989538Z 
---
2019-09-28T22:33:10.7836645Z   Installing     : libidn                                                 27/44 
2019-09-28T22:33:10.7836790Z 
2019-09-28T22:33:11.2738426Z   Installing     : keyutils-libs                                          28/44 
2019-09-28T22:33:11.2738555Z 
2019-09-28T22:33:11.8022428Z   Installing     : krb5-libs                                              29/44 
2019-09-28T22:33:12.3075631Z   Installing     : openssl                                                30/44 
2019-09-28T22:33:12.3075837Z 
2019-09-28T22:33:13.1914879Z   Installing     : gdbm                                                   31/44 
2019-09-28T22:33:13.1916299Z 
---
2019-09-28T22:33:19.2492588Z 
2019-09-28T22:33:19.3702889Z   Installing     : gcc-c++                                                44/44 
2019-09-28T22:33:19.3703501Z 
2019-09-28T22:33:19.3705122Z Installed:
2019-09-28T22:33:19.3705839Z   autoconf.noarch 0:2.59-12                                                     
2019-09-28T22:33:19.3706292Z   bzip2.x86_64 0:1.0.3-6.el5_5                                                  
2019-09-28T22:33:19.3706751Z   curl.i386 0:7.15.5-17.el5_9                                                   
2019-09-28T22:33:19.3707067Z   curl.x86_64 0:7.15.5-17.el5_9                                                 
2019-09-28T22:33:19.3707405Z   file.x86_64 0:4.17-28                                                         
2019-09-28T22:33:19.3707739Z   gcc.x86_64 0:4.1.2-55.el5                                                     
2019-09-28T22:33:19.3708052Z   gcc-c++.x86_64 0:4.1.2-55.el5                                                 
2019-09-28T22:33:19.3708516Z   gettext.i386 0:0.17-1.el5                                                     
2019-09-28T22:33:19.3708756Z   gettext.x86_64 0:0.17-1.el5                                                   
2019-09-28T22:33:19.3709016Z   glibc-devel.i386 0:2.5-123.el5_11.3                                           
2019-09-28T22:33:19.3709285Z   glibc-devel.x86_64 0:2.5-123.el5_11.3                                         
2019-09-28T22:33:19.3709537Z   make.x86_64 1:3.81-3.el5                                                      
2019-09-28T22:33:19.3709961Z   perl.i386 4:5.8.8-43.el5_11                                                   
2019-09-28T22:33:19.3710545Z   perl.x86_64 4:5.8.8-43.el5_11                                                 
2019-09-28T22:33:19.3710802Z   pkgconfig.x86_64 1:0.21-2.el5                                                 
2019-09-28T22:33:19.3711059Z   wget.x86_64 0:1.11.4-3.el5_8.2                                                
2019-09-28T22:33:19.3711302Z   which.x86_64 0:2.16-7                                                         
2019-09-28T22:33:19.3711562Z   xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5                                  
2019-09-28T22:33:19.3711804Z   zlib-devel.i386 0:1.2.3-7.el5                                                 
2019-09-28T22:33:19.3712065Z   zlib-devel.x86_64 0:1.2.3-7.el5                                               
2019-09-28T22:33:19.3712500Z Dependency Installed:
2019-09-28T22:33:19.3712500Z Dependency Installed:
2019-09-28T22:33:19.3712780Z   cpp.x86_64 0:4.1.2-55.el5                                                     
2019-09-28T22:33:19.3713045Z   db4.i386 0:4.3.29-10.el5_5.2                                                  
2019-09-28T22:33:19.3713288Z   device-mapper.i386 0:1.02.67-2.el5_11.1                                       
2019-09-28T22:33:19.3714192Z   e2fsprogs-libs.i386 0:1.39-37.el5                                             
2019-09-28T22:33:19.3714933Z   gdbm.i386 0:1.8.0-28.el5                                                      
2019-09-28T22:33:19.3715257Z   glibc.i686 0:2.5-123.el5_11.3                                                 
2019-09-28T22:33:19.3715592Z   glibc-headers.x86_64 0:2.5-123.el5_11.3                                       
2019-09-28T22:33:19.3715899Z   imake.x86_64 0:1.0.2-3                                                        
2019-09-28T22:33:19.3716248Z   kernel-headers.x86_64 0:2.6.18-419.el5                                        
2019-09-28T22:33:19.3716584Z   keyutils-libs.i386 0:1.2-1.el5                                                
2019-09-28T22:33:19.3716897Z   krb5-libs.i386 0:1.6.1-80.el5_11                                              
2019-09-28T22:33:19.3717223Z   libgcc.i386 0:4.1.2-55.el5                                                    
2019-09-28T22:33:19.3717531Z   libgomp.i386 0:4.4.7-1.el5                                                    
2019-09-28T22:33:19.3718028Z   libgomp.x86_64 0:4.4.7-1.el5                                                  
2019-09-28T22:33:19.3718298Z   libidn.i386 0:0.6.5-1.1                                                       
2019-09-28T22:33:19.3718554Z   libidn.x86_64 0:0.6.5-1.1                                                     
2019-09-28T22:33:19.3718834Z   libselinux.i386 0:1.33.4-5.7.el5.centos                                       
2019-09-28T22:33:19.3719097Z   libsepol.i386 0:1.15.2-3.el5                                                  
2019-09-28T22:33:19.3719378Z   libstdc++.i386 0:4.1.2-55.el5                                                 
2019-09-28T22:33:19.3719634Z   libstdc++-devel.x86_64 0:4.1.2-55.el5                                         
2019-09-28T22:33:19.3719910Z   m4.x86_64 0:1.4.5-3.el5.1                                                     
2019-09-28T22:33:19.3720405Z   openssl.i686 0:0.9.8e-40.el5_11                                               
2019-09-28T22:33:19.3720653Z   xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5                             
2019-09-28T22:33:19.3720916Z   zlib.i386 0:1.2.3-7.el5                                                       
2019-09-28T22:33:19.3721037Z Complete!
2019-09-28T22:33:35.5630113Z Removing intermediate container 0bce3380585c
2019-09-28T22:33:35.5630968Z  ---> f4a959bd0590
2019-09-28T22:33:35.5631037Z Step 7/41 : ENV PATH=/rustroot/bin:$PATH
---
2019-09-28T22:33:44.8126347Z 
2019-09-28T22:33:45.0355941Z   0 5184k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-09-28T22:33:45.3968172Z   4 5184k    4  219k    0     0   249k      0  0:00:20 --:--:--  0:00:20 1011k
2019-09-28T22:33:45.3969523Z 100 5184k  100 5184k    0     0  4151k      0  0:00:01  0:00:01 --:--:-- 8862k
2019-09-28T22:33:45.4101411Z + cd openssl-1.0.2k
2019-09-28T22:33:45.4103896Z + hide_output ./config --prefix=/rustroot shared -fPIC
2019-09-28T22:33:46.7530250Z + hide_output make -j10
2019-09-28T22:33:46.7531796Z + set +x
2019-09-28T22:34:16.7586561Z Sat Sep 28 22:34:16 UTC 2019 - building ...
2019-09-28T22:34:46.7763411Z Sat Sep 28 22:34:46 UTC 2019 - building ...
2019-09-28T22:34:46.7763411Z Sat Sep 28 22:34:46 UTC 2019 - building ...
2019-09-28T22:34:55.8928019Z shared.sh: line 1:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-28T22:34:55.8929098Z + hide_output make install
2019-09-28T22:34:55.8929345Z + set +x
2019-09-28T22:35:15.5990882Z shared.sh: line 1:   350 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-28T22:35:15.5991607Z ./build-openssl.sh: line 15:  4113 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-09-28T22:35:15.5991736Z + cd ..
2019-09-28T22:35:15.5999702Z + rm -rf openssl-1.0.2k
2019-09-28T22:35:15.6593357Z + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
2019-09-28T22:35:18.1468228Z  ---> f28d1cd41346
2019-09-28T22:35:18.1469116Z Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
2019-09-28T22:35:19.5388977Z  ---> 0bfac10ecaec
2019-09-28T22:35:19.5390051Z Step 15/41 : RUN ./build-curl.sh
2019-09-28T22:35:19.5390051Z Step 15/41 : RUN ./build-curl.sh
2019-09-28T22:35:19.6806777Z  ---> Running in d570edde2a6b
2019-09-28T22:35:20.1034581Z + source shared.sh
2019-09-28T22:35:20.1034949Z + VERSION=7.66.0
2019-09-28T22:35:20.1035389Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-09-28T22:35:20.1035628Z + tar xJf -
2019-09-28T22:35:20.1035811Z tar: invalid option -- J
2019-09-28T22:35:20.1036034Z Try `tar --help' or `tar --usage' for more information.
2019-09-28T22:35:20.4400067Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-09-28T22:35:20.4400134Z 
2019-09-28T22:35:20.4400134Z 
2019-09-28T22:35:20.8415480Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 64
2019-09-28T22:35:21.9569267Z Sending build context to Docker daemon  526.3kB
2019-09-28T22:35:21.9569490Z 
2019-09-28T22:35:21.9844654Z Step 1/41 : FROM centos:5
2019-09-28T22:35:21.9850359Z  ---> 1ae98b2c895d
---
2019-09-28T22:35:22.5990794Z  ---> Running in 638aef9a589a
2019-09-28T22:35:22.6185963Z + source shared.sh
2019-09-28T22:35:22.6186141Z + VERSION=7.66.0
2019-09-28T22:35:22.6186365Z + tar xJf -
2019-09-28T22:35:22.6186571Z tar: invalid option -- J
2019-09-28T22:35:22.6186816Z Try `tar --help' or `tar --usage' for more information.
2019-09-28T22:35:22.6187080Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-09-28T22:35:23.1468798Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-09-28T22:35:23.1468850Z 
2019-09-28T22:35:23.1468850Z 
2019-09-28T22:35:23.5918556Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 64
2019-09-28T22:35:25.7521402Z Sending build context to Docker daemon  526.3kB
2019-09-28T22:35:25.7521602Z 
2019-09-28T22:35:25.7726726Z Step 1/41 : FROM centos:5
2019-09-28T22:35:25.7730430Z  ---> 1ae98b2c895d
---
2019-09-28T22:35:26.3462611Z + source shared.sh
2019-09-28T22:35:26.3462743Z + VERSION=7.66.0
2019-09-28T22:35:26.3463074Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-09-28T22:35:26.3463299Z + tar xJf -
2019-09-28T22:35:26.3474263Z tar: invalid option -- J
2019-09-28T22:35:26.3474582Z Try `tar --help' or `tar --usage' for more information.
2019-09-28T22:35:26.6243679Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-09-28T22:35:26.6243738Z 
2019-09-28T22:35:26.6243738Z 
2019-09-28T22:35:27.0393410Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 64
2019-09-28T22:35:30.1354410Z Sending build context to Docker daemon  526.3kB
2019-09-28T22:35:30.1354541Z 
2019-09-28T22:35:30.1605690Z Step 1/41 : FROM centos:5
2019-09-28T22:35:30.1611942Z  ---> 1ae98b2c895d
---
2019-09-28T22:35:30.8296792Z + source shared.sh
2019-09-28T22:35:30.8298891Z + VERSION=7.66.0
2019-09-28T22:35:30.8299763Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-09-28T22:35:30.8300363Z + tar xJf -
2019-09-28T22:35:30.8300979Z tar: invalid option -- J
2019-09-28T22:35:30.8301681Z Try `tar --help' or `tar --usage' for more information.
2019-09-28T22:35:31.3439067Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-09-28T22:35:31.3439164Z 
2019-09-28T22:35:31.3439164Z 
2019-09-28T22:35:31.7366486Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 64
2019-09-28T22:35:35.8349498Z Sending build context to Docker daemon  526.3kB
2019-09-28T22:35:35.8350388Z 
2019-09-28T22:35:35.8566661Z Step 1/41 : FROM centos:5
2019-09-28T22:35:35.8570388Z  ---> 1ae98b2c895d
---
2019-09-28T22:35:36.4679919Z + source shared.sh
2019-09-28T22:35:36.4680200Z + VERSION=7.66.0
2019-09-28T22:35:36.4680610Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-09-28T22:35:36.4680908Z + tar xJf -
2019-09-28T22:35:36.4681185Z tar: invalid option -- J
2019-09-28T22:35:36.4681460Z Try `tar --help' or `tar --usage' for more information.
2019-09-28T22:35:36.9463973Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-09-28T22:35:36.9464024Z 
2019-09-28T22:35:36.9464024Z 
2019-09-28T22:35:38.1056177Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0The command '/bin/sh -c ./build-curl.sh' returned a non-zero code: 64
2019-09-28T22:35:38.1057039Z The command has failed after 5 attempts.
2019-09-28T22:35:38.1115245Z ##[error]Bash exited with code '1'.
2019-09-28T22:35:38.1147367Z ##[section]Starting: Upload CPU usage statistics
2019-09-28T22:35:38.1150909Z ==============================================================================
2019-09-28T22:35:38.1151026Z Task         : Bash
2019-09-28T22:35:38.1151120Z Description  : Run a Bash script on macOS, Linux, or Windows
