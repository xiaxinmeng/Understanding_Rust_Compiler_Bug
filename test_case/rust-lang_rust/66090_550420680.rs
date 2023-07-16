plain
2019-11-06T17:36:44.2348357Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-11-06T17:36:44.2348503Z ==============================================================================
2019-11-06T17:36:44.3735918Z Generating script.
2019-11-06T17:36:44.3748975Z Script contents:
2019-11-06T17:36:44.3749430Z src/ci/scripts/run-build-from-ci.sh
2019-11-06T17:36:44.3793434Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/49dc3ed6-4d59-43d2-b35f-c4b67332e2cf.sh
2019-11-06T17:36:44.7840487Z info: removing rustup home
2019-11-06T17:36:44.7949359Z info: removing cargo home
2019-11-06T17:36:45.3223259Z info: removing rustup binaries
---
2019-11-06T17:37:28.1597800Z Removing intermediate container 281ed86589d6
2019-11-06T17:37:28.1599471Z  ---> 7a552992ef9a
2019-11-06T17:37:28.1600323Z Step 6/41 : RUN yum upgrade -y && yum install -y       curl       bzip2       gcc       gcc-c++       make       glibc-devel       perl       zlib-devel       file       xz       which       pkgconfig       wget       autoconf       gettext
2019-11-06T17:37:28.3646119Z  ---> Running in 2b2b9ff8eee9
2019-11-06T17:37:32.4288911Z Reducing CentOS-5 - libselinux to included packages only
2019-11-06T17:37:32.4290429Z Setting up Upgrade Process
2019-11-06T17:37:32.8973798Z Resolving Dependencies
2019-11-06T17:37:32.8981062Z --> Running transaction check
2019-11-06T17:37:32.8981062Z --> Running transaction check
2019-11-06T17:37:32.9024628Z ---> Package bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-11-06T17:37:32.9140046Z ---> Package bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-11-06T17:37:32.9243503Z ---> Package nspr.x86_64 0:4.11.0-1.el5_11 set to be updated
2019-11-06T17:37:32.9356082Z ---> Package nss.x86_64 0:3.21.3-2.el5_11 set to be updated
2019-11-06T17:37:32.9457244Z ---> Package openssl.x86_64 0:0.9.8e-40.el5_11 set to be updated
2019-11-06T17:37:32.9814511Z ---> Package tzdata.x86_64 0:2017b-1.el5 set to be updated
2019-11-06T17:37:33.1018072Z 
2019-11-06T17:37:33.1018330Z Dependencies Resolved
2019-11-06T17:37:33.1096569Z 
2019-11-06T17:37:33.1096758Z ================================================================================
2019-11-06T17:37:33.1096758Z ================================================================================
2019-11-06T17:37:33.1096934Z  Package         Arch        Version                         Repository    Size
2019-11-06T17:37:33.1097048Z ================================================================================
2019-11-06T17:37:33.1097191Z Updating:
2019-11-06T17:37:33.1098330Z  bind-libs       x86_64      30:9.3.6-25.P1.el5_11.12        updates      902 k
2019-11-06T17:37:33.1099089Z  bind-utils      x86_64      30:9.3.6-25.P1.el5_11.12        updates      181 k
2019-11-06T17:37:33.1099528Z  nspr            x86_64      4.11.0-1.el5_11                 updates      123 k
2019-11-06T17:37:33.1099884Z  nss             x86_64      3.21.3-2.el5_11                 updates      1.3 M
2019-11-06T17:37:33.1100608Z  openssl         x86_64      0.9.8e-40.el5_11                updates      1.7 M
2019-11-06T17:37:33.1101012Z  tzdata          x86_64      2017b-1.el5                     updates      757 k
2019-11-06T17:37:33.1101209Z Transaction Summary
2019-11-06T17:37:33.1101357Z ================================================================================
2019-11-06T17:37:33.1101453Z Install       0 Package(s)
2019-11-06T17:37:33.1101550Z Upgrade       6 Package(s)
2019-11-06T17:37:33.1101550Z Upgrade       6 Package(s)
2019-11-06T17:37:33.1101598Z 
2019-11-06T17:37:33.1101716Z Total download size: 4.9 M
2019-11-06T17:37:33.1101804Z Downloading Packages:
2019-11-06T17:37:33.3339712Z --------------------------------------------------------------------------------
2019-11-06T17:37:33.3342709Z Total                                            22 MB/s | 4.9 MB     00:00     
2019-11-06T17:37:33.3351265Z warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
2019-11-06T17:37:33.3395652Z Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
2019-11-06T17:37:33.4629455Z Running rpm_check_debug
2019-11-06T17:37:33.6136813Z Finished Transaction Test
2019-11-06T17:37:33.6137130Z Transaction Test Succeeded
2019-11-06T17:37:33.6703365Z Running Transaction
2019-11-06T17:37:33.8163540Z 
---
2019-11-06T17:37:38.2526696Z   Updating       : tzdata                                                  6/12 
2019-11-06T17:37:38.2527557Z 
2019-11-06T17:37:38.3190330Z   Cleanup        : bind-utils                                              7/12 
2019-11-06T17:37:38.3191092Z 
2019-11-06T17:37:39.0588821Z   Cleanup        : nss                                                     8/12 
2019-11-06T17:37:39.7987299Z   Cleanup        : bind-libs                                               9/12 
2019-11-06T17:37:39.7987474Z 
2019-11-06T17:37:40.5600258Z   Cleanup        : openssl                                                10/12 
2019-11-06T17:37:40.5659372Z 
2019-11-06T17:37:40.5659372Z 
2019-11-06T17:37:41.2969416Z   Cleanup        : nspr                                                   11/12 
2019-11-06T17:37:41.4151658Z   Cleanup        : tzdata                                                 12/12 
2019-11-06T17:37:41.4151876Z 
2019-11-06T17:37:41.4152645Z Updated:
2019-11-06T17:37:41.4152645Z Updated:
2019-11-06T17:37:41.4153966Z   bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12                                     
2019-11-06T17:37:41.4154421Z   bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12                                    
2019-11-06T17:37:41.4154831Z   nspr.x86_64 0:4.11.0-1.el5_11                                                 
2019-11-06T17:37:41.4155174Z   nss.x86_64 0:3.21.3-2.el5_11                                                  
2019-11-06T17:37:41.4155577Z   openssl.x86_64 0:0.9.8e-40.el5_11                                             
2019-11-06T17:37:41.4156323Z   tzdata.x86_64 0:2017b-1.el5                                                   
2019-11-06T17:37:41.4157669Z Complete!
2019-11-06T17:37:41.4157669Z Complete!
2019-11-06T17:37:41.6525686Z Reducing CentOS-5 - libselinux to included packages only
2019-11-06T17:37:41.6551974Z Setting up Install Process
2019-11-06T17:37:42.8403978Z Resolving Dependencies
2019-11-06T17:37:42.8410049Z --> Running transaction check
2019-11-06T17:37:42.8410049Z --> Running transaction check
2019-11-06T17:37:42.8415315Z ---> Package autoconf.noarch 0:2.59-12 set to be updated
2019-11-06T17:37:42.8846360Z --> Processing Dependency: imake for package: autoconf
2019-11-06T17:37:42.8867092Z --> Processing Dependency: m4 for package: autoconf
2019-11-06T17:37:42.8897167Z ---> Package bzip2.x86_64 0:1.0.3-6.el5_5 set to be updated
2019-11-06T17:37:42.9025220Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be updated
2019-11-06T17:37:42.9247535Z --> Processing Dependency: libc.so.6(GLIBC_2.4) for package: curl
2019-11-06T17:37:42.9467543Z --> Processing Dependency: libgssapi_krb5.so.2 for package: curl
2019-11-06T17:37:42.9514804Z --> Processing Dependency: libdl.so.2(GLIBC_2.1) for package: curl
2019-11-06T17:37:42.9577021Z --> Processing Dependency: libc.so.6(GLIBC_2.1.3) for package: curl
2019-11-06T17:37:42.9577370Z --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
2019-11-06T17:37:42.9577739Z --> Processing Dependency: libidn.so.11 for package: curl
2019-11-06T17:37:42.9634115Z --> Processing Dependency: libz.so.1 for package: curl
2019-11-06T17:37:42.9657445Z --> Processing Dependency: libc.so.6 for package: curl
2019-11-06T17:37:42.9713886Z --> Processing Dependency: libdl.so.2 for package: curl
2019-11-06T17:37:42.9714246Z --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
2019-11-06T17:37:42.9739564Z --> Processing Dependency: libc.so.6(GLIBC_2.3) for package: curl
2019-11-06T17:37:42.9795803Z --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
2019-11-06T17:37:42.9796309Z --> Processing Dependency: libk5crypto.so.3 for package: curl
2019-11-06T17:37:42.9829448Z --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
2019-11-06T17:37:42.9884761Z --> Processing Dependency: libssl.so.6 for package: curl
2019-11-06T17:37:42.9988163Z --> Processing Dependency: libcom_err.so.2 for package: curl
2019-11-06T17:37:43.0017614Z --> Processing Dependency: libcrypto.so.6 for package: curl
2019-11-06T17:37:43.0087188Z --> Processing Dependency: libc.so.6(GLIBC_2.0) for package: curl
2019-11-06T17:37:43.0153124Z --> Processing Dependency: libdl.so.2(GLIBC_2.0) for package: curl
2019-11-06T17:37:43.0153659Z --> Processing Dependency: libkrb5.so.3 for package: curl
2019-11-06T17:37:43.0176917Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be updated
2019-11-06T17:37:43.0361432Z --> Processing Dependency: libidn.so.11()(64bit) for package: curl
2019-11-06T17:37:43.0405668Z ---> Package file.x86_64 0:4.17-28 set to be updated
2019-11-06T17:37:43.0448543Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-06T17:37:43.0560297Z --> Processing Dependency: cpp = 4.1.2-55.el5 for package: gcc
2019-11-06T17:37:43.0586160Z --> Processing Dependency: libgomp >= 4.1.2-55.el5 for package: gcc
2019-11-06T17:37:43.0671456Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-06T17:37:43.0729469Z --> Processing Dependency: libstdc++-devel = 4.1.2-55.el5 for package: gcc-c++
2019-11-06T17:37:43.0828384Z ---> Package gettext.i386 0:0.17-1.el5 set to be updated
2019-11-06T17:37:43.0976828Z --> Processing Dependency: libgomp.so.1 for package: gettext
2019-11-06T17:37:43.1016440Z --> Processing Dependency: libgomp.so.1(GOMP_1.0) for package: gettext
2019-11-06T17:37:43.1016918Z --> Processing Dependency: libgcc_s.so.1 for package: gettext
2019-11-06T17:37:43.1042958Z --> Processing Dependency: libgcc_s.so.1(GCC_3.3.1) for package: gettext
2019-11-06T17:37:43.1043495Z ---> Package gettext.x86_64 0:0.17-1.el5 set to be updated
2019-11-06T17:37:43.1143857Z ---> Package glibc-devel.i386 0:2.5-123.el5_11.3 set to be updated
2019-11-06T17:37:43.1335302Z --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
2019-11-06T17:37:43.1366736Z --> Processing Dependency: glibc-headers for package: glibc-devel
2019-11-06T17:37:43.1367217Z ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-11-06T17:37:43.1572640Z ---> Package make.x86_64 1:3.81-3.el5 set to be updated
2019-11-06T17:37:43.1592808Z ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
2019-11-06T17:37:43.3370478Z --> Processing Dependency: libgdbm.so.2 for package: perl
2019-11-06T17:37:43.3388423Z --> Processing Dependency: libdb-4.3.so for package: perl
2019-11-06T17:37:43.3408938Z ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
2019-11-06T17:37:43.3721801Z ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
2019-11-06T17:37:43.3729304Z ---> Package wget.x86_64 0:1.11.4-3.el5_8.2 set to be updated
2019-11-06T17:37:43.3749664Z ---> Package which.x86_64 0:2.16-7 set to be updated
2019-11-06T17:37:43.3795865Z ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-11-06T17:37:43.3837993Z --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
2019-11-06T17:37:43.3943867Z --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
2019-11-06T17:37:43.3944783Z ---> Package zlib-devel.i386 0:1.2.3-7.el5 set to be updated
2019-11-06T17:37:43.3961187Z ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
2019-11-06T17:37:43.3970612Z --> Running transaction check
2019-11-06T17:37:43.3971118Z ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-06T17:37:43.3985828Z ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
2019-11-06T17:37:43.4053528Z --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
2019-11-06T17:37:43.4085039Z --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
2019-11-06T17:37:43.4086929Z --> Processing Dependency: libstdc++.so.6 for package: db4
2019-11-06T17:37:43.4088564Z ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
2019-11-06T17:37:43.4141402Z --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
2019-11-06T17:37:43.4171154Z ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
2019-11-06T17:37:43.4182742Z ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
2019-11-06T17:37:43.4299202Z ---> Package glibc-headers.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-11-06T17:37:43.4332375Z --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
2019-11-06T17:37:43.4425118Z --> Processing Dependency: kernel-headers for package: glibc-headers
2019-11-06T17:37:43.4425527Z ---> Package imake.x86_64 0:1.0.2-3 set to be updated
2019-11-06T17:37:43.4438822Z ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
2019-11-06T17:37:43.4543253Z --> Processing Dependency: libkeyutils.so.1 for package: krb5-libs
2019-11-06T17:37:43.4566309Z --> Processing Dependency: libselinux.so.1 for package: krb5-libs
2019-11-06T17:37:43.4596726Z --> Processing Dependency: libkeyutils.so.1(KEYUTILS_0.3) for package: krb5-libs
2019-11-06T17:37:43.4610442Z ---> Package libgcc.i386 0:4.1.2-55.el5 set to be updated
2019-11-06T17:37:43.4636043Z ---> Package libgomp.i386 0:4.4.7-1.el5 set to be updated
2019-11-06T17:37:43.4667970Z ---> Package libgomp.x86_64 0:4.4.7-1.el5 set to be updated
2019-11-06T17:37:43.4699655Z ---> Package libidn.i386 0:0.6.5-1.1 set to be updated
2019-11-06T17:37:43.4716682Z ---> Package libidn.x86_64 0:0.6.5-1.1 set to be updated
2019-11-06T17:37:43.4731284Z ---> Package libstdc++-devel.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-06T17:37:48.8787690Z ---> Package m4.x86_64 0:1.4.5-3.el5.1 set to be updated
2019-11-06T17:37:48.8830039Z ---> Package openssl.i686 0:0.9.8e-40.el5_11 set to be updated
2019-11-06T17:37:48.8860801Z ---> Package xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-11-06T17:37:48.8873201Z ---> Package zlib.i386 0:1.2.3-7.el5 set to be updated
2019-11-06T17:37:48.8887975Z --> Running transaction check
2019-11-06T17:37:48.8896167Z ---> Package device-mapper.i386 0:1.02.67-2.el5_11.1 set to be updated
2019-11-06T17:37:48.8929338Z --> Processing Dependency: libsepol.so.1 for package: device-mapper
2019-11-06T17:37:48.8964998Z ---> Package kernel-headers.x86_64 0:2.6.18-419.el5 set to be updated
2019-11-06T17:37:48.8969208Z ---> Package keyutils-libs.i386 0:1.2-1.el5 set to be updated
2019-11-06T17:37:48.8983529Z ---> Package libselinux.i386 0:1.33.4-5.7.el5.centos set to be updated
2019-11-06T17:37:48.9033276Z ---> Package libstdc++.i386 0:4.1.2-55.el5 set to be updated
2019-11-06T17:37:48.9082989Z --> Running transaction check
2019-11-06T17:37:48.9083449Z ---> Package libsepol.i386 0:1.15.2-3.el5 set to be updated
2019-11-06T17:37:49.0492529Z 
2019-11-06T17:37:49.0492809Z Dependencies Resolved
2019-11-06T17:37:49.0994869Z 
2019-11-06T17:37:49.0995176Z ================================================================================
2019-11-06T17:37:49.0995176Z ================================================================================
2019-11-06T17:37:49.0995361Z  Package          Arch    Version                             Repository   Size
2019-11-06T17:37:49.0995517Z ================================================================================
2019-11-06T17:37:49.0995606Z Installing:
2019-11-06T17:37:49.0996496Z  autoconf         noarch  2.59-12                             base        647 k
2019-11-06T17:37:49.0996843Z  bzip2            x86_64  1.0.3-6.el5_5                       base         50 k
2019-11-06T17:37:49.0997218Z  curl             i386    7.15.5-17.el5_9                     base        235 k
2019-11-06T17:37:49.0997536Z  curl             x86_64  7.15.5-17.el5_9                     base        232 k
2019-11-06T17:37:49.0997882Z  file             x86_64  4.17-28                             base        321 k
2019-11-06T17:37:49.0998214Z  gcc              x86_64  4.1.2-55.el5                        base        5.3 M
2019-11-06T17:37:49.0998536Z  gcc-c++          x86_64  4.1.2-55.el5                        base        3.8 M
2019-11-06T17:37:49.0998868Z  gettext          i386    0.17-1.el5                          base        2.4 M
2019-11-06T17:37:49.0999175Z  gettext          x86_64  0.17-1.el5                          base        2.4 M
2019-11-06T17:37:49.0999504Z  glibc-devel      i386    2.5-123.el5_11.3                    updates     2.1 M
2019-11-06T17:37:49.0999845Z  glibc-devel      x86_64  2.5-123.el5_11.3                    updates     2.4 M
2019-11-06T17:37:49.1000155Z  make             x86_64  1:3.81-3.el5                        base        470 k
2019-11-06T17:37:49.1000472Z  perl             i386    4:5.8.8-43.el5_11                   updates      12 M
2019-11-06T17:37:49.1000774Z  perl             x86_64  4:5.8.8-43.el5_11                   updates      12 M
2019-11-06T17:37:49.1001095Z  pkgconfig        x86_64  1:0.21-2.el5                        base         61 k
2019-11-06T17:37:49.1001405Z  wget             x86_64  1.11.4-3.el5_8.2                    base        583 k
2019-11-06T17:37:49.1001730Z  which            x86_64  2.16-7                              base         24 k
2019-11-06T17:37:49.1002385Z  xz               x86_64  4.999.9-0.3.beta.20091007git.el5    base        146 k
2019-11-06T17:37:49.1002751Z  zlib-devel       i386    1.2.3-7.el5                         base        102 k
2019-11-06T17:37:49.1003081Z  zlib-devel       x86_64  1.2.3-7.el5                         base        103 k
2019-11-06T17:37:49.1003172Z Installing for dependencies:
2019-11-06T17:37:49.1003483Z  cpp              x86_64  4.1.2-55.el5                        base        2.9 M
2019-11-06T17:37:49.1003803Z  db4              i386    4.3.29-10.el5_5.2                   base        910 k
2019-11-06T17:37:49.1004105Z  device-mapper    i386    1.02.67-2.el5_11.1                  updates     804 k
2019-11-06T17:37:49.1004819Z  e2fsprogs-libs   i386    1.39-37.el5                         base        120 k
2019-11-06T17:37:49.1005307Z  gdbm             i386    1.8.0-28.el5                        base         28 k
2019-11-06T17:37:49.1005643Z  glibc            i686    2.5-123.el5_11.3                    updates     5.4 M
2019-11-06T17:37:49.1005947Z  glibc-headers    x86_64  2.5-123.el5_11.3                    updates     602 k
2019-11-06T17:37:49.1006270Z  imake            x86_64  1.0.2-3                             base        319 k
2019-11-06T17:37:49.1006590Z  kernel-headers   x86_64  2.6.18-419.el5                      updates     1.5 M
2019-11-06T17:37:49.1006893Z  keyutils-libs    i386    1.2-1.el5                           base         18 k
2019-11-06T17:37:49.1007215Z  krb5-libs        i386    1.6.1-80.el5_11                     updates     670 k
2019-11-06T17:37:49.1007513Z  libgcc           i386    4.1.2-55.el5                        base         97 k
2019-11-06T17:37:49.1007831Z  libgomp          i386    4.4.7-1.el5                         base         74 k
2019-11-06T17:37:49.1008149Z  libgomp          x86_64  4.4.7-1.el5                         base         71 k
2019-11-06T17:37:49.1008460Z  libidn           i386    0.6.5-1.1                           base        194 k
2019-11-06T17:37:49.1008787Z  libidn           x86_64  0.6.5-1.1                           base        195 k
2019-11-06T17:37:49.1009086Z  libselinux       i386    1.33.4-5.7.el5.centos               libselinux   77 k
2019-11-06T17:37:49.1009402Z  libsepol         i386    1.15.2-3.el5                        base        128 k
2019-11-06T17:37:49.1009721Z  libstdc++        i386    4.1.2-55.el5                        base        364 k
2019-11-06T17:37:49.1010021Z  libstdc++-devel  x86_64  4.1.2-55.el5                        base        2.8 M
2019-11-06T17:37:49.1010342Z  m4               x86_64  1.4.5-3.el5.1                       base        171 k
2019-11-06T17:37:49.1010646Z  openssl          i686    0.9.8e-40.el5_11                    updates     1.7 M
2019-11-06T17:37:49.1010972Z  xz-libs          x86_64  4.999.9-0.3.beta.20091007git.el5    base         95 k
2019-11-06T17:37:49.1011281Z  zlib             i386    1.2.3-7.el5                         base         51 k
2019-11-06T17:37:49.1011468Z Transaction Summary
2019-11-06T17:37:49.1011573Z ================================================================================
2019-11-06T17:37:49.1011659Z Install      44 Package(s)
2019-11-06T17:37:49.1011747Z Upgrade       0 Package(s)
2019-11-06T17:37:49.1011747Z Upgrade       0 Package(s)
2019-11-06T17:37:49.1011790Z 
2019-11-06T17:37:49.1011869Z Total download size: 65 M
2019-11-06T17:37:49.1011937Z Downloading Packages:
2019-11-06T17:37:51.0179304Z --------------------------------------------------------------------------------
2019-11-06T17:37:51.0183204Z Total                                            34 MB/s |  65 MB     00:01     
2019-11-06T17:37:51.0740465Z Running rpm_check_debug
2019-11-06T17:37:51.5056148Z Finished Transaction Test
2019-11-06T17:37:51.5057039Z Transaction Test Succeeded
2019-11-06T17:37:51.6031961Z Running Transaction
2019-11-06T17:37:51.8754838Z 
---
2019-11-06T17:37:57.6153276Z   Installing     : cpp                                                     8/44 
2019-11-06T17:37:57.6153547Z 
2019-11-06T17:37:58.3761928Z   Installing     : m4                                                      9/44 
2019-11-06T17:37:58.3762519Z 
2019-11-06T17:37:59.1792895Z   Installing     : xz-libs                                                10/44 
2019-11-06T17:37:59.4082429Z   Installing     : xz                                                     11/44 
2019-11-06T17:37:59.4082708Z 
2019-11-06T17:38:00.2593922Z   Installing     : gettext                                                12/44 
2019-11-06T17:38:00.2594160Z 
---
2019-11-06T17:38:04.7930971Z   Installing     : zlib                                                   19/44 
2019-11-06T17:38:04.7931201Z 
2019-11-06T17:38:05.6322749Z   Installing     : libstdc++                                              20/44 
2019-11-06T17:38:05.6322973Z 
2019-11-06T17:38:06.4031794Z   Installing     : libsepol                                               21/44 
2019-11-06T17:38:07.2215762Z   Installing     : libselinux                                             22/44 
2019-11-06T17:38:07.2216555Z 
2019-11-06T17:38:07.9771475Z   Installing     : device-mapper                                          23/44 
2019-11-06T17:38:07.9772101Z 
---
2019-11-06T17:38:11.2242121Z   Installing     : libidn                                                 27/44 
2019-11-06T17:38:11.2242720Z 
2019-11-06T17:38:12.0064564Z   Installing     : keyutils-libs                                          28/44 
2019-11-06T17:38:12.0065320Z 
2019-11-06T17:38:12.9294231Z   Installing     : krb5-libs                                              29/44 
2019-11-06T17:38:13.8280255Z   Installing     : openssl                                                30/44 
2019-11-06T17:38:13.8281134Z 
2019-11-06T17:38:15.3352722Z   Installing     : gdbm                                                   31/44 
2019-11-06T17:38:15.3353669Z 
---
2019-11-06T17:38:28.0292846Z 
2019-11-06T17:38:28.1657172Z   Installing     : gcc-c++                                                44/44 
2019-11-06T17:38:28.1658115Z 
2019-11-06T17:38:28.1658686Z Installed:
2019-11-06T17:38:28.1659397Z   autoconf.noarch 0:2.59-12                                                     
2019-11-06T17:38:28.1659750Z   bzip2.x86_64 0:1.0.3-6.el5_5                                                  
2019-11-06T17:38:28.1660168Z   curl.i386 0:7.15.5-17.el5_9                                                   
2019-11-06T17:38:28.1660622Z   curl.x86_64 0:7.15.5-17.el5_9                                                 
2019-11-06T17:38:28.1660919Z   file.x86_64 0:4.17-28                                                         
2019-11-06T17:38:28.1661313Z   gcc.x86_64 0:4.1.2-55.el5                                                     
2019-11-06T17:38:28.1661577Z   gcc-c++.x86_64 0:4.1.2-55.el5                                                 
2019-11-06T17:38:28.1661858Z   gettext.i386 0:0.17-1.el5                                                     
2019-11-06T17:38:28.1662150Z   gettext.x86_64 0:0.17-1.el5                                                   
2019-11-06T17:38:28.1662534Z   glibc-devel.i386 0:2.5-123.el5_11.3                                           
2019-11-06T17:38:28.1663185Z   glibc-devel.x86_64 0:2.5-123.el5_11.3                                         
2019-11-06T17:38:28.1663762Z   make.x86_64 1:3.81-3.el5                                                      
2019-11-06T17:38:28.1664104Z   perl.i386 4:5.8.8-43.el5_11                                                   
2019-11-06T17:38:28.1664407Z   perl.x86_64 4:5.8.8-43.el5_11                                                 
2019-11-06T17:38:28.1664730Z   pkgconfig.x86_64 1:0.21-2.el5                                                 
2019-11-06T17:38:28.1665204Z   wget.x86_64 0:1.11.4-3.el5_8.2                                                
2019-11-06T17:38:28.1665549Z   which.x86_64 0:2.16-7                                                         
2019-11-06T17:38:28.1665873Z   xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5                                  
2019-11-06T17:38:28.1666382Z   zlib-devel.i386 0:1.2.3-7.el5                                                 
2019-11-06T17:38:28.1667004Z   zlib-devel.x86_64 0:1.2.3-7.el5                                               
2019-11-06T17:38:28.1667177Z Dependency Installed:
2019-11-06T17:38:28.1667177Z Dependency Installed:
2019-11-06T17:38:28.1667716Z   cpp.x86_64 0:4.1.2-55.el5                                                     
2019-11-06T17:38:28.1668056Z   db4.i386 0:4.3.29-10.el5_5.2                                                  
2019-11-06T17:38:28.1668500Z   device-mapper.i386 0:1.02.67-2.el5_11.1                                       
2019-11-06T17:38:28.1668855Z   e2fsprogs-libs.i386 0:1.39-37.el5                                             
2019-11-06T17:38:28.1669176Z   gdbm.i386 0:1.8.0-28.el5                                                      
2019-11-06T17:38:28.1669595Z   glibc.i686 0:2.5-123.el5_11.3                                                 
2019-11-06T17:38:28.1670207Z   glibc-headers.x86_64 0:2.5-123.el5_11.3                                       
2019-11-06T17:38:28.1670741Z   imake.x86_64 0:1.0.2-3                                                        
2019-11-06T17:38:28.1671123Z   kernel-headers.x86_64 0:2.6.18-419.el5                                        
2019-11-06T17:38:28.1671451Z   keyutils-libs.i386 0:1.2-1.el5                                                
2019-11-06T17:38:28.1671889Z   krb5-libs.i386 0:1.6.1-80.el5_11                                              
2019-11-06T17:38:28.1672587Z   libgcc.i386 0:4.1.2-55.el5                                                    
2019-11-06T17:38:28.1673073Z   libgomp.i386 0:4.4.7-1.el5                                                    
2019-11-06T17:38:28.1673543Z   libgomp.x86_64 0:4.4.7-1.el5                                                  
2019-11-06T17:38:28.1673869Z   libidn.i386 0:0.6.5-1.1                                                       
2019-11-06T17:38:28.1674166Z   libidn.x86_64 0:0.6.5-1.1                                                     
2019-11-06T17:38:28.1674660Z   libselinux.i386 0:1.33.4-5.7.el5.centos                                       
2019-11-06T17:38:28.1674999Z   libsepol.i386 0:1.15.2-3.el5                                                  
2019-11-06T17:38:28.1675303Z   libstdc++.i386 0:4.1.2-55.el5                                                 
2019-11-06T17:38:28.1675780Z   libstdc++-devel.x86_64 0:4.1.2-55.el5                                         
2019-11-06T17:38:28.1676218Z   m4.x86_64 0:1.4.5-3.el5.1                                                     
2019-11-06T17:38:28.1676521Z   openssl.i686 0:0.9.8e-40.el5_11                                               
2019-11-06T17:38:28.1676953Z   xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5                             
2019-11-06T17:38:28.1677266Z   zlib.i386 0:1.2.3-7.el5                                                       
2019-11-06T17:38:28.1677402Z Complete!
2019-11-06T17:38:42.8476741Z Removing intermediate container 2b2b9ff8eee9
2019-11-06T17:38:42.8477671Z  ---> 6c88688daedf
2019-11-06T17:38:42.8477793Z Step 7/41 : ENV PATH=/rustroot/bin:$PATH
---
2019-11-06T17:38:52.8855245Z + URL=https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:38:52.8855649Z + curl https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:38:52.8855943Z + tar xzf -
2019-11-06T17:39:01.1466096Z 
2019-11-06T17:39:01.1467243Z curl: (35) error:14077410:SSL routines:SSL23_GET_SERVER_HELLO:sslv3 alert handshake failure
2019-11-06T17:39:01.1484187Z 
2019-11-06T17:39:01.1484755Z gzip: stdin: unexpected end of file
2019-11-06T17:39:01.1493019Z tar: Child returned status 1
2019-11-06T17:39:01.1497473Z tar: Error exit delayed from previous errors
2019-11-06T17:39:01.7146720Z The command '/bin/sh -c ./build-openssl.sh' returned a non-zero code: 2
2019-11-06T17:39:02.9190634Z Sending build context to Docker daemon  525.3kB
2019-11-06T17:39:02.9191410Z 
2019-11-06T17:39:02.9433737Z Step 1/41 : FROM centos:5
2019-11-06T17:39:02.9440600Z  ---> 1ae98b2c895d
---
2019-11-06T17:39:03.7466645Z + URL=https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:03.7467054Z + curl https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:03.7467599Z + tar xzf -
2019-11-06T17:39:11.9630506Z 
2019-11-06T17:39:11.9631604Z curl: (35) error:14077410:SSL routines:SSL23_GET_SERVER_HELLO:sslv3 alert handshake failure
2019-11-06T17:39:11.9637929Z 
2019-11-06T17:39:11.9638495Z gzip: stdin: unexpected end of file
2019-11-06T17:39:11.9643908Z tar: Child returned status 1
2019-11-06T17:39:11.9648201Z tar: Error exit delayed from previous errors
2019-11-06T17:39:12.4115710Z The command '/bin/sh -c ./build-openssl.sh' returned a non-zero code: 2
2019-11-06T17:39:14.5082956Z Sending build context to Docker daemon  525.3kB
2019-11-06T17:39:14.5083576Z 
2019-11-06T17:39:14.5272979Z Step 1/41 : FROM centos:5
2019-11-06T17:39:14.5278888Z  ---> 1ae98b2c895d
---
2019-11-06T17:39:15.1887312Z + URL=https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:15.1887606Z + curl https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:15.1887846Z + tar xzf -
2019-11-06T17:39:23.4190567Z 
2019-11-06T17:39:23.4190793Z curl: (35) error:14077410:SSL routines:SSL23_GET_SERVER_HELLO:sslv3 alert handshake failure
2019-11-06T17:39:23.4191103Z 
2019-11-06T17:39:23.4191200Z gzip: stdin: unexpected end of file
2019-11-06T17:39:23.4191302Z tar: Child returned status 1
2019-11-06T17:39:23.4191388Z tar: Error exit delayed from previous errors
2019-11-06T17:39:23.8325507Z The command '/bin/sh -c ./build-openssl.sh' returned a non-zero code: 2
2019-11-06T17:39:26.9852565Z Sending build context to Docker daemon  525.3kB
2019-11-06T17:39:26.9853403Z 
2019-11-06T17:39:27.0151938Z Step 1/41 : FROM centos:5
2019-11-06T17:39:27.0157452Z  ---> 1ae98b2c895d
---
2019-11-06T17:39:27.6672559Z + URL=https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:27.6673164Z + tar xzf -
2019-11-06T17:39:27.6673713Z + curl https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:35.8982138Z 
2019-11-06T17:39:35.8982490Z curl: (35) error:14077410:SSL routines:SSL23_GET_SERVER_HELLO:sslv3 alert handshake failure
2019-11-06T17:39:35.8986923Z 
2019-11-06T17:39:35.8987441Z gzip: stdin: unexpected end of file
2019-11-06T17:39:35.8987903Z tar: Child returned status 1
2019-11-06T17:39:35.8988388Z tar: Error exit delayed from previous errors
2019-11-06T17:39:36.2835144Z The command '/bin/sh -c ./build-openssl.sh' returned a non-zero code: 2
2019-11-06T17:39:40.4060949Z Sending build context to Docker daemon  525.3kB
2019-11-06T17:39:40.4062035Z 
2019-11-06T17:39:40.4270732Z Step 1/41 : FROM centos:5
2019-11-06T17:39:40.4276554Z  ---> 1ae98b2c895d
---
2019-11-06T17:39:41.1961585Z + URL=https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:41.1962147Z + curl https://ci-mirrors.rust-lang.org/rustc/openssl-1.0.2k.tar.gz
2019-11-06T17:39:41.1962624Z + tar xzf -
2019-11-06T17:39:49.4363260Z 
2019-11-06T17:39:49.4364464Z curl: (35) error:14077410:SSL routines:SSL23_GET_SERVER_HELLO:sslv3 alert handshake failure
2019-11-06T17:39:49.4370684Z 
2019-11-06T17:39:49.4371291Z gzip: stdin: unexpected end of file
2019-11-06T17:39:49.4372297Z tar: Child returned status 1
2019-11-06T17:39:49.4372447Z tar: Error exit delayed from previous errors
2019-11-06T17:39:49.8092045Z The command '/bin/sh -c ./build-openssl.sh' returned a non-zero code: 2
2019-11-06T17:39:49.8133126Z 
2019-11-06T17:39:49.8133126Z 
2019-11-06T17:39:49.8249569Z ##[error]Bash exited with code '1'.
2019-11-06T17:39:49.8287790Z ##[section]Starting: Checkout
2019-11-06T17:39:49.8290185Z ==============================================================================
2019-11-06T17:39:49.8290310Z Task         : Get sources
2019-11-06T17:39:49.8290407Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
