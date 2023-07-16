plain
2020-04-11T04:12:04.6619837Z ========================== Starting Command Output ===========================
2020-04-11T04:12:04.6622255Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/51b86111-722f-4127-8cbe-486009e3845e.sh
2020-04-11T04:12:04.6622565Z 
2020-04-11T04:12:04.6626396Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T04:12:04.6646102Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T04:12:04.6649318Z Task         : Get sources
2020-04-11T04:12:04.6649628Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T04:12:04.6649931Z Version      : 1.0.0
2020-04-11T04:12:04.6650136Z Author       : Microsoft
---
2020-04-11T04:12:05.6597244Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T04:12:05.6609572Z ##[command]git config gc.auto 0
2020-04-11T04:12:05.6613495Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T04:12:05.6616524Z ##[command]git config --get-all http.proxy
2020-04-11T04:12:05.6622153Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70989/merge:refs/remotes/pull/70989/merge
---
2020-04-11T04:15:17.4728088Z   iso-codes libaio1 libarchive13 libasan4 libasan4-armel-cross
2020-04-11T04:15:17.4728660Z   libasn1-8-heimdal libasound2 libasound2-data libasyncns0 libatomic1
2020-04-11T04:15:17.4729245Z   libatomic1-armel-cross libbabeltrace1 libbinutils libbluetooth3 libbrlapi0.6
2020-04-11T04:15:17.4729829Z   libbsd-dev libbsd0 libc-ares2 libc-dev-bin libc6-armel-cross libc6-dev
2020-04-11T04:15:17.4730419Z   libc6-dev-armel-cross libcaca0 libcacard0 libcap2 libcap2-bin libcc1-0
2020-04-11T04:15:17.4731567Z   libdpkg-perl libdw1 libedit2 libelf1 liberror-perl libexpat1 libfdt1
2020-04-11T04:15:17.4737272Z   libffi-dev libflac8 libgcc-7-dev libgcc-7-dev-armel-cross libgcc1
2020-04-11T04:15:17.4737854Z   libgcc1-armel-cross libgdbm-compat4 libgdbm5 libglib2.0-0 libgomp1
2020-04-11T04:15:17.4738406Z   libgomp1-armel-cross libgssapi-krb5-2 libgssapi3-heimdal
2020-04-11T04:15:17.4738406Z   libgomp1-armel-cross libgssapi-krb5-2 libgssapi3-heimdal
2020-04-11T04:15:17.4738972Z   libgstreamer-plugins-base1.0-0 libgstreamer1.0-0 libhcrypto4-heimdal
2020-04-11T04:15:17.4739528Z   libheimbase1-heimdal libheimntlm0-heimdal libhttp-parser2.7.1
2020-04-11T04:15:17.4740102Z   libhx509-5-heimdal libibverbs1 libicu60 libiscsi7 libisl19 libitm1
2020-04-11T04:15:17.4741223Z   libkrb5-26-heimdal libkrb5-3 libkrb5support0 libldap-2.4-2 libldap-common
2020-04-11T04:15:17.4741814Z   libllvm7 liblsan0 liblzo2-2 libmagic-mgc libmagic1 libmpc3 libmpdec2
2020-04-11T04:15:17.4741814Z   libllvm7 liblsan0 liblzo2-2 libmagic-mgc libmagic1 libmpc3 libmpdec2
2020-04-11T04:15:17.4742407Z   libmpfr6 libmpx2 libncurses5-dev libnghttp2-14 libnl-3-200 libnl-route-3-200
2020-04-11T04:15:17.4742989Z   libnspr4 libnss3 libnuma1 libogg0 libopus0 liborc-0.4-0 libperl5.26
2020-04-11T04:15:17.4744185Z   libpython2.7-minimal libpython2.7-stdlib libpython3.6 libpython3.6-minimal
2020-04-11T04:15:17.4744185Z   libpython2.7-minimal libpython2.7-stdlib libpython3.6 libpython3.6-minimal
2020-04-11T04:15:17.4745042Z   libpython3.6-stdlib libquadmath0 librados2 librbd1 librdmacm1 libreadline7
2020-04-11T04:15:17.4746288Z   libsdl1.2debian libslang2 libsndfile1 libspice-server1 libsqlite3-0
2020-04-11T04:15:17.4746956Z   libssl1.0.0 libssl1.1 libstdc++-7-dev libstdc++-7-dev-armel-cross libstdc++6
2020-04-11T04:15:17.4747528Z   libstdc++6-armel-cross libtinfo-dev libtsan0 libubsan0 libubsan0-armel-cross
2020-04-11T04:15:17.4748059Z   libusb-1.0-0 libusbredirparser1 libuv1 libvorbis0a libvorbisenc2
---
2020-04-11T04:15:17.4757373Z   libmpx2-dbg-armel-cross libquadmath0-dbg-armel-cross gdb-arm-linux-gnueabi
2020-04-11T04:15:17.4757963Z   gdb-doc gettext-base git-daemon-run | git-daemon-sysvinit git-doc git-el
2020-04-11T04:15:17.4758560Z   git-email git-gui gitk gitweb git-cvs git-mediawiki git-svn isoquery lrzip
2020-04-11T04:15:17.4759137Z   libasound2-plugins alsa-utils glibc-doc gnupg | gnupg2 bzr gdbm-l10n
2020-04-11T04:15:17.4759709Z   krb5-doc krb5-user libvisual-0.4-plugins gstreamer1.0-tools ncurses-doc
2020-04-11T04:15:17.4760299Z   opus-tools pulseaudio libssl-doc libstdc++-7-doc llvm-7-doc make-doc ed
2020-04-11T04:15:17.4761362Z   | libterm-readline-perl-perl python-doc python-tk python2.7-doc samba vde2
2020-04-11T04:15:17.4761362Z   | libterm-readline-perl-perl python-doc python-tk python2.7-doc samba vde2
2020-04-11T04:15:17.4761821Z   qemu-efi readline-doc
2020-04-11T04:15:17.4762517Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-11T04:15:17.4762517Z   build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
2020-04-11T04:15:17.4763094Z   gdbserver less ssh-client manpages manpages-dev libpam-cap dbus
2020-04-11T04:15:17.4764056Z   libfile-fcntllock-perl liblocale-gettext-perl libglib2.0-data
2020-04-11T04:15:17.4764632Z   shared-mime-info xdg-user-dirs gstreamer1.0-plugins-base ibverbs-providers
2020-04-11T04:15:17.4765221Z   krb5-locales publicsuffix libsasl2-modules gstreamer1.0-plugins-good
2020-04-11T04:15:17.4765793Z   nodejs-doc netbase qemu-utils ipxe-qemu ipxe-qemu-256k-compat-efi-roms
2020-04-11T04:15:17.7816159Z   acl binfmt-support binutils binutils-arm-linux-gnueabi binutils-common
2020-04-11T04:15:17.7817688Z   binutils-x86-64-linux-gnu ca-certificates cmake cmake-data cpp cpp-7
2020-04-11T04:15:17.7826546Z   cpp-7-arm-linux-gnueabi cpp-arm-linux-gnueabi curl dpkg-dev file g++ g++-7
2020-04-11T04:15:17.7828094Z   g++-7-arm-linux-gnueabi g++-arm-linux-gnueabi gcc gcc-7
2020-04-11T04:15:17.7828094Z   g++-7-arm-linux-gnueabi g++-arm-linux-gnueabi gcc gcc-7
2020-04-11T04:15:17.7829888Z   gcc-7-arm-linux-gnueabi gcc-7-arm-linux-gnueabi-base gcc-7-base
2020-04-11T04:15:17.7831459Z   gcc-7-cross-base gcc-8-cross-base gcc-arm-linux-gnueabi gdb git git-man
2020-04-11T04:15:17.7832892Z   iso-codes libaio1 libarchive13 libasan4 libasan4-armel-cross
2020-04-11T04:15:17.7834271Z   libasn1-8-heimdal libasound2 libasound2-data libasyncns0 libatomic1
2020-04-11T04:15:17.7835855Z   libatomic1-armel-cross libbabeltrace1 libbinutils libbluetooth3 libbrlapi0.6
2020-04-11T04:15:17.7837289Z   libbsd-dev libbsd0 libc-ares2 libc-dev-bin libc6-armel-cross libc6-dev
2020-04-11T04:15:17.7838742Z   libc6-dev-armel-cross libcaca0 libcacard0 libcap2 libcap2-bin libcc1-0
2020-04-11T04:15:17.7842740Z   libdpkg-perl libdw1 libedit-dev libedit2 libelf1 liberror-perl libexpat1
2020-04-11T04:15:17.7844175Z   libfdt1 libffi-dev libflac8 libgcc-7-dev libgcc-7-dev-armel-cross
2020-04-11T04:15:17.7845198Z   libgcc1-armel-cross libgdbm-compat4 libgdbm5 libglib2.0-0 libgomp1
2020-04-11T04:15:17.7846161Z   libgomp1-armel-cross libgssapi-krb5-2 libgssapi3-heimdal
2020-04-11T04:15:17.7846161Z   libgomp1-armel-cross libgssapi-krb5-2 libgssapi3-heimdal
2020-04-11T04:15:17.7847170Z   libgstreamer-plugins-base1.0-0 libgstreamer1.0-0 libhcrypto4-heimdal
2020-04-11T04:15:17.7848159Z   libheimbase1-heimdal libheimntlm0-heimdal libhttp-parser2.7.1
2020-04-11T04:15:17.7849158Z   libhx509-5-heimdal libibverbs1 libicu60 libiscsi7 libisl19 libitm1
2020-04-11T04:15:17.7851183Z   libkrb5-26-heimdal libkrb5-3 libkrb5support0 libldap-2.4-2 libldap-common
2020-04-11T04:15:17.7852231Z   libllvm7 liblsan0 liblzo2-2 libmagic-mgc libmagic1 libmpc3 libmpdec2
2020-04-11T04:15:17.7852231Z   libllvm7 liblsan0 liblzo2-2 libmagic-mgc libmagic1 libmpc3 libmpdec2
2020-04-11T04:15:17.7853292Z   libmpfr6 libmpx2 libncurses5-dev libnghttp2-14 libnl-3-200 libnl-route-3-200
2020-04-11T04:15:17.7854341Z   libnspr4 libnss3 libnuma1 libogg0 libopus0 liborc-0.4-0 libperl5.26
2020-04-11T04:15:17.7856482Z   libpython2.7-minimal libpython2.7-stdlib libpython3.6 libpython3.6-minimal
2020-04-11T04:15:17.7856482Z   libpython2.7-minimal libpython2.7-stdlib libpython3.6 libpython3.6-minimal
2020-04-11T04:15:17.7857554Z   libpython3.6-stdlib libquadmath0 librados2 librbd1 librdmacm1 libreadline7
2020-04-11T04:15:17.7859632Z   libsdl1.2debian libslang2 libsndfile1 libspice-server1 libsqlite3-0
2020-04-11T04:15:17.7860679Z   libssl-dev libssl1.0.0 libssl1.1 libstdc++-7-dev libstdc++-7-dev-armel-cross
2020-04-11T04:15:17.7861767Z   libstdc++6-armel-cross libtinfo-dev libtsan0 libubsan0 libubsan0-armel-cross
2020-04-11T04:15:17.7862784Z   libusb-1.0-0 libusbredirparser1 libuv1 libvorbis0a libvorbisenc2
---
2020-04-11T04:15:19.8133700Z Get:54 http://archive.ubuntu.com/ubuntu bionic/main amd64 libiscsi7 amd64 1.17.0-1.1 [55.4 kB]
2020-04-11T04:15:19.9096806Z Get:55 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 gcc-8-base amd64 8.4.0-1ubuntu1~18.04 [18.7 kB]
2020-04-11T04:15:19.9114089Z Get:56 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libgcc1 amd64 1:8.4.0-1ubuntu1~18.04 [40.6 kB]
2020-04-11T04:15:19.9136237Z Get:57 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libstdc++6 amd64 8.4.0-1ubuntu1~18.04 [400 kB]
2020-04-11T04:15:19.9217053Z Get:58 http://archive.ubuntu.com/ubuntu bionic/main amd64 libnl-3-200 amd64 3.2.29-0ubuntu3 [52.8 kB]
2020-04-11T04:15:19.9229499Z Get:59 http://archive.ubuntu.com/ubuntu bionic/main amd64 libnl-route-3-200 amd64 3.2.29-0ubuntu3 [146 kB]
2020-04-11T04:15:20.0133569Z Get:60 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libibverbs1 amd64 17.1-1ubuntu0.2 [44.4 kB]
2020-04-11T04:15:20.0186354Z Get:62 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libnss3 amd64 2:3.35-2ubuntu2.7 [1135 kB]
2020-04-11T04:15:20.0342871Z Get:63 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 librados2 amd64 12.2.12-0ubuntu0.18.04.5 [2704 kB]
2020-04-11T04:15:20.1349292Z Get:64 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 librbd1 amd64 12.2.12-0ubuntu0.18.04.5 [922 kB]
2020-04-11T04:15:20.1479063Z Get:65 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 qemu-block-extra amd64 1:2.11+dfsg-1ubuntu7.23 [39.6 kB]
2020-04-11T04:15:20.1479063Z Get:65 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 qemu-block-extra amd64 1:2.11+dfsg-1ubuntu7.23 [39.6 kB]
2020-04-11T04:15:20.1486270Z Get:66 http://archive.ubuntu.com/ubuntu bionic/main amd64 acl amd64 2.2.52-3build1 [38.5 kB]
2020-04-11T04:15:20.1495093Z Get:67 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 qemu-system-common amd64 1:2.11+dfsg-1ubuntu7.23 [672 kB]
2020-04-11T04:15:20.2252697Z Get:68 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 openssl amd64 1.1.1-1ubuntu2.1~18.04.5 [613 kB]
2020-04-11T04:15:20.2331909Z Get:69 http://archive.ubuntu.com/ubuntu bionic/main amd64 ca-certificates all 20180409 [151 kB]
2020-04-11T04:15:20.2362465Z Get:70 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libmagic-mgc amd64 1:5.32-2ubuntu0.3 [184 kB]
2020-04-11T04:15:20.2384523Z Get:71 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libmagic1 amd64 1:5.32-2ubuntu0.3 [68.7 kB]
2020-04-11T04:15:20.2394825Z Get:72 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 file amd64 1:5.32-2ubuntu0.3 [22.1 kB]
2020-04-11T04:15:20.3237528Z Get:73 http://archive.ubuntu.com/ubuntu bionic/main amd64 libcap2-bin amd64 1:2.25-1.2 [20.6 kB]
2020-04-11T04:15:20.3266056Z Get:75 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libelf1 amd64 0.170-0.4ubuntu0.1 [44.8 kB]
2020-04-11T04:15:20.3276802Z Get:76 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libicu60 amd64 60.2-3ubuntu3.1 [8054 kB]
2020-04-11T04:15:20.5676952Z Get:77 http://archive.ubuntu.com/ubuntu bionic/main amd64 libmpdec2 amd64 2.4.2-1ubuntu1 [84.1 kB]
2020-04-11T04:15:20.5688650Z Get:78 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libpython3.6-minimal amd64 3.6.9-1~18.04 [533 kB]
---
2020-04-11T04:15:25.0478211Z Get:171 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libtinfo-dev amd64 6.1-1ubuntu1.18.04 [81.3 kB]
2020-04-11T04:15:25.1216463Z Get:172 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libncurses5-dev amd64 6.1-1ubuntu1.18.04 [174 kB]
2020-04-11T04:15:25.2220383Z Get:173 http://archive.ubuntu.com/ubuntu bionic/main amd64 libedit-dev amd64 3.1-20170329-1 [99.1 kB]
2020-04-11T04:15:25.2246953Z Get:174 http://archive.ubuntu.com/ubuntu bionic/main amd64 libflac8 amd64 1.3.2-1 [213 kB]
2020-04-11T04:15:25.2282454Z Get:175 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libgstreamer1.0-0 amd64 1.14.5-0ubuntu1~18.04.1 [865 kB]
2020-04-11T04:15:25.2394857Z Get:176 http://archive.ubuntu.com/ubuntu bionic/main amd64 liborc-0.4-0 amd64 1:0.4.28-1 [137 kB]
2020-04-11T04:15:25.2417392Z Get:177 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libgstreamer-plugins-base1.0-0 amd64 1.14.5-0ubuntu1~18.04.1 [688 kB]
2020-04-11T04:15:25.2518228Z Get:179 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libllvm7 amd64 1:7-3~ubuntu0.18.04.1 [15.9 MB]
2020-04-11T04:15:25.8425676Z Get:180 http://archive.ubuntu.com/ubuntu bionic/main amd64 libopus0 amd64 1.1.2-1ubuntu1 [159 kB]
2020-04-11T04:15:25.8445650Z Get:181 http://archive.ubuntu.com/ubuntu bionic/main amd64 libpixman-1-0 amd64 0.34.0-2 [229 kB]
2020-04-11T04:15:25.8481517Z Get:182 http://archive.ubuntu.com/ubuntu bionic/main amd64 libvorbis0a amd64 1.3.5-4.2 [86.4 kB]
---
2020-04-11T04:15:27.0865281Z Get:195 http://archive.ubuntu.com/ubuntu bionic-updates/universe amd64 llvm-7-tools amd64 1:7-3~ubuntu0.18.04.1 [223 kB]
2020-04-11T04:15:27.0893822Z Get:196 http://archive.ubuntu.com/ubuntu bionic/main amd64 pkg-config amd64 0.29.1-0ubuntu2 [45.0 kB]
2020-04-11T04:15:27.0900556Z Get:197 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 libbrlapi0.6 amd64 5.5-4ubuntu2.0.1 [21.3 kB]
2020-04-11T04:15:27.0905132Z Get:198 http://archive.ubuntu.com/ubuntu bionic/main amd64 libfdt1 amd64 1.4.5-3 [15.7 kB]
2020-04-11T04:15:27.0909836Z Get:199 http://archive.ubuntu.com/ubuntu bionic-updates/main amd64 librdmacm1 amd64 17.1-1ubuntu0.2 [56.1 kB]
2020-04-11T04:15:27.2898349Z Get:201 http://archive.ubuntu.com/ubuntu bionic/main amd64 zlib1g-dev amd64 1:1.2.11.dfsg-0ubuntu2 [176 kB]
2020-04-11T04:15:27.2922019Z Get:202 http://archive.ubuntu.com/ubuntu bionic/main amd64 libc-ares2 amd64 1.14.0-1 [37.1 kB]
2020-04-11T04:15:27.4935784Z Get:203 http://archive.ubuntu.com/ubuntu bionic/main amd64 libhttp-parser2.7.1 amd64 2.7.1-2 [20.6 kB]
2020-04-11T04:15:27.7159496Z Get:204 http://archive.ubuntu.com/ubuntu bionic-updates/universe amd64 nodejs amd64 8.10.0~dfsg-2ubuntu0.4 [4796 kB]
---
2020-04-11T04:15:31.0408924Z (Reading database ... 100%
2020-04-11T04:15:31.0409221Z (Reading database ... 7093 files and directories currently installed.)
2020-04-11T04:15:31.0415409Z Preparing to unpack .../00-python_2.7.15~rc1-1_amd64.deb ...
2020-04-11T04:15:31.0478401Z Unpacking python (2.7.15~rc1-1) ...
2020-04-11T04:15:31.0754618Z Selecting previously unselected package libcap2:amd64.
2020-04-11T04:15:31.0767419Z Preparing to unpack .../01-libcap2_1%3a2.25-1.2_amd64.deb ...
2020-04-11T04:15:31.0781597Z Unpacking libcap2:amd64 (1:2.25-1.2) ...
2020-04-11T04:15:31.0972675Z Preparing to unpack .../02-libglib2.0-0_2.56.4-0ubuntu0.18.04.6_amd64.deb ...
2020-04-11T04:15:31.0991317Z Unpacking libglib2.0-0:amd64 (2.56.4-0ubuntu0.18.04.6) ...
2020-04-11T04:15:31.2273769Z Selecting previously unselected package libkrb5support0:amd64.
2020-04-11T04:15:31.2286346Z Preparing to unpack .../03-libkrb5support0_1.16-2ubuntu0.1_amd64.deb ...
---
2020-04-11T04:15:31.9767580Z (Reading database ... 95%
2020-04-11T04:15:31.9767801Z (Reading database ... 100%
2020-04-11T04:15:31.9768093Z (Reading database ... 7336 files and directories currently installed.)
2020-04-11T04:15:31.9769031Z Preparing to unpack .../libstdc++6_8.4.0-1ubuntu1~18.04_amd64.deb ...
2020-04-11T04:15:31.9974788Z Unpacking libstdc++6:amd64 (8.4.0-1ubuntu1~18.04) over (8.3.0-26ubuntu1~18.04) ...
2020-04-11T04:15:32.0819085Z Setting up libstdc++6:amd64 (8.4.0-1ubuntu1~18.04) ...
2020-04-11T04:15:32.1080792Z Selecting previously unselected package libnl-3-200:amd64.
2020-04-11T04:15:32.1081408Z (Reading database ... 5%
2020-04-11T04:15:32.1081632Z (Reading database ... 10%
2020-04-11T04:15:32.1081841Z (Reading database ... 15%
2020-04-11T04:15:32.1082047Z (Reading database ... 20%
---
2020-04-11T04:15:32.1124386Z (Reading database ... 90%
2020-04-11T04:15:32.1128826Z (Reading database ... 95%
2020-04-11T04:15:32.1129890Z (Reading database ... 100%
2020-04-11T04:15:32.1130192Z (Reading database ... 7336 files and directories currently installed.)
2020-04-11T04:15:32.1135096Z Preparing to unpack .../000-libnl-3-200_3.2.29-0ubuntu3_amd64.deb ...
2020-04-11T04:15:32.1150646Z Unpacking libnl-3-200:amd64 (3.2.29-0ubuntu3) ...
2020-04-11T04:15:32.1359918Z Selecting previously unselected package libnl-route-3-200:amd64.
2020-04-11T04:15:32.1360793Z Preparing to unpack .../001-libnl-route-3-200_3.2.29-0ubuntu3_amd64.deb ...
2020-04-11T04:15:32.1372254Z Unpacking libnl-route-3-200:amd64 (3.2.29-0ubuntu3) ...
2020-04-11T04:15:32.1685759Z Selecting previously unselected package libibverbs1:amd64.
2020-04-11T04:15:32.1686624Z Preparing to unpack .../002-libibverbs1_17.1-1ubuntu0.2_amd64.deb ...
2020-04-11T04:15:32.1700424Z Unpacking libibverbs1:amd64 (17.1-1ubuntu0.2) ...
2020-04-11T04:15:32.1905589Z Preparing to unpack .../003-libnspr4_2%3a4.18-1ubuntu1_amd64.deb ...
2020-04-11T04:15:32.1922632Z Unpacking libnspr4:amd64 (2:4.18-1ubuntu1) ...
2020-04-11T04:15:32.2205870Z Selecting previously unselected package libnss3:amd64.
2020-04-11T04:15:32.2206769Z Preparing to unpack .../004-libnss3_2%3a3.35-2ubuntu2.7_amd64.deb ...
---
2020-04-11T04:15:33.2442762Z Unpacking libmagic1:amd64 (1:5.32-2ubuntu0.3) ...
2020-04-11T04:15:33.2710075Z Selecting previously unselected package file.
2020-04-11T04:15:33.2723983Z Preparing to unpack .../014-file_1%3a5.32-2ubuntu0.3_amd64.deb ...
2020-04-11T04:15:33.2740792Z Unpacking file (1:5.32-2ubuntu0.3) ...
2020-04-11T04:15:33.2915068Z Selecting previously unselected package libcap2-bin.
2020-04-11T04:15:33.2929246Z Preparing to unpack .../015-libcap2-bin_1%3a2.25-1.2_amd64.deb ...
2020-04-11T04:15:33.2954540Z Unpacking libcap2-bin (1:2.25-1.2) ...
2020-04-11T04:15:33.3140680Z Preparing to unpack .../016-libdbus-1-3_1.12.2-1ubuntu1.1_amd64.deb ...
2020-04-11T04:15:33.3161956Z Unpacking libdbus-1-3:amd64 (1.12.2-1ubuntu1.1) ...
2020-04-11T04:15:33.3516663Z Selecting previously unselected package libelf1:amd64.
2020-04-11T04:15:33.3528949Z Preparing to unpack .../017-libelf1_0.170-0.4ubuntu0.1_amd64.deb ...
---
2020-04-11T04:15:48.2822152Z Unpacking libedit-dev:amd64 (3.1-20170329-1) ...
2020-04-11T04:15:48.3125582Z Selecting previously unselected package libflac8:amd64.
2020-04-11T04:15:48.3148099Z Preparing to unpack .../116-libflac8_1.3.2-1_amd64.deb ...
2020-04-11T04:15:48.3163098Z Unpacking libflac8:amd64 (1.3.2-1) ...
2020-04-11T04:15:48.3561203Z Selecting previously unselected package libgstreamer1.0-0:amd64.
2020-04-11T04:15:48.3583047Z Preparing to unpack .../117-libgstreamer1.0-0_1.14.5-0ubuntu1~18.04.1_amd64.deb ...
2020-04-11T04:15:48.3608190Z Unpacking libgstreamer1.0-0:amd64 (1.14.5-0ubuntu1~18.04.1) ...
2020-04-11T04:15:48.4602728Z Selecting previously unselected package liborc-0.4-0:amd64.
2020-04-11T04:15:48.4624770Z Preparing to unpack .../118-liborc-0.4-0_1%3a0.4.28-1_amd64.deb ...
2020-04-11T04:15:48.4637631Z Unpacking liborc-0.4-0:amd64 (1:0.4.28-1) ...
2020-04-11T04:15:48.4965547Z Selecting previously unselected package libgstreamer-plugins-base1.0-0:amd64.
2020-04-11T04:15:48.4987826Z Preparing to unpack .../119-libgstreamer-plugins-base1.0-0_1.14.5-0ubuntu1~18.04.1_amd64.deb ...
2020-04-11T04:15:48.5003145Z Unpacking libgstreamer-plugins-base1.0-0:amd64 (1.14.5-0ubuntu1~18.04.1) ...
2020-04-11T04:15:48.5836694Z Preparing to unpack .../120-libjpeg8_8c-2ubuntu8_amd64.deb ...
2020-04-11T04:15:48.5864767Z Unpacking libjpeg8:amd64 (8c-2ubuntu8) ...
2020-04-11T04:15:48.6030737Z Selecting previously unselected package libllvm7:amd64.
2020-04-11T04:15:48.6052348Z Preparing to unpack .../121-libllvm7_1%3a7-3~ubuntu0.18.04.1_amd64.deb ...
---
2020-04-11T04:15:55.0463736Z Unpacking libbrlapi0.6:amd64 (5.5-4ubuntu2.0.1) ...
2020-04-11T04:15:55.0645850Z Selecting previously unselected package libfdt1:amd64.
2020-04-11T04:15:55.0668662Z Preparing to unpack .../140-libfdt1_1.4.5-3_amd64.deb ...
2020-04-11T04:15:55.0699368Z Unpacking libfdt1:amd64 (1.4.5-3) ...
2020-04-11T04:15:55.0930563Z Selecting previously unselected package librdmacm1:amd64.
2020-04-11T04:15:55.0954960Z Preparing to unpack .../141-librdmacm1_17.1-1ubuntu0.2_amd64.deb ...
2020-04-11T04:15:55.0966799Z Unpacking librdmacm1:amd64 (17.1-1ubuntu0.2) ...
2020-04-11T04:15:55.1191428Z Preparing to unpack .../142-qemu-system-arm_1%3a2.11+dfsg-1ubuntu7.23_amd64.deb ...
2020-04-11T04:15:55.1203876Z Unpacking qemu-system-arm (1:2.11+dfsg-1ubuntu7.23) ...
2020-04-11T04:15:55.8321425Z Selecting previously unselected package zlib1g-dev:amd64.
2020-04-11T04:15:55.8345915Z Preparing to unpack .../143-zlib1g-dev_1%3a1.2.11.dfsg-0ubuntu2_amd64.deb ...
---
2020-04-11T04:15:57.1024237Z Setting up openssl (1.1.1-1ubuntu2.1~18.04.5) ...
2020-04-11T04:15:57.1094748Z Setting up libsqlite3-0:amd64 (3.22.0-1ubuntu0.3) ...
2020-04-11T04:15:57.1125017Z Setting up libmpc3:amd64 (1.1.0-1) ...
2020-04-11T04:15:57.1158597Z Setting up libc-dev-bin (2.27-3ubuntu1) ...
2020-04-11T04:15:57.1187065Z Setting up liborc-0.4-0:amd64 (1:0.4.28-1) ...
2020-04-11T04:15:57.1261897Z Setting up libgdbm-compat4:amd64 (1.14.1-6) ...
2020-04-11T04:15:57.1295397Z Setting up libkeyutils1:amd64 (1.5.9-9.2ubuntu2) ...
2020-04-11T04:15:57.1295397Z Setting up libkeyutils1:amd64 (1.5.9-9.2ubuntu2) ...
2020-04-11T04:15:57.1336465Z Setting up libnl-3-200:amd64 (3.2.29-0ubuntu3) ...
2020-04-11T04:15:57.1498193Z Setting up ca-certificates (20180409) ...
2020-04-11T04:15:57.2285198Z debconf: unable to initialize frontend: Dialog
2020-04-11T04:15:57.2286417Z debconf: (TERM is not set, so the dialog frontend is not usable.)
2020-04-11T04:15:57.2287008Z debconf: falling back to frontend: Readline
---
2020-04-11T04:15:58.3609433Z update-alternatives: warning: skip creation of /usr/share/man/man1/js.1.gz because associated file /usr/share/man/man1/nodejs.1.gz (of link group js) doesn't exist
2020-04-11T04:15:58.3633821Z Setting up libbinutils:amd64 (2.30-21ubuntu1~18.04.2) ...
2020-04-11T04:15:58.3667118Z Setting up libarchive13:amd64 (3.2.2-3.1ubuntu0.6) ...
2020-04-11T04:15:58.3712790Z Setting up libcilkrts5:amd64 (7.5.0-3ubuntu1~18.04) ...
2020-04-11T04:15:58.3752724Z Setting up libcap2-bin (1:2.25-1.2) ...
2020-04-11T04:15:59.2260605Z Setting up libasn1-8-heimdal:amd64 (7.5.0+dfsg-1) ...
2020-04-11T04:15:59.2261401Z Setting up libubsan0:amd64 (7.5.0-3ubuntu1~18.04) ...
2020-04-11T04:15:59.2262138Z Setting up libssl-dev:amd64 (1.1.1-1ubuntu2.1~18.04.5) ...
2020-04-11T04:15:59.2262690Z Setting up libc6-dev-armel-cross (2.27-3ubuntu1cross1.1) ...
2020-04-11T04:15:59.2262690Z Setting up libc6-dev-armel-cross (2.27-3ubuntu1cross1.1) ...
2020-04-11T04:15:59.2263206Z Setting up libhcrypto4-heimdal:amd64 (7.5.0+dfsg-1) ...
2020-04-11T04:15:59.2263690Z Setting up libflac8:amd64 (1.3.2-1) ...
2020-04-11T04:15:59.2264107Z Setting up python2.7 (2.7.17-1~18.04) ...
2020-04-11T04:15:59.2264578Z Setting up libnss3:amd64 (2:3.35-2ubuntu2.7) ...
2020-04-11T04:15:59.2265076Z Setting up libnl-route-3-200:amd64 (3.2.29-0ubuntu3) ...
2020-04-11T04:15:59.2266006Z Setting up libhx509-5-heimdal:amd64 (7.5.0+dfsg-1) ...
2020-04-11T04:15:59.2266518Z Setting up libstdc++6-armel-cross (8.4.0-1ubuntu1~18.04cross2) ...
2020-04-11T04:15:59.2267024Z Setting up libgcc-7-dev:amd64 (7.5.0-3ubuntu1~18.04) ...
2020-04-11T04:15:59.2267490Z Setting up cpp-7 (7.5.0-3ubuntu1~18.04) ...
---
2020-04-11T04:15:59.2526371Z Setting up libheimntlm0-heimdal:amd64 (7.5.0+dfsg-1) ...
2020-04-11T04:15:59.2589809Z Setting up python (2.7.15~rc1-1) ...
2020-04-11T04:15:59.2710928Z Setting up libbabeltrace1:amd64 (1.5.5-1) ...
2020-04-11T04:15:59.2739123Z Setting up binutils-x86-64-linux-gnu (2.30-21ubuntu1~18.04.2) ...
2020-04-11T04:15:59.2774881Z Setting up libibverbs1:amd64 (17.1-1ubuntu0.2) ...
2020-04-11T04:15:59.3297119Z Setting up libpython3.6-stdlib:amd64 (3.6.9-1~18.04) ...
2020-04-11T04:15:59.3347517Z Setting up cpp-arm-linux-gnueabi (4:7.4.0-1ubuntu2.3) ...
2020-04-11T04:15:59.3347517Z Setting up cpp-arm-linux-gnueabi (4:7.4.0-1ubuntu2.3) ...
2020-04-11T04:15:59.3399583Z Setting up libgstreamer1.0-0:amd64 (1.14.5-0ubuntu1~18.04.1) ...
2020-04-11T04:15:59.3447377Z Setcap worked! gst-ptp-helper is not suid!
2020-04-11T04:15:59.3469139Z Setting up libubsan0-armel-cross (7.5.0-3ubuntu1~18.04cross1) ...
2020-04-11T04:15:59.3509233Z Setting up libcilkrts5-armel-cross (7.5.0-3ubuntu1~18.04cross1) ...
2020-04-11T04:15:59.3543957Z Setting up libgcc-7-dev-armel-cross (7.5.0-3ubuntu1~18.04cross1) ...
2020-04-11T04:15:59.3594881Z Setting up gcc-7-arm-linux-gnueabi (7.5.0-3ubuntu1~18.04cross1) ...
2020-04-11T04:15:59.3635702Z Setting up libx11-6:amd64 (2:1.6.4-3ubuntu0.2) ...
2020-04-11T04:15:59.3701742Z Setting up librdmacm1:amd64 (17.1-1ubuntu0.2) ...
2020-04-11T04:15:59.3777843Z Setting up llvm-7-dev (1:7-3~ubuntu0.18.04.1) ...
2020-04-11T04:15:59.3821659Z Setting up librados2 (12.2.12-0ubuntu0.18.04.5) ...
2020-04-11T04:15:59.3854916Z Setting up libgssapi-krb5-2:amd64 (1.16-2ubuntu0.1) ...
2020-04-11T04:15:59.3968281Z Setting up perl (5.26.1-6ubuntu0.3) ...
2020-04-11T04:15:59.3968281Z Setting up perl (5.26.1-6ubuntu0.3) ...
2020-04-11T04:15:59.4133102Z Setting up libsndfile1:amd64 (1.0.28-4ubuntu0.18.04.1) ...
2020-04-11T04:15:59.4165324Z Setting up llvm-7-tools (1:7-3~ubuntu0.18.04.1) ...
2020-04-11T04:15:59.4206493Z Setting up gcc-arm-linux-gnueabi (4:7.4.0-1ubuntu2.3) ...
2020-04-11T04:15:59.4235925Z Setting up binutils (2.30-21ubuntu1~18.04.2) ...
2020-04-11T04:15:59.4270602Z Setting up libgssapi3-heimdal:amd64 (7.5.0+dfsg-1) ...
2020-04-11T04:15:59.4303092Z Setting up libstdc++-7-dev-armel-cross (7.5.0-3ubuntu1~18.04cross1) ...
2020-04-11T04:15:59.4334416Z Setting up libpython3.6:amd64 (3.6.9-1~18.04) ...
2020-04-11T04:15:59.4363434Z Setting up g++-7-arm-linux-gnueabi (7.5.0-3ubuntu1~18.04cross1) ...
2020-04-11T04:15:59.4393176Z Setting up libgstreamer-plugins-base1.0-0:amd64 (1.14.5-0ubuntu1~18.04.1) ...
2020-04-11T04:15:59.4458372Z Setting up libxext6:amd64 (2:1.3.3-1) ...
2020-04-11T04:15:59.4536996Z Setting up gcc-7 (7.5.0-3ubuntu1~18.04) ...
2020-04-11T04:15:59.4581107Z Setting up liberror-perl (0.17025-1) ...
2020-04-11T04:15:59.4612841Z Setting up g++-7 (7.5.0-3ubuntu1~18.04) ...
---
2020-04-11T04:16:25.0482004Z Successfully built 5dde7edd5870
2020-04-11T04:16:25.0624133Z Successfully tagged rust-ci:latest
2020-04-11T04:16:25.0870979Z Built container sha256:5dde7edd587069d31239ad48bb16ec3a5d3edcc1d956807cc92c10bb0a582be7
2020-04-11T04:16:25.0887862Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/9e251dd9ee17aab6234ce501db7345b7b70945767f0116a229472dddc1f334fd49a6dadd39d2697970bf5abff266e4995b06f373f0130e897a495d008a7482f1
2020-04-11T04:17:20.7547726Z upload failed: - to s3://rust-lang-ci-sccache2/docker/9e251dd9ee17aab6234ce501db7345b7b70945767f0116a229472dddc1f334fd49a6dadd39d2697970bf5abff266e4995b06f373f0130e897a495d008a7482f1 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-11T04:17:21.3001196Z [CI_JOB_NAME=x86_64-gnu-llvm-7]
2020-04-11T04:17:21.3026921Z == clock drift check ==
2020-04-11T04:17:21.3035851Z   local time: Sat Apr 11 04:17:21 UTC 2020
2020-04-11T04:17:21.3035851Z   local time: Sat Apr 11 04:17:21 UTC 2020
2020-04-11T04:17:21.6196320Z   network time: Sat, 11 Apr 2020 04:17:21 GMT
2020-04-11T04:17:21.6224886Z Starting sccache server...
2020-04-11T04:17:21.7002258Z configure: processing command line
2020-04-11T04:17:21.7004353Z configure: 
2020-04-11T04:17:21.7010906Z configure: rust.dist-src        := False
---
2020-04-11T04:22:08.5420745Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T04:22:09.8925583Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T04:22:11.3625265Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T04:22:12.2703330Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T04:22:20.3842534Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T04:22:22.5900865Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T04:22:26.6437186Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T04:22:30.3869196Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T04:22:38.9216307Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T04:43:00.4933978Z    Compiling rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T04:43:02.1118904Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T04:43:03.9727885Z    Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-11T04:43:05.0500062Z    Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T04:43:15.4540970Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T04:43:17.6492150Z    Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T04:43:22.6893852Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-11T04:43:27.7924216Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T04:43:38.7939718Z    Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-11T05:07:51.6423524Z .................................................................................................... 1600/9884
2020-04-11T05:07:57.6223824Z .................................................................................................... 1700/9884
2020-04-11T05:08:01.7265798Z .................................................................................................... 1800/9884
2020-04-11T05:08:10.0733266Z .................................................................................................... 1900/9884
2020-04-11T05:08:17.8145518Z ..i................................................................................................. 2000/9884
2020-04-11T05:08:23.9464830Z ............................................................................................iiiii... 2100/9884
2020-04-11T05:08:43.9861939Z .................................................................................................... 2300/9884
2020-04-11T05:08:46.1554747Z .................................................................................................... 2400/9884
2020-04-11T05:08:48.3811998Z .................................................................................................... 2500/9884
2020-04-11T05:08:54.0307460Z .................................................................................................... 2600/9884
---
2020-04-11T05:11:41.4491413Z ..................................................................i...............i................. 5000/9884
2020-04-11T05:11:48.5323096Z .................................................................................................... 5100/9884
2020-04-11T05:11:55.7446404Z .................................................................................................... 5200/9884
2020-04-11T05:12:00.6254442Z ...........i........................................................................................ 5300/9884
2020-04-11T05:12:10.2441929Z i................................................................................................... 5400/9884
2020-04-11T05:12:14.8510755Z ii.ii........i...i.................................................................................. 5500/9884
2020-04-11T05:12:22.4662879Z .............................................i...................................................... 5700/9884
2020-04-11T05:12:32.2794429Z .................................................................ii................................. 5800/9884
2020-04-11T05:12:38.8350811Z ....i............................................................................................... 5900/9884
2020-04-11T05:12:44.1502276Z .................................................................................................... 6000/9884
2020-04-11T05:12:44.1502276Z .................................................................................................... 6000/9884
2020-04-11T05:12:53.5790610Z ..................................................................................................ii 6100/9884
2020-04-11T05:13:04.6015700Z ...i..ii...........i................................................................................ 6200/9884
2020-04-11T05:13:18.5488248Z .................................................................................................... 6400/9884
2020-04-11T05:13:21.7634510Z .................................................................................................... 6500/9884
2020-04-11T05:13:21.7634510Z .................................................................................................... 6500/9884
2020-04-11T05:13:33.6844448Z ............................i..ii................................................................... 6600/9884
2020-04-11T05:13:53.4682997Z .................................................................................................... 6800/9884
2020-04-11T05:13:55.4224700Z ............................i....................................................................... 6900/9884
2020-04-11T05:13:57.4159218Z .................................................................................................... 7000/9884
2020-04-11T05:13:59.5389913Z ...................................................................i................................ 7100/9884
---
2020-04-11T05:15:32.1639684Z .................................................................................................... 7800/9884
2020-04-11T05:15:35.9765346Z .................................................................................................... 7900/9884
2020-04-11T05:15:42.3503373Z .................................................................................................... 8000/9884
2020-04-11T05:15:48.9715679Z ................................i................................................................... 8100/9884
2020-04-11T05:15:57.0455211Z ................................................................................iiiiii.iiiii.i...... 8200/9884
2020-04-11T05:16:11.8866890Z ..........................i......i.................................................................. 8400/9884
2020-04-11T05:16:15.5399178Z .................................................................................................... 8500/9884
2020-04-11T05:16:26.1268285Z .................................................................................................... 8600/9884
2020-04-11T05:16:37.9585336Z .................................................................................................... 8700/9884
---
2020-04-11T05:18:54.4478224Z Suite("src/test/codegen") not skipped for "bootstrap::test::Codegen" -- not in ["src/tools/tidy"]
2020-04-11T05:18:54.4666344Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T05:18:54.6675592Z 
2020-04-11T05:18:54.6676399Z running 185 tests
2020-04-11T05:18:57.2783066Z iiii......i............ii.i..iiii....i....i...........i............i..i..................i....i..... 100/185
2020-04-11T05:18:59.7279035Z .......i.i.i...iii..iiiiiiiiiiiiiiii.......................iii...............ii......
2020-04-11T05:18:59.7282985Z 
2020-04-11T05:18:59.7290161Z  finished in 5.262
2020-04-11T05:18:59.7294367Z Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
2020-04-11T05:18:59.7478881Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T05:19:01.7798009Z Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
2020-04-11T05:19:01.7972393Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T05:19:01.9436434Z 
2020-04-11T05:19:01.9437205Z running 9 tests
2020-04-11T05:19:01.9441724Z iiiiiiiii
2020-04-11T05:19:01.9443006Z 
2020-04-11T05:19:01.9443492Z  finished in 0.146
2020-04-11T05:19:01.9447100Z Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
2020-04-11T05:19:01.9650760Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T05:19:22.1397403Z Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
2020-04-11T05:19:22.1628061Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T05:19:22.3438426Z 
2020-04-11T05:19:22.3439349Z running 115 tests
2020-04-11T05:19:35.1889389Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii.........i.....i..i.......ii.i.ii.. 100/115
2020-04-11T05:19:36.8972672Z ...iiii.....ii.
2020-04-11T05:19:36.8977940Z 
2020-04-11T05:19:36.8978761Z  finished in 14.735
2020-04-11T05:19:36.8985680Z Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
2020-04-11T05:19:36.8992345Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
2020-04-11T05:30:59.8986700Z 
2020-04-11T05:30:59.8987968Z    Doc-tests core
2020-04-11T05:31:04.0198826Z 
2020-04-11T05:31:04.0200883Z running 2490 tests
2020-04-11T05:31:12.2386654Z ......iiiii......................................................................................... 100/2490
2020-04-11T05:31:20.3927866Z .....................................................................................ii............. 200/2490
2020-04-11T05:31:39.1037024Z ....................i............................................................................... 400/2490
2020-04-11T05:31:39.1037024Z ....................i............................................................................... 400/2490
2020-04-11T05:31:47.9979855Z ..........................................................................i..i..................iiii 500/2490
2020-04-11T05:32:02.7324196Z .................................................................................................... 700/2490
2020-04-11T05:32:10.5189583Z .................................................................................................... 800/2490
2020-04-11T05:32:18.1623694Z .................................................................................................... 900/2490
2020-04-11T05:32:25.7752763Z .................................................................................................... 1000/2490
---
2020-04-11T05:35:42.5565134Z 
2020-04-11T05:35:42.5566108Z running 1019 tests
2020-04-11T05:35:58.4117874Z i................................................................................................... 100/1019
2020-04-11T05:36:07.8119658Z .................................................................................................... 200/1019
2020-04-11T05:36:14.7450328Z ..................iii......i......i...i......i...................................................... 300/1019
2020-04-11T05:36:25.4242813Z ...................................................i....i......................................ii... 500/1019
2020-04-11T05:36:32.5068085Z .................................................................................................... 600/1019
2020-04-11T05:36:37.0664159Z .................................................................................................... 700/1019
2020-04-11T05:36:37.0664159Z .................................................................................................... 700/1019
2020-04-11T05:36:43.5490910Z .............................................iiii................................................... 800/1019
2020-04-11T05:36:56.0052309Z .................................................................................................... 900/1019
2020-04-11T05:37:01.4298050Z ...................................................................iiii............................. 1000/1019
2020-04-11T05:37:02.5680526Z test result: ok. 999 passed; 0 failed; 20 ignored; 0 measured; 0 filtered out
2020-04-11T05:37:02.5681117Z 
2020-04-11T05:37:02.5774437Z  finished in 151.879
2020-04-11T05:37:02.5780815Z Set({"src/libterm"}) not skipped for "bootstrap::test::Crate" -- not in ["src/tools/tidy"]
---
2020-04-11T05:39:55.0577741Z 
2020-04-11T05:39:55.0578005Z test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
2020-04-11T05:39:55.0578244Z 
2020-04-11T05:39:55.0636284Z  finished in 0.864
2020-04-11T05:39:55.0641954Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::test::CrateLibrustc" -- not in ["src/tools/tidy"]
2020-04-11T05:39:55.0657722Z Testing rustc_query_system stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T05:39:55.2327773Z    Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T05:39:56.1057169Z      Running build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/rustc_query_system-4ffc613fefb54d82
2020-04-11T05:39:56.1083941Z 
2020-04-11T05:39:56.1084553Z running 0 tests
2020-04-11T05:39:56.1084828Z 
---
2020-04-11T05:53:04.6276963Z /checkout/src/librustc_parse"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6278353Z Set({"/checkout/src/librustc_passes"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6279766Z Set({"/checkout/src/librustc_plugin_impl"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6281193Z Set({"/checkout/src/librustc_privacy"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6282614Z Set({"/checkout/src/librustc_query_system"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6286011Z Set({"/checkout/src/librustc_save_analysis"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6287486Z Set({"/checkout/src/librustc_session"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6289207Z Set({"/checkout/src/librustc_span"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
2020-04-11T05:53:04.6290685Z Set({"/checkout/src/librustc_symbol_mangling"}) not skipped for "bootstrap::doc::Rustc" -- not in ["src/tools/tidy"]
---
2020-04-11T05:54:02.5014877Z Suite("src/test/run-make-fulldeps") not skipped for "bootstrap::test::RunMakeFullDeps" -- not in ["src/tools/tidy"]
2020-04-11T05:54:02.5352180Z Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T05:54:02.7392417Z 
2020-04-11T05:54:02.7392772Z running 210 tests
2020-04-11T05:54:31.9539254Z ......................i...ii.......................................................................i 100/210
2020-04-11T05:55:06.7457742Z ........................................iiiiii......i..............iii.............................. 200/210
2020-04-11T05:55:10.4768052Z test result: ok. 195 passed; 0 failed; 15 ignored; 0 measured; 0 filtered out
2020-04-11T05:55:10.4768314Z 
2020-04-11T05:55:10.4775710Z  finished in 67.942
2020-04-11T05:55:10.4781495Z Set({"src/doc/rustdoc"}) not skipped for "bootstrap::test::RustdocBook" -- not in ["src/tools/tidy"]
---
2020-04-11T05:56:29.5568654Z Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
2020-04-11T05:56:29.5755792Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-11T05:56:29.7219379Z 
2020-04-11T05:56:29.7221594Z running 13 tests
2020-04-11T05:56:30.1242061Z .iiiiiii.iii.
2020-04-11T05:56:30.1243457Z 
2020-04-11T05:56:30.1243637Z  finished in 0.548
2020-04-11T05:56:30.1299073Z Build completed successfully in 1:37:39
2020-04-11T05:56:30.1310183Z + python2.7 ../x.py test src/test/mir-opt --target=armv5te-unknown-linux-gnueabi
---
2020-04-11T05:57:51.1250321Z 
2020-04-11T05:57:51.1250868Z ------------------------------------------
2020-04-11T05:57:51.1251357Z stderr:
2020-04-11T05:57:51.1252026Z ------------------------------------------
2020-04-11T05:57:51.1253306Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/array-index-is-temporary/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/array-index-is-temporary/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1254659Z ------------------------------------------
2020-04-11T05:57:51.1259095Z 
2020-04-11T05:57:51.1260659Z 
2020-04-11T05:57:51.1264172Z ---- [mir-opt] mir-opt/address-of.rs stdout ----
---
2020-04-11T05:57:51.1267755Z 
2020-04-11T05:57:51.1268295Z ------------------------------------------
2020-04-11T05:57:51.1274700Z stderr:
2020-04-11T05:57:51.1275304Z ------------------------------------------
2020-04-11T05:57:51.1276142Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/address-of/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/address-of/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1276911Z ------------------------------------------
2020-04-11T05:57:51.1277087Z 
2020-04-11T05:57:51.1277179Z 
2020-04-11T05:57:51.1277554Z ---- [mir-opt] mir-opt/basic_assignment.rs stdout ----
---
2020-04-11T05:57:51.1279594Z 
2020-04-11T05:57:51.1279934Z ------------------------------------------
2020-04-11T05:57:51.1280211Z stderr:
2020-04-11T05:57:51.1280572Z ------------------------------------------
2020-04-11T05:57:51.1281399Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/basic_assignment/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/basic_assignment/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1282295Z ------------------------------------------
2020-04-11T05:57:51.1282439Z 
2020-04-11T05:57:51.1282520Z 
2020-04-11T05:57:51.1282848Z ---- [mir-opt] mir-opt/box_expr.rs stdout ----
---
2020-04-11T05:57:51.1286029Z 
2020-04-11T05:57:51.1286335Z ------------------------------------------
2020-04-11T05:57:51.1286501Z stderr:
2020-04-11T05:57:51.1286807Z ------------------------------------------
2020-04-11T05:57:51.1287519Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/box_expr/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1288174Z ------------------------------------------
2020-04-11T05:57:51.1288331Z 
2020-04-11T05:57:51.1288411Z 
2020-04-11T05:57:51.1288938Z ---- [mir-opt] mir-opt/byte_slice.rs stdout ----
---
2020-04-11T05:57:51.1290753Z 
2020-04-11T05:57:51.1291094Z ------------------------------------------
2020-04-11T05:57:51.1291299Z stderr:
2020-04-11T05:57:51.1291648Z ------------------------------------------
2020-04-11T05:57:51.1292449Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/byte_slice/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/byte_slice/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1293223Z ------------------------------------------
2020-04-11T05:57:51.1293386Z 
2020-04-11T05:57:51.1293478Z 
2020-04-11T05:57:51.1293862Z ---- [mir-opt] mir-opt/combine_array_len.rs stdout ----
---
2020-04-11T05:57:51.1295696Z 
2020-04-11T05:57:51.1296053Z ------------------------------------------
2020-04-11T05:57:51.1296243Z stderr:
2020-04-11T05:57:51.1296587Z ------------------------------------------
2020-04-11T05:57:51.1297432Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/combine_array_len/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/combine_array_len/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1298204Z ------------------------------------------
2020-04-11T05:57:51.1298382Z 
2020-04-11T05:57:51.1298473Z 
2020-04-11T05:57:51.1298872Z ---- [mir-opt] mir-opt/const-promotion-extern-static.rs stdout ----
---
2020-04-11T05:57:51.1300943Z 
2020-04-11T05:57:51.1301289Z ------------------------------------------
2020-04-11T05:57:51.1301495Z stderr:
2020-04-11T05:57:51.1301842Z ------------------------------------------
2020-04-11T05:57:51.1302963Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const-promotion-extern-static/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const-promotion-extern-static/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1303861Z ------------------------------------------
2020-04-11T05:57:51.1304037Z 
2020-04-11T05:57:51.1304135Z 
2020-04-11T05:57:51.1304543Z ---- [mir-opt] mir-opt/const_allocation.rs stdout ----
---
2020-04-11T05:57:51.1306526Z 
2020-04-11T05:57:51.1306887Z ------------------------------------------
2020-04-11T05:57:51.1307091Z stderr:
2020-04-11T05:57:51.1307460Z ------------------------------------------
2020-04-11T05:57:51.1308365Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1309199Z ------------------------------------------
2020-04-11T05:57:51.1309389Z 
2020-04-11T05:57:51.1309489Z 
2020-04-11T05:57:51.1309888Z ---- [mir-opt] mir-opt/const_allocation2.rs stdout ----
---
2020-04-11T05:57:51.1311884Z 
2020-04-11T05:57:51.1312244Z ------------------------------------------
2020-04-11T05:57:51.1312467Z stderr:
2020-04-11T05:57:51.1312837Z ------------------------------------------
2020-04-11T05:57:51.1313725Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation2/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1314576Z ------------------------------------------
2020-04-11T05:57:51.1314752Z 
2020-04-11T05:57:51.1314851Z 
2020-04-11T05:57:51.1315267Z ---- [mir-opt] mir-opt/const_allocation3.rs stdout ----
---
2020-04-11T05:57:51.1318223Z 
2020-04-11T05:57:51.1318756Z ------------------------------------------
2020-04-11T05:57:51.1318957Z stderr:
2020-04-11T05:57:51.1319716Z ------------------------------------------
2020-04-11T05:57:51.1320732Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation3/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_allocation3/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1321511Z ------------------------------------------
2020-04-11T05:57:51.1321694Z 
2020-04-11T05:57:51.1321786Z 
2020-04-11T05:57:51.1322162Z ---- [mir-opt] mir-opt/const_prop/aggregate.rs stdout ----
---
2020-04-11T05:57:51.1324372Z 
2020-04-11T05:57:51.1324706Z ------------------------------------------
2020-04-11T05:57:51.1324915Z stderr:
2020-04-11T05:57:51.1325260Z ------------------------------------------
2020-04-11T05:57:51.1326104Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/aggregate/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/aggregate/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1326911Z ------------------------------------------
2020-04-11T05:57:51.1327075Z 
2020-04-11T05:57:51.1327168Z 
2020-04-11T05:57:51.1327567Z ---- [mir-opt] mir-opt/const_prop/array_index.rs stdout ----
---
2020-04-11T05:57:51.1329453Z 
2020-04-11T05:57:51.1329789Z ------------------------------------------
2020-04-11T05:57:51.1329979Z stderr:
2020-04-11T05:57:51.1330320Z ------------------------------------------
2020-04-11T05:57:51.1331267Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/array_index/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/array_index/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1332129Z ------------------------------------------
2020-04-11T05:57:51.1332310Z 
2020-04-11T05:57:51.1332403Z 
2020-04-11T05:57:51.1332772Z ---- [mir-opt] mir-opt/const_prop/boxes.rs stdout ----
---
2020-04-11T05:57:51.1334620Z 
2020-04-11T05:57:51.1334952Z ------------------------------------------
2020-04-11T05:57:51.1335161Z stderr:
2020-04-11T05:57:51.1335509Z ------------------------------------------
2020-04-11T05:57:51.1352623Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/boxes/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/boxes/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1353678Z ------------------------------------------
2020-04-11T05:57:51.1353843Z 
2020-04-11T05:57:51.1353937Z 
2020-04-11T05:57:51.1354331Z ---- [mir-opt] mir-opt/const_prop/cast.rs stdout ----
---
2020-04-11T05:57:51.1356207Z 
2020-04-11T05:57:51.1356546Z ------------------------------------------
2020-04-11T05:57:51.1356738Z stderr:
2020-04-11T05:57:51.1357086Z ------------------------------------------
2020-04-11T05:57:51.1357933Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/cast/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/cast/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1358919Z ------------------------------------------
2020-04-11T05:57:51.1359111Z 
2020-04-11T05:57:51.1359211Z 
2020-04-11T05:57:51.1359629Z ---- [mir-opt] mir-opt/const_prop/checked_add.rs stdout ----
---
2020-04-11T05:57:51.1362148Z 
2020-04-11T05:57:51.1362511Z ------------------------------------------
2020-04-11T05:57:51.1362735Z stderr:
2020-04-11T05:57:51.1363110Z ------------------------------------------
2020-04-11T05:57:51.1364818Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/checked_add/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/checked_add/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1365700Z ------------------------------------------
2020-04-11T05:57:51.1365875Z 
2020-04-11T05:57:51.1365981Z 
2020-04-11T05:57:51.1366453Z ---- [mir-opt] mir-opt/const_prop/const_prop_fails_gracefully.rs stdout ----
---
2020-04-11T05:57:51.1368580Z 
2020-04-11T05:57:51.1368943Z ------------------------------------------
2020-04-11T05:57:51.1369148Z stderr:
2020-04-11T05:57:51.1369568Z ------------------------------------------
2020-04-11T05:57:51.1370691Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/const_prop_fails_gracefully/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/const_prop_fails_gracefully/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1371554Z ------------------------------------------
2020-04-11T05:57:51.1371922Z 
2020-04-11T05:57:51.1372016Z 
2020-04-11T05:57:51.1372452Z ---- [mir-opt] mir-opt/const_prop/control-flow-simplification.rs stdout ----
---
2020-04-11T05:57:51.1374452Z 
2020-04-11T05:57:51.1374803Z ------------------------------------------
2020-04-11T05:57:51.1374994Z stderr:
2020-04-11T05:57:51.1375342Z ------------------------------------------
2020-04-11T05:57:51.1376272Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/control-flow-simplification/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/control-flow-simplification/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1377144Z ------------------------------------------
2020-04-11T05:57:51.1377305Z 
2020-04-11T05:57:51.1377396Z 
2020-04-11T05:57:51.1377798Z ---- [mir-opt] mir-opt/const_prop/discriminant.rs stdout ----
---
2020-04-11T05:57:51.1379665Z 
2020-04-11T05:57:51.1380000Z ------------------------------------------
2020-04-11T05:57:51.1380189Z stderr:
2020-04-11T05:57:51.1380548Z ------------------------------------------
2020-04-11T05:57:51.1381404Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/discriminant/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/discriminant/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1382358Z ------------------------------------------
2020-04-11T05:57:51.1382521Z 
2020-04-11T05:57:51.1382612Z 
2020-04-11T05:57:51.1382986Z ---- [mir-opt] mir-opt/const_prop/indirect.rs stdout ----
---
2020-04-11T05:57:51.1385971Z 
2020-04-11T05:57:51.1386369Z ------------------------------------------
2020-04-11T05:57:51.1386576Z stderr:
2020-04-11T05:57:51.1386949Z ------------------------------------------
2020-04-11T05:57:51.1387884Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/indirect/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/indirect/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1389004Z ------------------------------------------
2020-04-11T05:57:51.1389180Z 
2020-04-11T05:57:51.1389280Z 
2020-04-11T05:57:51.1389712Z ---- [mir-opt] mir-opt/const_prop/issue-66971.rs stdout ----
---
2020-04-11T05:57:51.1391730Z 
2020-04-11T05:57:51.1392093Z ------------------------------------------
2020-04-11T05:57:51.1392299Z stderr:
2020-04-11T05:57:51.1392682Z ------------------------------------------
2020-04-11T05:57:51.1393607Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/issue-66971/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/issue-66971/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1394485Z ------------------------------------------
2020-04-11T05:57:51.1395977Z 
2020-04-11T05:57:51.1396083Z 
2020-04-11T05:57:51.1396547Z ---- [mir-opt] mir-opt/const_prop/issue-67019.rs stdout ----
---
2020-04-11T05:57:51.1398800Z 
2020-04-11T05:57:51.1399172Z ------------------------------------------
2020-04-11T05:57:51.1399377Z stderr:
2020-04-11T05:57:51.1399750Z ------------------------------------------
2020-04-11T05:57:51.1400709Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/issue-67019/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/issue-67019/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1401577Z ------------------------------------------
2020-04-11T05:57:51.1401754Z 
2020-04-11T05:57:51.1401868Z 
2020-04-11T05:57:51.1402315Z ---- [mir-opt] mir-opt/const_prop/optimizes_into_variable.rs stdout ----
---
2020-04-11T05:57:51.1404823Z 
2020-04-11T05:57:51.1405184Z ------------------------------------------
2020-04-11T05:57:51.1405527Z stderr:
2020-04-11T05:57:51.1405925Z ------------------------------------------
2020-04-11T05:57:51.1406974Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/optimizes_into_variable/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/optimizes_into_variable/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1407903Z ------------------------------------------
2020-04-11T05:57:51.1408076Z 
2020-04-11T05:57:51.1408175Z 
2020-04-11T05:57:51.1408613Z ---- [mir-opt] mir-opt/const_prop/read_immutable_static.rs stdout ----
---
2020-04-11T05:57:51.1410693Z 
2020-04-11T05:57:51.1411071Z ------------------------------------------
2020-04-11T05:57:51.1411278Z stderr:
2020-04-11T05:57:51.1411655Z ------------------------------------------
2020-04-11T05:57:51.1412641Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/read_immutable_static/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/read_immutable_static/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1413532Z ------------------------------------------
2020-04-11T05:57:51.1413709Z 
2020-04-11T05:57:51.1413822Z 
2020-04-11T05:57:51.1414228Z ---- [mir-opt] mir-opt/const_prop/ref_deref.rs stdout ----
---
2020-04-11T05:57:51.1416234Z 
2020-04-11T05:57:51.1416597Z ------------------------------------------
2020-04-11T05:57:51.1416822Z stderr:
2020-04-11T05:57:51.1417195Z ------------------------------------------
2020-04-11T05:57:51.1418102Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/ref_deref/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/ref_deref/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1418966Z ------------------------------------------
2020-04-11T05:57:51.1419142Z 
2020-04-11T05:57:51.1419242Z 
2020-04-11T05:57:51.1419668Z ---- [mir-opt] mir-opt/const_prop/ref_deref_project.rs stdout ----
---
2020-04-11T05:57:51.1421726Z 
2020-04-11T05:57:51.1422106Z ------------------------------------------
2020-04-11T05:57:51.1422309Z stderr:
2020-04-11T05:57:51.1422681Z ------------------------------------------
2020-04-11T05:57:51.1423644Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/ref_deref_project/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/ref_deref_project/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1424514Z ------------------------------------------
2020-04-11T05:57:51.1424689Z 
2020-04-11T05:57:51.1424802Z 
2020-04-11T05:57:51.1425214Z ---- [mir-opt] mir-opt/const_prop/reify_fn_ptr.rs stdout ----
---
2020-04-11T05:57:51.1427338Z 
2020-04-11T05:57:51.1427706Z ------------------------------------------
2020-04-11T05:57:51.1427929Z stderr:
2020-04-11T05:57:51.1428298Z ------------------------------------------
2020-04-11T05:57:51.1429225Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/reify_fn_ptr/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/reify_fn_ptr/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1430095Z ------------------------------------------
2020-04-11T05:57:51.1430271Z 
2020-04-11T05:57:51.1430370Z 
2020-04-11T05:57:51.1430775Z ---- [mir-opt] mir-opt/const_prop/repeat.rs stdout ----
---
2020-04-11T05:57:51.1432741Z 
2020-04-11T05:57:51.1433209Z ------------------------------------------
2020-04-11T05:57:51.1433398Z stderr:
2020-04-11T05:57:51.1433742Z ------------------------------------------
2020-04-11T05:57:51.1434588Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/repeat/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/repeat/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1435359Z ------------------------------------------
2020-04-11T05:57:51.1435533Z 
2020-04-11T05:57:51.1435625Z 
2020-04-11T05:57:51.1436006Z ---- [mir-opt] mir-opt/const_prop/return_place.rs stdout ----
---
2020-04-11T05:57:51.1437889Z 
2020-04-11T05:57:51.1438221Z ------------------------------------------
2020-04-11T05:57:51.1438427Z stderr:
2020-04-11T05:57:51.1438770Z ------------------------------------------
2020-04-11T05:57:51.1439625Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/return_place/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/return_place/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1440430Z ------------------------------------------
2020-04-11T05:57:51.1440591Z 
2020-04-11T05:57:51.1440683Z 
2020-04-11T05:57:51.1441071Z ---- [mir-opt] mir-opt/const_prop/slice_len.rs stdout ----
---
2020-04-11T05:57:51.1442914Z 
2020-04-11T05:57:51.1443422Z ------------------------------------------
2020-04-11T05:57:51.1443616Z stderr:
2020-04-11T05:57:51.1443972Z ------------------------------------------
2020-04-11T05:57:51.1444830Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/slice_len/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/slice_len/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1447208Z ------------------------------------------
2020-04-11T05:57:51.1448237Z 
2020-04-11T05:57:51.1448332Z 
2020-04-11T05:57:51.1449026Z ---- [mir-opt] mir-opt/copy_propagation.rs stdout ----
---
2020-04-11T05:57:51.1452118Z 
2020-04-11T05:57:51.1452481Z ------------------------------------------
2020-04-11T05:57:51.1452688Z stderr:
2020-04-11T05:57:51.1453035Z ------------------------------------------
2020-04-11T05:57:51.1454321Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy_propagation/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy_propagation/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1455139Z ------------------------------------------
2020-04-11T05:57:51.1455303Z 
2020-04-11T05:57:51.1455396Z 
2020-04-11T05:57:51.1455793Z ---- [mir-opt] mir-opt/const_prop/switch_int.rs stdout ----
---
2020-04-11T05:57:51.1457947Z 
2020-04-11T05:57:51.1458310Z ------------------------------------------
2020-04-11T05:57:51.1458518Z stderr:
2020-04-11T05:57:51.1458890Z ------------------------------------------
2020-04-11T05:57:51.1459826Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/switch_int/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/const_prop/switch_int/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1460685Z ------------------------------------------
2020-04-11T05:57:51.1460873Z 
2020-04-11T05:57:51.1460972Z 
2020-04-11T05:57:51.1461369Z ---- [mir-opt] mir-opt/deaggregator_test.rs stdout ----
---
2020-04-11T05:57:51.1463358Z 
2020-04-11T05:57:51.1463719Z ------------------------------------------
2020-04-11T05:57:51.1463937Z stderr:
2020-04-11T05:57:51.1464308Z ------------------------------------------
2020-04-11T05:57:51.1465196Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1466050Z ------------------------------------------
2020-04-11T05:57:51.1466228Z 
2020-04-11T05:57:51.1466328Z 
2020-04-11T05:57:51.1466753Z ---- [mir-opt] mir-opt/copy_propagation_arg.rs stdout ----
---
2020-04-11T05:57:51.1468755Z 
2020-04-11T05:57:51.1469116Z ------------------------------------------
2020-04-11T05:57:51.1469320Z stderr:
2020-04-11T05:57:51.1469687Z ------------------------------------------
2020-04-11T05:57:51.1470612Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy_propagation_arg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/copy_propagation_arg/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1471460Z ------------------------------------------
2020-04-11T05:57:51.1471649Z 
2020-04-11T05:57:51.1471853Z 
2020-04-11T05:57:51.1472316Z ---- [mir-opt] mir-opt/deaggregator_test_enum_2.rs stdout ----
---
2020-04-11T05:57:51.1474414Z 
2020-04-11T05:57:51.1474774Z ------------------------------------------
2020-04-11T05:57:51.1474992Z stderr:
2020-04-11T05:57:51.1475362Z ------------------------------------------
2020-04-11T05:57:51.1476291Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test_enum_2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test_enum_2/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1477171Z ------------------------------------------
2020-04-11T05:57:51.1477349Z 
2020-04-11T05:57:51.1477448Z 
2020-04-11T05:57:51.1477878Z ---- [mir-opt] mir-opt/deaggregator_test_enum.rs stdout ----
---
2020-04-11T05:57:51.1479889Z 
2020-04-11T05:57:51.1480250Z ------------------------------------------
2020-04-11T05:57:51.1480454Z stderr:
2020-04-11T05:57:51.1480821Z ------------------------------------------
2020-04-11T05:57:51.1481752Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test_enum/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test_enum/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1482611Z ------------------------------------------
2020-04-11T05:57:51.1482804Z 
2020-04-11T05:57:51.1482903Z 
2020-04-11T05:57:51.1483556Z ---- [mir-opt] mir-opt/deaggregator_test_multiple.rs stdout ----
---
2020-04-11T05:57:51.1485599Z 
2020-04-11T05:57:51.1485955Z ------------------------------------------
2020-04-11T05:57:51.1486173Z stderr:
2020-04-11T05:57:51.1486544Z ------------------------------------------
2020-04-11T05:57:51.1487482Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test_multiple/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/deaggregator_test_multiple/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1488372Z ------------------------------------------
2020-04-11T05:57:51.1488555Z 
2020-04-11T05:57:51.1488655Z 
2020-04-11T05:57:51.1489061Z ---- [mir-opt] mir-opt/exponential-or.rs stdout ----
---
2020-04-11T05:57:51.1491029Z 
2020-04-11T05:57:51.1491391Z ------------------------------------------
2020-04-11T05:57:51.1491595Z stderr:
2020-04-11T05:57:51.1491978Z ------------------------------------------
2020-04-11T05:57:51.1492857Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/exponential-or/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/exponential-or/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1493811Z ------------------------------------------
2020-04-11T05:57:51.1494007Z 
2020-04-11T05:57:51.1494104Z 
2020-04-11T05:57:51.1494525Z ---- [mir-opt] mir-opt/generator-drop-cleanup.rs stdout ----
---
2020-04-11T05:57:51.1496621Z 
2020-04-11T05:57:51.1496955Z ------------------------------------------
2020-04-11T05:57:51.1497159Z stderr:
2020-04-11T05:57:51.1497505Z ------------------------------------------
2020-04-11T05:57:51.1498359Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-drop-cleanup/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-drop-cleanup/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1499180Z ------------------------------------------
2020-04-11T05:57:51.1499343Z 
2020-04-11T05:57:51.1499436Z 
2020-04-11T05:57:51.1500050Z ---- [mir-opt] mir-opt/generator-storage-dead-unwind.rs stdout ----
---
2020-04-11T05:57:51.1502097Z 
2020-04-11T05:57:51.1502459Z ------------------------------------------
2020-04-11T05:57:51.1502665Z stderr:
2020-04-11T05:57:51.1503048Z ------------------------------------------
2020-04-11T05:57:51.1504013Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-storage-dead-unwind/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-storage-dead-unwind/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1504896Z ------------------------------------------
2020-04-11T05:57:51.1505086Z 
2020-04-11T05:57:51.1505184Z 
2020-04-11T05:57:51.1505682Z ---- [mir-opt] mir-opt/generator-tiny.rs stdout ----
---
2020-04-11T05:57:51.1507502Z 
2020-04-11T05:57:51.1507834Z ------------------------------------------
2020-04-11T05:57:51.1508037Z stderr:
2020-04-11T05:57:51.1508380Z ------------------------------------------
2020-04-11T05:57:51.1509207Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1509985Z ------------------------------------------
2020-04-11T05:57:51.1510147Z 
2020-04-11T05:57:51.1510238Z 
2020-04-11T05:57:51.1510595Z ---- [mir-opt] mir-opt/graphviz.rs stdout ----
---
2020-04-11T05:57:51.1512376Z 
2020-04-11T05:57:51.1512707Z ------------------------------------------
2020-04-11T05:57:51.1513069Z stderr:
2020-04-11T05:57:51.1513451Z ------------------------------------------
2020-04-11T05:57:51.1514434Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/graphviz/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/graphviz/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1515259Z ------------------------------------------
2020-04-11T05:57:51.1515451Z 
2020-04-11T05:57:51.1515550Z 
2020-04-11T05:57:51.1516084Z ---- [mir-opt] mir-opt/inline/inline-any-operand.rs stdout ----
---
2020-04-11T05:57:51.1518227Z 
2020-04-11T05:57:51.1518585Z ------------------------------------------
2020-04-11T05:57:51.1518802Z stderr:
2020-04-11T05:57:51.1519173Z ------------------------------------------
2020-04-11T05:57:51.1520121Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-any-operand/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-any-operand/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1520998Z ------------------------------------------
2020-04-11T05:57:51.1521175Z 
2020-04-11T05:57:51.1521276Z 
2020-04-11T05:57:51.1521728Z ---- [mir-opt] mir-opt/inline/inline-closure-borrows-arg.rs stdout ----
---
2020-04-11T05:57:51.1523973Z 
2020-04-11T05:57:51.1524330Z ------------------------------------------
2020-04-11T05:57:51.1524543Z stderr:
2020-04-11T05:57:51.1524928Z ------------------------------------------
2020-04-11T05:57:51.1525914Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-closure-borrows-arg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-closure-borrows-arg/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1526819Z ------------------------------------------
2020-04-11T05:57:51.1526994Z 
2020-04-11T05:57:51.1527094Z 
2020-04-11T05:57:51.1527506Z ---- [mir-opt] mir-opt/inline/inline-closure.rs stdout ----
---
2020-04-11T05:57:51.1529507Z 
2020-04-11T05:57:51.1530046Z ------------------------------------------
2020-04-11T05:57:51.1530249Z stderr:
2020-04-11T05:57:51.1530599Z ------------------------------------------
2020-04-11T05:57:51.1531442Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-closure/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-closure/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1532248Z ------------------------------------------
2020-04-11T05:57:51.1532411Z 
2020-04-11T05:57:51.1532504Z 
2020-04-11T05:57:51.1532918Z ---- [mir-opt] mir-opt/inline/inline-closure-captures.rs stdout ----
---
2020-04-11T05:57:51.1534887Z 
2020-04-11T05:57:51.1535231Z ------------------------------------------
2020-04-11T05:57:51.1535422Z stderr:
2020-04-11T05:57:51.1535834Z ------------------------------------------
2020-04-11T05:57:51.1536740Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-closure-captures/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-closure-captures/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1537566Z ------------------------------------------
2020-04-11T05:57:51.1537727Z 
2020-04-11T05:57:51.1537820Z 
2020-04-11T05:57:51.1538215Z ---- [mir-opt] mir-opt/inline/inline-into-box-place.rs stdout ----
---
2020-04-11T05:57:51.1540121Z 
2020-04-11T05:57:51.1540470Z ------------------------------------------
2020-04-11T05:57:51.1540661Z stderr:
2020-04-11T05:57:51.1541007Z ------------------------------------------
2020-04-11T05:57:51.1542745Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-into-box-place/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-into-box-place/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1543642Z ------------------------------------------
2020-04-11T05:57:51.1543819Z 
2020-04-11T05:57:51.1543932Z 
2020-04-11T05:57:51.1544338Z ---- [mir-opt] mir-opt/inline/inline-retag.rs stdout ----
---
2020-04-11T05:57:51.1546350Z 
2020-04-11T05:57:51.1546714Z ------------------------------------------
2020-04-11T05:57:51.1546920Z stderr:
2020-04-11T05:57:51.1547305Z ------------------------------------------
2020-04-11T05:57:51.1548208Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-retag/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-retag/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1549069Z ------------------------------------------
2020-04-11T05:57:51.1549243Z 
2020-04-11T05:57:51.1549342Z 
2020-04-11T05:57:51.1549766Z ---- [mir-opt] mir-opt/inline/inline-specialization.rs stdout ----
---
2020-04-11T05:57:51.1551817Z 
2020-04-11T05:57:51.1552191Z ------------------------------------------
2020-04-11T05:57:51.1552398Z stderr:
2020-04-11T05:57:51.1552769Z ------------------------------------------
2020-04-11T05:57:51.1553733Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-specialization/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-specialization/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1554610Z ------------------------------------------
2020-04-11T05:57:51.1554906Z 
2020-04-11T05:57:51.1555025Z 
2020-04-11T05:57:51.1555458Z ---- [mir-opt] mir-opt/inline/inline-trait-method.rs stdout ----
---
2020-04-11T05:57:51.1557654Z 
2020-04-11T05:57:51.1558014Z ------------------------------------------
2020-04-11T05:57:51.1558217Z stderr:
2020-04-11T05:57:51.1558698Z ------------------------------------------
2020-04-11T05:57:51.1559572Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-trait-method/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-trait-method/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1560391Z ------------------------------------------
2020-04-11T05:57:51.1560552Z 
2020-04-11T05:57:51.1560644Z 
2020-04-11T05:57:51.1561038Z ---- [mir-opt] mir-opt/inline/inline-trait-method_2.rs stdout ----
---
2020-04-11T05:57:51.1563121Z 
2020-04-11T05:57:51.1563736Z ------------------------------------------
2020-04-11T05:57:51.1563928Z stderr:
2020-04-11T05:57:51.1564275Z ------------------------------------------
2020-04-11T05:57:51.1565163Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-trait-method_2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-trait-method_2/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1565971Z ------------------------------------------
2020-04-11T05:57:51.1566135Z 
2020-04-11T05:57:51.1566242Z 
2020-04-11T05:57:51.1566601Z ---- [mir-opt] mir-opt/issue-38669.rs stdout ----
---
2020-04-11T05:57:51.1568425Z 
2020-04-11T05:57:51.1568946Z ------------------------------------------
2020-04-11T05:57:51.1569165Z stderr:
2020-04-11T05:57:51.1569536Z ------------------------------------------
2020-04-11T05:57:51.1570410Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-38669/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-38669/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1571273Z ------------------------------------------
2020-04-11T05:57:51.1571449Z 
2020-04-11T05:57:51.1571549Z 
2020-04-11T05:57:51.1571930Z ---- [mir-opt] mir-opt/issue-41110.rs stdout ----
---
2020-04-11T05:57:51.1573888Z 
2020-04-11T05:57:51.1574261Z ------------------------------------------
2020-04-11T05:57:51.1574465Z stderr:
2020-04-11T05:57:51.1574834Z ------------------------------------------
2020-04-11T05:57:51.1575712Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-41110/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-41110/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1576535Z ------------------------------------------
2020-04-11T05:57:51.1576709Z 
2020-04-11T05:57:51.1576823Z 
2020-04-11T05:57:51.1577208Z ---- [mir-opt] mir-opt/issue-41888.rs stdout ----
---
2020-04-11T05:57:51.1579280Z 
2020-04-11T05:57:51.1579643Z ------------------------------------------
2020-04-11T05:57:51.1579852Z stderr:
2020-04-11T05:57:51.1580237Z ------------------------------------------
2020-04-11T05:57:51.1581099Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-41888/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-41888/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1581927Z ------------------------------------------
2020-04-11T05:57:51.1582105Z 
2020-04-11T05:57:51.1582204Z 
2020-04-11T05:57:51.1582587Z ---- [mir-opt] mir-opt/issue-49232.rs stdout ----
---
2020-04-11T05:57:51.1584544Z 
2020-04-11T05:57:51.1584916Z ------------------------------------------
2020-04-11T05:57:51.1585122Z stderr:
2020-04-11T05:57:51.1585489Z ------------------------------------------
2020-04-11T05:57:51.1586361Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-49232/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-49232/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1587177Z ------------------------------------------
2020-04-11T05:57:51.1587351Z 
2020-04-11T05:57:51.1587466Z 
2020-04-11T05:57:51.1587863Z ---- [mir-opt] mir-opt/issue-41697.rs stdout ----
---
2020-04-11T05:57:51.1589847Z 
2020-04-11T05:57:51.1590180Z ------------------------------------------
2020-04-11T05:57:51.1590371Z stderr:
2020-04-11T05:57:51.1590727Z ------------------------------------------
2020-04-11T05:57:51.1591526Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-41697/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-41697/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1592296Z ------------------------------------------
2020-04-11T05:57:51.1592457Z 
2020-04-11T05:57:51.1592547Z 
2020-04-11T05:57:51.1592907Z ---- [mir-opt] mir-opt/issue-62289.rs stdout ----
---
2020-04-11T05:57:51.1594720Z 
2020-04-11T05:57:51.1595064Z ------------------------------------------
2020-04-11T05:57:51.1595255Z stderr:
2020-04-11T05:57:51.1595596Z ------------------------------------------
2020-04-11T05:57:51.1596407Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-62289/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/issue-62289/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1597167Z ------------------------------------------
2020-04-11T05:57:51.1597329Z 
2020-04-11T05:57:51.1597434Z 
2020-04-11T05:57:51.1597841Z ---- [mir-opt] mir-opt/loop_test.rs stdout ----
---
2020-04-11T05:57:51.1599685Z 
2020-04-11T05:57:51.1600018Z ------------------------------------------
2020-04-11T05:57:51.1600207Z stderr:
2020-04-11T05:57:51.1600560Z ------------------------------------------
2020-04-11T05:57:51.1601343Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/loop_test/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/loop_test/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1602100Z ------------------------------------------
2020-04-11T05:57:51.1602260Z 
2020-04-11T05:57:51.1602351Z 
2020-04-11T05:57:51.1602725Z ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
---
2020-04-11T05:57:51.1605015Z 
2020-04-11T05:57:51.1605389Z ------------------------------------------
2020-04-11T05:57:51.1605596Z stderr:
2020-04-11T05:57:51.1605966Z ------------------------------------------
2020-04-11T05:57:51.1606875Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_false_edges/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_false_edges/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1607715Z ------------------------------------------
2020-04-11T05:57:51.1608009Z 
2020-04-11T05:57:51.1608120Z 
2020-04-11T05:57:51.1608739Z ---- [mir-opt] mir-opt/match-arm-scopes.rs stdout ----
---
2020-04-11T05:57:51.1610691Z 
2020-04-11T05:57:51.1611052Z ------------------------------------------
2020-04-11T05:57:51.1611259Z stderr:
2020-04-11T05:57:51.1611642Z ------------------------------------------
2020-04-11T05:57:51.1612532Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match-arm-scopes/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1613386Z ------------------------------------------
2020-04-11T05:57:51.1613567Z 
2020-04-11T05:57:51.1613667Z 
2020-04-11T05:57:51.1614045Z ---- [mir-opt] mir-opt/match_test.rs stdout ----
---
2020-04-11T05:57:51.1616092Z 
2020-04-11T05:57:51.1616438Z ------------------------------------------
2020-04-11T05:57:51.1616631Z stderr:
2020-04-11T05:57:51.1616973Z ------------------------------------------
2020-04-11T05:57:51.1617783Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_test/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/match_test/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1618543Z ------------------------------------------
2020-04-11T05:57:51.1618768Z 
2020-04-11T05:57:51.1618872Z 
2020-04-11T05:57:51.1619273Z ---- [mir-opt] mir-opt/nll/named-lifetimes-basic.rs stdout ----
---
2020-04-11T05:57:51.1621218Z 
2020-04-11T05:57:51.1621552Z ------------------------------------------
2020-04-11T05:57:51.1621742Z stderr:
2020-04-11T05:57:51.1622101Z ------------------------------------------
2020-04-11T05:57:51.1622966Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/named-lifetimes-basic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/named-lifetimes-basic/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1623783Z ------------------------------------------
2020-04-11T05:57:51.1623950Z 
2020-04-11T05:57:51.1624041Z 
2020-04-11T05:57:51.1624441Z ---- [mir-opt] mir-opt/nll/region-subtyping-basic.rs stdout ----
---
2020-04-11T05:57:51.1626333Z 
2020-04-11T05:57:51.1626679Z ------------------------------------------
2020-04-11T05:57:51.1626867Z stderr:
2020-04-11T05:57:51.1627212Z ------------------------------------------
2020-04-11T05:57:51.1628096Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/region-subtyping-basic/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/nll/region-subtyping-basic/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1628904Z ------------------------------------------
2020-04-11T05:57:51.1629065Z 
2020-04-11T05:57:51.1629175Z 
2020-04-11T05:57:51.1629568Z ---- [mir-opt] mir-opt/no-drop-for-inactive-variant.rs stdout ----
---
2020-04-11T05:57:51.1631638Z 
2020-04-11T05:57:51.1631997Z ------------------------------------------
2020-04-11T05:57:51.1632204Z stderr:
2020-04-11T05:57:51.1632588Z ------------------------------------------
2020-04-11T05:57:51.1633535Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/no-drop-for-inactive-variant/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/no-drop-for-inactive-variant/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1634431Z ------------------------------------------
2020-04-11T05:57:51.1634607Z 
2020-04-11T05:57:51.1634705Z 
2020-04-11T05:57:51.1635130Z ---- [mir-opt] mir-opt/no-spurious-drop-after-call.rs stdout ----
---
2020-04-11T05:57:51.1637171Z 
2020-04-11T05:57:51.1637544Z ------------------------------------------
2020-04-11T05:57:51.1637747Z stderr:
2020-04-11T05:57:51.1638115Z ------------------------------------------
2020-04-11T05:57:51.1639071Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/no-spurious-drop-after-call/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/no-spurious-drop-after-call/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1640041Z ------------------------------------------
2020-04-11T05:57:51.1640217Z 
2020-04-11T05:57:51.1640331Z 
2020-04-11T05:57:51.1640752Z ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
---
2020-04-11T05:57:51.1642784Z 
2020-04-11T05:57:51.1643145Z ------------------------------------------
2020-04-11T05:57:51.1643458Z stderr:
2020-04-11T05:57:51.1643830Z ------------------------------------------
2020-04-11T05:57:51.1644743Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/packed-struct-drop-aligned/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/packed-struct-drop-aligned/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1645627Z ------------------------------------------
2020-04-11T05:57:51.1645801Z 
2020-04-11T05:57:51.1645900Z 
2020-04-11T05:57:51.1646304Z ---- [mir-opt] mir-opt/remove_fake_borrows.rs stdout ----
---
2020-04-11T05:57:51.1648301Z 
2020-04-11T05:57:51.1648676Z ------------------------------------------
2020-04-11T05:57:51.1648881Z stderr:
2020-04-11T05:57:51.1649253Z ------------------------------------------
2020-04-11T05:57:51.1650186Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/remove_fake_borrows/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/remove_fake_borrows/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1651034Z ------------------------------------------
2020-04-11T05:57:51.1651224Z 
2020-04-11T05:57:51.1651322Z 
2020-04-11T05:57:51.1651685Z ---- [mir-opt] mir-opt/retag.rs stdout ----
---
2020-04-11T05:57:51.1653596Z 
2020-04-11T05:57:51.1653959Z ------------------------------------------
2020-04-11T05:57:51.1654178Z stderr:
2020-04-11T05:57:51.1654548Z ------------------------------------------
2020-04-11T05:57:51.1655392Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retag/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retag/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1656204Z ------------------------------------------
2020-04-11T05:57:51.1656380Z 
2020-04-11T05:57:51.1656480Z 
2020-04-11T05:57:51.1656880Z ---- [mir-opt] mir-opt/retain-never-const.rs stdout ----
---
2020-04-11T05:57:51.1658902Z 
2020-04-11T05:57:51.1659249Z ------------------------------------------
2020-04-11T05:57:51.1659440Z stderr:
2020-04-11T05:57:51.1659783Z ------------------------------------------
2020-04-11T05:57:51.1660776Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retain-never-const/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/retain-never-const/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1661576Z ------------------------------------------
2020-04-11T05:57:51.1661737Z 
2020-04-11T05:57:51.1661843Z 
2020-04-11T05:57:51.1662203Z ---- [mir-opt] mir-opt/return_an_array.rs stdout ----
---
2020-04-11T05:57:51.1664022Z 
2020-04-11T05:57:51.1664354Z ------------------------------------------
2020-04-11T05:57:51.1664560Z stderr:
2020-04-11T05:57:51.1664910Z ------------------------------------------
2020-04-11T05:57:51.1665739Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/return_an_array/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/return_an_array/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1666523Z ------------------------------------------
2020-04-11T05:57:51.1666689Z 
2020-04-11T05:57:51.1666782Z 
2020-04-11T05:57:51.1667136Z ---- [mir-opt] mir-opt/simple-match.rs stdout ----
---
2020-04-11T05:57:51.1668952Z 
2020-04-11T05:57:51.1669299Z ------------------------------------------
2020-04-11T05:57:51.1669523Z stderr:
2020-04-11T05:57:51.1669876Z ------------------------------------------
2020-04-11T05:57:51.1670704Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simple-match/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simple-match/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1671471Z ------------------------------------------
2020-04-11T05:57:51.1671636Z 
2020-04-11T05:57:51.1671742Z 
2020-04-11T05:57:51.1672118Z ---- [mir-opt] mir-opt/simplify-arm-identity.rs stdout ----
---
2020-04-11T05:57:51.1673976Z 
2020-04-11T05:57:51.1674310Z ------------------------------------------
2020-04-11T05:57:51.1674520Z stderr:
2020-04-11T05:57:51.1674865Z ------------------------------------------
2020-04-11T05:57:51.1675718Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify-arm-identity/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify-arm-identity/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1676520Z ------------------------------------------
2020-04-11T05:57:51.1676684Z 
2020-04-11T05:57:51.1676775Z 
2020-04-11T05:57:51.1677191Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-consts.rs stdout ----
---
2020-04-11T05:57:51.1679208Z 
2020-04-11T05:57:51.1679564Z ------------------------------------------
2020-04-11T05:57:51.1679754Z stderr:
2020-04-11T05:57:51.1680143Z ------------------------------------------
2020-04-11T05:57:51.1681265Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify-locals-removes-unused-consts/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify-locals-removes-unused-consts/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1682167Z ------------------------------------------
2020-04-11T05:57:51.1682358Z 
2020-04-11T05:57:51.1682458Z 
2020-04-11T05:57:51.1683060Z ---- [mir-opt] mir-opt/simplify-locals-removes-unused-discriminant-reads.rs stdout ----
---
2020-04-11T05:57:51.1685430Z 
2020-04-11T05:57:51.1685790Z ------------------------------------------
2020-04-11T05:57:51.1686000Z stderr:
2020-04-11T05:57:51.1686363Z ------------------------------------------
2020-04-11T05:57:51.1687490Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify-locals-removes-unused-discriminant-reads/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify-locals-removes-unused-discriminant-reads/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1688349Z ------------------------------------------
2020-04-11T05:57:51.1688504Z 
2020-04-11T05:57:51.1688593Z 
2020-04-11T05:57:51.1688952Z ---- [mir-opt] mir-opt/simplify_cfg.rs stdout ----
---
2020-04-11T05:57:51.1690763Z 
2020-04-11T05:57:51.1691095Z ------------------------------------------
2020-04-11T05:57:51.1691285Z stderr:
2020-04-11T05:57:51.1691627Z ------------------------------------------
2020-04-11T05:57:51.1692623Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_cfg/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1693443Z ------------------------------------------
2020-04-11T05:57:51.1693632Z 
2020-04-11T05:57:51.1693732Z 
2020-04-11T05:57:51.1694114Z ---- [mir-opt] mir-opt/simplify_if.rs stdout ----
---
2020-04-11T05:57:51.1696113Z 
2020-04-11T05:57:51.1696446Z ------------------------------------------
2020-04-11T05:57:51.1696648Z stderr:
2020-04-11T05:57:51.1696990Z ------------------------------------------
2020-04-11T05:57:51.1698335Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_if/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_if/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1699409Z ------------------------------------------
2020-04-11T05:57:51.1699587Z 
2020-04-11T05:57:51.1699686Z 
2020-04-11T05:57:51.1700089Z ---- [mir-opt] mir-opt/simplify_match.rs stdout ----
---
2020-04-11T05:57:51.1702214Z 
2020-04-11T05:57:51.1702584Z ------------------------------------------
2020-04-11T05:57:51.1702791Z stderr:
2020-04-11T05:57:51.1703161Z ------------------------------------------
2020-04-11T05:57:51.1704054Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_match/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_match/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1704877Z ------------------------------------------
2020-04-11T05:57:51.1705064Z 
2020-04-11T05:57:51.1705162Z 
2020-04-11T05:57:51.1705546Z ---- [mir-opt] mir-opt/simplify_try.rs stdout ----
---
2020-04-11T05:57:51.1707506Z 
2020-04-11T05:57:51.1707865Z ------------------------------------------
2020-04-11T05:57:51.1708089Z stderr:
2020-04-11T05:57:51.1708458Z ------------------------------------------
2020-04-11T05:57:51.1709325Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_try/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/simplify_try/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1710174Z ------------------------------------------
2020-04-11T05:57:51.1710463Z 
2020-04-11T05:57:51.1710544Z 
2020-04-11T05:57:51.1710872Z ---- [mir-opt] mir-opt/slice-drop-shim.rs stdout ----
---
2020-04-11T05:57:51.1712480Z 
2020-04-11T05:57:51.1712772Z ------------------------------------------
2020-04-11T05:57:51.1712937Z stderr:
2020-04-11T05:57:51.1713235Z ------------------------------------------
2020-04-11T05:57:51.1713964Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/slice-drop-shim/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/slice-drop-shim/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1714633Z ------------------------------------------
2020-04-11T05:57:51.1714785Z 
2020-04-11T05:57:51.1714867Z 
2020-04-11T05:57:51.1715180Z ---- [mir-opt] mir-opt/storage_ranges.rs stdout ----
---
2020-04-11T05:57:51.1716778Z 
2020-04-11T05:57:51.1717069Z ------------------------------------------
2020-04-11T05:57:51.1717249Z stderr:
2020-04-11T05:57:51.1717547Z ------------------------------------------
2020-04-11T05:57:51.1718253Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/storage_ranges/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/storage_ranges/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1719195Z ------------------------------------------
2020-04-11T05:57:51.1719357Z 
2020-04-11T05:57:51.1719448Z 
2020-04-11T05:57:51.1719841Z ---- [mir-opt] mir-opt/uniform_array_move_out.rs stdout ----
---
2020-04-11T05:57:51.1721818Z 
2020-04-11T05:57:51.1722154Z ------------------------------------------
2020-04-11T05:57:51.1722347Z stderr:
2020-04-11T05:57:51.1722692Z ------------------------------------------
2020-04-11T05:57:51.1723660Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/uniform_array_move_out/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/uniform_array_move_out/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1724460Z ------------------------------------------
2020-04-11T05:57:51.1724633Z 
2020-04-11T05:57:51.1724725Z 
2020-04-11T05:57:51.1725121Z ---- [mir-opt] mir-opt/storage_live_dead_in_statics.rs stdout ----
---
2020-04-11T05:57:51.1727290Z 
2020-04-11T05:57:51.1727651Z ------------------------------------------
2020-04-11T05:57:51.1727871Z stderr:
2020-04-11T05:57:51.1728242Z ------------------------------------------
2020-04-11T05:57:51.1729192Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/storage_live_dead_in_statics/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/storage_live_dead_in_statics/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1730078Z ------------------------------------------
2020-04-11T05:57:51.1730254Z 
2020-04-11T05:57:51.1730355Z 
2020-04-11T05:57:51.1730764Z ---- [mir-opt] mir-opt/uninhabited-enum.rs stdout ----
---
2020-04-11T05:57:51.1732749Z 
2020-04-11T05:57:51.1733109Z ------------------------------------------
2020-04-11T05:57:51.1733313Z stderr:
2020-04-11T05:57:51.1733683Z ------------------------------------------
2020-04-11T05:57:51.1734586Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/uninhabited-enum/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/uninhabited-enum/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1735417Z ------------------------------------------
2020-04-11T05:57:51.1735607Z 
2020-04-11T05:57:51.1735706Z 
2020-04-11T05:57:51.1736130Z ---- [mir-opt] mir-opt/uninhabited_enum_branching.rs stdout ----
---
2020-04-11T05:57:51.1738166Z 
2020-04-11T05:57:51.1738706Z ------------------------------------------
2020-04-11T05:57:51.1738910Z stderr:
2020-04-11T05:57:51.1739255Z ------------------------------------------
2020-04-11T05:57:51.1740126Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/uninhabited_enum_branching/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/uninhabited_enum_branching/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1740951Z ------------------------------------------
2020-04-11T05:57:51.1741172Z 
2020-04-11T05:57:51.1741264Z 
2020-04-11T05:57:51.1741640Z ---- [mir-opt] mir-opt/unreachable.rs stdout ----
---
2020-04-11T05:57:51.1743514Z 
2020-04-11T05:57:51.1743848Z ------------------------------------------
2020-04-11T05:57:51.1744038Z stderr:
2020-04-11T05:57:51.1744394Z ------------------------------------------
2020-04-11T05:57:51.1745282Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1745938Z ------------------------------------------
2020-04-11T05:57:51.1746097Z 
2020-04-11T05:57:51.1746177Z 
2020-04-11T05:57:51.1746494Z ---- [mir-opt] mir-opt/unreachable_asm.rs stdout ----
---
2020-04-11T05:57:51.1748091Z 
2020-04-11T05:57:51.1748381Z ------------------------------------------
2020-04-11T05:57:51.1748560Z stderr:
2020-04-11T05:57:51.1748858Z ------------------------------------------
2020-04-11T05:57:51.1749567Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable_asm/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable_asm/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1750250Z ------------------------------------------
2020-04-11T05:57:51.1750395Z 
2020-04-11T05:57:51.1750475Z 
2020-04-11T05:57:51.1750814Z ---- [mir-opt] mir-opt/unreachable_asm_2.rs stdout ----
---
2020-04-11T05:57:51.1752411Z 
2020-04-11T05:57:51.1752703Z ------------------------------------------
2020-04-11T05:57:51.1752869Z stderr:
2020-04-11T05:57:51.1753180Z ------------------------------------------
2020-04-11T05:57:51.1753905Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable_asm_2/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable_asm_2/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1754582Z ------------------------------------------
2020-04-11T05:57:51.1754738Z 
2020-04-11T05:57:51.1754818Z 
2020-04-11T05:57:51.1755150Z ---- [mir-opt] mir-opt/unreachable_diverging.rs stdout ----
---
2020-04-11T05:57:51.1756767Z 
2020-04-11T05:57:51.1757056Z ------------------------------------------
2020-04-11T05:57:51.1757236Z stderr:
2020-04-11T05:57:51.1757534Z ------------------------------------------
2020-04-11T05:57:51.1758269Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable_diverging/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unreachable_diverging/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1759023Z ------------------------------------------
2020-04-11T05:57:51.1759165Z 
2020-04-11T05:57:51.1759281Z 
2020-04-11T05:57:51.1759616Z ---- [mir-opt] mir-opt/while-storage.rs stdout ----
---
2020-04-11T05:57:51.1761192Z 
2020-04-11T05:57:51.1761482Z ------------------------------------------
2020-04-11T05:57:51.1761645Z stderr:
2020-04-11T05:57:51.1761955Z ------------------------------------------
2020-04-11T05:57:51.1762662Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/while-storage/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/while-storage/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1763508Z ------------------------------------------
2020-04-11T05:57:51.1763666Z 
2020-04-11T05:57:51.1763752Z 
2020-04-11T05:57:51.1764079Z ---- [mir-opt] mir-opt/unusual-item-types.rs stdout ----
---
2020-04-11T05:57:51.1765687Z 
2020-04-11T05:57:51.1765978Z ------------------------------------------
2020-04-11T05:57:51.1766158Z stderr:
2020-04-11T05:57:51.1766455Z ------------------------------------------
2020-04-11T05:57:51.1767178Z /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unusual-item-types/a: 1: /checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/unusual-item-types/a: Syntax error: word unexpected (expecting ")")
2020-04-11T05:57:51.1768112Z ------------------------------------------
2020-04-11T05:57:51.1768308Z 
2020-04-11T05:57:51.1768402Z 
2020-04-11T05:57:51.1768505Z 
---
2020-04-11T05:57:51.1809362Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-04-11T05:57:51.1809764Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-04-11T05:57:51.1810069Z 
2020-04-11T05:57:51.1810163Z 
2020-04-11T05:57:51.1813804Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/armv5te-unknown-linux-gnueabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-armv5te-unknown-linux-gnueabi" "--mode" "mir-opt" "--target" "armv5te-unknown-linux-gnueabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--linker" "arm-linux-gnueabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/armv5te-unknown-linux-gnueabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-04-11T05:57:51.1816477Z 
2020-04-11T05:57:51.1816574Z 
2020-04-11T05:57:51.1817128Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/mir-opt --target=armv5te-unknown-linux-gnueabi
2020-04-11T05:57:51.1817530Z Build completed unsuccessfully in 0:01:20
2020-04-11T05:57:51.1817530Z Build completed unsuccessfully in 0:01:20
2020-04-11T05:57:51.1817778Z == clock drift check ==
2020-04-11T05:57:51.1818017Z   local time: Sat Apr 11 05:57:51 UTC 2020
2020-04-11T05:57:51.3226600Z   network time: Sat, 11 Apr 2020 05:57:51 GMT
2020-04-11T05:57:53.7849633Z 
2020-04-11T05:57:53.7849633Z 
2020-04-11T05:57:53.7919820Z ##[error]Bash exited with code '1'.
2020-04-11T05:57:53.7934733Z ##[section]Finishing: Run build
2020-04-11T05:57:53.7996356Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T05:57:53.8004960Z Task         : Get sources
2020-04-11T05:57:53.8005592Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T05:57:53.8006171Z Version      : 1.0.0
2020-04-11T05:57:53.8006604Z Author       : Microsoft
2020-04-11T05:57:53.8006604Z Author       : Microsoft
2020-04-11T05:57:53.8007250Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T05:57:53.8007998Z ==============================================================================
2020-04-11T05:57:54.1052865Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T05:57:54.1096863Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70989/merge to s
2020-04-11T05:57:54.1179311Z Cleaning up task key
2020-04-11T05:57:54.1180563Z Start cleaning up orphan processes.
2020-04-11T05:57:54.1351637Z Terminate orphan process: pid (3612) (python)
2020-04-11T05:57:54.1848958Z ##[section]Finishing: Finalize Job
