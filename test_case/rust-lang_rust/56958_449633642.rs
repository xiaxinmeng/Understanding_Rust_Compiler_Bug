plain
travis_fold:start:worker_info
Worker information
hostname: 87389ffd-73c1-468d-a5f1-84862a0d3c1f@1.production-1-worker-org-gce-kwhz
version: v6.1.0 https://github.com/travis-ci/worker/tree/a6071dd64658ecf13ec3190bfb9e9ca3c8f3e10d
instance: travis-job-768bba1a-47bf-415f-af1b-88f9a6911cd9 travis-ci-stevonnie-xenial-1541445931-e193d27 (via amqp)
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
Build language: shell
---
CMake suite maintained and supported by Kitware (kitware.com/cmake).
heroku version
heroku/7.18.4 linux-x64 node-v11.0.0
imagemagick version
Version: ImageMagick 6.8.9-9 Q16 x86_64 2018-09-28 http://www.imagemagick.org
4.4
mercurial version
Mercurial Distributed SCM (version 4.8)
(see https://mercurial-scm.org for more information)
---
* ra_serf : Module for accessing a repository via WebDAV protocol using serf.
  - using serf 1.3.8 (compiled with 1.3.8)
  - handles 'http' scheme
  - handles 'https' scheme
The following authentication credential caches are available:
* Plaintext cache in /home/travis/.subversion
* Gnome Keyring
* GPG-Agent
* KWallet (KDE)
Sudo version 1.8.16
Sudo version 1.8.16
Configure options: --prefix=/usr -v --with-all-insults --with-pam --with-fqdn --with-logging=syslog --with-logfac=authpriv --with-env-editor --with-editor=/usr/bin/editor --with-exampledir=/usr/share/doc/sudo/examples --with-timeout=15 --with-password-timeout=0 --with-passprompt=[sudo] password for %p:  --without-lecture --with-tty-tickets --disable-root-mailer --enable-admin-flag --with-sendmail=/usr/sbin/sendmail --with-rundir=/var/run/sudo --mandir=/usr/share/man --libexecdir=/usr/lib/sudo --with-sssd --with-sssd-lib=/usr/lib/x86_64-linux-gnu --with-selinux --with-linux-audit
Sudoers file grammar version 45
Sudoers path: /etc/sudoers
Authentication methods: 'pam'
Syslog facility if syslog is being used for logging: authpriv
---
Flags for mail program: -t
Address to send mail to: root
Subject line for mail messages: *** SECURITY information for %h ***
Incorrect password message: Sorry, try again.
Path to lecture status dir: /var/lib/sudo/lectured
Path to authentication timestamp dir: /var/run/sudo/ts
Default user to run commands as: root
Value to override user's $PATH with: /usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/snap/bin
Path to the editor for use by visudo: /usr/bin/editor
When to require a password for 'list' pseudocommand: any
---
 LANGUAGE
 LANG
 COLORTERM
Environment variables to remove:
 __BASH_FUNC<*
 BASH_FUNC_*
 RUBYLIB
 PYTHONUSERBASE
 PYTHONINSPECT
 PYTHONPATH
---
File in which to store the input/output log: %{seq}
Add an entry to the utmp/utmpx file when allocating a pty
PAM service name to use
PAM service name to use for login shells
Attempt to establish PAM credentials for the target user
Maximum I/O log sequence number: 0
Enable sudoers netgroup support
Enable sudoers netgroup support
Check parent directories for writability when editing files with sudoedit
 10.240.0.7/255.255.255.255
 172.17.0.1/255.255.0.0
Sudoers I/O plugin version 1.8.16
gzip version
---
     user vimrc file: "$HOME/.vimrc"
 2nd user vimrc file: "~/.vim/vimrc"
      user exrc file: "$HOME/.exrc"
  fall-back for $VIM: "/usr/share/vim"
Compilation: gcc -c -I. -Iproto -DHAVE_CONFIG_H   -Wdate-time  -g -O2 -fPIE -fstack-protector-strong -Wformat -Werror=format-security -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=1      
Linking: gcc   -Wl,-Bsymbolic-functions -fPIE -pie -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -o vim        -lm -ltinfo -lnsl  -lselinux  -lacl -lattr -lgpm -ldl     -L/usr/lib/python3.5/config-3.5m-x86_64-linux-gnu -lpython3.5m -lpthread -ldl -lutil -lm      
iptables v1.6.0
curl version
curl version
curl 7.47.0 (x86_64-pc-linux-gnu) libcurl/7.47.0 GnuTLS/3.4.10 zlib/1.2.8 libidn/1.32 librtmp/2.3
GNU Wget 1.17.1 built on linux-gnu.
rsync version
rsync  version 3.1.1  protocol version 31
gimme version
---
travis_time:end:21049b88:start=1545557754277587101,finish=1545557759850868819,duration=5573281718
travis_fold:end:apt
travis_fold:start:services
travis_time:start:0031a0ae
$ sudo systemctl start docker
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:13895ddf
---
4.3.48(1)-release
travis_fold:start:before_install.1
travis_time:start:2434a2fa
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
  Downloading https://files.pythonhosted.org/packages/22/13/5c2df72102ab95f13a0b2ad470500327765e163d6867afd6421316fa6692/awscli-1.16.81-py2.py3-none-any.whl (1.4MB)
    0% |▎                               | 10kB 16.5MB/s eta 0:00:01
    1% |▌                               | 20kB 2.2MB/s eta 0:00:01
    2% |▊                               | 30kB 3.1MB/s eta 0:00:01
---
[00:04:56] + hide_output make install
[00:04:56] + set +x
[00:05:17] shared.sh: line 11:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:17] + cd ..
[00:05:17] + rm -rf openssl-1.0.2k
[00:05:17] ./build-openssl.sh: line 25:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:05:17] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:05:18]  ---> 316d8d740b37
[00:05:18] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:05:19]  ---> baf8ba8239d5
[00:05:19] Step 15/41 : RUN ./build-curl.sh
[00:05:19] Step 15/41 : RUN ./build-curl.sh
[00:05:19]  ---> Running in 2f1f377e1228
[00:05:19] + source shared.sh
[00:05:19] + VERSION=7.51.0
[00:05:19] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:05:21]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:21]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:22] 
  0 2509k    0  8423    0     0   5384      0  0:07:57  0:00:01  0:07:56  5384
  0 2509k    0  8423    0     0   5384      0  0:07:57  0:00:01  0:07:56  5384
  4 2509k    4  113k    0     0  59930      0  0:00:42  0:00:01  0:00:41  282k
 91 2509k   91 2302k    0     0   807k      0  0:00:03  0:00:02  0:00:01 1781k
