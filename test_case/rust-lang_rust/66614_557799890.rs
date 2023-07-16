plain
2019-11-23T10:45:53.1361772Z Removing intermediate container 17a43c544471
2019-11-23T10:45:53.1362558Z  ---> 48ce726644dc
2019-11-23T10:45:53.1363168Z Step 6/41 : RUN yum upgrade -y && yum install -y       curl       bzip2       gcc       gcc-c++       make       glibc-devel       perl       zlib-devel       file       xz       which       pkgconfig       wget       autoconf       gettext
2019-11-23T10:45:53.3603116Z  ---> Running in 6bfee322e724
2019-11-23T10:45:57.9222964Z Reducing CentOS-5 - libselinux to included packages only
2019-11-23T10:45:57.9239645Z Setting up Upgrade Process
2019-11-23T10:45:58.3822315Z Resolving Dependencies
2019-11-23T10:45:58.3832736Z --> Running transaction check
2019-11-23T10:45:58.3832736Z --> Running transaction check
2019-11-23T10:45:58.3856058Z ---> Package bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-11-23T10:45:58.3941855Z ---> Package bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12 set to be updated
2019-11-23T10:45:58.4024658Z ---> Package nspr.x86_64 0:4.11.0-1.el5_11 set to be updated
2019-11-23T10:45:58.4104120Z ---> Package nss.x86_64 0:3.21.3-2.el5_11 set to be updated
2019-11-23T10:45:58.4181790Z ---> Package openssl.x86_64 0:0.9.8e-40.el5_11 set to be updated
2019-11-23T10:45:58.4411559Z ---> Package tzdata.x86_64 0:2017b-1.el5 set to be updated
2019-11-23T10:45:58.5414306Z 
2019-11-23T10:45:58.5414454Z Dependencies Resolved
2019-11-23T10:45:58.5481913Z 
2019-11-23T10:45:58.5482060Z ================================================================================
2019-11-23T10:45:58.5482060Z ================================================================================
2019-11-23T10:45:58.5482234Z  Package         Arch        Version                         Repository    Size
2019-11-23T10:45:58.5482373Z ================================================================================
2019-11-23T10:45:58.5482455Z Updating:
2019-11-23T10:45:58.5483168Z  bind-libs       x86_64      30:9.3.6-25.P1.el5_11.12        updates      902 k
2019-11-23T10:45:58.5483495Z  bind-utils      x86_64      30:9.3.6-25.P1.el5_11.12        updates      181 k
2019-11-23T10:45:58.5483843Z  nspr            x86_64      4.11.0-1.el5_11                 updates      123 k
2019-11-23T10:45:58.5484196Z  nss             x86_64      3.21.3-2.el5_11                 updates      1.3 M
2019-11-23T10:45:58.5484811Z  openssl         x86_64      0.9.8e-40.el5_11                updates      1.7 M
2019-11-23T10:45:58.5485160Z  tzdata          x86_64      2017b-1.el5                     updates      757 k
2019-11-23T10:45:58.5485316Z Transaction Summary
2019-11-23T10:45:58.5485399Z ================================================================================
2019-11-23T10:45:58.5485496Z Install       0 Package(s)
2019-11-23T10:45:58.5485564Z Upgrade       6 Package(s)
2019-11-23T10:45:58.5485564Z Upgrade       6 Package(s)
2019-11-23T10:45:58.5485636Z 
2019-11-23T10:45:58.5485695Z Total download size: 4.9 M
2019-11-23T10:45:58.5485788Z Downloading Packages:
2019-11-23T10:45:59.1549781Z --------------------------------------------------------------------------------
2019-11-23T10:45:59.1553897Z Total                                           8.1 MB/s | 4.9 MB     00:00     
2019-11-23T10:45:59.1567607Z warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
2019-11-23T10:45:59.1609944Z Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
2019-11-23T10:45:59.3203009Z Running rpm_check_debug
2019-11-23T10:45:59.4355289Z Finished Transaction Test
2019-11-23T10:45:59.4355510Z Transaction Test Succeeded
2019-11-23T10:45:59.4791634Z Running Transaction
2019-11-23T10:45:59.6088581Z 
---
2019-11-23T10:46:03.4590451Z   Updating       : tzdata                                                  6/12 
2019-11-23T10:46:03.4590929Z 
2019-11-23T10:46:03.5372604Z   Cleanup        : bind-utils                                              7/12 
2019-11-23T10:46:03.5372719Z 
2019-11-23T10:46:04.2025908Z   Cleanup        : nss                                                     8/12 
2019-11-23T10:46:04.8483622Z   Cleanup        : bind-libs                                               9/12 
2019-11-23T10:46:04.8483827Z 
2019-11-23T10:46:05.5012522Z   Cleanup        : openssl                                                10/12 
2019-11-23T10:46:05.5014179Z 
2019-11-23T10:46:05.5014179Z 
2019-11-23T10:46:06.1379153Z   Cleanup        : nspr                                                   11/12 
2019-11-23T10:46:06.2689456Z   Cleanup        : tzdata                                                 12/12 
2019-11-23T10:46:06.2689660Z 
2019-11-23T10:46:06.2689782Z Updated:
2019-11-23T10:46:06.2689782Z Updated:
2019-11-23T10:46:06.2690689Z   bind-libs.x86_64 30:9.3.6-25.P1.el5_11.12                                     
2019-11-23T10:46:06.2691452Z   bind-utils.x86_64 30:9.3.6-25.P1.el5_11.12                                    
2019-11-23T10:46:06.2691829Z   nspr.x86_64 0:4.11.0-1.el5_11                                                 
2019-11-23T10:46:06.2692152Z   nss.x86_64 0:3.21.3-2.el5_11                                                  
2019-11-23T10:46:06.2692509Z   openssl.x86_64 0:0.9.8e-40.el5_11                                             
2019-11-23T10:46:06.2692821Z   tzdata.x86_64 0:2017b-1.el5                                                   
2019-11-23T10:46:06.2693307Z Complete!
2019-11-23T10:46:06.2693307Z Complete!
2019-11-23T10:46:06.4397030Z Reducing CentOS-5 - libselinux to included packages only
2019-11-23T10:46:06.4408484Z Setting up Install Process
2019-11-23T10:46:07.3392052Z Resolving Dependencies
2019-11-23T10:46:07.3398105Z --> Running transaction check
2019-11-23T10:46:07.3398105Z --> Running transaction check
2019-11-23T10:46:07.3401505Z ---> Package autoconf.noarch 0:2.59-12 set to be updated
2019-11-23T10:46:07.3749561Z --> Processing Dependency: imake for package: autoconf
2019-11-23T10:46:07.3772346Z --> Processing Dependency: m4 for package: autoconf
2019-11-23T10:46:07.3794303Z ---> Package bzip2.x86_64 0:1.0.3-6.el5_5 set to be updated
2019-11-23T10:46:07.3904592Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be updated
2019-11-23T10:46:07.4089436Z --> Processing Dependency: libc.so.6(GLIBC_2.4) for package: curl
2019-11-23T10:46:07.4258766Z --> Processing Dependency: libgssapi_krb5.so.2 for package: curl
2019-11-23T10:46:07.4295322Z --> Processing Dependency: libdl.so.2(GLIBC_2.1) for package: curl
2019-11-23T10:46:07.4341723Z --> Processing Dependency: libc.so.6(GLIBC_2.1.3) for package: curl
2019-11-23T10:46:07.4342183Z --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
2019-11-23T10:46:07.4342512Z --> Processing Dependency: libidn.so.11 for package: curl
2019-11-23T10:46:07.4386071Z --> Processing Dependency: libz.so.1 for package: curl
2019-11-23T10:46:07.4403654Z --> Processing Dependency: libc.so.6 for package: curl
2019-11-23T10:46:07.4445555Z --> Processing Dependency: libdl.so.2 for package: curl
2019-11-23T10:46:07.4446058Z --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
2019-11-23T10:46:07.4466105Z --> Processing Dependency: libc.so.6(GLIBC_2.3) for package: curl
2019-11-23T10:46:07.4510945Z --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
2019-11-23T10:46:07.4511250Z --> Processing Dependency: libk5crypto.so.3 for package: curl
2019-11-23T10:46:07.4529687Z --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
2019-11-23T10:46:07.4573659Z --> Processing Dependency: libssl.so.6 for package: curl
2019-11-23T10:46:07.4647556Z --> Processing Dependency: libcom_err.so.2 for package: curl
2019-11-23T10:46:07.4670629Z --> Processing Dependency: libcrypto.so.6 for package: curl
2019-11-23T10:46:07.4717576Z --> Processing Dependency: libc.so.6(GLIBC_2.0) for package: curl
2019-11-23T10:46:07.4759432Z --> Processing Dependency: libdl.so.2(GLIBC_2.0) for package: curl
2019-11-23T10:46:07.4759800Z --> Processing Dependency: libkrb5.so.3 for package: curl
2019-11-23T10:46:07.4778043Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be updated
2019-11-23T10:46:07.4903615Z --> Processing Dependency: libidn.so.11()(64bit) for package: curl
2019-11-23T10:46:07.4968131Z ---> Package file.x86_64 0:4.17-28 set to be updated
2019-11-23T10:46:07.4969516Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-23T10:46:07.5049951Z --> Processing Dependency: cpp = 4.1.2-55.el5 for package: gcc
2019-11-23T10:46:07.5073705Z --> Processing Dependency: libgomp >= 4.1.2-55.el5 for package: gcc
2019-11-23T10:46:07.5154604Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-23T10:46:07.5204364Z --> Processing Dependency: libstdc++-devel = 4.1.2-55.el5 for package: gcc-c++
2019-11-23T10:46:07.5286590Z ---> Package gettext.i386 0:0.17-1.el5 set to be updated
2019-11-23T10:46:07.5410914Z --> Processing Dependency: libgomp.so.1 for package: gettext
2019-11-23T10:46:07.5446390Z --> Processing Dependency: libgomp.so.1(GOMP_1.0) for package: gettext
2019-11-23T10:46:07.5446810Z --> Processing Dependency: libgcc_s.so.1 for package: gettext
2019-11-23T10:46:07.5470264Z --> Processing Dependency: libgcc_s.so.1(GCC_3.3.1) for package: gettext
2019-11-23T10:46:07.5470576Z ---> Package gettext.x86_64 0:0.17-1.el5 set to be updated
2019-11-23T10:46:07.5557835Z ---> Package glibc-devel.i386 0:2.5-123.el5_11.3 set to be updated
2019-11-23T10:46:07.5729914Z --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
2019-11-23T10:46:07.5759015Z --> Processing Dependency: glibc-headers for package: glibc-devel
2019-11-23T10:46:07.5759440Z ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-11-23T10:46:07.5935394Z ---> Package make.x86_64 1:3.81-3.el5 set to be updated
2019-11-23T10:46:07.5946916Z ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
2019-11-23T10:46:07.7430514Z --> Processing Dependency: libgdbm.so.2 for package: perl
2019-11-23T10:46:07.7448589Z --> Processing Dependency: libdb-4.3.so for package: perl
2019-11-23T10:46:07.7466977Z ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
2019-11-23T10:46:07.7730290Z ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
2019-11-23T10:46:07.7740390Z ---> Package wget.x86_64 0:1.11.4-3.el5_8.2 set to be updated
2019-11-23T10:46:07.7758462Z ---> Package which.x86_64 0:2.16-7 set to be updated
2019-11-23T10:46:07.7784906Z ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-11-23T10:46:07.7814133Z --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
2019-11-23T10:46:07.7890213Z --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
2019-11-23T10:46:07.7890535Z ---> Package zlib-devel.i386 0:1.2.3-7.el5 set to be updated
2019-11-23T10:46:07.7904490Z ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
2019-11-23T10:46:07.7910536Z --> Running transaction check
2019-11-23T10:46:07.7913717Z ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-23T10:46:07.7924909Z ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
2019-11-23T10:46:07.7976372Z --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
2019-11-23T10:46:07.8001420Z --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
2019-11-23T10:46:07.8001826Z --> Processing Dependency: libstdc++.so.6 for package: db4
2019-11-23T10:46:07.8002106Z ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
2019-11-23T10:46:07.8044135Z --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
2019-11-23T10:46:07.8068607Z ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
2019-11-23T10:46:07.8077830Z ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
2019-11-23T10:46:07.8148821Z ---> Package glibc-headers.x86_64 0:2.5-123.el5_11.3 set to be updated
2019-11-23T10:46:07.8200843Z --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
2019-11-23T10:46:07.8267214Z --> Processing Dependency: kernel-headers for package: glibc-headers
2019-11-23T10:46:07.8267581Z ---> Package imake.x86_64 0:1.0.2-3 set to be updated
2019-11-23T10:46:07.8279616Z ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
2019-11-23T10:46:07.8355144Z --> Processing Dependency: libkeyutils.so.1 for package: krb5-libs
2019-11-23T10:46:07.8374994Z --> Processing Dependency: libselinux.so.1 for package: krb5-libs
2019-11-23T10:46:07.8399607Z --> Processing Dependency: libkeyutils.so.1(KEYUTILS_0.3) for package: krb5-libs
2019-11-23T10:46:07.8418383Z ---> Package libgcc.i386 0:4.1.2-55.el5 set to be updated
2019-11-23T10:46:07.8441469Z ---> Package libgomp.i386 0:4.4.7-1.el5 set to be updated
2019-11-23T10:46:07.8470091Z ---> Package libgomp.x86_64 0:4.4.7-1.el5 set to be updated
2019-11-23T10:46:07.8500640Z ---> Package libidn.i386 0:0.6.5-1.1 set to be updated
2019-11-23T10:46:07.8519607Z ---> Package libidn.x86_64 0:0.6.5-1.1 set to be updated
2019-11-23T10:46:07.8534556Z ---> Package libstdc++-devel.x86_64 0:4.1.2-55.el5 set to be updated
2019-11-23T10:46:13.2562586Z ---> Package m4.x86_64 0:1.4.5-3.el5.1 set to be updated
2019-11-23T10:46:13.2574912Z ---> Package openssl.i686 0:0.9.8e-40.el5_11 set to be updated
2019-11-23T10:46:13.2616743Z ---> Package xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
2019-11-23T10:46:13.2628998Z ---> Package zlib.i386 0:1.2.3-7.el5 set to be updated
2019-11-23T10:46:13.2643332Z --> Running transaction check
2019-11-23T10:46:13.2646796Z ---> Package device-mapper.i386 0:1.02.67-2.el5_11.1 set to be updated
2019-11-23T10:46:13.2669274Z --> Processing Dependency: libsepol.so.1 for package: device-mapper
2019-11-23T10:46:13.2690128Z ---> Package kernel-headers.x86_64 0:2.6.18-419.el5 set to be updated
2019-11-23T10:46:13.2693813Z ---> Package keyutils-libs.i386 0:1.2-1.el5 set to be updated
2019-11-23T10:46:13.2705841Z ---> Package libselinux.i386 0:1.33.4-5.7.el5.centos set to be updated
2019-11-23T10:46:13.2748114Z ---> Package libstdc++.i386 0:4.1.2-55.el5 set to be updated
2019-11-23T10:46:13.2794152Z --> Running transaction check
2019-11-23T10:46:13.2797938Z ---> Package libsepol.i386 0:1.15.2-3.el5 set to be updated
2019-11-23T10:46:13.3950074Z 
2019-11-23T10:46:13.3950541Z Dependencies Resolved
2019-11-23T10:46:13.4387248Z 
2019-11-23T10:46:13.4388596Z ================================================================================
2019-11-23T10:46:13.4388596Z ================================================================================
2019-11-23T10:46:13.4389768Z  Package          Arch    Version                             Repository   Size
2019-11-23T10:46:13.4390746Z ================================================================================
2019-11-23T10:46:13.4391022Z Installing:
2019-11-23T10:46:13.4391871Z  autoconf         noarch  2.59-12                             base        647 k
2019-11-23T10:46:13.4392433Z  bzip2            x86_64  1.0.3-6.el5_5                       base         50 k
2019-11-23T10:46:13.4393847Z  curl             i386    7.15.5-17.el5_9                     base        235 k
2019-11-23T10:46:13.4394878Z  curl             x86_64  7.15.5-17.el5_9                     base        232 k
2019-11-23T10:46:13.4395642Z  file             x86_64  4.17-28                             base        321 k
2019-11-23T10:46:13.4396171Z  gcc              x86_64  4.1.2-55.el5                        base        5.3 M
2019-11-23T10:46:13.4396644Z  gcc-c++          x86_64  4.1.2-55.el5                        base        3.8 M
2019-11-23T10:46:13.4397132Z  gettext          i386    0.17-1.el5                          base        2.4 M
2019-11-23T10:46:13.4397633Z  gettext          x86_64  0.17-1.el5                          base        2.4 M
2019-11-23T10:46:13.4398129Z  glibc-devel      i386    2.5-123.el5_11.3                    updates     2.1 M
2019-11-23T10:46:13.4398626Z  glibc-devel      x86_64  2.5-123.el5_11.3                    updates     2.4 M
2019-11-23T10:46:13.4399097Z  make             x86_64  1:3.81-3.el5                        base        470 k
2019-11-23T10:46:13.4399587Z  perl             i386    4:5.8.8-43.el5_11                   updates      12 M
2019-11-23T10:46:13.4400068Z  perl             x86_64  4:5.8.8-43.el5_11                   updates      12 M
2019-11-23T10:46:13.4400537Z  pkgconfig        x86_64  1:0.21-2.el5                        base         61 k
2019-11-23T10:46:13.4401030Z  wget             x86_64  1.11.4-3.el5_8.2                    base        583 k
2019-11-23T10:46:13.4401512Z  which            x86_64  2.16-7                              base         24 k
2019-11-23T10:46:13.4401985Z  xz               x86_64  4.999.9-0.3.beta.20091007git.el5    base        146 k
2019-11-23T10:46:13.4403560Z  zlib-devel       i386    1.2.3-7.el5                         base        102 k
2019-11-23T10:46:13.4404190Z  zlib-devel       x86_64  1.2.3-7.el5                         base        103 k
2019-11-23T10:46:13.4404432Z Installing for dependencies:
2019-11-23T10:46:13.4404880Z  cpp              x86_64  4.1.2-55.el5                        base        2.9 M
2019-11-23T10:46:13.4405356Z  db4              i386    4.3.29-10.el5_5.2                   base        910 k
2019-11-23T10:46:13.4405858Z  device-mapper    i386    1.02.67-2.el5_11.1                  updates     804 k
2019-11-23T10:46:13.4406353Z  e2fsprogs-libs   i386    1.39-37.el5                         base        120 k
2019-11-23T10:46:13.4406827Z  gdbm             i386    1.8.0-28.el5                        base         28 k
2019-11-23T10:46:13.4407890Z  glibc            i686    2.5-123.el5_11.3                    updates     5.4 M
2019-11-23T10:46:13.4408431Z  glibc-headers    x86_64  2.5-123.el5_11.3                    updates     602 k
2019-11-23T10:46:13.4408919Z  imake            x86_64  1.0.2-3                             base        319 k
2019-11-23T10:46:13.4409651Z  kernel-headers   x86_64  2.6.18-419.el5                      updates     1.5 M
2019-11-23T10:46:13.4410147Z  keyutils-libs    i386    1.2-1.el5                           base         18 k
2019-11-23T10:46:13.4410623Z  krb5-libs        i386    1.6.1-80.el5_11                     updates     670 k
2019-11-23T10:46:13.4411115Z  libgcc           i386    4.1.2-55.el5                        base         97 k
2019-11-23T10:46:13.4411582Z  libgomp          i386    4.4.7-1.el5                         base         74 k
2019-11-23T10:46:13.4412075Z  libgomp          x86_64  4.4.7-1.el5                         base         71 k
2019-11-23T10:46:13.4412563Z  libidn           i386    0.6.5-1.1                           base        194 k
2019-11-23T10:46:13.4413034Z  libidn           x86_64  0.6.5-1.1                           base        195 k
2019-11-23T10:46:13.4413535Z  libselinux       i386    1.33.4-5.7.el5.centos               libselinux   77 k
2019-11-23T10:46:13.4414044Z  libsepol         i386    1.15.2-3.el5                        base        128 k
2019-11-23T10:46:13.4414517Z  libstdc++        i386    4.1.2-55.el5                        base        364 k
2019-11-23T10:46:13.4415004Z  libstdc++-devel  x86_64  4.1.2-55.el5                        base        2.8 M
2019-11-23T10:46:13.4415540Z  m4               x86_64  1.4.5-3.el5.1                       base        171 k
2019-11-23T10:46:13.4416046Z  openssl          i686    0.9.8e-40.el5_11                    updates     1.7 M
2019-11-23T10:46:13.4416596Z  xz-libs          x86_64  4.999.9-0.3.beta.20091007git.el5    base         95 k
2019-11-23T10:46:13.4417087Z  zlib             i386    1.2.3-7.el5                         base         51 k
2019-11-23T10:46:13.4417513Z Transaction Summary
2019-11-23T10:46:13.4417708Z ================================================================================
2019-11-23T10:46:13.4417887Z Install      44 Package(s)
2019-11-23T10:46:13.4418093Z Upgrade       0 Package(s)
2019-11-23T10:46:13.4418093Z Upgrade       0 Package(s)
2019-11-23T10:46:13.4418251Z 
2019-11-23T10:46:13.4418466Z Total download size: 65 M
2019-11-23T10:46:13.4418667Z Downloading Packages:
2019-11-23T10:46:16.2476369Z --------------------------------------------------------------------------------
2019-11-23T10:46:16.2480374Z Total                                            23 MB/s |  65 MB     00:02     
2019-11-23T10:46:16.2951256Z Running rpm_check_debug
2019-11-23T10:46:16.6519732Z Finished Transaction Test
2019-11-23T10:46:16.6519929Z Transaction Test Succeeded
2019-11-23T10:46:16.7274122Z Running Transaction
2019-11-23T10:46:16.9426077Z 
---
2019-11-23T10:46:21.9043069Z   Installing     : cpp                                                     8/44 
2019-11-23T10:46:21.9043322Z 
2019-11-23T10:46:22.6041118Z   Installing     : m4                                                      9/44 
2019-11-23T10:46:22.6041891Z 
2019-11-23T10:46:23.2921348Z   Installing     : xz-libs                                                10/44 
2019-11-23T10:46:23.4873569Z   Installing     : xz                                                     11/44 
2019-11-23T10:46:23.4873698Z 
2019-11-23T10:46:24.2546355Z   Installing     : gettext                                                12/44 
2019-11-23T10:46:24.2547193Z 
---
2019-11-23T10:46:27.9949766Z   Installing     : zlib                                                   19/44 
2019-11-23T10:46:27.9950008Z 
2019-11-23T10:46:28.6786994Z   Installing     : libstdc++                                              20/44 
2019-11-23T10:46:28.6787236Z 
2019-11-23T10:46:29.5774540Z   Installing     : libsepol                                               21/44 
2019-11-23T10:46:30.2717903Z   Installing     : libselinux                                             22/44 
2019-11-23T10:46:30.2718098Z 
2019-11-23T10:46:30.9512950Z   Installing     : device-mapper                                          23/44 
2019-11-23T10:46:30.9513686Z 
---
2019-11-23T10:46:33.7590184Z   Installing     : libidn                                                 27/44 
2019-11-23T10:46:33.7590355Z 
2019-11-23T10:46:34.7360427Z   Installing     : keyutils-libs                                          28/44 
2019-11-23T10:46:34.7361318Z 
2019-11-23T10:46:35.5015170Z   Installing     : krb5-libs                                              29/44 
2019-11-23T10:46:36.2997300Z   Installing     : openssl                                                30/44 
2019-11-23T10:46:36.2998124Z 
2019-11-23T10:46:37.6000311Z   Installing     : gdbm                                                   31/44 
2019-11-23T10:46:37.6000429Z 
---
2019-11-23T10:46:47.2733228Z 
2019-11-23T10:46:47.4200092Z   Installing     : gcc-c++                                                44/44 
2019-11-23T10:46:47.4200730Z 
2019-11-23T10:46:47.4201016Z Installed:
2019-11-23T10:46:47.4201953Z   autoconf.noarch 0:2.59-12                                                     
2019-11-23T10:46:47.4202864Z   bzip2.x86_64 0:1.0.3-6.el5_5                                                  
2019-11-23T10:46:47.4203632Z   curl.i386 0:7.15.5-17.el5_9                                                   
2019-11-23T10:46:47.4204223Z   curl.x86_64 0:7.15.5-17.el5_9                                                 
2019-11-23T10:46:47.4204949Z   file.x86_64 0:4.17-28                                                         
2019-11-23T10:46:47.4205674Z   gcc.x86_64 0:4.1.2-55.el5                                                     
2019-11-23T10:46:47.4206255Z   gcc-c++.x86_64 0:4.1.2-55.el5                                                 
2019-11-23T10:46:47.4207246Z   gettext.i386 0:0.17-1.el5                                                     
2019-11-23T10:46:47.4207882Z   gettext.x86_64 0:0.17-1.el5                                                   
2019-11-23T10:46:47.4208636Z   glibc-devel.i386 0:2.5-123.el5_11.3                                           
2019-11-23T10:46:47.4209468Z   glibc-devel.x86_64 0:2.5-123.el5_11.3                                         
2019-11-23T10:46:47.4210285Z   make.x86_64 1:3.81-3.el5                                                      
2019-11-23T10:46:47.4210853Z   perl.i386 4:5.8.8-43.el5_11                                                   
2019-11-23T10:46:47.4211533Z   perl.x86_64 4:5.8.8-43.el5_11                                                 
2019-11-23T10:46:47.4212192Z   pkgconfig.x86_64 1:0.21-2.el5                                                 
2019-11-23T10:46:47.4212747Z   wget.x86_64 0:1.11.4-3.el5_8.2                                                
2019-11-23T10:46:47.4213446Z   which.x86_64 0:2.16-7                                                         
2019-11-23T10:46:47.4214142Z   xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5                                  
2019-11-23T10:46:47.4214669Z   zlib-devel.i386 0:1.2.3-7.el5                                                 
2019-11-23T10:46:47.4215169Z   zlib-devel.x86_64 0:1.2.3-7.el5                                               
2019-11-23T10:46:47.4215747Z Dependency Installed:
2019-11-23T10:46:47.4215747Z Dependency Installed:
2019-11-23T10:46:47.4216309Z   cpp.x86_64 0:4.1.2-55.el5                                                     
2019-11-23T10:46:47.4216878Z   db4.i386 0:4.3.29-10.el5_5.2                                                  
2019-11-23T10:46:47.4217341Z   device-mapper.i386 0:1.02.67-2.el5_11.1                                       
2019-11-23T10:46:47.4217807Z   e2fsprogs-libs.i386 0:1.39-37.el5                                             
2019-11-23T10:46:47.4218255Z   gdbm.i386 0:1.8.0-28.el5                                                      
2019-11-23T10:46:47.4218722Z   glibc.i686 0:2.5-123.el5_11.3                                                 
2019-11-23T10:46:47.4219186Z   glibc-headers.x86_64 0:2.5-123.el5_11.3                                       
2019-11-23T10:46:47.4219651Z   imake.x86_64 0:1.0.2-3                                                        
2019-11-23T10:46:47.4220115Z   kernel-headers.x86_64 0:2.6.18-419.el5                                        
2019-11-23T10:46:47.4220597Z   keyutils-libs.i386 0:1.2-1.el5                                                
2019-11-23T10:46:47.4221241Z   krb5-libs.i386 0:1.6.1-80.el5_11                                              
2019-11-23T10:46:47.4221733Z   libgcc.i386 0:4.1.2-55.el5                                                    
2019-11-23T10:46:47.4222205Z   libgomp.i386 0:4.4.7-1.el5                                                    
2019-11-23T10:46:47.4222646Z   libgomp.x86_64 0:4.4.7-1.el5                                                  
2019-11-23T10:46:47.4223086Z   libidn.i386 0:0.6.5-1.1                                                       
2019-11-23T10:46:47.4223550Z   libidn.x86_64 0:0.6.5-1.1                                                     
2019-11-23T10:46:47.4224013Z   libselinux.i386 0:1.33.4-5.7.el5.centos                                       
2019-11-23T10:46:47.4224456Z   libsepol.i386 0:1.15.2-3.el5                                                  
2019-11-23T10:46:47.4224922Z   libstdc++.i386 0:4.1.2-55.el5                                                 
2019-11-23T10:46:47.4225403Z   libstdc++-devel.x86_64 0:4.1.2-55.el5                                         
2019-11-23T10:46:47.4225853Z   m4.x86_64 0:1.4.5-3.el5.1                                                     
2019-11-23T10:46:47.4226322Z   openssl.i686 0:0.9.8e-40.el5_11                                               
2019-11-23T10:46:47.4226790Z   xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5                             
2019-11-23T10:46:47.4227232Z   zlib.i386 0:1.2.3-7.el5                                                       
2019-11-23T10:46:47.4230403Z Complete!
2019-11-23T10:47:01.2995135Z Removing intermediate container 6bfee322e724
2019-11-23T10:47:01.2996690Z  ---> e984827e40e6
2019-11-23T10:47:01.2997163Z Step 7/41 : ENV PATH=/rustroot/bin:$PATH
---
2019-11-23T10:48:15.9219930Z + hide_output make install
2019-11-23T10:48:15.9220753Z + set +x
2019-11-23T10:48:34.3748000Z shared.sh: line 1:   350 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T10:48:34.3748956Z + cd ..
2019-11-23T10:48:34.3749922Z ./build-openssl.sh: line 20:  4113 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
2019-11-23T10:48:34.3750620Z + rm -rf openssl-1.0.2k
2019-11-23T10:48:34.3751383Z + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
2019-11-23T10:48:36.3518097Z  ---> eb38598635ff
2019-11-23T10:48:36.3518908Z Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
2019-11-23T10:48:37.7132986Z  ---> 9cb2dc7df5ac
2019-11-23T10:48:37.7133964Z Step 15/41 : RUN ./build-curl.sh
2019-11-23T10:48:37.7133964Z Step 15/41 : RUN ./build-curl.sh
2019-11-23T10:48:37.9120014Z  ---> Running in 3f7a0f9cf0e8
2019-11-23T10:48:38.3144752Z + source shared.sh
2019-11-23T10:48:38.3145476Z + VERSION=7.66.0
2019-11-23T10:48:38.3146022Z + xz --decompress
2019-11-23T10:48:38.3146669Z + curl https://rust-lang-ci-mirrors.s3-us-west-1.amazonaws.com/rustc/curl-7.66.0.tar.xz
2019-11-23T10:48:38.3147086Z + tar xf -
2019-11-23T10:48:38.7547760Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T10:48:38.7547945Z 
2019-11-23T10:48:39.0021127Z   0 2358k    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-23T10:48:39.3052718Z  14 2358k   14  330k    0     0   479k      0  0:00:04 --:--:--  0:00:04 1332k
2019-11-23T10:48:39.3052718Z  14 2358k   14  330k    0     0   479k      0  0:00:04 --:--:--  0:00:04 1332k
2019-11-23T10:48:39.3059216Z 100 2358k  100 2358k    0     0  2375k      0 --:--:-- --:--:-- --:--:-- 4279k
2019-11-23T10:48:39.3149207Z + mkdir curl-build
2019-11-23T10:48:39.3161902Z + cd curl-build
2019-11-23T10:48:39.3164659Z + hide_output ../curl-7.66.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
2019-11-23T10:48:57.5724595Z + hide_output make -j10
2019-11-23T10:48:57.5724999Z + set +x
2019-11-23T10:49:13.1931482Z shared.sh: line 1:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T10:49:13.1934018Z + hide_output make install
2019-11-23T10:49:13.1934018Z + hide_output make install
2019-11-23T10:49:13.1938191Z + set +x
2019-11-23T10:49:14.0557883Z shared.sh: line 1: 11953 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T10:49:14.0562863Z + cd ..
2019-11-23T10:49:14.0563436Z ./build-curl.sh: line 37: 16074 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/curl-build)
2019-11-23T10:49:14.0567476Z + rm -rf curl-build
2019-11-23T10:49:14.0870254Z + rm -rf curl-7.66.0
2019-11-23T10:49:14.1392790Z + yum erase -y curl
2019-11-23T10:49:15.1105080Z Setting up Remove Process
2019-11-23T10:49:15.2908041Z --> Running transaction check
2019-11-23T10:49:15.2908041Z --> Running transaction check
2019-11-23T10:49:15.2911081Z ---> Package curl.i386 0:7.15.5-17.el5_9 set to be erased
2019-11-23T10:49:15.2918001Z ---> Package curl.x86_64 0:7.15.5-17.el5_9 set to be erased
2019-11-23T10:49:15.3956528Z 
2019-11-23T10:49:15.3957623Z Dependencies Resolved
2019-11-23T10:49:15.3984316Z 
2019-11-23T10:49:15.3984710Z ================================================================================
2019-11-23T10:49:15.3984710Z ================================================================================
2019-11-23T10:49:15.3984981Z  Package      Arch           Version                    Repository         Size
2019-11-23T10:49:15.3985222Z ================================================================================
2019-11-23T10:49:15.3985437Z Removing:
2019-11-23T10:49:15.3986081Z  curl         i386           7.15.5-17.el5_9            installed         464 k
2019-11-23T10:49:15.3986635Z  curl         x86_64         7.15.5-17.el5_9            installed         473 k
2019-11-23T10:49:15.3987018Z Transaction Summary
2019-11-23T10:49:15.3987233Z ================================================================================
2019-11-23T10:49:15.3987412Z Remove        2 Package(s)
2019-11-23T10:49:15.3987591Z Reinstall     0 Package(s)
2019-11-23T10:49:15.3987591Z Reinstall     0 Package(s)
2019-11-23T10:49:15.3987748Z Downgrade     0 Package(s)
2019-11-23T10:49:15.3987891Z 
2019-11-23T10:49:15.3988105Z Downloading Packages:
2019-11-23T10:49:15.3991704Z Running rpm_check_debug
2019-11-23T10:49:16.3837256Z Finished Transaction Test
2019-11-23T10:49:16.3837935Z Transaction Test Succeeded
2019-11-23T10:49:16.3838289Z Running Transaction
2019-11-23T10:49:16.3838468Z 
2019-11-23T10:49:16.3838468Z 
2019-11-23T10:49:16.3838798Z   Erasing        : curl                                                     1/2 
2019-11-23T10:49:16.3838954Z 
2019-11-23T10:49:17.0030731Z   Erasing        : curl                                                     2/2 
2019-11-23T10:49:17.0030941Z 
2019-11-23T10:49:17.0031308Z Removed:
2019-11-23T10:49:17.0032547Z   curl.i386 0:7.15.5-17.el5_9           curl.x86_64 0:7.15.5-17.el5_9          
2019-11-23T10:49:17.0032721Z Complete!
2019-11-23T10:49:19.2120866Z Removing intermediate container 3f7a0f9cf0e8
2019-11-23T10:49:19.2122292Z  ---> 65d2c025945f
2019-11-23T10:49:19.2123016Z Step 16/41 : COPY dist-x86_64-linux/build-binutils.sh /tmp/
---
2019-11-23T10:50:30.6788633Z  ---> ef5e2504acf7
2019-11-23T10:50:30.6789755Z Step 19/41 : RUN ./build-cmake.sh
2019-11-23T10:50:30.8660594Z  ---> Running in 0d230439c73e
2019-11-23T10:50:31.3183971Z + source shared.sh
2019-11-23T10:50:31.3184456Z + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
2019-11-23T10:50:31.3184969Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-23T10:50:31.3185069Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T10:50:31.3185140Z 
2019-11-23T10:50:31.7341970Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-23T10:53:34.7385434Z  96 67.8M   96 65.5M    0     0  4676k      0  0:00:14  0:00:14 --:--:-- 3584k
2019-11-23T10:53:34.9867313Z  97 67.8M   97 65.8M    0     0  4674k      0  0:00:14  0:00:14 --:--:-- 3152k
2019-11-23T10:53:34.9880384Z 100 67.8M  100 67.8M    0     0  4729k      0  0:00:14  0:00:14 --:--:-- 2185k
2019-11-23T10:53:34.9958118Z + cd gcc-5.5.0
2019-11-23T10:53:34.9961300Z + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
2019-11-23T10:53:35.0107591Z + ./contrib/download_prerequisites
2019-11-23T10:53:35.0130627Z --2019-11-23 10:53:35--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
2019-11-23T10:53:35.0859130Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-23T10:53:35.1482961Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-23T10:53:35.2135636Z HTTP request sent, awaiting response... 200 OK
2019-11-23T10:53:35.2136022Z Length: 1077886 (1.0M) [application/x-bzip2]
2019-11-23T10:53:35.2138033Z Saving to: `mpfr-2.4.2.tar.bz2'
2019-11-23T10:53:35.2141041Z 
2019-11-23T10:53:35.4005347Z      0K .......... .......... .......... .......... ..........  4%  268K 4s
2019-11-23T10:53:35.5243262Z     50K .......... .......... .......... .......... ..........  9%  403K 3s
2019-11-23T10:53:35.6488762Z    100K .......... .......... .......... .......... .......... 14%  402K 3s
2019-11-23T10:53:35.7728071Z    150K .......... .......... .......... .......... .......... 19%  404K 2s
2019-11-23T10:53:35.8348788Z    200K .......... .......... .......... .......... .......... 23%  803K 2s
2019-11-23T10:53:35.8971922Z    250K .......... .......... .......... .......... .......... 28%  803K 2s
2019-11-23T10:53:35.9595542Z    300K .......... .......... .......... .......... .......... 33%  805K 1s
2019-11-23T10:53:36.0211977Z    350K .......... .......... .......... .......... .......... 38%  807K 1s
2019-11-23T10:53:36.0832088Z    400K .......... .......... .......... .......... .......... 42%  805K 1s
2019-11-23T10:53:36.1455941Z    450K .......... .......... .......... .......... .......... 47%  804K 1s
2019-11-23T10:53:36.2077408Z    500K .......... .......... .......... .......... .......... 52%  806K 1s
2019-11-23T10:53:36.2696094Z    550K .......... .......... .......... .......... .......... 57%  809K 1s
2019-11-23T10:53:36.2699152Z    600K .......... .......... .......... .......... .......... 61%  121M 1s
2019-11-23T10:53:36.3320615Z    650K .......... .......... .......... .......... .......... 66%  805K 1s
2019-11-23T10:53:36.3946345Z    700K .......... .......... .......... .......... .......... 71%  810K 0s
2019-11-23T10:53:36.3946802Z    750K .......... .......... .......... .......... .......... 76% 64.5M 0s
2019-11-23T10:53:36.4560982Z    800K .......... .......... .......... .......... .......... 80%  812K 0s
2019-11-23T10:53:36.4567302Z    850K .......... .......... .......... .......... .......... 85% 64.3M 0s
2019-11-23T10:53:36.5184077Z    900K .......... .......... .......... .......... .......... 90%  816K 0s
2019-11-23T10:53:36.5191965Z    950K .......... .......... .......... .......... .......... 95% 89.5M 0s
2019-11-23T10:53:36.5805171Z   1000K .......... .......... .......... .......... .......... 99%  813K 0s
2019-11-23T10:53:36.5816665Z   1050K ..                                                    100% 21.2M=1.4s
2019-11-23T10:53:36.5816785Z 
2019-11-23T10:53:36.5818667Z 2019-11-23 10:53:36 (770 KB/s) - `mpfr-2.4.2.tar.bz2' saved [1077886/1077886]
2019-11-23T10:53:36.5818950Z 
2019-11-23T10:53:40.8047857Z --2019-11-23 10:53:40--  http://gcc.gnu.org/pub/gcc/infrastructure/gmp-4.3.2.tar.bz2
2019-11-23T10:53:40.8541172Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-23T10:53:40.9136351Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-23T10:53:40.9766186Z HTTP request sent, awaiting response... 200 OK
2019-11-23T10:53:40.9766674Z Length: 1897483 (1.8M) [application/x-bzip2]
2019-11-23T10:53:40.9767408Z Saving to: `gmp-4.3.2.tar.bz2'
2019-11-23T10:53:40.9767736Z 
2019-11-23T10:53:41.1628377Z      0K .......... .......... .......... .......... ..........  2%  269K 7s
2019-11-23T10:53:41.2871260Z     50K .......... .......... .......... .......... ..........  5%  402K 5s
2019-11-23T10:53:41.4114781Z    100K .......... .......... .......... .......... ..........  8%  402K 5s
2019-11-23T10:53:41.4738603Z    150K .......... .......... .......... .......... .......... 10%  801K 4s
2019-11-23T10:53:41.5983534Z    200K .......... .......... .......... .......... .......... 13%  402K 4s
2019-11-23T10:53:41.6606778Z    250K .......... .......... .......... .......... .......... 16%  801K 4s
2019-11-23T10:53:41.7232718Z    300K .......... .......... .......... .......... .......... 18%  798K 3s
2019-11-23T10:53:41.7857744Z    350K .......... .......... .......... .......... .......... 21%  800K 3s
2019-11-23T10:53:41.8481115Z    400K .......... .......... .......... .......... .......... 24%  802K 3s
2019-11-23T10:53:41.9105091Z    450K .......... .......... .......... .......... .......... 26%  803K 3s
2019-11-23T10:53:41.9729026Z    500K .......... .......... .......... .......... .......... 29%  802K 2s
2019-11-23T10:53:41.9729479Z    550K .......... .......... .......... .......... .......... 32%  267M 2s
2019-11-23T10:53:42.0350995Z    600K .......... .......... .......... .......... .......... 35%  804K 2s
2019-11-23T10:53:42.0975934Z    650K .......... .......... .......... .......... .......... 37%  801K 2s
2019-11-23T10:53:42.1599980Z    700K .......... .......... .......... .......... .......... 40%  806K 2s
2019-11-23T10:53:42.1600227Z    750K .......... .......... .......... .......... .......... 43%  142M 2s
2019-11-23T10:53:42.2224027Z    800K .......... .......... .......... .......... .......... 45%  805K 1s
2019-11-23T10:53:42.2224579Z    850K .......... .......... .......... .......... .......... 48%  207M 1s
2019-11-23T10:53:42.2847921Z    900K .......... .......... .......... .......... .......... 51%  803K 1s
2019-11-23T10:53:42.3473086Z    950K .......... .......... .......... .......... .......... 53%  807K 1s
2019-11-23T10:53:42.3473584Z   1000K .......... .......... .......... .......... .......... 56%  107M 1s
2019-11-23T10:53:42.4095759Z   1050K .......... .......... .......... .......... .......... 59%  808K 1s
2019-11-23T10:53:42.4096314Z   1100K .......... .......... .......... .......... .......... 62%  213M 1s
2019-11-23T10:53:42.4718323Z   1150K .......... .......... .......... .......... .......... 64%  806K 1s
2019-11-23T10:53:42.4718762Z   1200K .......... .......... .......... .......... .......... 67%  225M 1s
2019-11-23T10:53:42.4719105Z   1250K .......... .......... .......... .......... .......... 70%  180M 1s
2019-11-23T10:53:42.5339982Z   1300K .......... .......... .......... .......... .......... 72%  810K 1s
2019-11-23T10:53:42.5340415Z   1350K .......... .......... .......... .......... .......... 75% 90.7M 1s
2019-11-23T10:53:42.5963427Z   1400K .......... .......... .......... .......... .......... 78%  809K 0s
2019-11-23T10:53:42.5963875Z   1450K .......... .......... .......... .......... .......... 80%  227M 0s
2019-11-23T10:53:42.5964226Z   1500K .......... .......... .......... .......... .......... 83%  349M 0s
2019-11-23T10:53:42.6589383Z   1550K .......... .......... .......... .......... .......... 86%  807K 0s
2019-11-23T10:53:42.6589846Z   1600K .......... .......... .......... .......... .......... 89%  200M 0s
2019-11-23T10:53:42.6590360Z   1650K .......... .......... .......... .......... .......... 91%  218M 0s
2019-11-23T10:53:42.7213275Z   1700K .......... .......... .......... .......... .......... 94%  808K 0s
2019-11-23T10:53:42.7213790Z   1750K .......... .......... .......... .......... .......... 97%  141M 0s
2019-11-23T10:53:42.7214148Z   1800K .......... .......... .......... .......... .......... 99%  230M 0s
2019-11-23T10:53:42.7214513Z   1850K ...                                                   100% 5743G=1.7s
2019-11-23T10:53:42.7214581Z 
2019-11-23T10:53:42.7214924Z 2019-11-23 10:53:42 (1.04 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
2019-11-23T10:53:42.7214994Z 
2019-11-23T10:53:43.0901790Z --2019-11-23 10:53:43--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
2019-11-23T10:53:43.1298044Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-23T10:53:43.1918751Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-23T10:53:43.2547500Z HTTP request sent, awaiting response... 200 OK
2019-11-23T10:53:43.2548631Z Length: 544950 (532K) [application/x-gzip]
2019-11-23T10:53:43.2549454Z Saving to: `mpc-0.8.1.tar.gz'
2019-11-23T10:53:43.2549719Z 
2019-11-23T10:53:43.4415808Z      0K .......... .......... .......... .......... ..........  9%  268K 2s
2019-11-23T10:53:43.5661988Z     50K .......... .......... .......... .......... .......... 18%  402K 1s
2019-11-23T10:53:43.6906812Z    100K .......... .......... .......... .......... .......... 28%  401K 1s
2019-11-23T10:53:43.8153004Z    150K .......... .......... .......... .......... .......... 37%  402K 1s
2019-11-23T10:53:43.8775798Z    200K .......... .......... .......... .......... .......... 46%  801K 1s
2019-11-23T10:53:43.9396084Z    250K .......... .......... .......... .......... .......... 56%  804K 1s
2019-11-23T10:53:44.0656620Z    300K .......... .......... .......... .......... .......... 65%  401K 0s
2019-11-23T10:53:44.1268355Z    350K .......... .......... .......... .......... .......... 75%  805K 0s
2019-11-23T10:53:44.1894354Z    400K .......... .......... .......... .......... .......... 84%  799K 0s
2019-11-23T10:53:44.2520481Z    450K .......... .......... .......... .......... .......... 93%  802K 0s
2019-11-23T10:53:44.2520962Z    500K .......... .......... .......... ..                   100%  129M=1.0s
2019-11-23T10:53:44.2521041Z 
2019-11-23T10:53:44.2521531Z 2019-11-23 10:53:44 (534 KB/s) - `mpc-0.8.1.tar.gz' saved [544950/544950]
2019-11-23T10:53:44.2521750Z 
2019-11-23T10:53:44.2781706Z --2019-11-23 10:53:44--  http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.14.tar.bz2
2019-11-23T10:53:44.3401834Z Resolving gcc.gnu.org... 209.132.180.131
2019-11-23T10:53:44.4023650Z Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
2019-11-23T10:53:44.4661517Z HTTP request sent, awaiting response... 200 OK
2019-11-23T10:53:44.4661946Z Length: 1399896 (1.3M) [application/x-bzip2]
2019-11-23T10:53:44.4662211Z Saving to: `isl-0.14.tar.bz2'
2019-11-23T10:53:44.4662261Z 
2019-11-23T10:53:44.6521278Z      0K .......... .......... .......... .......... ..........  3%  269K 5s
2019-11-23T10:53:44.7764997Z     50K .......... .......... .......... .......... ..........  7%  402K 4s
2019-11-23T10:53:44.9006705Z    100K .......... .......... .......... .......... .......... 10%  402K 4s
2019-11-23T10:53:45.0253286Z    150K .......... .......... .......... .......... .......... 14%  402K 3s
2019-11-23T10:53:45.0875336Z    200K .......... .......... .......... .......... .......... 18%  802K 3s
2019-11-23T10:53:45.1498237Z    250K .......... .......... .......... .......... .......... 21%  804K 2s
2019-11-23T10:53:45.2738803Z    300K .......... .......... .......... .......... .......... 25%  404K 2s
2019-11-23T10:53:45.3358247Z    350K .......... .......... .......... .......... .......... 29%  803K 2s
2019-11-23T10:53:45.3980092Z    400K .......... .......... .......... .......... .......... 32%  807K 2s
2019-11-23T10:53:45.4599128Z    450K .......... .......... .......... .......... .......... 36%  806K 2s
2019-11-23T10:53:45.4599881Z    500K .......... .......... .......... .......... .......... 40%  189M 1s
2019-11-23T10:53:45.5220351Z    550K .......... .......... .......... .......... .......... 43%  807K 1s
2019-11-23T10:53:45.5840076Z    600K .......... .......... .......... .......... .......... 47%  807K 1s
2019-11-23T10:53:45.6457249Z    650K .......... .......... .......... .......... .......... 51%  811K 1s
2019-11-23T10:53:45.6868828Z    700K .......... .......... .......... .......... .......... 54% 1.18M 1s
2019-11-23T10:53:45.7085572Z    750K .......... .......... .......... .......... .......... 58% 2.24M 1s
2019-11-23T10:53:45.7708520Z    800K .......... .......... .......... .......... .......... 62%  805K 1s
2019-11-23T10:53:45.7708947Z    850K .......... .......... .......... .......... .......... 65%  212M 1s
2019-11-23T10:53:45.8333568Z    900K .......... .......... .......... .......... .......... 69%  805K 1s
2019-11-23T10:53:45.8729177Z    950K .......... .......... .......... .......... .......... 73% 1.23M 1s
2019-11-23T10:53:45.8953813Z   1000K .......... .......... .......... .......... .......... 76% 2.17M 0s
2019-11-23T10:53:45.9348344Z   1050K .......... .......... .......... .......... .......... 80% 1.23M 0s
2019-11-23T10:53:45.9575556Z   1100K .......... .......... .......... .......... .......... 84% 2.15M 0s
2019-11-23T10:53:45.9970258Z   1150K .......... .......... .......... .......... .......... 87% 1.24M 0s
2019-11-23T10:53:46.0198293Z   1200K .......... .......... .......... .......... .......... 91% 2.14M 0s
2019-11-23T10:53:46.0589176Z   1250K .......... .......... .......... .......... .......... 95% 1.25M 0s
2019-11-23T10:53:46.0818929Z   1300K .......... .......... .......... .......... .......... 98% 2.16M 0s
2019-11-23T10:53:46.0819154Z   1350K .......... .......                                    100%  147M=1.6s
2019-11-23T10:53:46.0819243Z 
2019-11-23T10:53:46.0823497Z 2019-11-23 10:53:46 (846 KB/s) - `isl-0.14.tar.bz2' saved [1399896/1399896]
2019-11-23T10:53:46.3270539Z + mkdir ../gcc-build
2019-11-23T10:53:46.3284680Z + cd ../gcc-build
2019-11-23T10:53:46.3285610Z + hide_output ../gcc-5.5.0/configure --prefix=/rustroot --enable-languages=c,c++
2019-11-23T10:53:46.3286002Z + set +x
---
2019-11-23T12:11:07.1940674Z ./build-gcc.sh: line 35: 97068 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T12:11:07.1941207Z + cd ..
2019-11-23T12:11:07.1941617Z + rm -rf gcc-build
2019-11-23T12:11:08.8662466Z + rm -rf gcc-5.5.0
2019-11-23T12:11:17.9041160Z + yum erase -y gcc gcc-c++ binutils
2019-11-23T12:11:20.4783228Z Setting up Remove Process
2019-11-23T12:11:20.6983526Z --> Running transaction check
2019-11-23T12:11:20.6983526Z --> Running transaction check
2019-11-23T12:11:20.6986748Z ---> Package binutils.x86_64 0:2.17.50.0.6-26.el5 set to be erased
2019-11-23T12:11:20.6998660Z ---> Package gcc.x86_64 0:4.1.2-55.el5 set to be erased
2019-11-23T12:11:20.7004003Z ---> Package gcc-c++.x86_64 0:4.1.2-55.el5 set to be erased
2019-11-23T12:11:20.8003967Z 
2019-11-23T12:11:20.8004279Z Dependencies Resolved
2019-11-23T12:11:20.8042733Z 
2019-11-23T12:11:20.8043067Z ================================================================================
2019-11-23T12:11:20.8043067Z ================================================================================
2019-11-23T12:11:20.8043341Z  Package         Arch          Version                   Repository        Size
2019-11-23T12:11:20.8043573Z ================================================================================
2019-11-23T12:11:20.8043749Z Removing:
2019-11-23T12:11:20.8044361Z  binutils        x86_64        2.17.50.0.6-26.el5        installed        7.1 M
2019-11-23T12:11:20.8044900Z  gcc             x86_64        4.1.2-55.el5              installed        9.9 M
2019-11-23T12:11:20.8045397Z  gcc-c++         x86_64        4.1.2-55.el5              installed        7.5 M
2019-11-23T12:11:20.8045769Z Transaction Summary
2019-11-23T12:11:20.8045974Z ================================================================================
2019-11-23T12:11:20.8046147Z Remove        3 Package(s)
2019-11-23T12:11:20.8046319Z Reinstall     0 Package(s)
2019-11-23T12:11:20.8046319Z Reinstall     0 Package(s)
2019-11-23T12:11:20.8046487Z Downgrade     0 Package(s)
2019-11-23T12:11:20.8046611Z 
2019-11-23T12:11:20.8050070Z Downloading Packages:
2019-11-23T12:11:20.8050170Z Running rpm_check_debug
2019-11-23T12:11:21.3697214Z Finished Transaction Test
2019-11-23T12:11:21.3743648Z Transaction Test Succeeded
2019-11-23T12:11:21.3747624Z Running Transaction
2019-11-23T12:11:21.3747715Z 
2019-11-23T12:11:21.3747715Z 
2019-11-23T12:11:21.8307387Z   Erasing        : gcc-c++                                                  1/3 
2019-11-23T12:11:21.8307513Z 
2019-11-23T12:11:22.6034990Z   Erasing        : gcc                                                      2/3install-info: warning: no entries found for `/usr/share/info/as.info.gz'; nothing deleted
2019-11-23T12:11:22.6044983Z install-info: warning: no entries found for `/usr/share/info/binutils.info.gz'; nothing deleted
2019-11-23T12:11:22.6058703Z install-info: warning: no entries found for `/usr/share/info/gprof.info.gz'; nothing deleted
2019-11-23T12:11:22.6072163Z install-info: warning: no entries found for `/usr/share/info/ld.info.gz'; nothing deleted
2019-11-23T12:11:22.6085824Z install-info: warning: no entries found for `/usr/share/info/standards.info.gz'; nothing deleted
2019-11-23T12:11:22.6099022Z install-info: warning: no entries found for `/usr/share/info/configure.info.gz'; nothing deleted
2019-11-23T12:11:22.6142420Z 
2019-11-23T12:11:23.4110645Z   Erasing        : binutils                                                 3/3 
2019-11-23T12:11:23.4110983Z 
2019-11-23T12:11:23.4111121Z Removed:
2019-11-23T12:11:23.4111121Z Removed:
2019-11-23T12:11:23.4112166Z   binutils.x86_64 0:2.17.50.0.6-26.el5         gcc.x86_64 0:4.1.2-55.el5        
2019-11-23T12:11:23.4112713Z   gcc-c++.x86_64 0:4.1.2-55.el5               
2019-11-23T12:11:23.4113449Z Complete!
2019-11-23T12:11:46.7910615Z Removing intermediate container dbe8160794ca
2019-11-23T12:11:46.7912541Z  ---> 9cfc98f3b744
2019-11-23T12:11:46.7913384Z Step 22/41 : COPY dist-x86_64-linux/build-python.sh /tmp/
2019-11-23T12:11:46.7913384Z Step 22/41 : COPY dist-x86_64-linux/build-python.sh /tmp/
2019-11-23T12:11:48.6951946Z  ---> dbfc15cdc12b
2019-11-23T12:11:48.6953564Z Step 23/41 : RUN ./build-python.sh
2019-11-23T12:11:48.8908508Z  ---> Running in c262c19b9383
2019-11-23T12:11:49.3020331Z + source shared.sh
2019-11-23T12:11:49.3030998Z + tar xzf -
2019-11-23T12:11:49.3031876Z + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
2019-11-23T12:11:49.3063163Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T12:11:49.3063262Z 
2019-11-23T12:11:49.7763017Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-23T12:11:50.5841932Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-23T12:11:50.5841932Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-23T12:11:50.5842534Z 100 16.1M  100 16.1M    0     0  12.6M      0  0:00:01  0:00:01 --:--:-- 12.6M
2019-11-23T12:11:50.5933085Z + mkdir python-build
2019-11-23T12:11:50.5951243Z + cd python-build
2019-11-23T12:11:50.5954591Z + CFLAGS='-I /rustroot/include'
2019-11-23T12:11:50.5956047Z + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
2019-11-23T12:11:50.5956811Z + hide_output ../Python-2.7.12/configure --prefix=/rustroot
2019-11-23T12:12:04.8577261Z + hide_output make -j10
2019-11-23T12:12:04.8577983Z + set +x
2019-11-23T12:12:34.8896180Z Sat Nov 23 12:12:34 UTC 2019 - building ...
2019-11-23T12:13:04.8666946Z Sat Nov 23 12:13:04 UTC 2019 - building ...
---
2019-11-23T12:13:30.3433969Z  ---> 92a49a2f04d1
2019-11-23T12:13:30.3435127Z Step 25/41 : RUN ./build-clang.sh
2019-11-23T12:13:30.4876035Z  ---> Running in 8e5364f03cba
2019-11-23T12:13:30.8976932Z + source shared.sh
2019-11-23T12:13:30.8977472Z + LLVM=llvmorg-9.0.0
2019-11-23T12:13:30.8977757Z + mkdir llvm-project
2019-11-23T12:13:30.8977986Z + cd llvm-project
2019-11-23T12:13:30.8978274Z + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-9.0.0.tar.gz
2019-11-23T12:13:30.8978784Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T12:13:30.8978865Z 
2019-11-23T12:13:30.8979264Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0+ tar xzf - --strip-components=1
2019-11-23T12:13:31.1028556Z 
---
2019-11-23T12:13:53.5882833Z 100 90.9M    0 90.9M    0     0  4433k      0 --:--:--  0:00:21 --:--:-- 2419k
2019-11-23T12:13:53.7335069Z 100 96.1M    0 96.1M    0     0  4339k      0 --:--:--  0:00:22 --:--:-- 2667k
2019-11-23T12:13:53.9747562Z 100 99.2M    0 99.2M    0     0  4450k      0 --:--:--  0:00:22 --:--:-- 2487k
2019-11-23T12:13:53.9809779Z 100  107M    0  107M    0     0  4788k      0 --:--:--  0:00:23 --:--:-- 3185k
2019-11-23T12:13:53.9862453Z + yum install -y patch
2019-11-23T12:14:01.8868433Z Reducing CentOS-5 - libselinux to included packages only
2019-11-23T12:14:01.8880193Z Setting up Install Process
2019-11-23T12:14:02.4186783Z Resolving Dependencies
2019-11-23T12:14:02.4192856Z --> Running transaction check
2019-11-23T12:14:02.4192856Z --> Running transaction check
2019-11-23T12:14:02.4196661Z ---> Package patch.x86_64 0:2.5.4-31.el5 set to be updated
2019-11-23T12:14:02.5642822Z 
2019-11-23T12:14:02.5644596Z Dependencies Resolved
2019-11-23T12:14:02.5668610Z 
2019-11-23T12:14:02.5668768Z ================================================================================
2019-11-23T12:14:02.5668768Z ================================================================================
2019-11-23T12:14:02.5668889Z  Package         Arch             Version                  Repository      Size
2019-11-23T12:14:02.5669240Z ================================================================================
2019-11-23T12:14:02.5669368Z Installing:
2019-11-23T12:14:02.5669874Z  patch           x86_64           2.5.4-31.el5             base            63 k
2019-11-23T12:14:02.5669992Z Transaction Summary
2019-11-23T12:14:02.5670082Z ================================================================================
2019-11-23T12:14:02.5670175Z Install       1 Package(s)
2019-11-23T12:14:02.5670235Z Upgrade       0 Package(s)
2019-11-23T12:14:02.5670235Z Upgrade       0 Package(s)
2019-11-23T12:14:02.5670273Z 
2019-11-23T12:14:02.5670344Z Total download size: 63 k
2019-11-23T12:14:02.5670423Z Downloading Packages:
2019-11-23T12:14:02.7590350Z Running rpm_check_debug
2019-11-23T12:14:02.7754821Z Finished Transaction Test
2019-11-23T12:14:02.7758721Z Transaction Test Succeeded
2019-11-23T12:14:02.7798643Z Running Transaction
2019-11-23T12:14:02.9942028Z 
2019-11-23T12:14:02.9942028Z 
2019-11-23T12:14:03.0954631Z   Installing     : patch                                                    1/1 
2019-11-23T12:14:03.0955080Z 
2019-11-23T12:14:03.0955145Z Installed:
2019-11-23T12:14:03.0955915Z   patch.x86_64 0:2.5.4-31.el5                                                   
2019-11-23T12:14:03.0956049Z Complete!
2019-11-23T12:14:03.1187249Z + patch -Np1
2019-11-23T12:14:03.1187763Z patching file clang/lib/DirectoryWatcher/linux/DirectoryWatcher-linux.cpp
2019-11-23T12:14:03.1188016Z + mkdir clang-build
2019-11-23T12:14:03.1188016Z + mkdir clang-build
2019-11-23T12:14:03.1217911Z + cd clang-build
2019-11-23T12:14:03.1218613Z + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DGCC_INSTALL_PREFIX=/rustroot
2019-11-23T12:14:33.1252589Z Sat Nov 23 12:14:33 UTC 2019 - building ...
2019-11-23T12:14:44.4781987Z + hide_output make -j10
2019-11-23T12:14:44.4782475Z + set +x
2019-11-23T12:15:14.5187251Z Sat Nov 23 12:15:14 UTC 2019 - building ...
---
2019-11-23T13:53:59.0086550Z  ---> ead6d4f619f9
2019-11-23T13:53:59.0087117Z Step 28/41 : RUN ./build-git.sh
2019-11-23T13:53:59.1544773Z  ---> Running in 39f3c5dc651f
2019-11-23T13:53:59.5858404Z + source shared.sh
2019-11-23T13:53:59.5865348Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-23T13:53:59.5985117Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-23T13:53:59.5985434Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T13:53:59.5985504Z 
2019-11-23T13:53:59.8737894Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-23T13:54:01.4841650Z + set +x
2019-11-23T13:54:07.2543976Z + hide_output make -j10
2019-11-23T13:54:07.2545948Z + set +x
2019-11-23T13:54:07.7500339Z shared.sh: line 1:   156 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T13:54:07.7505463Z ERROR: An error was encountered with the build.
2019-11-23T13:54:07.7523071Z     * new build flags
2019-11-23T13:54:07.7528342Z     * new prefix flags
2019-11-23T13:54:07.7528430Z     * new link flags
2019-11-23T13:54:07.7533243Z     GEN common-cmds.h
2019-11-23T13:54:07.7534548Z     CC hex.o
2019-11-23T13:54:07.7534646Z     CC ident.o
2019-11-23T13:54:07.7546317Z     CC kwset.o
2019-11-23T13:54:07.7546427Z     CC levenshtein.o
2019-11-23T13:54:07.7548421Z     CC line-log.o
2019-11-23T13:54:07.7548769Z     CC line-range.o
2019-11-23T13:54:07.7552502Z     CC list-objects.o
2019-11-23T13:54:07.7552943Z     CC ll-merge.o
2019-11-23T13:54:07.7554846Z     CC lockfile.o
2019-11-23T13:54:07.7555225Z In file included from line-range.c:1:
2019-11-23T13:54:07.7559720Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7561043Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7561198Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7562383Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7562383Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7563359Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7563569Z In file included from hex.c:1:
2019-11-23T13:54:07.7564560Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7565021Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7565021Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7565400Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7565509Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7565700Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7565700Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7567256Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7570649Z In file included from ident.c:8:
2019-11-23T13:54:07.7570767Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7571156Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7571156Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7572718Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7573070Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7574416Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7574416Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7574528Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7574985Z In file included from ll-merge.c:7:
2019-11-23T13:54:07.7575122Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7578868Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7578868Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7579390Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7580373Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7580628Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7580628Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7580725Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7581649Z        ^
2019-11-23T13:54:07.7581721Z In file included from kwset.c:37:
2019-11-23T13:54:07.7581838Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7585599Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7586094Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7586259Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7587433Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7587433Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7587531Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7587825Z In file included from levenshtein.c:1:
2019-11-23T13:54:07.7587896Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7591561Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7591561Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7592185Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7592364Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7592691Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7592691Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7592824Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7593218Z In file included from line-log.c:1:
2019-11-23T13:54:07.7593495Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7593495Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7593884Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7593996Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7595130Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7595130Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7595415Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7595924Z In file included from list-objects.c:1:
2019-11-23T13:54:07.7596064Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7596339Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7596339Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7600277Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7600393Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7600637Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7600637Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7600724Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7600894Z In file included from lockfile.c:5:
2019-11-23T13:54:07.7602308Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7602663Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7602663Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:07.7603090Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:07.7603408Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:07.7606862Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7606862Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:07.7608141Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:07.7612268Z In file included from line-log.c:1:
2019-11-23T13:54:07.7612268Z In file included from line-log.c:1:
2019-11-23T13:54:07.7612634Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7612719Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7614381Z In file included from line-range.c:1:
2019-11-23T13:54:07.7614381Z In file included from line-range.c:1:
2019-11-23T13:54:07.7614677Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7618685Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7618934Z In file included from hex.c:1:
2019-11-23T13:54:07.7619010Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7619010Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7619462Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7619547Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7619667Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:07.7619774Z In file included from kwset.c:37:
2019-11-23T13:54:07.7619844Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7623325Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7623451Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7623629Z In file included from ident.c:8:
2019-11-23T13:54:07.7623736Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7623736Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7624138Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7625535Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7625991Z In file included from ll-merge.c:7:
2019-11-23T13:54:07.7626132Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7626132Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7626434Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7626568Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7630165Z In file included from levenshtein.c:1:
2019-11-23T13:54:07.7630249Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7630249Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7630707Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7630849Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7631023Z 2 errors generated.
2019-11-23T13:54:07.7631023Z 2 errors generated.
2019-11-23T13:54:07.7631266Z make: *** [line-range.o] Error 1
2019-11-23T13:54:07.7631391Z make: *** Waiting for unfinished jobs....
2019-11-23T13:54:07.7631459Z 2 errors generated.
2019-11-23T13:54:07.7633147Z make: *** [hex.o] Error 1
2019-11-23T13:54:07.7633794Z In file included from list-objects.c:1:
2019-11-23T13:54:07.7633962Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7633962Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7634288Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7634419Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7634488Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:07.7637938Z make: *** [levenshtein.o] Error 1
2019-11-23T13:54:07.7638017Z 2 errors generated.
2019-11-23T13:54:07.7638423Z make: *** [ll-merge.o] Error 1
2019-11-23T13:54:07.7638497Z In file included from lockfile.c:5:
2019-11-23T13:54:07.7638617Z In file included from ./cache.h:4:
2019-11-23T13:54:07.7638949Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:07.7639031Z #include <openssl/ssl.h>
2019-11-23T13:54:07.7640152Z 2 errors generated.
2019-11-23T13:54:07.7640152Z 2 errors generated.
2019-11-23T13:54:07.7643723Z make: *** [ident.o] Error 1
2019-11-23T13:54:07.7643831Z 2 errors generated.
2019-11-23T13:54:07.7643951Z make: *** [kwset.o] Error 1
2019-11-23T13:54:07.7644432Z 2 errors generated.
2019-11-23T13:54:07.7644432Z 2 errors generated.
2019-11-23T13:54:07.7644802Z make: *** [line-log.o] Error 1
2019-11-23T13:54:07.7644943Z make: *** [lockfile.o] Error 1
2019-11-23T13:54:07.7648595Z 2 errors generated.
2019-11-23T13:54:07.7649151Z make: *** [list-objects.o] Error 1
2019-11-23T13:54:08.9668047Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-23T13:54:10.4098681Z Sending build context to Docker daemon  527.9kB
2019-11-23T13:54:10.4099464Z 
2019-11-23T13:54:10.4265591Z Step 1/41 : FROM centos:5
2019-11-23T13:54:10.4271619Z  ---> 1ae98b2c895d
---
2019-11-23T13:54:10.4575266Z  ---> ead6d4f619f9
2019-11-23T13:54:10.4575651Z Step 28/41 : RUN ./build-git.sh
2019-11-23T13:54:10.7396861Z  ---> Running in dc6e597e5ba8
2019-11-23T13:54:11.1496145Z + source shared.sh
2019-11-23T13:54:11.1496710Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-23T13:54:11.1512378Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-23T13:54:11.1512623Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T13:54:11.1512691Z 
2019-11-23T13:54:11.4111004Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-23T13:54:12.8879270Z + set +x
2019-11-23T13:54:18.6650443Z + hide_output make -j10
2019-11-23T13:54:18.6650861Z + set +x
2019-11-23T13:54:19.1518876Z shared.sh: line 1:   157 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T13:54:19.1521112Z ERROR: An error was encountered with the build.
2019-11-23T13:54:19.1530982Z     * new build flags
2019-11-23T13:54:19.1531939Z     CC base85.o
2019-11-23T13:54:19.1536695Z     * new link flags
2019-11-23T13:54:19.1536779Z     CC blob.o
2019-11-23T13:54:19.1546922Z     * new prefix flags
2019-11-23T13:54:19.1555014Z     CC bisect.o
2019-11-23T13:54:19.1555140Z     CC branch.o
2019-11-23T13:54:19.1562505Z     CC bulk-checkin.o
2019-11-23T13:54:19.1562628Z     CC bundle.o
2019-11-23T13:54:19.1571604Z     CC cache-tree.o
2019-11-23T13:54:19.1571732Z     CC color.o
2019-11-23T13:54:19.1572754Z     CC column.o
2019-11-23T13:54:19.1573233Z     CC combine-diff.o
2019-11-23T13:54:19.1577034Z In file included from branch.c:1:
2019-11-23T13:54:19.1577412Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1578786Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1580102Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1581100Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1581100Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1581218Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1584991Z In file included from blob.c:1:
2019-11-23T13:54:19.1585118Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1589882Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1589882Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1590578Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1590694Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1595252Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1595252Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1596171Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1600786Z In file included from bisect.c:1:
2019-11-23T13:54:19.1605052Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1605505Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1605505Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1609525Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1609666Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1613557Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1613557Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1614630Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1618401Z In file included from base85.c:1:
2019-11-23T13:54:19.1618535Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1618961Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1618961Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1620202Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1620328Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1624071Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1624071Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1624237Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1625252Z In file included from bundle.c:1:
2019-11-23T13:54:19.1626207Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1626732Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1626732Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1630510Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1630636Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1632822Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1632822Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1632982Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1636681Z        ^
2019-11-23T13:54:19.1637102Z In file included from bulk-checkin.c:4:
2019-11-23T13:54:19.1641015Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1641417Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1643474Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1647907Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1651794Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1651794Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1651933Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1655565Z In file included from color.c:1:
2019-11-23T13:54:19.1655715Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1656129Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1656129Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1656751Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1657858Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1661658Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1661658Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1661782Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1663094Z In file included from cache-tree.c:1:
2019-11-23T13:54:19.1664165Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1665367Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1665367Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1665851Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1666812Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1667074Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1667074Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1667168Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1674700Z In file included from column.c:1:
2019-11-23T13:54:19.1674850Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1675314Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1675314Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1675659Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1679371Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1679590Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1679590Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1679682Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1687727Z In file included from combine-diff.c:1:
2019-11-23T13:54:19.1687853Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1695020Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1695020Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:19.1695535Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:19.1700008Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:19.1707154Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1707154Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:19.1707325Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:19.1711150Z In file included from bisect.c:1:
2019-11-23T13:54:19.1711271Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1711271Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1712420Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1712522Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1713695Z In file included from blob.c:1:
2019-11-23T13:54:19.1713823Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1713823Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1715377Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1715507Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1715703Z In file included from base85.c:1:
2019-11-23T13:54:19.1716666Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1716666Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1717057Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1718166Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1719310Z In file included from branch.c:1:
2019-11-23T13:54:19.1719310Z In file included from branch.c:1:
2019-11-23T13:54:19.1720639Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1720765Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1720947Z In file included from bundle.c:1:
2019-11-23T13:54:19.1721073Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1721073Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1721472Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1721657Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1721869Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:19.1726038Z In file included from bulk-checkin.c:4:
2019-11-23T13:54:19.1729719Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1730185Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1731146Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1735104Z In file included from color.c:1:
2019-11-23T13:54:19.1735190Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1735190Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1736489Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1736652Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1737810Z 2 errors generated.
2019-11-23T13:54:19.1737810Z 2 errors generated.
2019-11-23T13:54:19.1737925Z make: *** [blob.o] Error 1
2019-11-23T13:54:19.1738042Z make: *** Waiting for unfinished jobs....
2019-11-23T13:54:19.1738408Z In file included from cache-tree.c:1:
2019-11-23T13:54:19.1738556Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1738854Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1738989Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1739992Z In file included from column.c:1:
2019-11-23T13:54:19.1740819Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1740819Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1741262Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1745425Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1746400Z 2 errors generated.
2019-11-23T13:54:19.1746400Z 2 errors generated.
2019-11-23T13:54:19.1746480Z make: *** [base85.o] Error 1
2019-11-23T13:54:19.1747759Z In file included from combine-diff.c:1:
2019-11-23T13:54:19.1747844Z In file included from ./cache.h:4:
2019-11-23T13:54:19.1749674Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:19.1750561Z #include <openssl/ssl.h>
2019-11-23T13:54:19.1754237Z 2 errors generated.
2019-11-23T13:54:19.1754352Z 2 errors generated.
2019-11-23T13:54:19.1754352Z 2 errors generated.
2019-11-23T13:54:19.1757992Z make: *** [color.o] Error 1
2019-11-23T13:54:19.1758076Z make: *** [branch.o] Error 1
2019-11-23T13:54:19.1758213Z 2 errors generated.
2019-11-23T13:54:19.1758279Z make: *** [bisect.o] Error 1
2019-11-23T13:54:19.1759323Z 2 errors generated.
2019-11-23T13:54:19.1763571Z make: *** [bulk-checkin.o] Error 1
2019-11-23T13:54:19.1763696Z 2 errors generated.
2019-11-23T13:54:19.1764130Z make: *** [combine-diff.o] Error 1
2019-11-23T13:54:19.1764209Z 2 errors generated.
2019-11-23T13:54:19.1764508Z make: *** [cache-tree.o] Error 1
2019-11-23T13:54:19.1765815Z 2 errors generated.
2019-11-23T13:54:19.1765815Z 2 errors generated.
2019-11-23T13:54:19.1765892Z make: *** [column.o] Error 1
2019-11-23T13:54:19.1766010Z make: *** [bundle.o] Error 1
2019-11-23T13:54:20.3828421Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-23T13:54:22.4638101Z Sending build context to Docker daemon  527.9kB
2019-11-23T13:54:22.4638231Z 
2019-11-23T13:54:22.4819344Z Step 1/41 : FROM centos:5
2019-11-23T13:54:22.4823087Z  ---> 1ae98b2c895d
---
2019-11-23T13:54:22.5625842Z Step 28/41 : RUN ./build-git.sh
2019-11-23T13:54:22.7283037Z  ---> Running in ea0dce69965b
2019-11-23T13:54:23.2000716Z + source shared.sh
2019-11-23T13:54:23.2001171Z + tar xzf -
2019-11-23T13:54:23.2001504Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-23T13:54:23.2002104Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T13:54:23.2002167Z 
2019-11-23T13:54:23.4368764Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
2019-11-23T13:54:23.4369447Z 100   162  100   162    0     0    680      0 --:--:-- --:--:-- --:--:--   680
---
2019-11-23T13:54:24.9498520Z + set +x
2019-11-23T13:54:30.7402884Z + hide_output make -j10
2019-11-23T13:54:30.7406161Z + set +x
2019-11-23T13:54:31.2532447Z shared.sh: line 1:   156 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T13:54:31.2533672Z ERROR: An error was encountered with the build.
2019-11-23T13:54:31.2548092Z     * new build flags
2019-11-23T13:54:31.2548207Z     * new link flags
2019-11-23T13:54:31.2548280Z     * new prefix flags
2019-11-23T13:54:31.2548376Z     CC base85.o
2019-11-23T13:54:31.2548444Z     CC bisect.o
2019-11-23T13:54:31.2548525Z     CC blob.o
2019-11-23T13:54:31.2548591Z     CC branch.o
2019-11-23T13:54:31.2548674Z     CC bundle.o
2019-11-23T13:54:31.2549124Z     CC bulk-checkin.o
2019-11-23T13:54:31.2549408Z     CC cache-tree.o
2019-11-23T13:54:31.2549482Z     CC color.o
2019-11-23T13:54:31.2549564Z     CC column.o
2019-11-23T13:54:31.2549646Z In file included from blob.c:1:
2019-11-23T13:54:31.2549719Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2550006Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2550344Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2550472Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2550646Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2550646Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2550754Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2551071Z     CC combine-diff.o
2019-11-23T13:54:31.2551146Z In file included from base85.c:1:
2019-11-23T13:54:31.2551449Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2552100Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2552100Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2552459Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2552572Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2552755Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2552755Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2552846Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2553000Z In file included from branch.c:1:
2019-11-23T13:54:31.2553281Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2553281Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2553612Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2553736Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2553913Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2553913Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2554156Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2554314Z In file included from bisect.c:1:
2019-11-23T13:54:31.2554386Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2554703Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2554703Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2555162Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2555282Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2555445Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2555445Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2555549Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2555622Z        ^
2019-11-23T13:54:31.2555877Z In file included from bulk-checkin.c:4:
2019-11-23T13:54:31.2555952Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2556226Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2556573Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2556680Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2556842Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2556842Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2556944Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2557093Z In file included from bundle.c:1:
2019-11-23T13:54:31.2557162Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2557432Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2557432Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2557767Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2557873Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2558043Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2558043Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2558154Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2558478Z In file included from cache-tree.c:1:
2019-11-23T13:54:31.2558566Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2558816Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2558816Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2559149Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2559255Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2559417Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2559417Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2559519Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2559667Z In file included from color.c:1:
2019-11-23T13:54:31.2559751Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2560092Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2560092Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2560601Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2560710Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2560875Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2560875Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2560981Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2561136Z In file included from column.c:1:
2019-11-23T13:54:31.2561224Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2561491Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2561491Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2561836Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2561944Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2562134Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2562134Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2562317Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2562472Z In file included from blob.c:1:
2019-11-23T13:54:31.2562558Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2562558Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2562878Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2562982Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2563316Z In file included from combine-diff.c:1:
2019-11-23T13:54:31.2563406Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2563667Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2563667Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:31.2564015Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:31.2564122Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:31.2564300Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2564300Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:31.2564413Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:31.2564568Z In file included from base85.c:1:
2019-11-23T13:54:31.2564654Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2564654Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2565072Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2565170Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2565320Z 2 errors generated.
2019-11-23T13:54:31.2565320Z 2 errors generated.
2019-11-23T13:54:31.2565386Z make: *** [blob.o] Error 1
2019-11-23T13:54:31.2565474Z make: *** Waiting for unfinished jobs....
2019-11-23T13:54:31.2565545Z In file included from branch.c:1:
2019-11-23T13:54:31.2565843Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2565942Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2566098Z In file included from bisect.c:1:
2019-11-23T13:54:31.2566176Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2566176Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2566475Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2566557Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2566639Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:31.2566889Z In file included from bulk-checkin.c:4:
2019-11-23T13:54:31.2567312Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2567674Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2567758Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2567908Z In file included from color.c:1:
2019-11-23T13:54:31.2567994Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2567994Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2568276Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2568374Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2568639Z In file included from bundle.c:1:
2019-11-23T13:54:31.2568747Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2568747Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2569061Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2569158Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2569308Z In file included from column.c:1:
2019-11-23T13:54:31.2569376Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2569376Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2569669Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2569765Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2570083Z In file included from cache-tree.c:1:
2019-11-23T13:54:31.2570157Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2570157Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2570450Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2570533Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2570683Z 2 errors generated.
2019-11-23T13:54:31.2570683Z 2 errors generated.
2019-11-23T13:54:31.2570773Z make: *** [base85.o] Error 1
2019-11-23T13:54:31.2571151Z In file included from combine-diff.c:1:
2019-11-23T13:54:31.2571226Z In file included from ./cache.h:4:
2019-11-23T13:54:31.2571520Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:31.2571602Z #include <openssl/ssl.h>
2019-11-23T13:54:31.2571751Z 2 errors generated.
2019-11-23T13:54:31.2571751Z 2 errors generated.
2019-11-23T13:54:31.2571834Z make: *** [branch.o] Error 1
2019-11-23T13:54:31.2571901Z 2 errors generated.
2019-11-23T13:54:31.2571982Z make: *** [color.o] Error 1
2019-11-23T13:54:31.2572064Z 2 errors generated.
2019-11-23T13:54:31.2572299Z make: *** [bulk-checkin.o] Error 1
2019-11-23T13:54:31.2572453Z 2 errors generated.
2019-11-23T13:54:31.2572453Z 2 errors generated.
2019-11-23T13:54:31.2572533Z make: *** [bisect.o] Error 1
2019-11-23T13:54:31.2572763Z make: *** [cache-tree.o] Error 1
2019-11-23T13:54:31.2572922Z 2 errors generated.
2019-11-23T13:54:31.2572922Z 2 errors generated.
2019-11-23T13:54:31.2573013Z make: *** [bundle.o] Error 1
2019-11-23T13:54:31.2573079Z make: *** [column.o] Error 1
2019-11-23T13:54:31.2573160Z 2 errors generated.
2019-11-23T13:54:31.2573412Z make: *** [combine-diff.o] Error 1
2019-11-23T13:54:32.4958480Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-23T13:54:35.6378385Z Sending build context to Docker daemon  527.9kB
2019-11-23T13:54:35.6378729Z 
2019-11-23T13:54:35.6539864Z Step 1/41 : FROM centos:5
2019-11-23T13:54:35.6545381Z  ---> 1ae98b2c895d
---
2019-11-23T13:54:35.6660678Z  ---> ead6d4f619f9
2019-11-23T13:54:35.6664304Z Step 28/41 : RUN ./build-git.sh
2019-11-23T13:54:35.9182430Z  ---> Running in 322466f224d3
2019-11-23T13:54:36.3481425Z + source shared.sh
2019-11-23T13:54:36.3481980Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-23T13:54:36.3482320Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-23T13:54:36.3482420Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T13:54:36.3482494Z 
2019-11-23T13:54:36.5816974Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-23T13:54:38.0057591Z + set +x
2019-11-23T13:54:43.7341401Z + hide_output make -j10
2019-11-23T13:54:43.7341937Z + set +x
2019-11-23T13:54:44.1905086Z shared.sh: line 1:   156 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T13:54:44.1906429Z ERROR: An error was encountered with the build.
2019-11-23T13:54:44.2334185Z     * new build flags
2019-11-23T13:54:44.2334348Z     * new link flags
2019-11-23T13:54:44.2334415Z     * new prefix flags
2019-11-23T13:54:44.2335237Z     GEN common-cmds.h
2019-11-23T13:54:44.2335323Z     CC hex.o
2019-11-23T13:54:44.2335408Z     CC kwset.o
2019-11-23T13:54:44.2335471Z     CC ident.o
2019-11-23T13:54:44.2335549Z     CC levenshtein.o
2019-11-23T13:54:44.2336002Z     CC line-log.o
2019-11-23T13:54:44.2336252Z     CC line-range.o
2019-11-23T13:54:44.2336709Z     CC list-objects.o
2019-11-23T13:54:44.2336969Z     CC ll-merge.o
2019-11-23T13:54:44.2337054Z     CC lockfile.o
2019-11-23T13:54:44.2337502Z In file included from line-log.c:1:
2019-11-23T13:54:44.2339449Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2340537Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2340886Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2341106Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2341106Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2341214Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2341373Z In file included from ident.c:8:
2019-11-23T13:54:44.2341606Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2342091Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2342091Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2342763Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2342898Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2343221Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2343221Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2343324Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2343694Z In file included from line-range.c:1:
2019-11-23T13:54:44.2344166Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2344166Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2344530Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2344822Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2344987Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2344987Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2345071Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2345218Z In file included from hex.c:1:
2019-11-23T13:54:44.2345300Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2345746Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2345746Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2346095Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2346382Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2346754Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2346754Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2346865Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2347129Z        ^
2019-11-23T13:54:44.2347194Z In file included from kwset.c:37:
2019-11-23T13:54:44.2347280Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2347632Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2348168Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2348306Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2348466Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2348466Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2348711Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2348867Z In file included from levenshtein.c:1:
2019-11-23T13:54:44.2348953Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2349416Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2349416Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2349997Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2350295Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2350472Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2350472Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2350555Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2351124Z In file included from list-objects.c:1:
2019-11-23T13:54:44.2351212Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2351510Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2351510Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2352044Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2352179Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2352349Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2352349Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2352454Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2352771Z In file included from lockfile.c:5:
2019-11-23T13:54:44.2352839Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2353163Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2353163Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2353692Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2353825Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2353987Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2353987Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2354236Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2354616Z In file included from ll-merge.c:7:
2019-11-23T13:54:44.2354699Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2355184Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2355184Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:44.2355510Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:44.2355777Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:44.2355958Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2355958Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:44.2356058Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:44.2356570Z In file included from line-range.c:1:
2019-11-23T13:54:44.2356570Z In file included from line-range.c:1:
2019-11-23T13:54:44.2356923Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2357027Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2357173Z In file included from hex.c:1:
2019-11-23T13:54:44.2357426Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2357426Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2357866Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2358142Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2358579Z In file included from line-log.c:1:
2019-11-23T13:54:44.2358579Z In file included from line-log.c:1:
2019-11-23T13:54:44.2359035Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2359149Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2359213Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:44.2359293Z In file included from kwset.c:37:
2019-11-23T13:54:44.2359374Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2359877Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2359990Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2360134Z In file included from levenshtein.c:1:
2019-11-23T13:54:44.2360202Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2360202Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2360728Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2360852Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2361106Z In file included from ident.c:8:
2019-11-23T13:54:44.2361335Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2361335Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2361740Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2361822Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2362106Z 2 errors generated.
2019-11-23T13:54:44.2362228Z In file included from lockfile.c:5:
2019-11-23T13:54:44.2362296Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2362296Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2362642Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2362738Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2362970Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:44.2363274Z make: *** [line-range.o] Error 1
2019-11-23T13:54:44.2363348Z make: *** Waiting for unfinished jobs....
2019-11-23T13:54:44.2363885Z In file included from list-objects.c:1:
2019-11-23T13:54:44.2365431Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2365431Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2365864Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2366120Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2366228Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:44.2366292Z make: *** [hex.o] Error 1
2019-11-23T13:54:44.2366598Z In file included from ll-merge.c:7:
2019-11-23T13:54:44.2366671Z In file included from ./cache.h:4:
2019-11-23T13:54:44.2367369Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:44.2367482Z #include <openssl/ssl.h>
2019-11-23T13:54:44.2367808Z 2 errors generated.
2019-11-23T13:54:44.2367808Z 2 errors generated.
2019-11-23T13:54:44.2367871Z make: *** [levenshtein.o] Error 1
2019-11-23T13:54:44.2367951Z 2 errors generated.
2019-11-23T13:54:44.2368013Z make: *** [kwset.o] Error 1
2019-11-23T13:54:44.2368093Z 2 errors generated.
2019-11-23T13:54:44.2368171Z make: *** [lockfile.o] Error 1
2019-11-23T13:54:44.2368258Z 2 errors generated.
2019-11-23T13:54:44.2368321Z make: *** [ident.o] Error 1
2019-11-23T13:54:44.2368538Z 2 errors generated.
2019-11-23T13:54:44.2368850Z make: *** [line-log.o] Error 1
2019-11-23T13:54:44.2369000Z 2 errors generated.
2019-11-23T13:54:44.2369000Z 2 errors generated.
2019-11-23T13:54:44.2369443Z make: *** [list-objects.o] Error 1
2019-11-23T13:54:44.2369702Z make: *** [ll-merge.o] Error 1
2019-11-23T13:54:45.3641394Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-23T13:54:49.4400247Z Sending build context to Docker daemon  527.9kB
2019-11-23T13:54:49.4400690Z 
2019-11-23T13:54:49.4619245Z Step 1/41 : FROM centos:5
2019-11-23T13:54:49.4623248Z  ---> 1ae98b2c895d
---
2019-11-23T13:54:49.4768315Z  ---> ead6d4f619f9
2019-11-23T13:54:49.4768626Z Step 28/41 : RUN ./build-git.sh
2019-11-23T13:54:49.7308335Z  ---> Running in 3690703fc513
2019-11-23T13:54:50.1444837Z + source shared.sh
2019-11-23T13:54:50.1445330Z + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
2019-11-23T13:54:50.1445650Z   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
2019-11-23T13:54:50.1445760Z                                  Dload  Upload   Total   Spent    Left  Speed
2019-11-23T13:54:50.1445963Z 
2019-11-23T13:54:50.3823481Z   0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
2019-11-23T13:54:51.8212991Z + set +x
2019-11-23T13:54:57.5698838Z + hide_output make -j10
2019-11-23T13:54:57.5699159Z + set +x
2019-11-23T13:54:58.0805969Z shared.sh: line 1:   156 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
2019-11-23T13:54:58.0806483Z ERROR: An error was encountered with the build.
2019-11-23T13:54:58.0819641Z     * new build flags
2019-11-23T13:54:58.1076891Z     * new link flags
2019-11-23T13:54:58.1077141Z     CC base85.o
2019-11-23T13:54:58.1077297Z     CC bisect.o
2019-11-23T13:54:58.1077469Z     CC blob.o
2019-11-23T13:54:58.1077616Z     CC branch.o
2019-11-23T13:54:58.1077789Z     * new prefix flags
2019-11-23T13:54:58.1078853Z     CC bulk-checkin.o
2019-11-23T13:54:58.1079046Z     CC bundle.o
2019-11-23T13:54:58.1079554Z     CC cache-tree.o
2019-11-23T13:54:58.1079635Z     CC color.o
2019-11-23T13:54:58.1079694Z     CC column.o
2019-11-23T13:54:58.1079912Z     CC combine-diff.o
2019-11-23T13:54:58.1079993Z In file included from base85.c:1:
2019-11-23T13:54:58.1080057Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1080322Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1080652Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1080779Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1080932Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1080932Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1081029Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1081168Z In file included from blob.c:1:
2019-11-23T13:54:58.1081230Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1081494Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1081494Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1081792Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1081905Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1082054Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1082054Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1082378Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1082540Z In file included from bisect.c:1:
2019-11-23T13:54:58.1082603Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1082904Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1082904Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1083200Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1083312Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1083459Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1083459Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1083553Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1083689Z In file included from branch.c:1:
2019-11-23T13:54:58.1083921Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1083921Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1084238Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1084450Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1084617Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1084617Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1084878Z In file included from bulk-checkin.c:4:
2019-11-23T13:54:58.1084961Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1085190Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1085497Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1085591Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1085755Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1085755Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1085834Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1085916Z        ^
2019-11-23T13:54:58.1085986Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1086128Z In file included from bundle.c:1:
2019-11-23T13:54:58.1086207Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1086446Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1086446Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1086758Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1086868Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1087273Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1087273Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1087350Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1087487Z In file included from color.c:1:
2019-11-23T13:54:58.1087567Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1087866Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1087866Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1088297Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1089354Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1089949Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1089949Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1090597Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1091073Z In file included from cache-tree.c:1:
2019-11-23T13:54:58.1091172Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1091453Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1091453Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1091789Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1091914Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1092407Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1092407Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1092665Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1093143Z In file included from combine-diff.c:1:
2019-11-23T13:54:58.1093221Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1093499Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1093499Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1093834Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1093958Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1094127Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1094127Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1094232Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1094387Z In file included from column.c:1:
2019-11-23T13:54:58.1094458Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1094741Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1094741Z In file included from ./git-compat-util.h:160:
2019-11-23T13:54:58.1095084Z /usr/include/sys/stat.h:483:8: error: definition with same mangled name 'stat64' as another definition
2019-11-23T13:54:58.1095323Z __NTH (stat64 (__const char *__path, struct stat64 *__statbuf))
2019-11-23T13:54:58.1095491Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1095491Z /usr/include/sys/stat.h:434:8: note: previous definition is here
2019-11-23T13:54:58.1095596Z __NTH (stat (__const char *__path, struct stat *__statbuf))
2019-11-23T13:54:58.1095872Z In file included from blob.c:1:
2019-11-23T13:54:58.1095943Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1095943Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1096282Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1096492Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1096655Z In file included from base85.c:1:
2019-11-23T13:54:58.1096722Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1096722Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1097101Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1097184Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1097325Z In file included from bisect.c:1:
2019-11-23T13:54:58.1097403Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1097403Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1097660Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1097750Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1097826Z          ^~~~~~~~~~~~~~~
2019-11-23T13:54:58.1098040Z In file included from bulk-checkin.c:4:
2019-11-23T13:54:58.1098124Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1098375Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1098539Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1099724Z In file included from branch.c:1:
2019-11-23T13:54:58.1099724Z In file included from branch.c:1:
2019-11-23T13:54:58.1100115Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1103775Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1104967Z In file included from bundle.c:1:
2019-11-23T13:54:58.1106020Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1106020Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1127980Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1128117Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1128246Z 2 errors generated.
2019-11-23T13:54:58.1128246Z 2 errors generated.
2019-11-23T13:54:58.1128311Z make: *** [blob.o] Error 1
2019-11-23T13:54:58.1128373Z make: *** Waiting for unfinished jobs....
2019-11-23T13:54:58.1128442Z 2 errors generated.
2019-11-23T13:54:58.1128501Z make: *** [base85.o] Error 1
2019-11-23T13:54:58.1128570Z In file included from color.c:1:
2019-11-23T13:54:58.1128636Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1129080Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1129167Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1129740Z In file included from cache-tree.c:1:
2019-11-23T13:54:58.1129840Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1129840Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1130329Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1130423Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1130573Z In file included from column.c:1:
2019-11-23T13:54:58.1130644Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1130644Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1130951Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1131034Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1131358Z In file included from combine-diff.c:1:
2019-11-23T13:54:58.1131440Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1131440Z In file included from ./cache.h:4:
2019-11-23T13:54:58.1131733Z ./git-compat-util.h:280:10: fatal error: 'openssl/ssl.h' file not found
2019-11-23T13:54:58.1131817Z #include <openssl/ssl.h>
2019-11-23T13:54:58.1131960Z 2 errors generated.
2019-11-23T13:54:58.1131960Z 2 errors generated.
2019-11-23T13:54:58.1132219Z make: *** [bulk-checkin.o] Error 1
2019-11-23T13:54:58.1132475Z 2 errors generated.
2019-11-23T13:54:58.1132475Z 2 errors generated.
2019-11-23T13:54:58.1132542Z make: *** [branch.o] Error 1
2019-11-23T13:54:58.1132618Z make: *** [color.o] Error 1
2019-11-23T13:54:58.1132757Z 2 errors generated.
2019-11-23T13:54:58.1132757Z 2 errors generated.
2019-11-23T13:54:58.1133038Z make: *** [cache-tree.o] Error 1
2019-11-23T13:54:58.1133115Z make: *** [bisect.o] Error 1
2019-11-23T13:54:58.1133189Z 2 errors generated.
2019-11-23T13:54:58.1133256Z make: *** [bundle.o] Error 1
2019-11-23T13:54:58.1133499Z 2 errors generated.
2019-11-23T13:54:58.1133563Z make: *** [column.o] Error 1
2019-11-23T13:54:58.1133761Z 2 errors generated.
2019-11-23T13:54:58.1134090Z make: *** [combine-diff.o] Error 1
2019-11-23T13:54:59.2612156Z The command '/bin/sh -c ./build-git.sh' returned a non-zero code: 1
2019-11-23T13:54:59.3127743Z 
2019-11-23T13:54:59.3127743Z 
2019-11-23T13:54:59.4267184Z ##[error]Bash exited with code '1'.
2019-11-23T13:54:59.4631967Z ##[section]Starting: Checkout
2019-11-23T13:54:59.4676901Z ==============================================================================
2019-11-23T13:54:59.4677032Z Task         : Get sources
2019-11-23T13:54:59.4677111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
