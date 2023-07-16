plain
2019-11-26T08:42:30.2847942Z  ---> Running in 417ba35deb40
2019-11-26T08:42:32.1003781Z Removing intermediate container 417ba35deb40
2019-11-26T08:42:32.1005581Z  ---> 35321c46e35e
2019-11-26T08:42:32.1007179Z Step 6/41 : RUN yum upgrade -y && yum install -y       curl       bzip2       gcc       gcc-c++       make       glibc-devel       perl       zlib-devel       file       xz       which       pkgconfig       wget       autoconf       gettext
2019-11-26T08:42:32.2890312Z  ---> Running in 3b502a9b9e7b
2019-11-26T08:42:36.7442459Z Reducing CentOS-5 - libselinux to included packages only
2019-11-26T08:42:36.7454037Z Setting up Upgrade Process
2019-11-26T08:42:37.2681651Z Resolving Dependencies
2019-11-26T08:42:37.2682483Z --> Running transaction check
2019-11-26T08:42:37.2682483Z --> Running transaction check
2019-11-26T08:42:38.0288460Z ---> Package bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-11-26T08:42:38.0295704Z ---> Package bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-11-26T08:42:38.0296941Z ---> Package nspr.x86_64 0:4.11.0-1.el5_11 set to be updated
2019-11-26T08:42:38.0297518Z ---> Package nss.x86_64 0:3.21.3-2.el5_11 set to be updated
2019-11-26T08:42:38.0298194Z ---> Package openssl.x86_64 0:0.9.8e-40.el5_11 set to be updated
2019-11-26T08:42:38.0298797Z ---> Package tzdata.x86_64 0:2017b-1.el5 set to be updated
2019-11-26T08:42:38.0299621Z 
2019-11-26T08:42:38.0299796Z Dependencies Resolved
2019-11-26T08:42:38.0299909Z 
2019-11-26T08:42:38.0300086Z ================================================================================
2019-11-26T08:42:38.0300086Z ================================================================================
2019-11-26T08:42:38.0300247Z  Package         Arch        Version                         Repository    Size
2019-11-26T08:42:38.0300449Z ================================================================================
2019-11-26T08:42:38.0300607Z Updating:
2019-11-26T08:42:38.0301004Z  bind-libs       x86_64      30:9.3.6-25.P1.el5_11.12        updates      902 k
2019-11-26T08:42:38.0301432Z  bind-utils      x86_64      30:9.3.6-25.P1.el5_11.12        updates      181 k
2019-11-26T08:42:38.0301820Z  nspr            x86_64      4.11.0-1.el5_11                 updates      123 k
2019-11-26T08:42:38.0302257Z  nss             x86_64      3.21.3-2.el5_11                 updates      1.3 M
2019-11-26T08:42:38.0302688Z  openssl         x86_64      0.9.8e-40.el5_11                updates      1.7 M
2019-11-26T08:42:38.0303075Z  tzdata          x86_64      2017b-1.el5                     updates      757 k
2019-11-26T08:42:38.0303404Z Transaction Summary
2019-11-26T08:42:38.0303592Z ================================================================================
2019-11-26T08:42:38.0303733Z Install       0 Package(s)
2019-11-26T08:42:38.0303927Z Upgrade       6 Package(s)
2019-11-26T08:42:38.0303927Z Upgrade       6 Package(s)
2019-11-26T08:42:38.0304031Z 
2019-11-26T08:42:38.0304195Z Total download size: 4.9 M
2019-11-26T08:42:38.0304360Z Downloading Packages:
2019-11-26T08:42:38.0304707Z --------------------------------------------------------------------------------
2019-11-26T08:42:38.0327290Z Total                                           8.5 MB/s | 4.9 MB     00:00     
2019-11-26T08:42:38.0338496Z warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
2019-11-26T08:42:38.0388546Z Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
2019-11-26T08:42:38.2100051Z Running rpm_check_debug
2019-11-26T08:42:38.3395058Z Finished Transaction Test
2019-11-26T08:42:38.3395190Z Transaction Test Succeeded
2019-11-26T08:42:38.3845106Z Running Transaction
2019-11-26T08:42:38.5264701Z 
---
2019-11-26T08:42:41.4556694Z   Updating       : tzdata                                                  6/12 
2019-11-26T08:42:41.4557304Z 
2019-11-26T08:42:41.5567705Z   Cleanup        : bind-utils                                              7/12 
2019-11-26T08:42:41.5568315Z 
2019-11-26T08:42:42.0264823Z   Cleanup        : nss                                                     8/12 
2019-11-26T08:42:42.4767691Z   Cleanup        : bind-libs                                               9/12 
2019-11-26T08:42:42.4767862Z 
2019-11-26T08:42:42.9342177Z   Cleanup        : openssl                                                10/12 
2019-11-26T08:42:42.9342843Z 
2019-11-26T08:42:42.9342843Z 
2019-11-26T08:42:43.3836901Z   Cleanup        : nspr                                                   11/12 
2019-11-26T08:42:43.4841395Z   Cleanup        : tzdata                                                 12/12 
2019-11-26T08:42:43.4841726Z 
2019-11-26T08:42:43.4841831Z Updated:
2019-11-26T08:42:43.4841831Z Updated:
2019-11-26T08:42:43.4842695Z   bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12                                     
2019-11-26T08:42:43.4843239Z   bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12                                    
2019-11-26T08:42:43.4843652Z   nspr.x86_64 0:4.11.0-1.el5_11                                                 
2019-11-26T08:42:43.4843968Z   nss.x86_64 0:3.21.3-2.el5_11                                                  
2019-11-26T08:42:43.4844256Z   openssl.x86_64 0:0.9.8e-40.el5_11                                             
2019-11-26T08:42:43.4844555Z   tzdata.x86_64 0:2017b-1.el5                                                   
2019-11-26T08:42:43.4844689Z Complete!
2019-11-26T08:42:43.4844689Z Complete!
2019-11-26T08:42:43.6720685Z Reducing CentOS-5 - libselinux to included packages only
2019-11-26T08:42:43.6734293Z Setting up Install Process
2019-11-26T08:42:44.6339986Z Resolving Dependencies
2019-11-26T08:42:44.6340770Z --> Running transaction check
2019-11-26T08:42:44.6340770Z --> Running transaction check
2019-11-26T08:42:44.6341030Z ---> Package autoconf.noarch 0:2.59-12 set to be updated
2019-11-26T08:42:44.6720371Z --> Processing Dependency: imake for package: autoconf
2019-11-26T08:42:44.6746868Z --> Processing Dependency: m4 for package: autoconf
2019-11-26T08:42:44.6771076Z ---> Package bzip2.x86_64 0:1.0.3-6.el5_5 set to be updated
2019-11-26T08:42:44.6887469Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be updated
2019-11-26T08:42:44.7082583Z --> Processing Dependency: libc.so.6(GLIBC_2.4) for package: curl
2019-11-26T08:42:44.7270959Z --> Processing Dependency: libgssapi_krb5.so.2 for package: curl
2019-11-26T08:42:44.7310480Z --> Processing Dependency: libdl.so.2(GLIBC_2.1) for package: curl
2019-11-26T08:42:44.7365289Z --> Processing Dependency: libc.so.6(GLIBC_2.1.3) for package: curl
2019-11-26T08:42:44.7365847Z --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
2019-11-26T08:42:44.7366099Z --> Processing Dependency: libidn.so.11 for package: curl
2019-11-26T08:42:44.7421222Z --> Processing Dependency: libz.so.1 for package: curl
2019-11-26T08:42:44.7440538Z --> Processing Dependency: libc.so.6 for package: curl
2019-11-26T08:42:44.7490887Z --> Processing Dependency: libdl.so.2 for package: curl
2019-11-26T08:42:44.7491277Z --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
2019-11-26T08:42:44.7514522Z --> Processing Dependency: libc.so.6(GLIBC_2.3) for package: curl
2019-11-26T08:42:44.7580320Z --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
2019-11-26T08:42:44.7580879Z --> Processing Dependency: libk5crypto.so.3 for package: curl
2019-11-26T08:42:44.7594641Z --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
2019-11-26T08:42:44.7641684Z --> Processing Dependency: libssl.so.6 for package: curl
2019-11-26T08:42:44.7727016Z --> Processing Dependency: libcom_err.so.2 for package: curl
2019-11-26T08:42:44.7751631Z --> Processing Dependency: libcrypto.so.6 for package: curl
2019-11-26T08:42:44.7810065Z --> Processing Dependency: libc.so.6(GLIBC_2.0) for package: curl
2019-11-26T08:42:44.7852552Z --> Processing Dependency: libdl.so.2(GLIBC_2.0) for package: curl
2019-11-26T08:42:44.7852882Z --> Processing Dependency: libkrb5.so.3 for package: curl
2019-11-26T08:42:44.7874887Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be updated
2019-11-26T08:42:44.8013371Z --> Processing Dependency: libidn.so.11()(64bit) for package: curl
2019-11-26T08:42:44.8051708Z ---> Package file.x86_64 0:4.17-28 set to be updated
2019-11-26T08:42:44.8089410Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-26T08:42:44.8169617Z --> Processing Dependency: cpp = 4.1.2-55.el5 for package: gcc
2019-11-26T08:42:44.8193025Z --> Processing Dependency: libgomp >= 4.1.2-55.el5 for package: gcc
2019-11-26T08:42:44.8280552Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-26T08:42:44.8343278Z --> Processing Dependency: libstdc++-devel = 4.1.2-55.el5 for package: gcc-c++
2019-11-26T08:42:44.8442556Z ---> Package gettext.i386 0:0.17-1.el5 set to be updated
2019-11-26T08:42:44.8581325Z --> Processing Dependency: libgomp.so.1 for package: gettext
2019-11-26T08:42:44.8617893Z --> Processing Dependency: libgomp.so.1(GOMP_1.0) for package: gettext
2019-11-26T08:42:44.8618234Z --> Processing Dependency: libgcc_s.so.1 for package: gettext
2019-11-26T08:42:44.8642758Z --> Processing Dependency: libgcc_s.so.1(GCC_3.3.1) for package: gettext
2019-11-26T08:42:44.8643490Z ---> Package gettext.x86_64 0:0.17-1.el5 set to be updated
2019-11-26T08:42:44.8738415Z ---> Package glibc-devel.i386 0:2.5-123.el5_11.3 set to be updated
2019-11-26T08:42:44.8919159Z --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
2019-11-26T08:42:44.8947339Z --> Processing Dependency: glibc-headers for package: glibc-devel
2019-11-26T08:42:44.8947786Z ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-11-26T08:42:44.9144856Z ---> Package make.x86_64 1:3.81-3.el5 set to be updated
2019-11-26T08:42:44.9158336Z ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
2019-11-26T08:42:45.0741050Z --> Processing Dependency: libgdbm.so.2 for package: perl
2019-11-26T08:42:45.0759763Z --> Processing Dependency: libdb-4.3.so for package: perl
2019-11-26T08:42:45.0780998Z ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
2019-11-26T08:42:45.1072487Z ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
2019-11-26T08:42:45.1084121Z ---> Package wget.x86_64 0:1.11.4-3.el5_8.2 set to be updated
2019-11-26T08:42:45.1104166Z ---> Package which.x86_64 0:2.16-7 set to be updated
2019-11-26T08:42:45.1134654Z ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-11-26T08:42:45.1165338Z --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
2019-11-26T08:42:45.1249216Z --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
2019-11-26T08:42:45.1249636Z ---> Package zlib-devel.i386 0:1.2.3-7.el5 set to be updated
2019-11-26T08:42:45.1264094Z ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
2019-11-26T08:42:45.1273539Z --> Running transaction check
2019-11-26T08:42:45.1274082Z ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-26T08:42:45.1287093Z ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
2019-11-26T08:42:45.1344098Z --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
2019-11-26T08:42:45.1376012Z --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
2019-11-26T08:42:45.1377005Z --> Processing Dependency: libstdc++.so.6 for package: db4
2019-11-26T08:42:45.1377383Z ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
2019-11-26T08:42:45.1421370Z --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
2019-11-26T08:42:45.1448912Z ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
2019-11-26T08:42:45.1459191Z ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
2019-11-26T08:42:45.1538952Z ---> Package glibc-headers.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-11-26T08:42:45.1581974Z --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
2019-11-26T08:42:45.1707066Z --> Processing Dependency: kernel-headers for package: glibc-headers
2019-11-26T08:42:45.1707468Z ---> Package imake.x86_64 0:1.0.2-3 set to be updated
2019-11-26T08:42:45.1707814Z ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
2019-11-26T08:42:45.1783010Z --> Processing Dependency: libkeyutils.so.1 for package: krb5-libs
2019-11-26T08:42:45.1807523Z --> Processing Dependency: libselinux.so.1 for package: krb5-libs
2019-11-26T08:42:45.1840925Z --> Processing Dependency: libkeyutils.so.1(KEYUTILS_0.3) for package: krb5-libs
2019-11-26T08:42:45.1854476Z ---> Package libgcc.i386 0:4.1.2-55.el5 set to be updated
2019-11-26T08:42:45.1880578Z ---> Package libgomp.i386 0:4.4.7-1.el5 set to be updated
2019-11-26T08:42:45.1911449Z ---> Package libgomp.x86_64 0:4.4.7-1.el5 set to be updated
2019-11-26T08:42:45.1941762Z ---> Package libidn.i386 0:0.6.5-1.1 set to be updated
2019-11-26T08:42:45.1957547Z ---> Package libidn.x86_64 0:0.6.5-1.1 set to be updated
2019-11-26T08:42:45.1971277Z ---> Package libstdc++-devel.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-26T08:42:51.1643421Z ---> Package m4.x86_64 0:1.4.5-3.el5.1 set to be updated
2019-11-26T08:42:51.1655260Z ---> Package openssl.i686 0:0.9.8e-40.el5_11 set to be updated
2019-11-26T08:42:51.1699616Z ---> Package xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-11-26T08:42:51.1711289Z ---> Package zlib.i386 0:1.2.3-7.el5 set to be updated
2019-11-26T08:42:51.1725535Z --> Running transaction check
2019-11-26T08:42:51.1726611Z ---> Package device-mapper.i386 0:1.02.67-2.el5_11.1 set to be updated
2019-11-26T08:42:51.1749554Z --> Processing Dependency: libsepol.so.1 for package: device-mapper
2019-11-26T08:42:51.1771280Z ---> Package kernel-headers.x86_64 0:2.6.18-419.el5 set to be updated
2019-11-26T08:42:51.1774604Z ---> Package keyutils-libs.i386 0:1.2-1.el5 set to be updated
2019-11-26T08:42:51.1785613Z ---> Package libselinux.i386 0:1.33.4-5.7.el5.centos set to be updated
2019-11-26T08:42:51.1837002Z ---> Package libstdc++.i386 0:4.1.2-55.el5 set to be updated
2019-11-26T08:42:51.1885976Z --> Running transaction check
2019-11-26T08:42:51.1887162Z ---> Package libsepol.i386 0:1.15.2-3.el5 set to be updated
2019-11-26T08:42:51.3010661Z 
2019-11-26T08:42:51.3010984Z Dependencies Resolved
2019-11-26T08:42:51.3468858Z 
2019-11-26T08:42:51.3469396Z ================================================================================
2019-11-26T08:42:51.3469396Z ================================================================================
2019-11-26T08:42:51.3469714Z  Package          Arch    Version                             Repository   Size
2019-11-26T08:42:51.3470172Z ================================================================================
2019-11-26T08:42:51.3470349Z Installing:
2019-11-26T08:42:51.3471109Z  autoconf         noarch  2.59-12                             base        647 k
2019-11-26T08:42:51.3474909Z  bzip2            x86_64  1.0.3-6.el5_5                       base         50 k
2019-11-26T08:42:51.3475644Z  curl             i386    7.15.5-17.el5_9                     base        235 k
2019-11-26T08:42:51.3476108Z  curl             x86_64  7.15.5-17.el5_9                     base        232 k
2019-11-26T08:42:51.3477198Z  file             x86_64  4.17-28                             base        321 k
2019-11-26T08:42:51.3477765Z  gcc              x86_64  4.1.2-55.el5                        base        5.3 M
2019-11-26T08:42:51.3478266Z  gcc-c++          x86_64  4.1.2-55.el5                        base        3.8 M
2019-11-26T08:42:51.3478871Z  gettext          i386    0.17-1.el5                          base        2.4 M
2019-11-26T08:42:51.3479393Z  gettext          x86_64  0.17-1.el5                          base        2.4 M
2019-11-26T08:42:51.3480031Z  glibc-devel      i386    2.5-123.el5_11.3                    updates     2.1 M
2019-11-26T08:42:51.3480539Z  glibc-devel      x86_64  2.5-123.el5_11.3                    updates     2.4 M
2019-11-26T08:42:51.3480955Z  make             x86_64  1:3.81-3.el5                        base        470 k
2019-11-26T08:42:51.3481359Z  perl             i386    4:5.8.8-43.el5_11                   updates      12 M
2019-11-26T08:42:51.3481860Z  perl             x86_64  4:5.8.8-43.el5_11                   updates      12 M
2019-11-26T08:42:51.3482558Z  pkgconfig        x86_64  1:0.21-2.el5                        base         61 k
2019-11-26T08:42:51.3483203Z  wget             x86_64  1.11.4-3.el5_8.2                    base        583 k
2019-11-26T08:42:51.3483776Z  which            x86_64  2.16-7                              base         24 k
2019-11-26T08:42:51.3484308Z  xz               x86_64  4.999.9-0.3.beta.20091007git.el5    base        146 k
2019-11-26T08:42:51.3484827Z  zlib-devel       i386    1.2.3-7.el5                         base        102 k
2019-11-26T08:42:51.3485244Z  zlib-devel       x86_64  1.2.3-7.el5                         base        103 k
2019-11-26T08:42:51.3485457Z Installing for dependencies:
2019-11-26T08:42:51.3486315Z  cpp              x86_64  4.1.2-55.el5                        base        2.9 M
2019-11-26T08:42:51.3486874Z  db4              i386    4.3.29-10.el5_5.2                   base        910 k
2019-11-26T08:42:51.3487394Z  device-mapper    i386    1.02.67-2.el5_11.1                  updates     804 k
2019-11-26T08:42:51.3487933Z  e2fsprogs-libs   i386    1.39-37.el5                         base        120 k
2019-11-26T08:42:51.3488480Z  gdbm             i386    1.8.0-28.el5                        base         28 k
2019-11-26T08:42:51.3488987Z  glibc            i686    2.5-123.el5_11.3                    updates     5.4 M
2019-11-26T08:42:51.3490002Z  glibc-headers    x86_64  2.5-123.el5_11.3                    updates     602 k
2019-11-26T08:42:51.3490455Z  imake            x86_64  1.0.2-3                             base        319 k
2019-11-26T08:42:51.3490894Z  kernel-headers   x86_64  2.6.18-419.el5                      updates     1.5 M
2019-11-26T08:42:51.3491293Z  keyutils-libs    i386    1.2-1.el5                           base         18 k
2019-11-26T08:42:51.3491689Z  krb5-libs        i386    1.6.1-80.el5_11                     updates     670 k
2019-11-26T08:42:51.3492163Z  libgcc           i386    4.1.2-55.el5                        base         97 k
2019-11-26T08:42:51.3492573Z  libgomp          i386    4.4.7-1.el5                         base         74 k
2019-11-26T08:42:51.3492965Z  libgomp          x86_64  4.4.7-1.el5                         base         71 k
2019-11-26T08:42:51.3493424Z  libidn           i386    0.6.5-1.1                           base        194 k
2019-11-26T08:42:51.3493868Z  libidn           x86_64  0.6.5-1.1                           base        195 k
2019-11-26T08:42:51.3494275Z  libselinux       i386    1.33.4-5.7.el5.centos               libselinux   77 k
2019-11-26T08:42:51.3494777Z  libsepol         i386    1.15.2-3.el5                        base        128 k
2019-11-26T08:42:51.3495182Z  libstdc++        i386    4.1.2-55.el5                        base        364 k
2019-11-26T08:42:51.3495583Z  libstdc++-devel  x86_64  4.1.2-55.el5                        base        2.8 M
2019-11-26T08:42:51.3496046Z  m4               x86_64  1.4.5-3.el5.1                       base        171 k
2019-11-26T08:42:51.3496968Z  openssl          i686    0.9.8e-40.el5_11                    updates     1.7 M
2019-11-26T08:42:51.3497495Z  xz-libs          x86_64  4.999.9-0.3.beta.20091007git.el5    base         95 k
2019-11-26T08:42:51.3498091Z  zlib             i386    1.2.3-7.el5                         base         51 k
2019-11-26T08:42:51.3498465Z Transaction Summary
2019-11-26T08:42:51.3498673Z ================================================================================
2019-11-26T08:42:51.3498925Z Install      44 Package(s)
2019-11-26T08:42:51.3499488Z Upgrade       0 Package(s)
2019-11-26T08:42:51.3499488Z Upgrade       0 Package(s)
2019-11-26T08:42:51.3499695Z 
2019-11-26T08:42:51.3500317Z Total download size: 65 M
2019-11-26T08:42:51.3500716Z Downloading Packages:
2019-11-26T08:42:55.3585071Z --------------------------------------------------------------------------------
2019-11-26T08:42:55.3589122Z Total                                            16 MB/s |  65 MB     00:03     
2019-11-26T08:42:55.4071557Z Running rpm_check_debug
2019-11-26T08:42:55.7710847Z Finished Transaction Test
2019-11-26T08:42:55.7711144Z Transaction Test Succeeded
2019-11-26T08:42:55.8512600Z Running Transaction
2019-11-26T08:42:56.0639429Z 
---
2019-11-26T08:42:59.9702919Z   Installing     : cpp                                                     8/44 
2019-11-26T08:42:59.9704107Z 
2019-11-26T08:43:00.4335348Z   Installing     : m4                                                      9/44 
2019-11-26T08:43:00.4335438Z 
2019-11-26T08:43:00.8991807Z   Installing     : xz-libs                                                10/44 
2019-11-26T08:43:01.0928680Z   Installing     : xz                                                     11/44 
2019-11-26T08:43:01.0928936Z 
2019-11-26T08:43:01.6138685Z   Installing     : gettext                                                12/44 
2019-11-26T08:43:01.6138858Z 
---
2019-11-26T08:43:04.2006613Z   Installing     : zlib                                                   19/44 
2019-11-26T08:43:04.2006766Z 
2019-11-26T08:43:04.6860902Z   Installing     : libstdc++                                              20/44 
2019-11-26T08:43:04.6861143Z 
2019-11-26T08:43:05.1468576Z   Installing     : libsepol                                               21/44 
2019-11-26T08:43:05.6320642Z   Installing     : libselinux                                             22/44 
2019-11-26T08:43:05.6320835Z 
2019-11-26T08:43:06.1235000Z   Installing     : device-mapper                                          23/44 
2019-11-26T08:43:06.1235440Z 
---
2019-11-26T08:43:08.3077117Z   Installing     : libidn                                                 27/44 
2019-11-26T08:43:08.3077342Z 
2019-11-26T08:43:08.8365208Z   Installing     : keyutils-libs                                          28/44 
2019-11-26T08:43:08.8365840Z 
2019-11-26T08:43:09.4134129Z   Installing     : krb5-libs                                              29/44 
2019-11-26T08:43:09.9559211Z   Installing     : openssl                                                30/44 
2019-11-26T08:43:09.9559624Z 
2019-11-26T08:43:10.8811057Z   Installing     : gdbm                                                   31/44 
2019-11-26T08:43:10.8811192Z 
---
2019-11-26T08:43:17.0050847Z 
2019-11-26T08:43:17.1728024Z   Installing     : gcc-c++                                                44/44 
2019-11-26T08:43:17.1728156Z 
2019-11-26T08:43:17.1728250Z Installed:
2019-11-26T08:43:17.1728581Z   autoconf.noarch 0:2.59-12                                                     
2019-11-26T08:43:17.1728947Z   bzip2.x86_64 0:1.0.3-6.el5_5                                                  
2019-11-26T08:43:17.1729285Z   curl.i386 0:7.15.5-17.el5_9                                                   
2019-11-26T08:43:17.1730132Z   curl.x86_64 0:7.15.5-17.el5_9                                                 
2019-11-26T08:43:17.1730571Z   file.x86_64 0:4.17-28                                                         
2019-11-26T08:43:17.1730800Z   gcc.x86_64 0:4.1.2-55.el5                                                     
2019-11-26T08:43:17.1731048Z   gcc-c++.x86_64 0:4.1.2-55.el5                                                 
2019-11-26T08:43:17.1731291Z   gettext.i386 0:0.17-1.el5                                                     
2019-11-26T08:43:17.1731539Z   gettext.x86_64 0:0.17-1.el5                                                   
2019-11-26T08:43:17.1731789Z   glibc-devel.i386 0:2.5-123.el5_11.3                                           
2019-11-26T08:43:17.1732021Z   glibc-devel.x86_64 0:2.5-123.el5_11.3                                         
2019-11-26T08:43:17.1732278Z   make.x86_64 1:3.81-3.el5                                                      
2019-11-26T08:43:17.1732508Z   perl.i386 4:5.8.8-43.el5_11                                                   
2019-11-26T08:43:17.1732756Z   perl.x86_64 4:5.8.8-43.el5_11                                                 
2019-11-26T08:43:17.1733002Z   pkgconfig.x86_64 1:0.21-2.el5                                                 
2019-11-26T08:43:17.1733233Z   wget.x86_64 0:1.11.4-3.el5_8.2                                                
2019-11-26T08:43:17.1733477Z   which.x86_64 0:2.16-7                                                         
2019-11-26T08:43:17.1733947Z   xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5                                  
2019-11-26T08:43:17.1734196Z   zlib-devel.i386 0:1.2.3-7.el5                                                 
2019-11-26T08:43:17.1734445Z   zlib-devel.x86_64 0:1.2.3-7.el5                                               
2019-11-26T08:43:17.1734630Z Dependency Installed:
2019-11-26T08:43:17.1734630Z Dependency Installed:
2019-11-26T08:43:17.1734902Z   cpp.x86_64 0:4.1.2-55.el5                                                     
2019-11-26T08:43:17.1735150Z   db4.i386 0:4.3.29-10.el5_5.2                                                  
2019-11-26T08:43:17.1735381Z   device-mapper.i386 0:1.02.67-2.el5_11.1                                       
2019-11-26T08:43:17.1735623Z   e2fsprogs-libs.i386 0:1.39-37.el5                                             
2019-11-26T08:43:17.1736300Z   gdbm.i386 0:1.8.0-28.el5                                                      
2019-11-26T08:43:17.1736674Z   glibc.i686 0:2.5-123.el5_11.3                                                 
2019-11-26T08:43:17.1737005Z   glibc-headers.x86_64 0:2.5-123.el5_11.3                                       
2019-11-26T08:43:17.1737321Z   imake.x86_64 0:1.0.2-3                                                        
2019-11-26T08:43:17.1737659Z   kernel-headers.x86_64 0:2.6.18-419.el5                                        
2019-11-26T08:43:17.1737978Z   keyutils-libs.i386 0:1.2-1.el5                                                
2019-11-26T08:43:17.1738308Z   krb5-libs.i386 0:1.6.1-80.el5_11                                              
2019-11-26T08:43:17.1738622Z   libgcc.i386 0:4.1.2-55.el5                                                    
2019-11-26T08:43:17.1738961Z   libgomp.i386 0:4.4.7-1.el5                                                    
2019-11-26T08:43:17.1739446Z   libgomp.x86_64 0:4.4.7-1.el5                                                  
2019-11-26T08:43:17.1739862Z   libidn.i386 0:0.6.5-1.1                                                       
2019-11-26T08:43:17.1740107Z   libidn.x86_64 0:0.6.5-1.1                                                     
2019-11-26T08:43:17.1740333Z   libselinux.i386 0:1.33.4-5.7.el5.centos                                       
2019-11-26T08:43:17.1740581Z   libsepol.i386 0:1.15.2-3.el5                                                  
2019-11-26T08:43:17.1740830Z   libstdc++.i386 0:4.1.2-55.el5                                                 
2019-11-26T08:43:17.1741066Z   libstdc++-devel.x86_64 0:4.1.2-55.el5                                         
2019-11-26T08:43:17.1741311Z   m4.x86_64 0:1.4.5-3.el5.1                                                     
2019-11-26T08:43:17.1741541Z   openssl.i686 0:0.9.8e-40.el5_11                                               
2019-11-26T08:43:17.1741788Z   xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5                             
2019-11-26T08:43:17.1742387Z   zlib.i386 0:1.2.3-7.el5                                                       
2019-11-26T08:43:17.1742500Z Complete!
2019-11-26T08:43:33.3873072Z Removing intermediate container 3b502a9b9e7b
2019-11-26T08:43:33.3874074Z  ---> 226c9f212dcc
2019-11-26T08:43:33.3874183Z Step 7/41 : ENV PATH=/rustroot/bin:$PATH
---
2019-11-26T08:44:55.6353580Z + hide_output make install
2019-11-26T08:44:55.6354326Z + set +x
2019-11-26T08:45:16.2269545Z shared.sh: line 1:   350 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T08:45:16.2270765Z + cd ..
2019-11-26T08:45:16.2272434Z ./build-openssl.sh: line 20:  4113 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
2019-11-26T08:45:16.2273549Z + rm -rf openssl-1.0.2k
2019-11-26T08:45:16.2907676Z + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
2019-11-26T08:45:20.5871080Z Removing intermediate container c91071d4fe6d
2019-11-26T08:45:20.5872340Z Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
2019-11-26T08:45:22.3139523Z  ---> 17c9c49261b5
2019-11-26T08:45:22.3139916Z Step 15/41 : RUN ./build-curl.sh
2019-11-26T08:45:22.9170106Z  ---> Running in 173a0c6463a0
2019-11-26T08:45:22.9170106Z  ---> Running in 173a0c6463a0
2019-11-26T08:45:23.8661471Z + source shared.sh
2019-11-26T08:45:23.8661661Z + VERSION=7.66.0
2019-11-26T08:45:23.8661879Z + xz --decompress
2019-11-26T08:45:23.8662144Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-11-26T08:45:23.8662926Z + tar xf -
2019-11-26T08:45:24.1818420Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T08:45:24.1818486Z 
2019-11-26T08:45:24.7214724Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-26T08:45:24.7215947Z 100 2358k  100 2358k    0     0  2761k      0 --:--:-- --:--:-- --:--:-- 4375k
2019-11-26T08:45:24.7215947Z 100 2358k  100 2358k    0     0  2761k      0 --:--:-- --:--:-- --:--:-- 4375k
2019-11-26T08:45:24.7317186Z + mkdir curl-build
2019-11-26T08:45:24.7331827Z + cd curl-build
2019-11-26T08:45:24.7335455Z + hide_output ../curl-7.66.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
2019-11-26T08:45:46.8587331Z + hide_output make -j10
2019-11-26T08:45:46.8588473Z + set +x
2019-11-26T08:46:04.6790611Z shared.sh: line 1:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T08:46:04.6791561Z + hide_output make install
2019-11-26T08:46:04.6791561Z + hide_output make install
2019-11-26T08:46:04.6791740Z + set +x
2019-11-26T08:46:05.5496933Z shared.sh: line 1: 11953 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T08:46:05.5497301Z + cd ..
2019-11-26T08:46:05.5501748Z ./build-curl.sh: line 37: 16065 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/curl-build)
2019-11-26T08:46:05.5502091Z + rm -rf curl-build
2019-11-26T08:46:05.5822024Z + rm -rf curl-7.66.0
2019-11-26T08:46:05.6339946Z + yum erase -y curl
2019-11-26T08:46:06.5436310Z Setting up Remove Process
2019-11-26T08:46:06.7262820Z --> Running transaction check
2019-11-26T08:46:06.7262820Z --> Running transaction check
2019-11-26T08:46:06.7264692Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be erased
2019-11-26T08:46:06.7274293Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be erased
2019-11-26T08:46:06.8340820Z 
2019-11-26T08:46:06.8341319Z Dependencies Resolved
2019-11-26T08:46:06.8373964Z 
2019-11-26T08:46:06.8374132Z ================================================================================
2019-11-26T08:46:06.8374132Z ================================================================================
2019-11-26T08:46:06.8374224Z  Package      Arch           Version                    Repository         Size
2019-11-26T08:46:06.8374354Z ================================================================================
2019-11-26T08:46:06.8374440Z Removing:
2019-11-26T08:46:06.8374922Z  curl         i386           7.15.5-17.el5_9            installed         464 k
2019-11-26T08:46:06.8375216Z  curl         x86_64         7.15.5-17.el5_9            installed         473 k
2019-11-26T08:46:06.8375379Z Transaction Summary
2019-11-26T08:46:06.8375452Z ================================================================================
2019-11-26T08:46:06.8375538Z Remove        2 Package(s)
2019-11-26T08:46:06.8375595Z Reinstall     0 Package(s)
2019-11-26T08:46:06.8375595Z Reinstall     0 Package(s)
2019-11-26T08:46:06.8375668Z Downgrade     0 Package(s)
2019-11-26T08:46:06.8375703Z 
2019-11-26T08:46:06.8375770Z Downloading Packages:
2019-11-26T08:46:06.8375827Z Running rpm_check_debug
2019-11-26T08:46:06.8586258Z Finished Transaction Test
2019-11-26T08:46:06.8586358Z Transaction Test Succeeded
2019-11-26T08:46:06.8613553Z Running Transaction
2019-11-26T08:46:07.0377624Z 
2019-11-26T08:46:07.0377624Z 
2019-11-26T08:46:07.5209763Z   Erasing        : curl                                                     1/2 
2019-11-26T08:46:07.5210514Z 
2019-11-26T08:46:07.9980753Z   Erasing        : curl                                                     2/2 
2019-11-26T08:46:07.9981019Z 
2019-11-26T08:46:07.9981093Z Removed:
2019-11-26T08:46:07.9981967Z   curl.i386 0:7.15.5-17.el5_9           curl.x86_64 0:7.15.5-17.el5_9          
2019-11-26T08:46:07.9984167Z Complete!
2019-11-26T08:46:10.2205960Z Removing intermediate container 173a0c6463a0
2019-11-26T08:46:10.2207620Z  ---> 88403f0723bb
2019-11-26T08:46:10.2208253Z Step 16/41 : COPY dist-x86_64-linux/build-binutils.sh /tmp/
---
2019-11-26T08:47:26.8366789Z  ---> 9b0155f4da4a
2019-11-26T08:47:26.8367943Z Step 19/41 : RUN ./build-cmake.sh
2019-11-26T08:47:26.9866562Z  ---> Running in eab3d8ac7fea
2019-11-26T08:47:27.3911357Z + source shared.sh
2019-11-26T08:47:27.3913213Z + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
2019-11-26T08:47:27.3924636Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-26T08:47:27.3924798Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T08:47:27.3924855Z 
2019-11-26T08:47:27.4594441Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-26T08:50:43.4594076Z  91 67.8M   91 62.3M    0     0  4420k      0  0:00:15  0:00:14  0:00:01 3394k
2019-11-26T08:50:43.7434248Z  97 67.8M   97 65.7M    0     0  4515k      0  0:00:15  0:00:14  0:00:01 3549k
2019-11-26T08:50:43.7434901Z 100 67.8M  100 67.8M    0     0  4566k      0  0:00:15  0:00:15 --:--:-- 3275k
2019-11-26T08:50:43.7506311Z + cd gcc-5.5.0
2019-11-26T08:50:43.7510107Z + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
2019-11-26T08:50:43.7526721Z + ./contrib/download_prerequisites
2019-11-26T08:50:43.7563490Z --2019-11-26 08:50:43--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
2019-11-26T08:50:43.8382495Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-26T08:50:43.8997816Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-26T08:50:43.9648545Z HTTP request sent, awaiting response... 200 OK
2019-11-26T08:50:43.9648954Z Length: 1077886 (1.0M) [application/x-bzip2]
2019-11-26T08:50:43.9649722Z Saving to: `mpfr-2.4.2.tar.bz2'
2019-11-26T08:50:43.9650410Z 
2019-11-26T08:50:44.1478922Z      0K .......... .......... .......... .......... ..........  4%  273K 4s
2019-11-26T08:50:44.2701417Z     50K .......... .......... .......... .......... ..........  9%  409K 3s
2019-11-26T08:50:44.3314038Z    100K .......... .......... .......... .......... .......... 14%  819K 2s
2019-11-26T08:50:44.4534200Z    150K .......... .......... .......... .......... .......... 19%  410K 2s
2019-11-26T08:50:44.5146870Z    200K .......... .......... .......... .......... .......... 23%  820K 2s
2019-11-26T08:50:44.5766266Z    250K .......... .......... .......... .......... .......... 28%  818K 2s
2019-11-26T08:50:44.5766880Z    300K .......... .......... .......... .......... .......... 33% 93.1M 1s
2019-11-26T08:50:44.6368739Z    350K .......... .......... .......... .......... .......... 38%  820K 1s
2019-11-26T08:50:44.6974369Z    400K .......... .......... .......... .......... .......... 42%  827K 1s
2019-11-26T08:50:44.7590466Z    450K .......... .......... .......... .......... .......... 47%  821K 1s
2019-11-26T08:50:44.7590826Z    500K .......... .......... .......... .......... .......... 52% 89.7M 1s
2019-11-26T08:50:44.8198776Z    550K .......... .......... .......... .......... .......... 57%  825K 1s
2019-11-26T08:50:44.8199694Z    600K .......... .......... .......... .......... .......... 61%  120M 1s
2019-11-26T08:50:44.8815924Z    650K .......... .......... .......... .......... .......... 66%  823K 0s
2019-11-26T08:50:44.8816498Z    700K .......... .......... .......... .......... .......... 71%  126M 0s
2019-11-26T08:50:44.9422516Z    750K .......... .......... .......... .......... .......... 76%  827K 0s
2019-11-26T08:50:44.9424192Z    800K .......... .......... .......... .......... .......... 80%  132M 0s
2019-11-26T08:50:44.9425102Z    850K .......... .......... .......... .......... .......... 85% 72.1M 0s
2019-11-26T08:50:45.0033236Z    900K .......... .......... .......... .......... .......... 90%  830K 0s
2019-11-26T08:50:45.0051832Z    950K .......... .......... .......... .......... .......... 95% 89.4M 0s
2019-11-26T08:50:45.0052492Z   1000K .......... .......... .......... .......... .......... 99%  159M 0s
2019-11-26T08:50:45.0053571Z   1050K ..                                                    100% 5003G=1.0s
2019-11-26T08:50:45.0053851Z 
2019-11-26T08:50:45.0054280Z 2019-11-26 08:50:44 (1013 KB/s) - `mpfr-2.4.2.tar.bz2' saved [1077886/1077886]
2019-11-26T08:50:45.0054498Z 
2019-11-26T08:50:45.2573531Z --2019-11-26 08:50:45--  http://gcc.gnu.org/pub/gcc/infrastructure/gmp-4.3.2.tar.bz2
2019-11-26T08:50:45.2616075Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-26T08:50:45.3227348Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-26T08:50:45.3850438Z HTTP request sent, awaiting response... 200 OK
2019-11-26T08:50:45.3850770Z Length: 1897483 (1.8M) [application/x-bzip2]
2019-11-26T08:50:45.3851017Z Saving to: `gmp-4.3.2.tar.bz2'
2019-11-26T08:50:45.3851240Z 
2019-11-26T08:50:45.5688794Z      0K .......... .......... .......... .......... ..........  2%  272K 7s
2019-11-26T08:50:45.6907549Z     50K .......... .......... .......... .......... ..........  5%  410K 5s
2019-11-26T08:50:45.8130755Z    100K .......... .......... .......... .......... ..........  8%  409K 5s
2019-11-26T08:50:45.8741624Z    150K .......... .......... .......... .......... .......... 10%  817K 4s
2019-11-26T08:50:45.9352565Z    200K .......... .......... .......... .......... .......... 13%  815K 4s
2019-11-26T08:50:45.9971623Z    250K .......... .......... .......... .......... .......... 16%  819K 3s
2019-11-26T08:50:46.0581627Z    300K .......... .......... .......... .......... .......... 18%  816K 3s
2019-11-26T08:50:46.0582087Z    350K .......... .......... .......... .......... .......... 21% 82.6M 2s
2019-11-26T08:50:46.1191277Z    400K .......... .......... .......... .......... .......... 24%  823K 2s
2019-11-26T08:50:46.1804155Z    450K .......... .......... .......... .......... .......... 26%  816K 2s
2019-11-26T08:50:46.1804519Z    500K .......... .......... .......... .......... .......... 29%  231M 2s
2019-11-26T08:50:46.2418111Z    550K .......... .......... .......... .......... .......... 32%  820K 2s
2019-11-26T08:50:46.3028821Z    600K .......... .......... .......... .......... .......... 35%  820K 2s
2019-11-26T08:50:46.3029205Z    650K .......... .......... .......... .......... .......... 37%  193M 2s
2019-11-26T08:50:46.3500562Z    700K .......... .......... .......... .......... .......... 40% 1.03M 1s
2019-11-26T08:50:46.3641715Z    750K .......... .......... .......... .......... .......... 43% 3.55M 1s
2019-11-26T08:50:46.3643085Z    800K .......... .......... .......... .......... .......... 45% 83.0M 1s
2019-11-26T08:50:46.4249651Z    850K .......... .......... .......... .......... .......... 48%  828K 1s
2019-11-26T08:50:46.4257022Z    900K .......... .......... .......... .......... .......... 51% 82.7M 1s
2019-11-26T08:50:46.4720624Z    950K .......... .......... .......... .......... .......... 53% 1.16M 1s
2019-11-26T08:50:46.4864136Z   1000K .......... .......... .......... .......... .......... 56% 2.61M 1s
2019-11-26T08:50:46.4864673Z   1050K .......... .......... .......... .......... .......... 59%  154M 1s
2019-11-26T08:50:46.4869585Z   1100K .......... .......... .......... .......... .......... 62% 70.1M 1s
2019-11-26T08:50:46.5480085Z   1150K .......... .......... .......... .......... .......... 64%  835K 1s
2019-11-26T08:50:46.5481036Z   1200K .......... .......... .......... .......... .......... 67%  127M 1s
2019-11-26T08:50:46.5483144Z   1250K .......... .......... .......... .......... .......... 70% 90.4M 0s
2019-11-26T08:50:46.5945767Z   1300K .......... .......... .......... .......... .......... 72% 1.04M 0s
2019-11-26T08:50:46.6092387Z   1350K .......... .......... .......... .......... .......... 75% 3.59M 0s
2019-11-26T08:50:46.6094423Z   1400K .......... .......... .......... .......... .......... 78%  151M 0s
2019-11-26T08:50:46.6094712Z   1450K .......... .......... .......... .......... .......... 80%  109M 0s
2019-11-26T08:50:46.6095032Z   1500K .......... .......... .......... .......... .......... 83%  116M 0s
2019-11-26T08:50:46.6708083Z   1550K .......... .......... .......... .......... .......... 86%  835K 0s
2019-11-26T08:50:46.6708509Z   1600K .......... .......... .......... .......... .......... 89%  159M 0s
2019-11-26T08:50:46.6709061Z   1650K .......... .......... .......... .......... .......... 91% 99.4M 0s
2019-11-26T08:50:46.6709426Z   1700K .......... .......... .......... .......... .......... 94%  134M 0s
2019-11-26T08:50:46.6709732Z   1750K .......... .......... .......... .......... .......... 97% 90.8M 0s
2019-11-26T08:50:46.7321395Z   1800K .......... .......... .......... .......... .......... 99%  843K 0s
2019-11-26T08:50:46.7321590Z   1850K ...                                                   100% 5743G=1.3s
2019-11-26T08:50:46.7321689Z 
2019-11-26T08:50:46.7322008Z 2019-11-26 08:50:46 (1.35 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
2019-11-26T08:50:46.7322090Z 
2019-11-26T08:50:49.9219960Z --2019-11-26 08:50:49--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
2019-11-26T08:50:49.9270665Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-26T08:50:49.9884158Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-26T08:50:50.0513136Z HTTP request sent, awaiting response... 200 OK
2019-11-26T08:50:50.0514064Z Length: 544950 (532K) [application/x-gzip]
2019-11-26T08:50:50.0514318Z Saving to: `mpc-0.8.1.tar.gz'
2019-11-26T08:50:50.0514365Z 
2019-11-26T08:50:50.1732482Z      0K .......... .......... .......... .......... ..........  9%  410K 1s
2019-11-26T08:50:50.2954036Z     50K .......... .......... .......... .......... .......... 18%  410K 1s
2019-11-26T08:50:50.3563956Z    100K .......... .......... .......... .......... .......... 28%  819K 1s
2019-11-26T08:50:50.4174589Z    150K .......... .......... .......... .......... .......... 37%  817K 1s
2019-11-26T08:50:50.4786109Z    200K .......... .......... .......... .......... .......... 46%  820K 0s
2019-11-26T08:50:50.5394851Z    250K .......... .......... .......... .......... .......... 56%  821K 0s
2019-11-26T08:50:50.6003133Z    300K .......... .......... .......... .......... .......... 65%  822K 0s
2019-11-26T08:50:50.6619217Z    350K .......... .......... .......... .......... .......... 75%  820K 0s
2019-11-26T08:50:50.6619620Z    400K .......... .......... .......... .......... .......... 84% 97.9M 0s
2019-11-26T08:50:50.7233094Z    450K .......... .......... .......... .......... .......... 93%  821K 0s
2019-11-26T08:50:50.7233440Z    500K .......... .......... .......... ..                   100%  231M=0.7s
2019-11-26T08:50:50.7233499Z 
2019-11-26T08:50:50.7234540Z 2019-11-26 08:50:50 (793 KB/s) - `mpc-0.8.1.tar.gz' saved [544950/544950]
2019-11-26T08:50:50.7234610Z 
2019-11-26T08:50:50.7508887Z --2019-11-26 08:50:50--  http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.14.tar.bz2
2019-11-26T08:50:50.7553473Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-26T08:50:50.8167628Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-26T08:50:50.8782723Z HTTP request sent, awaiting response... 200 OK
2019-11-26T08:50:50.8783390Z Length: 1399896 (1.3M) [application/x-bzip2]
2019-11-26T08:50:50.8784022Z Saving to: `isl-0.14.tar.bz2'
2019-11-26T08:50:50.8789660Z 
2019-11-26T08:50:51.0611400Z      0K .......... .......... .......... .......... ..........  3%  274K 5s
2019-11-26T08:50:51.1841792Z     50K .......... .......... .......... .......... ..........  7%  406K 4s
2019-11-26T08:50:51.2453721Z    100K .......... .......... .......... .......... .......... 10%  819K 3s
2019-11-26T08:50:51.3061664Z    150K .......... .......... .......... .......... .......... 14%  819K 2s
2019-11-26T08:50:51.3673874Z    200K .......... .......... .......... .......... .......... 18%  818K 2s
2019-11-26T08:50:51.4286806Z    250K .......... .......... .......... .......... .......... 21%  817K 2s
2019-11-26T08:50:51.4898530Z    300K .......... .......... .......... .......... .......... 25%  816K 2s
2019-11-26T08:50:51.5508160Z    350K .......... .......... .......... .......... .......... 29%  826K 2s
2019-11-26T08:50:51.6123591Z    400K .......... .......... .......... .......... .......... 32%  817K 1s
2019-11-26T08:50:51.6124000Z    450K .......... .......... .......... .......... .......... 36% 93.3M 1s
2019-11-26T08:50:51.6731638Z    500K .......... .......... .......... .......... .......... 40%  823K 1s
2019-11-26T08:50:51.6735532Z    550K .......... .......... .......... .......... .......... 43% 85.9M 1s
2019-11-26T08:50:51.7347043Z    600K .......... .......... .......... .......... .......... 47%  825K 1s
2019-11-26T08:50:51.7957330Z    650K .......... .......... .......... .......... .......... 51%  820K 1s
2019-11-26T08:50:51.7957905Z    700K .......... .......... .......... .......... .......... 54%  258M 1s
2019-11-26T08:50:51.8569486Z    750K .......... .......... .......... .......... .......... 58%  821K 1s
2019-11-26T08:50:51.8569830Z    800K .......... .......... .......... .......... .......... 62%  116M 1s
2019-11-26T08:50:51.8570108Z    850K .......... .......... .......... .......... .......... 65%  121M 1s
2019-11-26T08:50:51.9180450Z    900K .......... .......... .......... .......... .......... 69%  826K 0s
2019-11-26T08:50:51.9181685Z    950K .......... .......... .......... .......... .......... 73%  113M 0s
2019-11-26T08:50:51.9793516Z   1000K .......... .......... .......... .......... .......... 76%  825K 0s
2019-11-26T08:50:51.9794044Z   1050K .......... .......... .......... .......... .......... 80%  108M 0s
2019-11-26T08:50:51.9794368Z   1100K .......... .......... .......... .......... .......... 84%  145M 0s
2019-11-26T08:50:52.0404150Z   1150K .......... .......... .......... .......... .......... 87%  826K 0s
2019-11-26T08:50:52.0404679Z   1200K .......... .......... .......... .......... .......... 91%  174M 0s
2019-11-26T08:50:52.0405719Z   1250K .......... .......... .......... .......... .......... 95% 97.4M 0s
2019-11-26T08:50:52.1012479Z   1300K .......... .......... .......... .......... .......... 98%  827K 0s
2019-11-26T08:50:52.1028830Z   1350K .......... .......                                    100%  387M=1.2s
2019-11-26T08:50:52.1031853Z 
2019-11-26T08:50:52.1032506Z 2019-11-26 08:50:52 (1.09 MB/s) - `isl-0.14.tar.bz2' saved [1399896/1399896]
2019-11-26T08:50:52.3696682Z + mkdir ../gcc-build
2019-11-26T08:50:52.3712901Z + cd ../gcc-build
2019-11-26T08:50:52.3713283Z + hide_output ../gcc-5.5.0/configure --prefix=/rustroot --enable-languages=c,c++
2019-11-26T08:50:52.3713509Z + set +x
---
2019-11-26T10:13:42.4476819Z ./build-gcc.sh: line 35: 97104 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T10:13:42.4476983Z + cd ..
2019-11-26T10:13:42.4477877Z + rm -rf gcc-build
2019-11-26T10:13:43.5662686Z + rm -rf gcc-5.5.0
2019-11-26T10:13:48.5988067Z + yum erase -y gcc gcc-c++ binutils
2019-11-26T10:13:52.5160016Z Setting up Remove Process
2019-11-26T10:13:52.7287319Z --> Running transaction check
2019-11-26T10:13:52.7287319Z --> Running transaction check
2019-11-26T10:13:52.7289291Z ---> Package binutils.x86_64 0:2.17.50.0.6-26.el5 set to be erased
2019-11-26T10:13:52.7306960Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be erased
2019-11-26T10:13:52.7315887Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be erased
2019-11-26T10:13:52.8338323Z 
2019-11-26T10:13:52.8338714Z Dependencies Resolved
2019-11-26T10:13:52.8395575Z 
2019-11-26T10:13:52.8395968Z ================================================================================
2019-11-26T10:13:52.8395968Z ================================================================================
2019-11-26T10:13:52.8396362Z  Package         Arch          Version                   Repository        Size
2019-11-26T10:13:52.8397280Z ================================================================================
2019-11-26T10:13:52.8401915Z Removing:
2019-11-26T10:13:52.8403445Z  binutils        x86_64        2.17.50.0.6-26.el5        installed        7.1 M
2019-11-26T10:13:52.8403814Z  gcc             x86_64        4.1.2-55.el5              installed        9.9 M
2019-11-26T10:13:52.8404741Z  gcc-c++         x86_64        4.1.2-55.el5              installed        7.5 M
2019-11-26T10:13:52.8405300Z Transaction Summary
2019-11-26T10:13:52.8405533Z ================================================================================
2019-11-26T10:13:52.8405625Z Remove        3 Package(s)
2019-11-26T10:13:52.8405698Z Reinstall     0 Package(s)
2019-11-26T10:13:52.8405698Z Reinstall     0 Package(s)
2019-11-26T10:13:52.8405755Z Downgrade     0 Package(s)
2019-11-26T10:13:52.8405791Z 
2019-11-26T10:13:52.8419478Z Downloading Packages:
2019-11-26T10:13:52.8419575Z Running rpm_check_debug
2019-11-26T10:13:52.8955603Z Finished Transaction Test
2019-11-26T10:13:52.8955750Z Transaction Test Succeeded
2019-11-26T10:13:52.9018574Z Running Transaction
2019-11-26T10:13:53.0927492Z 
2019-11-26T10:13:53.0927492Z 
2019-11-26T10:13:53.6010499Z   Erasing        : gcc-c++                                                  1/3 
2019-11-26T10:13:53.6010749Z 
2019-11-26T10:13:54.1084101Z   Erasing        : gcc                                                      2/3install-info: warning: no entries found for `/usr/share/info/as.info.gz'; nothing deleted
2019-11-26T10:13:54.1095618Z install-info: warning: no entries found for `/usr/share/info/binutils.info.gz'; nothing deleted
2019-11-26T10:13:54.1110598Z install-info: warning: no entries found for `/usr/share/info/gprof.info.gz'; nothing deleted
2019-11-26T10:13:54.1125620Z install-info: warning: no entries found for `/usr/share/info/ld.info.gz'; nothing deleted
2019-11-26T10:13:54.1142263Z install-info: warning: no entries found for `/usr/share/info/standards.info.gz'; nothing deleted
2019-11-26T10:13:54.1157153Z install-info: warning: no entries found for `/usr/share/info/configure.info.gz'; nothing deleted
2019-11-26T10:13:54.1200397Z 
2019-11-26T10:13:54.7180624Z   Erasing        : binutils                                                 3/3 
2019-11-26T10:13:54.7181773Z 
2019-11-26T10:13:54.7182213Z Removed:
2019-11-26T10:13:54.7182213Z Removed:
2019-11-26T10:13:54.7183088Z   binutils.x86_64 0:2.17.50.0.6-26.el5         gcc.x86_64 0:4.1.2-55.el5        
2019-11-26T10:13:54.7183442Z   gcc-c++.x86_64 0:4.1.2-55.el5               
2019-11-26T10:13:54.7183642Z Complete!
2019-11-26T10:14:29.3558986Z Removing intermediate container 975539a30c55
2019-11-26T10:14:29.3560492Z  ---> fae84caf1e0c
2019-11-26T10:14:29.3560824Z Step 22/41 : COPY dist-x86_64-linux/build-python.sh /tmp/
2019-11-26T10:14:29.3560824Z Step 22/41 : COPY dist-x86_64-linux/build-python.sh /tmp/
2019-11-26T10:14:30.8922002Z  ---> 1f7b97045f8d
2019-11-26T10:14:30.8922359Z Step 23/41 : RUN ./build-python.sh
2019-11-26T10:14:31.0312926Z  ---> Running in 0d97a3527e00
2019-11-26T10:14:31.4607591Z + source shared.sh
2019-11-26T10:14:31.4607895Z + tar xzf -
2019-11-26T10:14:31.4608182Z + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
2019-11-26T10:14:31.4608615Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T10:14:31.4608671Z 
2019-11-26T10:14:32.4576338Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-26T10:14:32.8413976Z  31 16.1M   31 5290k    0     0  5306k      0  0:00:03 --:--:--  0:00:03 5300k
2019-11-26T10:14:32.8413976Z  31 16.1M   31 5290k    0     0  5306k      0  0:00:03 --:--:--  0:00:03 5300k
2019-11-26T10:14:32.8414603Z 100 16.1M  100 16.1M    0     0  11.6M      0  0:00:01  0:00:01 --:--:-- 11.6M
2019-11-26T10:14:32.8513524Z + mkdir python-build
2019-11-26T10:14:32.8528957Z + cd python-build
2019-11-26T10:14:32.8532458Z + CFLAGS='-I /rustroot/include'
2019-11-26T10:14:32.8534701Z + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
2019-11-26T10:14:32.8535237Z + hide_output ../Python-2.7.12/configure --prefix=/rustroot
2019-11-26T10:14:47.9226332Z + hide_output make -j10
2019-11-26T10:14:47.9226634Z + set +x
2019-11-26T10:15:17.9269613Z Tue Nov 26 10:15:17 UTC 2019 - building ...
2019-11-26T10:15:47.9300776Z Tue Nov 26 10:15:47 UTC 2019 - building ...
---
2019-11-26T10:16:20.5502421Z  ---> dfaae580ffc9
2019-11-26T10:16:20.5502779Z Step 25/41 : RUN ./build-clang.sh
2019-11-26T10:16:20.7534521Z  ---> Running in 2e2850b574e6
2019-11-26T10:16:21.2073053Z + source shared.sh
2019-11-26T10:16:21.2073512Z + LLVM=llvmorg-9.0.0
2019-11-26T10:16:21.2073688Z + mkdir llvm-project
2019-11-26T10:16:21.2078581Z + cd llvm-project
2019-11-26T10:16:21.2078880Z + tar xzf - --strip-components=1
2019-11-26T10:16:21.2079188Z + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-9.0.0.tar.gz
2019-11-26T10:16:21.2079737Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T10:16:21.2079822Z 
2019-11-26T10:16:21.4253091Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-26T10:16:21.4257445Z 100   132    0   132    0     0    605      0 --:--:-- --:--:-- --:--:--   605
---
2019-11-26T10:16:42.2484997Z 100 89.6M    0 89.6M    0     0  5318k      0 --:--:--  0:00:17 --:--:-- 4948k
2019-11-26T10:16:43.4783165Z 100 95.1M    0 95.1M    0     0  4632k      0 --:--:--  0:00:21 --:--:-- 3181k
2019-11-26T10:16:43.8523081Z 100 96.1M    0 96.1M    0     0  4422k      0 --:--:--  0:00:22 --:--:-- 3027k
2019-11-26T10:16:43.8524101Z 100  107M    0  107M    0     0  4880k      0 --:--:--  0:00:22 --:--:-- 4229k
2019-11-26T10:16:43.8707223Z + yum install -y patch
2019-11-26T10:16:51.9620753Z Reducing CentOS-5 - libselinux to included packages only
2019-11-26T10:16:51.9636931Z Setting up Install Process
2019-11-26T10:16:52.5255398Z Resolving Dependencies
2019-11-26T10:16:52.5262108Z --> Running transaction check
2019-11-26T10:16:52.5262108Z --> Running transaction check
2019-11-26T10:16:52.5262392Z ---> Package patch.x86_64 0:2.5.4-31.el5 set to be updated
2019-11-26T10:16:52.6749303Z 
2019-11-26T10:16:52.6749477Z Dependencies Resolved
2019-11-26T10:16:52.6780385Z 
2019-11-26T10:16:52.6780548Z ================================================================================
2019-11-26T10:16:52.6780548Z ================================================================================
2019-11-26T10:16:52.6780633Z  Package         Arch             Version                  Repository      Size
2019-11-26T10:16:52.6780735Z ================================================================================
2019-11-26T10:16:52.6780822Z Installing:
2019-11-26T10:16:52.6781316Z  patch           x86_64           2.5.4-31.el5             base            63 k
2019-11-26T10:16:52.6781442Z Transaction Summary
2019-11-26T10:16:52.6781525Z ================================================================================
2019-11-26T10:16:52.6781595Z Install       1 Package(s)
2019-11-26T10:16:52.6781667Z Upgrade       0 Package(s)
2019-11-26T10:16:52.6781667Z Upgrade       0 Package(s)
2019-11-26T10:16:52.6781702Z 
2019-11-26T10:16:52.6781776Z Total download size: 63 k
2019-11-26T10:16:52.6781833Z Downloading Packages:
2019-11-26T10:16:52.7980416Z Running rpm_check_debug
2019-11-26T10:16:52.8180465Z Finished Transaction Test
2019-11-26T10:16:52.8180554Z Transaction Test Succeeded
2019-11-26T10:16:52.8231114Z Running Transaction
2019-11-26T10:16:53.0185473Z 
2019-11-26T10:16:53.0185473Z 
2019-11-26T10:16:53.0969732Z   Installing     : patch                                                    1/1 
2019-11-26T10:16:53.0970146Z 
2019-11-26T10:16:53.0970345Z Installed:
2019-11-26T10:16:53.0973135Z   patch.x86_64 0:2.5.4-31.el5                                                   
2019-11-26T10:16:53.0974318Z Complete!
2019-11-26T10:16:53.1203937Z + patch -Np1
2019-11-26T10:16:53.1205079Z patching file clang/lib/DirectoryWatcher/linux/DirectoryWatcher-linux.cpp
2019-11-26T10:16:53.1217664Z + mkdir clang-build
2019-11-26T10:16:53.1217664Z + mkdir clang-build
2019-11-26T10:16:53.1220327Z + cd clang-build
2019-11-26T10:16:53.1225650Z + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DGCC_INSTALL_PREFIX=/rustroot
2019-11-26T10:17:23.1270080Z Tue Nov 26 10:17:23 UTC 2019 - building ...
2019-11-26T10:17:35.7717555Z + hide_output make -j10
2019-11-26T10:17:35.7718916Z + set +x
2019-11-26T10:18:05.8015613Z Tue Nov 26 10:18:05 UTC 2019 - building ...
---
2019-11-26T12:01:40.0976805Z  ---> 8aee5e072908
2019-11-26T12:01:40.0977251Z Step 28/41 : RUN ./build-git.sh
2019-11-26T12:01:40.2417030Z  ---> Running in 49d7d928d92d
2019-11-26T12:01:40.7191449Z + source shared.sh
2019-11-26T12:01:40.7192076Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-26T12:01:40.7371283Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-26T12:01:40.7371930Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T12:01:40.7372368Z 
2019-11-26T12:01:41.1345640Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-26T12:01:47.2775661Z + set +x
2019-11-26T12:01:53.3278107Z + hide_output make -j10
2019-11-26T12:01:53.3279767Z + set +x
2019-11-26T12:01:53.8783860Z shared.sh: line 1:   157 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T12:01:53.8790581Z ERROR: An error was encountered with the build.
2019-11-26T12:01:53.8804773Z     * new build flags
2019-11-26T12:01:53.8805588Z     CC base85.o
2019-11-26T12:01:53.8805966Z     * new link flags
2019-11-26T12:01:53.8807588Z     CC bisect.o
2019-11-26T12:01:53.8807702Z     CC blob.o
2019-11-26T12:01:53.8862875Z     CC branch.o
2019-11-26T12:01:53.8864371Z     * new prefix flags
2019-11-26T12:01:53.8865204Z     CC bulk-checkin.o
2019-11-26T12:01:53.8865472Z     CC cache-tree.o
2019-11-26T12:01:53.8865542Z     CC color.o
2019-11-26T12:01:53.8865637Z     CC column.o
2019-11-26T12:01:53.8865697Z     CC bundle.o
2019-11-26T12:01:53.8865945Z     CC combine-diff.o
2019-11-26T12:01:53.8866013Z In file included from color.c:1:
2019-11-26T12:01:53.8871254Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8871648Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8872252Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8872391Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8872560Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8872560Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8872650Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8873677Z In file included from branch.c:1:
2019-11-26T12:01:53.8874004Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8874004Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8874333Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8874430Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8874600Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8874600Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8879862Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8880059Z In file included from bisect.c:1:
2019-11-26T12:01:53.8880175Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8880567Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8880567Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8881124Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8881255Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8882767Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8882767Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8883156Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8883507Z In file included from base85.c:1:
2019-11-26T12:01:53.8883619Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8884170Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8884170Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8893014Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8893486Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8893664Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8893664Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8894002Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8896211Z In file included from column.c:1:
2019-11-26T12:01:53.8896452Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8897113Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8897113Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8897594Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8897733Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8897906Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8897906Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8904359Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8904583Z In file included from blob.c:1:
2019-11-26T12:01:53.8904648Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8905083Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8905083Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8905394Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8905547Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8906860Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8906860Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8906978Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8907425Z In file included from combine-diff.c:1:
2019-11-26T12:01:53.8907493Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8907771Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8907771Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8908077Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8913238Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8913419Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8913419Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8913550Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8913778Z        ^
2019-11-26T12:01:53.8914126Z In file included from bulk-checkin.c:4:
2019-11-26T12:01:53.8914527Z In file included from ./cache.hIn file included from cache-tree.c:1:
2019-11-26T12:01:53.8915585Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8915970Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8920390Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8920689Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8924083Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8924083Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8924208Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8924356Z :4:
2019-11-26T12:01:53.8929853Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8929853Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8930560Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8930769Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8930967Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8930967Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8931097Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8943758Z In file included from bundle.c:1:
2019-11-26T12:01:53.8943887Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8945791Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8945791Z In file included from ./git-compat-util.h:160:
2019-11-26T12:01:53.8946127Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:01:53.8947292Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:01:53.8947879Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8947879Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:01:53.8952130Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:01:53.8952385Z In file included from color.c:1:
2019-11-26T12:01:53.8952464Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8952464Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8952980Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8953070Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8955075Z In file included from branch.c:1:
2019-11-26T12:01:53.8955075Z In file included from branch.c:1:
2019-11-26T12:01:53.8955425Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8955561Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8955895Z In file included from bisect.c:1:
2019-11-26T12:01:53.8955961Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8955961Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8956435Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8956510Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8960369Z In file included from base85.c:1:
2019-11-26T12:01:53.8960444Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8960444Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8960907Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8960997Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8961181Z In file included from column.c:1:
2019-11-26T12:01:53.8962435Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8962435Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8962801Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8963509Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8963681Z In file included from blob.c:1:
2019-11-26T12:01:53.8963784Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8963784Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8964118Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8967998Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8968917Z In file included from combine-diff.c:1:
2019-11-26T12:01:53.8969181Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8969181Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8978946Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8979358Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8981638Z In file included from cache-tree.c:1:
2019-11-26T12:01:53.8981759Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8981759Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8982244Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8982333Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8983860Z          ^~~~~~~~~~~~~~~
2019-11-26T12:01:53.8984215Z In file included from bulk-checkin.c:4:
2019-11-26T12:01:53.8984342Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8984656Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8984733Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8985033Z In file included from bundle.c:1:
2019-11-26T12:01:53.8993146Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8993146Z In file included from ./cache.h:4:
2019-11-26T12:01:53.8993742Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:01:53.8993870Z #include <openssl/ssl.h>
2019-11-26T12:01:53.8994025Z 2 errors generated.
2019-11-26T12:01:53.8994025Z 2 errors generated.
2019-11-26T12:01:53.8994118Z make: *** [color.o] Error 1
2019-11-26T12:01:53.8994184Z make: *** Waiting for unfinished jobs....
2019-11-26T12:01:53.8995381Z 2 errors generated.
2019-11-26T12:01:53.8995480Z make: *** [base85.o] Error 1
2019-11-26T12:01:53.8995597Z 2 errors generated.
2019-11-26T12:01:53.8995656Z make: *** [blob.o] Error 1
2019-11-26T12:01:53.8995748Z 2 errors generated.
2019-11-26T12:01:53.8995807Z make: *** [branch.o] Error 1
2019-11-26T12:01:53.8995920Z 2 errors generated.
2019-11-26T12:01:53.9003770Z make: *** [bulk-checkin.o] Error 1
2019-11-26T12:01:53.9005820Z 2 errors generated.
2019-11-26T12:01:53.9005820Z 2 errors generated.
2019-11-26T12:01:53.9006209Z make: *** [cache-tree.o] Error 1
2019-11-26T12:01:53.9009814Z make: *** [column.o] Error 1
2019-11-26T12:01:53.9009934Z 2 errors generated.
2019-11-26T12:01:53.9013830Z make: *** [bundle.o] Error 1
2019-11-26T12:01:53.9013901Z 2 errors generated.
2019-11-26T12:01:53.9013999Z make: *** [bisect.o] Error 1
2019-11-26T12:01:53.9014056Z 2 errors generated.
2019-11-26T12:01:53.9016246Z make: *** [combine-diff.o] Error 1
2019-11-26T12:01:55.0979822Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-26T12:01:56.5587208Z Sending build context to Docker daemon  527.9kB
2019-11-26T12:01:56.5587806Z 
2019-11-26T12:01:56.5832200Z Step 1/41 : FROM centos:5
2019-11-26T12:01:56.5876560Z  ---> 1ae98b2c895d
---
2019-11-26T12:01:56.6803519Z  ---> 8aee5e072908
2019-11-26T12:01:56.6808675Z Step 28/41 : RUN ./build-git.sh
2019-11-26T12:01:56.8134779Z  ---> Running in 1ddb48328038
2019-11-26T12:01:57.2359829Z + source shared.sh
2019-11-26T12:01:57.2360910Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-26T12:01:57.2382275Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-26T12:01:57.2382961Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T12:01:57.2383360Z 
2019-11-26T12:01:57.4575468Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-26T12:02:11.8502273Z + hide_output make -j10
2019-11-26T12:02:11.8503186Z + set +x
2019-11-26T12:02:12.3510481Z ERROR: An error was encountered with the build.
2019-11-26T12:02:12.3511583Z shared.sh: line 1:   157 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T12:02:12.3522330Z     * new build flags
2019-11-26T12:02:12.3522451Z     * new prefix flags
2019-11-26T12:02:12.3523888Z     * new link flags
2019-11-26T12:02:12.3524238Z     GEN common-cmds.h
2019-11-26T12:02:12.3524315Z     CC hex.o
2019-11-26T12:02:12.3524367Z     CC ident.o
2019-11-26T12:02:12.3524435Z     CC kwset.o
2019-11-26T12:02:12.3524487Z     CC levenshtein.o
2019-11-26T12:02:12.3524677Z     CC line-log.o
2019-11-26T12:02:12.3524849Z     CC line-range.o
2019-11-26T12:02:12.3524921Z In file included from hex.c:1:
2019-11-26T12:02:12.3524993Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3525224Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3525545Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3525639Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3525780Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3525780Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3525868Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3526136Z     CC list-objects.o
2019-11-26T12:02:12.3526136Z     CC list-objects.o
2019-11-26T12:02:12.3526321Z     CC ll-merge.o
2019-11-26T12:02:12.3526376Z     CC lockfile.o
2019-11-26T12:02:12.3526446Z In file included from ident.c:8:
2019-11-26T12:02:12.3526503Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3526721Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3526985Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3527089Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3527379Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3527379Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3527467Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3527525Z        ^
2019-11-26T12:02:12.3527588Z In file included from kwset.c:37:
2019-11-26T12:02:12.3527641Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3528024Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3528962Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3529094Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3529268Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3529268Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3529486Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3529657Z In file included from levenshtein.c:1:
2019-11-26T12:02:12.3529734Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3530041Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3530041Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3530374Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3530503Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3530677Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3530677Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3530797Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3531132Z In file included from line-log.c:1:
2019-11-26T12:02:12.3531393Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3531393Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3531901Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3531998Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3532128Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3532128Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3532194Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3532313Z In file included from hex.c:1:
2019-11-26T12:02:12.3532382Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3532382Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3532619Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3532691Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3532940Z In file included from line-range.c:1:
2019-11-26T12:02:12.3533145Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3533145Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3533387Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3533489Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3533618Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3533618Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3533700Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3533818Z In file included from lockfile.c:5:
2019-11-26T12:02:12.3533871Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3534084Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3534084Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3534327Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3534431Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3534558Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3534558Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3534640Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3534901Z In file included from list-objects.c:1:
2019-11-26T12:02:12.3534958Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3535166Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3535166Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3535407Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3535502Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3535630Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3535630Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3535712Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3536054Z In file included from ll-merge.c:7:
2019-11-26T12:02:12.3536111Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3536315Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3536315Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:12.3536571Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:12.3536705Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:12.3536840Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3536840Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:12.3536926Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:12.3536982Z        ^
2019-11-26T12:02:12.3537044Z In file included from kwset.c:37:
2019-11-26T12:02:12.3537096Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3537348Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3537427Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3537553Z In file included from levenshtein.c:1:
2019-11-26T12:02:12.3537606Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3537606Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3538008Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3538077Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3538544Z 2 errors generated.
2019-11-26T12:02:12.3538618Z In file included from ident.c:8:
2019-11-26T12:02:12.3538709Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3538709Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3539036Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3539142Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3539211Z          ^~~~~~~~~~~~~~~
2019-11-26T12:02:12.3539297Z make: *** [hex.o] Error 1
2019-11-26T12:02:12.3539387Z make: *** Waiting for unfinished jobs....
2019-11-26T12:02:12.3539636Z In file included from line-log.c:1:
2019-11-26T12:02:12.3539942Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3540038Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3540367Z In file included from line-range.c:1:
2019-11-26T12:02:12.3540367Z In file included from line-range.c:1:
2019-11-26T12:02:12.3540678Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3540764Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3540951Z In file included from lockfile.c:5:
2019-11-26T12:02:12.3541025Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3541025Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3541336Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3541423Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3541733Z 2 errors generated.
2019-11-26T12:02:12.3541733Z 2 errors generated.
2019-11-26T12:02:12.3541994Z make: *** [levenshtein.o] Error 1
2019-11-26T12:02:12.3542046Z 2 errors generated.
2019-11-26T12:02:12.3542112Z make: *** [kwset.o] Error 1
2019-11-26T12:02:12.3542325Z In file included from list-objects.c:1:
2019-11-26T12:02:12.3542382Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3542608Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3542671Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3542912Z In file included from ll-merge.c:7:
2019-11-26T12:02:12.3542991Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3542991Z In file included from ./cache.h:4:
2019-11-26T12:02:12.3543202Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:12.3543280Z #include <openssl/ssl.h>
2019-11-26T12:02:12.3543398Z 2 errors generated.
2019-11-26T12:02:12.3543398Z 2 errors generated.
2019-11-26T12:02:12.3543588Z make: *** [line-range.o] Error 1
2019-11-26T12:02:12.3543644Z 2 errors generated.
2019-11-26T12:02:12.3543711Z make: *** [ident.o] Error 1
2019-11-26T12:02:12.3543762Z 2 errors generated.
2019-11-26T12:02:12.3543828Z make: *** [lockfile.o] Error 1
2019-11-26T12:02:12.3543879Z 2 errors generated.
2019-11-26T12:02:12.3544180Z make: *** [line-log.o] Error 1
2019-11-26T12:02:12.3544250Z 2 errors generated.
2019-11-26T12:02:12.3544424Z make: *** [list-objects.o] Error 1
2019-11-26T12:02:12.3544494Z 2 errors generated.
2019-11-26T12:02:12.3544665Z make: *** [ll-merge.o] Error 1
2019-11-26T12:02:13.5193489Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-26T12:02:15.5994136Z Sending build context to Docker daemon  527.9kB
2019-11-26T12:02:15.5994756Z 
2019-11-26T12:02:15.6258972Z Step 1/41 : FROM centos:5
2019-11-26T12:02:15.6262978Z  ---> 1ae98b2c895d
---
2019-11-26T12:02:15.6396073Z  ---> 8aee5e072908
2019-11-26T12:02:15.6396450Z Step 28/41 : RUN ./build-git.sh
2019-11-26T12:02:15.8004794Z  ---> Running in 95d8a28dfc9e
2019-11-26T12:02:16.2156590Z + source shared.sh
2019-11-26T12:02:16.2156958Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-26T12:02:16.2157455Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-26T12:02:16.2157551Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T12:02:16.2157625Z 
2019-11-26T12:02:16.4633134Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-26T12:02:30.3575925Z + hide_output make -j10
2019-11-26T12:02:30.3576823Z + set +x
2019-11-26T12:02:30.8508253Z ERROR: An error was encountered with the build.
2019-11-26T12:02:30.8508762Z shared.sh: line 1:   157 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T12:02:30.8516148Z     * new build flags
2019-11-26T12:02:30.8516260Z     * new prefix flags
2019-11-26T12:02:30.8516319Z     * new link flags
2019-11-26T12:02:30.8516734Z     GEN common-cmds.h
2019-11-26T12:02:30.8516809Z     CC hex.o
2019-11-26T12:02:30.8516881Z     CC ident.o
2019-11-26T12:02:30.8516935Z     CC kwset.o
2019-11-26T12:02:30.8517006Z     CC levenshtein.o
2019-11-26T12:02:30.8517885Z     CC line-log.o
2019-11-26T12:02:30.8518167Z     CC line-range.o
2019-11-26T12:02:30.8518692Z     CC list-objects.o
2019-11-26T12:02:30.8519313Z     CC ll-merge.o
2019-11-26T12:02:30.8519428Z In file included from levenshtein.c:1:
2019-11-26T12:02:30.8519674Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8520049Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8520414Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8521022Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8521191Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8521191Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8521298Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8521636Z     CC lockfile.o
2019-11-26T12:02:30.8521636Z     CC lockfile.o
2019-11-26T12:02:30.8521696Z In file included from kwset.c:37:
2019-11-26T12:02:30.8521775Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8522033Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8522461Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8522584Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8522743Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8522743Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8522820Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8525013Z In file included from ident.c:8:
2019-11-26T12:02:30.8525089Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8525396Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8525396Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8525679Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8525783Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8525924Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8525924Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8525997Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8526126Z In file included from hex.c:1:
2019-11-26T12:02:30.8526202Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8526446Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8526446Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8526712Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8526815Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8526983Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8526983Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8527062Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8528409Z In file included from line-log.c:1:
2019-11-26T12:02:30.8528686Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8528686Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8529039Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8529148Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8529321Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8529321Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8529606Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8529978Z In file included from line-range.c:1:
2019-11-26T12:02:30.8530261Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8530261Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8530598Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8530810Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8530993Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8530993Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8531084Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8531600Z In file included from list-objects.c:1:
2019-11-26T12:02:30.8531660Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8531881Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8531881Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8532140Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8532248Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8532385Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8532385Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8532474Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8532606Z In file included from levenshtein.c:1:
2019-11-26T12:02:30.8532663Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8532663Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8532912Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8532978Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8533053Z          ^~~~~~~~~~~~~~~
2019-11-26T12:02:30.8533107Z In file included from kwset.c:37:
2019-11-26T12:02:30.8533185Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8533430Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8533503Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8533628Z In file included from ident.c:8:
2019-11-26T12:02:30.8533702Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8533702Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8533929Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8534013Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8534146Z In file included from lockfile.c:5:
2019-11-26T12:02:30.8534220Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8534423Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8534423Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8534697Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8534780Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8534916Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8534916Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8535012Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8535276Z In file included from ll-merge.c:7:
2019-11-26T12:02:30.8535353Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8535554Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8535554Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:30.8535828Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:30.8535920Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:30.8536073Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8536073Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:30.8536143Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:30.8536270Z In file included from hex.c:1:
2019-11-26T12:02:30.8536342Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8536342Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8536571Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8536737Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8536863Z 2 errors generated.
2019-11-26T12:02:30.8536863Z 2 errors generated.
2019-11-26T12:02:30.8536934Z make: *** [levenshtein.o] Error 1
2019-11-26T12:02:30.8536992Z make: *** Waiting for unfinished jobs....
2019-11-26T12:02:30.8537398Z In file included from line-log.c:1:
2019-11-26T12:02:30.8538141Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8538266Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8538632Z In file included from list-objects.c:1:
2019-11-26T12:02:30.8538709Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8538709Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8539020Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8539124Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8539283Z 2 errors generated.
2019-11-26T12:02:30.8539352Z In file included from lockfile.c:5:
2019-11-26T12:02:30.8539454Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8539454Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8539749Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8539853Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8540991Z 2In file included from line-range.c:1:
2019-11-26T12:02:30.8540991Z 2In file included from line-range.c:1:
2019-11-26T12:02:30.8541603Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8541803Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8542145Z  errors generated.
2019-11-26T12:02:30.8542145Z  errors generated.
2019-11-26T12:02:30.8543106Z make: *** [kwset.o] Error 1
2019-11-26T12:02:30.8543307Z 2 errors generated.
2019-11-26T12:02:30.8543433Z make: *** [ident.o] Error 1
2019-11-26T12:02:30.8543489Z make: *** [hex.o] Error 1
2019-11-26T12:02:30.8543665Z 2 errors generated.
2019-11-26T12:02:30.8543968Z make: *** [line-range.o] Error 1
2019-11-26T12:02:30.8544317Z In file included from ll-merge.c:7:
2019-11-26T12:02:30.8544513Z In file included from ./cache.h:4:
2019-11-26T12:02:30.8544861Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:30.8544955Z #include <openssl/ssl.h>
2019-11-26T12:02:30.8545206Z 2 errors generated.
2019-11-26T12:02:30.8545206Z 2 errors generated.
2019-11-26T12:02:30.8545340Z make: *** [lockfile.o] Error 1
2019-11-26T12:02:30.8545417Z 2 errors generated.
2019-11-26T12:02:30.8545777Z make: *** [ll-merge.o] Error 1
2019-11-26T12:02:30.8545859Z 2 errors generated.
2019-11-26T12:02:30.8546196Z make: *** [list-objects.o] Error 1
2019-11-26T12:02:30.8546388Z 2 errors generated.
2019-11-26T12:02:30.8546656Z make: *** [line-log.o] Error 1
2019-11-26T12:02:32.0039376Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-26T12:02:35.0951372Z Sending build context to Docker daemon  527.9kB
2019-11-26T12:02:35.0951451Z 
2019-11-26T12:02:35.1183878Z Step 1/41 : FROM centos:5
2019-11-26T12:02:35.1184548Z  ---> 1ae98b2c895d
---
2019-11-26T12:02:35.1389913Z Step 28/41 : RUN ./build-git.sh
2019-11-26T12:02:35.3023936Z  ---> Running in 0ba64831a9d6
2019-11-26T12:02:35.7290015Z + source shared.sh
2019-11-26T12:02:35.7290371Z + tar xzf -
2019-11-26T12:02:35.7290680Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-26T12:02:35.7292029Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T12:02:35.7292085Z 
2019-11-26T12:02:35.9856027Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-26T12:02:35.9856657Z 100   162  100   162    0     0    627      0 --:--:-- --:--:-- --:--:--   630
---
2019-11-26T12:02:42.7362093Z + set +x
2019-11-26T12:02:48.8633867Z + hide_output make -j10
2019-11-26T12:02:48.8634136Z + set +x
2019-11-26T12:02:49.4047523Z shared.sh: line 1:   156 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T12:02:49.4048578Z ERROR: An error was encountered with the build.
2019-11-26T12:02:49.4058225Z     * new build flags
2019-11-26T12:02:49.4058736Z     CC base85.o
2019-11-26T12:02:49.4059714Z     CC bisect.o
2019-11-26T12:02:49.4059948Z     CC blob.o
2019-11-26T12:02:49.4060167Z     CC branch.o
2019-11-26T12:02:49.4064183Z     CC bulk-checkin.o
2019-11-26T12:02:49.4064605Z     * new prefix flags
2019-11-26T12:02:49.4064708Z     * new link flags
2019-11-26T12:02:49.4064963Z     CC bundle.o
2019-11-26T12:02:49.4065428Z     CC cache-tree.o
2019-11-26T12:02:49.4065497Z     CC color.o
2019-11-26T12:02:49.4065578Z     CC column.o
2019-11-26T12:02:49.4065789Z     CC combine-diff.o
2019-11-26T12:02:49.4065879Z In file included from base85.c:1:
2019-11-26T12:02:49.4065967Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4066887Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4067398Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4067532Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4067717Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4067717Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4067831Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4067928Z        ^
2019-11-26T12:02:49.4068178Z In file included from bulk-checkin.c:4:
2019-11-26T12:02:49.4068286Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4068549Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4069132Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4069249Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4069444Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4069444Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4069539Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4070100Z In file included from bisect.c:1:
2019-11-26T12:02:49.4070189Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4070483Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4070483Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4071276Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4071540Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4071839Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4071839Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4071920Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4072062Z In file included from blob.c:1:
2019-11-26T12:02:49.4072146Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4072414Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4072414Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4072725Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4072847Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4072999Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4072999Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4073080Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4073380Z In file included from cache-tree.c:1:
2019-11-26T12:02:49.4073476Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4074034Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4074034Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4074396Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4074509Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4074808Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4074808Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4074889Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4075219Z In file included from branch.c:1:
2019-11-26T12:02:49.4075685Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4075685Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4076193Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4076299Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4077008Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4077008Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4077142Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4077301Z In file included from bundle.c:1:
2019-11-26T12:02:49.4077375Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4077718Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4077718Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4078073Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4078184Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4078370Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4078370Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4078481Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4078641Z In file included from base85.c:1:
2019-11-26T12:02:49.4078729Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4078729Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4079041Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4079149Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4079317Z In file included from column.c:1:
2019-11-26T12:02:49.4079390Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4079670Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4079670Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4080004Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4080420Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4080872Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4080872Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4080972Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4081121Z In file included from color.c:1:
2019-11-26T12:02:49.4081187Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4081572Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4081572Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4082237Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4082508Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4082661Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4082661Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4082758Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4083695Z In file included from combine-diff.c:1:
2019-11-26T12:02:49.4083779Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4084206Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4084206Z In file included from ./git-compat-util.h:160:
2019-11-26T12:02:49.4084538Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:02:49.4084635Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:02:49.4084899Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4084899Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:02:49.4085006Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:02:49.4085073Z        ^
2019-11-26T12:02:49.4085329Z In file included from bulk-checkin.c:4:
2019-11-26T12:02:49.4085418Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4085834Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4085926Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4086070Z 2 errors generated.
2019-11-26T12:02:49.4086280Z In file included from cache-tree.c:1:
2019-11-26T12:02:49.4086363Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4086363Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4087035Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4087151Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4087240Z          ^~~~~~~~~~~~~~~
2019-11-26T12:02:49.4087311Z make: *** [base85.o] Error 1
2019-11-26T12:02:49.4087414Z make: *** Waiting for unfinished jobs....
2019-11-26T12:02:49.4087627Z In file included from blob.c:1:
2019-11-26T12:02:49.4087745Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4088056Z In file included from ./git-compat-util.hbisect.c::2801::
2019-11-26T12:02:49.4088172Z 10In file included from :./cache.h :fatal error4: :
2019-11-26T12:02:49.4088584Z 'openssl/ssl.h' file not found./git-compat-util.h
2019-11-26T12:02:49.4088935Z :280:10: fatal error: 'openssl/ssl.h' file not found#include <openssl/ssl.h>
2019-11-26T12:02:49.4089100Z          ^~~~~~~~~~~~~~~
2019-11-26T12:02:49.4089100Z          ^~~~~~~~~~~~~~~
2019-11-26T12:02:49.4089171Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4089349Z In file included from branch.c:1:
2019-11-26T12:02:49.4089349Z In file included from branch.c:1:
2019-11-26T12:02:49.4089646Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4089752Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4089918Z 2 errors generated.
2019-11-26T12:02:49.4089918Z 2 errors generated.
2019-11-26T12:02:49.4090319Z make: *** [bulk-checkin.o] Error 1
2019-11-26T12:02:49.4090408Z In file included from bundle.c:1:
2019-11-26T12:02:49.4090473Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4090748Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4090844Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4090986Z In file included from column.c:1:
2019-11-26T12:02:49.4091051Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4091051Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4091510Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4091732Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4092166Z In file included from color.c:1:
2019-11-26T12:02:49.4092410Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4092410Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4092737Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4093022Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4093402Z In file included from combine-diff.c:1:
2019-11-26T12:02:49.4093492Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4093492Z In file included from ./cache.h:4:
2019-11-26T12:02:49.4093744Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:02:49.4093837Z #include <openssl/ssl.h>
2019-11-26T12:02:49.4093975Z 2 errors generated.
2019-11-26T12:02:49.4093975Z 2 errors generated.
2019-11-26T12:02:49.4094199Z make: *** [cache-tree.o] Error 1
2019-11-26T12:02:49.4094429Z 2 errors generated.
2019-11-26T12:02:49.4094513Z make: *** [blob.o] Error 1
2019-11-26T12:02:49.4094571Z 2 errors generated.
2019-11-26T12:02:49.4094647Z make: *** [color.o] Error 1
2019-11-26T12:02:49.4094707Z 2 errors generated.
2019-11-26T12:02:49.4094782Z make: *** [branch.o] Error 1
2019-11-26T12:02:49.4094915Z 2 errors generated.
2019-11-26T12:02:49.4094915Z 2 errors generated.
2019-11-26T12:02:49.4094990Z make: *** [column.o] Error 1
2019-11-26T12:02:49.4095210Z make: *** [combine-diff.o] Error 1
2019-11-26T12:02:49.4095293Z 2 errors generated.
2019-11-26T12:02:49.4095350Z make: *** [bisect.o] Error 1
2019-11-26T12:02:49.4095425Z 2 errors generated.
2019-11-26T12:02:49.4095482Z make: *** [bundle.o] Error 1
2019-11-26T12:02:50.5667302Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-26T12:02:54.6474494Z Sending build context to Docker daemon  527.9kB
2019-11-26T12:02:54.6474911Z 
2019-11-26T12:02:54.6654744Z Step 1/41 : FROM centos:5
2019-11-26T12:02:54.6655790Z  ---> 1ae98b2c895d
---
2019-11-26T12:02:54.6814134Z  ---> 8aee5e072908
2019-11-26T12:02:54.6814487Z Step 28/41 : RUN ./build-git.sh
2019-11-26T12:02:54.8935407Z  ---> Running in 9ed5d55d7c33
2019-11-26T12:02:55.3504938Z + source shared.sh
2019-11-26T12:02:55.3505709Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-26T12:02:55.3530481Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-26T12:02:55.3531112Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-26T12:02:55.3531308Z 
2019-11-26T12:02:55.5590189Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-26T12:03:02.8246748Z + set +x
2019-11-26T12:03:08.8597262Z + hide_output make -j10
2019-11-26T12:03:08.8598337Z + set +x
2019-11-26T12:03:09.3778794Z shared.sh: line 1:   156 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-26T12:03:09.3780691Z ERROR: An error was encountered with the build.
2019-11-26T12:03:09.3795880Z     * new build flags
2019-11-26T12:03:09.3796634Z     * new link flags
2019-11-26T12:03:09.3796854Z     * new prefix flags
2019-11-26T12:03:09.3797380Z     GEN common-cmds.h
2019-11-26T12:03:09.3797624Z     CC hex.o
2019-11-26T12:03:09.3797796Z     CC ident.o
2019-11-26T12:03:09.3797979Z     CC kwset.o
2019-11-26T12:03:09.3798147Z     CC levenshtein.o
2019-11-26T12:03:09.3798548Z     CC line-log.o
2019-11-26T12:03:09.3798994Z     CC line-range.o
2019-11-26T12:03:09.3799387Z     CC list-objects.o
2019-11-26T12:03:09.3800100Z     CC ll-merge.o
2019-11-26T12:03:09.3800317Z In file included from hex.c:1:
2019-11-26T12:03:09.3800463Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3800823Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3801257Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3801477Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3801814Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3801814Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3801999Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3802448Z     CC lockfile.o
2019-11-26T12:03:09.3802448Z     CC lockfile.o
2019-11-26T12:03:09.3802582Z In file included from kwset.c:37:
2019-11-26T12:03:09.3802756Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3803997Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3804445Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3804664Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3804967Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3804967Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3805143Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3805431Z In file included from ident.c:8:
2019-11-26T12:03:09.3805581Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3806440Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3806440Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3807049Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3807322Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3807699Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3807699Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3808132Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3808522Z In file included from levenshtein.c:1:
2019-11-26T12:03:09.3808694Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3809146Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3809146Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3810134Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3810380Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3810679Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3810679Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3810867Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3811364Z In file included from list-objects.c:1:
2019-11-26T12:03:09.3811535Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3811865Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3811865Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3812286Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3812505Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3812798Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3812798Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3812955Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3813257Z In file included from hex.c:1:
2019-11-26T12:03:09.3813388Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3813388Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3813746Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3813944Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3814398Z In file included from line-log.c:1:
2019-11-26T12:03:09.3814740Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3814740Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3815165Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3815376Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3815844Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3815844Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3816813Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3817494Z In file included from ll-merge.c:7:
2019-11-26T12:03:09.3817714Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3818141Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3818141Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3818645Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3818928Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3819300Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3819300Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3819683Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3819824Z        ^
2019-11-26T12:03:09.3819969Z In file included from kwset.c:37:
2019-11-26T12:03:09.3820117Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3820458Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3820655Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3820949Z In file included from lockfile.c:5:
2019-11-26T12:03:09.3821105Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3821438Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3821438Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3822041Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3822283Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3822605Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3822605Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3823194Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3823727Z In file included from line-range.c:1:
2019-11-26T12:03:09.3824233Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3824233Z In file included from ./git-compat-util.h:160:
2019-11-26T12:03:09.3824701Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-26T12:03:09.3825343Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-26T12:03:09.3825868Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3825868Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-26T12:03:09.3826498Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-26T12:03:09.3826868Z 2 errors generated.
2019-11-26T12:03:09.3826868Z 2 errors generated.
2019-11-26T12:03:09.3827039Z make: *** [hex.o] Error 1
2019-11-26T12:03:09.3827233Z make: *** Waiting for unfinished jobs....
2019-11-26T12:03:09.3827426Z In file included from ident.c:8:
2019-11-26T12:03:09.3827612Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3828101Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3828365Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3828721Z In file included from levenshtein.c:1:
2019-11-26T12:03:09.3828892Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3828892Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3829353Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3829913Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3830366Z In file included from line-log.c:1:
2019-11-26T12:03:09.3830366Z In file included from line-log.c:1:
2019-11-26T12:03:09.3830730Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3830925Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3831382Z In file included from list-objects.c:1:
2019-11-26T12:03:09.3831573Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3831573Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3831941Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3832115Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3832395Z In file included from lockfile.c:5:
2019-11-26T12:03:09.3832526Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3832526Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3832901Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3833087Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3833530Z In file included from ll-merge.c:7:
2019-11-26T12:03:09.3833699Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3833699Z In file included from ./cache.h:4:
2019-11-26T12:03:09.3834054Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3834249Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3834571Z 2 errors generated.
2019-11-26T12:03:09.3834571Z 2 errors generated.
2019-11-26T12:03:09.3834701Z make: *** [kwset.o] Error 1
2019-11-26T12:03:09.3835002Z In file included from line-range.c:1:
2019-11-26T12:03:09.3835415Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-26T12:03:09.3835591Z #include <openssl/ssl.h>
2019-11-26T12:03:09.3835995Z 2 errors generated.
2019-11-26T12:03:09.3835995Z 2 errors generated.
2019-11-26T12:03:09.3836426Z make: *** [levenshtein.o] Error 1
2019-11-26T12:03:09.3836783Z 2 errors generated.
2019-11-26T12:03:09.3836783Z 2 errors generated.
2019-11-26T12:03:09.3836962Z make: *** [ident.o] Error 1
2019-11-26T12:03:09.3837152Z 2 errors generated.
2019-11-26T12:03:09.3837340Z make: *** [lockfile.o] Error 1
2019-11-26T12:03:09.3837753Z make: *** [line-range.o] Error 1
2019-11-26T12:03:09.3837969Z 2 errors generated.
2019-11-26T12:03:09.3838370Z make: *** [list-objects.o] Error 1
2019-11-26T12:03:09.3838596Z 2 errors generated.
2019-11-26T12:03:09.3838970Z make: *** [ll-merge.o] Error 1
2019-11-26T12:03:09.3839196Z 2 errors generated.
2019-11-26T12:03:09.3839889Z make: *** [line-log.o] Error 1
2019-11-26T12:03:10.5594733Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-26T12:03:10.5989372Z 
2019-11-26T12:03:10.5989372Z 
2019-11-26T12:03:10.7804023Z ##[error]Bash exited with code '1'.
2019-11-26T12:03:10.8068677Z ##[section]Starting: Checkout
2019-11-26T12:03:10.8136439Z ==============================================================================
2019-11-26T12:03:10.8136552Z Task         : Get sources
2019-11-26T12:03:10.8136662Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