100 2509k  100 2509k    0     0   862k      0  0:00:02  0:00:02 --:--:-- 1859k
[00:05:22] + mkdir curl-build
[00:05:22] + cd curl-build
[00:05:22] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:46] + hide_output make -j10
[00:05:46] + set +x
[00:06:00] shared.sh: line 11:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:06:00] + hide_output make install
---
 93 82.1M   93 76.9M    0     0  2961k      0  0:00:28  0:00:26  0:00:02 2835k
 97 82.1M   97 79.9M    0     0  2962k      0  0:00:28  0:00:27  0:00:01 2962k
100 82.1M  100 82.1M    0     0  2964k      0  0:00:28  0:00:28 --:--:-- 3001k
[00:10:23] + cd gcc-4.8.5
[00:10:23] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:10:23] --2018-12-23 09:46:43--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:10:24] Resolving gcc.gnu.org... 209.132.180.131
[00:10:24] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:10:24] HTTP request sent, awaiting response... 200 OK
---
[00:48:29]  ---> d08563b626c8
[00:48:29] Step 25/41 : RUN ./build-clang.sh
[00:48:29]  ---> Running in 66aa949c1675
[00:48:29] + source shared.sh
[00:48:29] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:48:29] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:48:29] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:48:29] + mkdir clang
[00:48:29] + cd clang
[00:48:29] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:48:29] + tar xzf - --strip-components=1
[00:48:29]                                  Dload  Upload   Total   Spent    Left  Speed
[00:48:29] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    822      0 --:--:-- --:--:-- --:--:--   868
---
100 8576k    0 8576k    0     0  4344k      0 --:--:--  0:00:01 --:--:-- 5565k
100 13.7M    0 13.7M    0     0  4750k      0 --:--:--  0:00:02 --:--:-- 5559k
100 17.1M    0 17.1M    0     0  5104k      0 --:--:--  0:00:03 --:--:-- 5900k
[00:48:42] + mkdir -p tools/lld
[00:48:42] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:48:42] + tar zxf - --strip-components=1 -C tools/lld
[00:48:42]                                  Dload  Upload   Total   Spent    Left  Speed
[00:48:43] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:48:43] + cd clang-build
[00:48:43] + INC=/rustroot/include
[00:48:43] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:48:43] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:48:43] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:49:13] Sun Dec 23 10:25:33 UTC 2018 - building ...
[00:49:19] + hide_output make -j10
[00:49:19] + set +x
[00:49:50] Sun Dec 23 10:26:09 UTC 2018 - building ...
---
[01:57:57] + rm -rf clang
[01:57:59] ./build-clang.sh: line 67: 16842 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:58:28] Removing intermediate container 66aa949c1675
[01:58:28]  ---> dd67209063eb
[01:58:28] Step 26/41 : ENV CC=clang CXX=clang++
[01:58:28] Removing intermediate container e16986d47f26
[01:58:28]  ---> a0fc5f8f332d
[01:58:28] Step 27/41 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:58:28]  ---> 5aac233c0eaa
---
[01:59:41]  ---> e9896eeb904f
[01:59:41] Step 32/41 : RUN ./build-perl.sh
[01:59:41]  ---> Running in 680aa3d0f9dd
[01:59:41] + source shared.sh
[01:59:41] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:59:41]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:59:41]                                  Dload  Upload   Total   Spent    Left  Speed
[01:59:42] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:06:27] fpu  : yes
[02:06:27] fpu_exception : yes
[02:06:27] cpuid level : 13
[02:06:27] wp  : yes
[02:06:27] flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic popcnt aes xsave avx f16c rdrand hypervisor lahf_lm pti ssbd ibrs ibpb stibp fsgsbase tsc_adjust smep erms xsaveopt arat arch_capabilities
[02:06:27] bugs  : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf
[02:06:27] clflush size : 64
[02:06:27] cache_alignment : 64
[02:06:27] address sizes : 46 bits physical, 48 bits virtual
[02:06:27] power management:
---
[02:06:27] fpu  : yes
[02:06:27] fpu_exception : yes
[02:06:27] cpuid level : 13
[02:06:27] wp  : yes
[02:06:27] flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic popcnt aes xsave avx f16c rdrand hypervisor lahf_lm pti ssbd ibrs ibpb stibp fsgsbase tsc_adjust smep erms xsaveopt arat arch_capabilities
[02:06:27] bugs  : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf
[02:06:27] clflush size : 64
[02:06:27] cache_alignment : 64
[02:06:27] address sizes : 46 bits physical, 48 bits virtual
[02:06:27] power management:
---
[02:06:27] fpu  : yes
[02:06:27] fpu_exception : yes
[02:06:27] cpuid level : 13
[02:06:27] wp  : yes
[02:06:27] flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic popcnt aes xsave avx f16c rdrand hypervisor lahf_lm pti ssbd ibrs ibpb stibp fsgsbase tsc_adjust smep erms xsaveopt arat arch_capabilities
[02:06:27] bugs  : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf
[02:06:27] clflush size : 64
[02:06:27] cache_alignment : 64
[02:06:27] address sizes : 46 bits physical, 48 bits virtual
[02:06:27] power management:
---
[02:06:27] fpu  : yes
[02:06:27] fpu_exception : yes
[02:06:27] cpuid level : 13
[02:06:27] wp  : yes
[02:06:27] flags  : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ss ht syscall nx pdpe1gb rdtscp lm constant_tsc rep_good nopl xtopology nonstop_tsc cpuid pni pclmulqdq ssse3 cx16 pcid sse4_1 sse4_2 x2apic popcnt aes xsave avx f16c rdrand hypervisor lahf_lm pti ssbd ibrs ibpb stibp fsgsbase tsc_adjust smep erms xsaveopt arat arch_capabilities
[02:06:27] bugs  : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf
[02:06:27] clflush size : 64
[02:06:27] cache_alignment : 64
[02:06:27] address sizes : 46 bits physical, 48 bits virtual
[02:06:27] power management:
---
[02:06:27] VmallocUsed:           0 kB
[02:06:27] VmallocChunk:          0 kB
[02:06:27] HardwareCorrupted:     0 kB
[02:06:27] AnonHugePages:         0 kB
[02:06:27] ShmemHugePages:        0 kB
[02:06:27] ShmemPmdMapped:        0 kB
[02:06:27] CmaFree:               0 kB
[02:06:27] HugePages_Total:       0
[02:06:27] HugePages_Free:        0
[02:06:27] HugePages_Rsvd:        0
---
[02:55:47] In file included from /checkout/src/llvm-emscripten/tools/lli/lli.cpp:28:
[02:55:47] /checkout/src/llvm-emscripten/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h:74:5: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:55:47]     operator=(RemoteRTDyldMemoryManager &&) = default;
[02:55:47]     ^
[02:55:47] /checkout/src/llvm-emscripten/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h:315:28: note: move assignment operator of 'RemoteRTDyldMemoryManager' is implicitly deleted because field 'Client' is of reference type 'llvm::orc::remote::OrcRemoteTargetClient &'
[02:55:47]     OrcRemoteTargetClient &Client;
[02:55:47] 1 warning generated.
[02:55:47] [ 99%] Building CXX object tools/lli/CMakeFiles/lli.dir/OrcLazyJIT.cpp.o
[02:55:47] [ 99%] Building CXX object tools/bugpoint/CMakeFiles/bugpoint.dir/OptimizerDriver.cpp.o
[02:55:48] [ 99%] Building CXX object tools/dsymutil/CMakeFiles/llvm-dsymutil.dir/DebugMap.cpp.o
