plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0a1d57a0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
$ export SCCACHE_REGION=us-west-1
$ export TOOLSTATE_REPO_ACCESS_TOKEN=[secure]
$ export AWS_ACCESS_KEY_ID=[secure]
$ export AWS_SECRET_ACCESS_KEY=[secure]
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export GCP_CLIENT_SECRET=[secure]
$ export IMAGE=dist-x86_64-linux
$ export DEPLOY=1
$ bash -c 'echo $BASH_VERSION'
4.3.48(1)-release
---
travis_time:end:0f8f78cd:start=1551778344888621285,finish=1551778344894282696,duration=5661411
travis_fold:end:before_install.2
travis_fold:start:before_install.3
travis_time:start:26b1fe05
$ src/ci/init_gcloud.sh
Downloading the Google Cloud SDK...
                                 Dload  Upload   Total   Spent    Left  Speed
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  4 24.7M    4 1256k    0     0  1843k      0  0:00:13 --:--:--  0:00:13 1841k
100 24.7M  100 24.7M    0     0  24.5M      0  0:00:01  0:00:01 --:--:-- 24.5M
100 24.7M  100 24.7M    0     0  24.5M      0  0:00:01  0:00:01 --:--:-- 24.5M
Extracting the Google Cloud SDK...
Logging in Google Cloud through the service account...
Activated service account credentials for: [ci-travis-rustc@rust-lang.iam.gserviceaccount.com]
travis_fold:end:before_install.3
travis_fold:start:before_install.4
travis_time:start:0d2f2df0
$ if [ "$TRAVIS_OS_NAME" = linux ]; then echo '{"ipv6":true,"fixed-cidr-v6":"fd9a:8454:6789:13f7::/64"}' | sudo tee /etc/docker/daemon.json; sudo service docker restart; fi
---
[00:00:11] +src/ci/docker/run.sh dist-x86_64-linux
[00:00:11] travis_time:end:007b48c0:start=1551778353074851903,finish=1551778364081374830,duration=11006522927
travis_fold:start:build_docker
travis_time:start:022120ee
Attempting to download gs://rust-lang-ci-cache/docker/34e1bfa5d2020a1617acad01efdf17506bbee2c181b2ba15c735760ec3ad409bfdcf54856011a205fc0e5714ef5d7b306822a181e59f8abc98245ddfda645917
[00:00:11]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:00:11]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:11] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:01:13] 
[00:01:13] Total download size: 4.9 M
[00:01:13] Downloading Packages:
[00:01:16] --------------------------------------------------------------------------------
[00:01:16] Total                                           1.5 MB/s | 4.9 MB     00:03     
[00:01:16] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:16] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:17] Running Transaction Test
[00:01:17] Finished Transaction Test
[00:01:17] Transaction Test Succeeded
[00:01:17] Running Transaction
---
[00:04:00] + hide_output make install
[00:04:00] + set +x
[00:04:21] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:21] + cd ..
[00:04:21] + rm -rf openssl-1.0.2k
[00:04:21] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:21] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:21]  ---> d8c542a1e367
[00:04:21] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:22]  ---> c0c0d4021b3b
[00:04:22] Step 15/41 : RUN ./build-curl.sh
[00:04:22] Step 15/41 : RUN ./build-curl.sh
[00:04:22]  ---> Running in bcb8d2653e76
[00:04:22] + source shared.sh
[00:04:22] + VERSION=7.51.0
[00:04:22] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:24]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:24]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:25] 
  0 2509k    0 14215    0     0   9273      0  0:04:37  0:00:01  0:04:36  9273
  0 2509k    0 14215    0     0   9273      0  0:04:37  0:00:01  0:04:36  9273
 34 2509k   34  877k    0     0   373k      0  0:00:06  0:00:02  0:00:04 1054k
100 2509k  100 2509k    0     0   889k      0  0:00:02  0:00:02 --:--:-- 1939k
[00:04:25] + mkdir curl-build
[00:04:25] + cd curl-build
[00:04:25] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:49] + hide_output make -j10
[00:04:49] + set +x
[00:05:02] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:02] + hide_output make install
---
 81 67.8M   81 55.1M    0     0  7126k      0  0:00:09  0:00:07  0:00:02 7782k
 93 67.8M   93 63.7M    0     0  7307k      0  0:00:09  0:00:08  0:00:01 7929k
