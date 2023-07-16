plain
$ pip install --user awscli; export PATH=$PATH:$HOME/.local/bin:$HOME/Library/Python/2.7/bin/
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/__init__.py:83: RequestsDependencyWarning: Old version of cryptography ([1, 2, 3]) may cause slowdown.
  warnings.warn(warning, RequestsDependencyWarning)
Collecting awscli
  Downloading https://files.pythonhosted.org/packages/c2/fb/db83d39c7577dd6719963f86afaa9bf886ba4ae6a1429b66d457da24720a/awscli-1.16.108-py2.py3-none-any.whl (1.4MB)
  Downloading https://files.pythonhosted.org/packages/d7/de/5737f602e22073ecbded7a0c590707085e154e32b68d86545dcc31004c02/s3transfer-0.2.0-py2.py3-none-any.whl (69kB)
Collecting docutils>=0.10 (from awscli)
  Downloading https://files.pythonhosted.org/packages/50/09/c53398e0005b11f7ffb27b7aa720c617aba53be4fb4f4f3f06b9b5c60f28/docutils-0.14-py2-none-any.whl (543kB)
Collecting botocore==1.12.98 (from awscli)
---
 99 82.1M   99 81.4M    0     0   253k      0  0:05:31  0:05:28  0:00:03  322k
 99 82.1M   99 81.8M    0     0   254k      0  0:05:31  0:05:29  0:00:02  325k
100 82.1M  100 82.1M    0     0   254k      0  0:05:30  0:05:30 --:--:--  356k
[00:16:46] + cd gcc-4.8.5
[00:16:46] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:16:46] --2019-02-19 22:38:20--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:16:46] Resolving gcc.gnu.org... 209.132.180.131
[00:16:46] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:16:46] HTTP request sent, awaiting response... 200 OK
---
[00:52:46]  ---> d56b0f4c8480
[00:52:46] Step 25/41 : RUN ./build-clang.sh
[00:52:46]  ---> Running in 68bcecf47eca
[00:52:46] + source shared.sh
[00:52:46] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:52:46] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:52:46] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:52:46] + mkdir clang
[00:52:46] + cd clang
[00:52:46] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:52:46] + tar xzf - --strip-components=1
[00:52:47]                                  Dload  Upload   Total   Spent    Left  Speed
[00:52:47] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 10.5M    0 10.5M    0     0  4361k      0 --:--:--  0:00:02 --:--:-- 5374k
100 15.7M    0 15.7M    0     0  4659k      0 --:--:--  0:00:03 --:--:-- 5382k
100 17.1M    0 17.1M    0     0  4858k      0 --:--:--  0:00:03 --:--:-- 5579k
[00:52:59] + mkdir -p tools/lld
[00:52:59] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:52:59] + tar zxf - --strip-components=1 -C tools/lld
[00:52:59]                                  Dload  Upload   Total   Spent    Left  Speed
[00:52:59] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   157    0   157    0     0    820      0 --:--:-- --:--:-- --:--:--   835
100   157    0   157    0     0    820      0 --:--:-- --:--:-- --:--:--   835
[00:52:59] 
100 1346k    0 1346k    0     0  2222k      0 --:--:-- --:--:-- --:--:-- 2222k
[00:52:59] + mkdir clang-build
[00:52:59] + cd clang-build
[00:52:59] + INC=/rustroot/include
[00:52:59] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:52:59] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:52:59] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:53:29] Tue Feb 19 23:15:04 UTC 2019 - building ...
[00:53:34] + hide_output make -j10
[00:53:34] + set +x
[00:54:04] Tue Feb 19 23:15:39 UTC 2019 - building ...
---
[01:59:14]  ---> 8db16806acc0
[01:59:14] Step 32/41 : RUN ./build-perl.sh
[01:59:14]  ---> Running in 0aeb4f55099c
[01:59:14] + source shared.sh
[01:59:14] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:59:15]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:59:15]                                  Dload  Upload   Total   Spent    Left  Speed
[01:59:16] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:58:26] warning: clang-8: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[02:58:26] warning: clang-8: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[02:58:26] warning: clang-8: warning: argument unused during compilation: '-static-libstdc++' [-Wunused-command-line-argument]
[02:58:26] [RUSTC-TIMING] rustc_llvm test:false 0.178
The job exceeded the maximum time limit for jobs, and has been terminated.
