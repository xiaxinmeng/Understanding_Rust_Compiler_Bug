plain
[00:03:12] + cd ../clang-build
[00:03:12] + INC=/rustroot/include
[00:03:12] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:03:12] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:03:12] + hide_output cmake ../llvm-7.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:03:42] Sun Dec 23 05:50:03 UTC 2018 - building ...
[00:03:46] + hide_output make -j10
[00:03:46] + set +x
[00:04:16] Sun Dec 23 05:50:37 UTC 2018 - building ...
---
[01:04:20] ./build-clang.sh: line 70: 15990 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:04:36]  ---> d490a7a83e34
[01:04:36] Removing intermediate container a2046053867a
[01:04:36] Step 26/41 : ENV CC clang CXX clang++
[01:04:36]  ---> [Warning] IPv4 forwarding is disabled. Networking will not work.
[01:04:36]  ---> 071ef22121ac
[01:04:36] Removing intermediate container 42e5d6e94ffe
[01:04:36] Step 27/41 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:04:36]  ---> 7823ade6bee4
[01:04:36]  ---> 7823ade6bee4
[01:04:36] Step 28/41 : RUN ./build-git.sh
[01:04:36]  ---> [Warning] IPv4 forwarding is disabled. Networking will not work.
[01:04:37] + source shared.sh
[01:04:37] + tar xzf -
[01:04:37] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:05:17] curl: (6) Couldn't resolve host 'www.kernel.org'
---
[01:05:18] Step 27/41 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:05:18]  ---> Using cache
[01:05:18]  ---> 7823ade6bee4
[01:05:18] Step 28/41 : RUN ./build-git.sh
[01:05:18]  ---> [Warning] IPv4 forwarding is disabled. Networking will not work.
[01:05:18] + source shared.sh
[01:05:18] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:05:18] + tar xzf -
[01:05:58] curl: (6) Couldn't resolve host 'www.kernel.org'
---
[01:06:01] Step 27/41 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:06:01]  ---> Using cache
[01:06:01]  ---> 7823ade6bee4
[01:06:01] Step 28/41 : RUN ./build-git.sh
[01:06:01]  ---> [Warning] IPv4 forwarding is disabled. Networking will not work.
[01:06:01] + source shared.sh
[01:06:01] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:06:01] + tar xzf -
[01:06:41] curl: (6) Couldn't resolve host 'www.kernel.org'
---
[01:06:44] Step 27/41 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:06:44]  ---> Using cache
[01:06:44]  ---> 7823ade6bee4
[01:06:44] Step 28/41 : RUN ./build-git.sh
[01:06:45]  ---> [Warning] IPv4 forwarding is disabled. Networking will not work.
[01:06:45] + source shared.sh
[01:06:45] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:06:45] + tar xzf -
[01:07:25] curl: (6) Couldn't resolve host 'www.kernel.org'
---
[01:07:29] Step 27/41 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:07:29]  ---> Using cache
[01:07:29]  ---> 7823ade6bee4
[01:07:29] Step 28/41 : RUN ./build-git.sh
[01:07:29]  ---> [Warning] IPv4 forwarding is disabled. Networking will not work.
[01:07:29] + source shared.sh
[01:07:29] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:07:29] + tar xzf -
[01:08:09] curl: (6) Couldn't resolve host 'www.kernel.org'
---
travis_time:end:25f0dca8:start=1545548071565510232,finish=1545548071573308236,duration=7798004
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08fe7718
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01d182ea
travis_time:start:01d182ea
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1710b4bc
$ dmesg | grep -i kill