100 67.8M  100 67.8M    0     0  7322k      0  0:00:09  0:00:09 --:--:-- 8243k
[00:08:50] + cd gcc-5.5.0
[00:08:50] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:50] --2019-03-05 09:41:23--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:50] Resolving gcc.gnu.org... 209.132.180.131
[00:08:50] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:50] HTTP request sent, awaiting response... 200 OK
---
[01:15:41]  ---> 154272fb1336
[01:15:41] Step 25/41 : RUN ./build-clang.sh
[01:15:41]  ---> Running in c6493bb561fa
[01:15:42] + source shared.sh
[01:15:42] + LLVM=llvmorg-8.0.0-rc2
[01:15:42] + mkdir llvm-project
[01:15:42] + tar xzf - --strip-components=1
[01:15:42] + tar xzf - --strip-components=1
[01:15:42] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:15:42]                                  Dload  Upload   Total   Spent    Left  Speed
[01:15:42] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    364      0 --:--:-- --:--:-- --:--:--   394
---
[01:16:01] + cd clang-build
[01:16:01] + INC=/rustroot/include
[01:16:01] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:16:01] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:01] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:31] Tue Mar 5 10:49:04 UTC 2019 - building ...
[01:16:41] + hide_output make -j10
[01:16:41] + set +x
[01:17:11] Tue Mar 5 10:49:44 UTC 2019 - building ...
---
[02:43:26] Step 32/41 : RUN ./build-perl.sh
[02:43:26]  ---> Running in 3980a04f08df
[02:43:27] + source shared.sh
[02:43:27] + tar xzf -
[02:43:27] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:43:27]                                  Dload  Upload   Total   Spent    Left  Speed
[02:43:28] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100 17.0M  100 17.0M    0     0  13.9M      0  0:00:01  0:00:01 --:--:-- 15.8M
[02:43:28] + cd perl-5.28.0
[02:43:28] + CC=gcc
[02:43:28] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:43:28] + hide_output ./configure.gnu
[02:43:28] + set +x
[02:43:58] Tue Mar 5 12:16:31 UTC 2019 - building ...
[02:44:09] + hide_output make -j10
---
[02:45:42]  ---> 1a03f838db6a
[02:45:42] Successfully built 1a03f838db6a
[02:45:42] Successfully tagged rust-ci:latest
[02:45:42] Built container sha256:1a03f838db6abb773ef3c7e193c4a1c310852c0390c7de8b4a4adde0b3f2008f
[02:45:42] Uploading finished image to gs://rust-lang-ci-cache/docker/34e1bfa5d2020a1617acad01efdf17506bbee2c181b2ba15c735760ec3ad409bfdcf54856011a205fc0e5714ef5d7b306822a181e59f8abc98245ddfda645917
[02:45:44] Copying from <STDIN>...
[02:49:47] / [0 files][    0.0 B/    0.0 B]                                                
/ [0 files][263.3 KiB/    0.0 B]                                                
/ [0 files][100.0 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][200.1 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][300.1 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][400.1 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][500.2 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][598.4 MiB/    0.0 B]   98.2 MiB/s                                   
/ [0 files][600.2 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][700.2 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][800.3 MiB/    0.0 B]      0.0 B/s                                   
/ [0 files][900.0 MiB/    0.0 B]      0.0 B/s                                   
/ [1 files][    0.0 B/    0.0 B]      0.0 B/s                                   
[02:49:47] Operation completed over 1 objects.                                              

[02:49:48] travis_time:end:022120ee:start=1551778364093203929,finish=1551788540945683434,duration=10176852479505
[CI_JOB_NAME=dist-x86_64-linux]
[02:49:48] [CI_JOB_NAME=dist-x86_64-linux]
---
[02:53:54]    Compiling arena v0.0.0 (/checkout/src/libarena)
[02:53:55]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[02:54:05]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[02:55:10]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
The job exceeded the maximum time limit for jobs, and has been terminated.
