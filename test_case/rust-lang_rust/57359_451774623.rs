plain
travis_time:end:04a62458:start=1546805715492876390,finish=1546805786526760449,duration=71033884059
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:08] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:08]  ---> Running in a5d5ed0a7d39
[00:01:09] + TARGET=x86_64-linux-musl
[00:01:09] + ARCH=x86_64
[00:01:09] + OUTPUT=/usr/local
[00:01:09] + shift
[00:01:09] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:09] Cloning into 'musl-cross-make'...
[00:01:09] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:09] 
[00:01:09] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:09] changes and commit them, and you can discard any commits you make in this
[00:01:09] state without impacting any branches by performing another checkout.
[00:01:09] 
[00:01:09] If you want to create a new branch to retain commits you create, you may
[00:01:09] do so (now or later) by using -b with the checkout command again. Example:
[00:01:09] 
[00:01:09]   git checkout -b <new-branch-name>
[00:01:09] + cd musl-cross-make
[00:01:09] ++ nproc
[00:01:09] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:09] + set +x
---
[00:10:10] Sun Jan 6 20:26:45 UTC 2019 - building ...
[00:10:40] Sun Jan 6 20:27:15 UTC 2019 - building ...
[00:11:10] Sun Jan 6 20:27:45 UTC 2019 - building ...
[00:11:40] Sun Jan 6 20:28:15 UTC 2019 - building ...
[00:11:56] musl-toolchain.sh: line 13:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:56] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:09] /build
[00:12:09] musl-toolchain.sh: line 13:  6374 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:12:09] + cd -
[00:12:09] + cd -
[00:12:09] + ln -s /usr/local/x86_64-linux-musl/lib/libc.so /lib/ld-musl-x86_64.so.1
[00:12:09] + echo /usr/local/x86_64-linux-musl/lib
[00:12:09] + export CC=x86_64-linux-musl-gcc
[00:12:09] + CC=x86_64-linux-musl-gcc
[00:12:09] + export CXX=x86_64-linux-musl-g++
[00:12:09] + CXX=x86_64-linux-musl-g++
[00:12:09] + CFLAGS='-fPIC '
[00:12:09] + LLVM=60
[00:12:09] + '[' '!' -d libunwind-release_60 ']'
[00:12:09] + curl -L https://github.com/llvm-mirror/llvm/archive/release_60.tar.gz
---
[00:15:11] Step 8/10 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:11]  ---> Running in 3c13db64bda8
[00:15:11] Removing intermediate container 3c13db64bda8
[00:15:11]  ---> 03c46e95feac
[00:15:11] Step 9/10 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:11] Removing intermediate container 646ed4f28620
[00:15:11]  ---> 263501355b97
[00:15:11] Step 10/10 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:11]  ---> Running in 3d9e8e6223fe
[00:15:11]  ---> Running in 3d9e8e6223fe
[00:15:11] Removing intermediate container 3d9e8e6223fe
[00:15:11]  ---> c5ccc9e76c7f
[00:15:11] Successfully built c5ccc9e76c7f
[00:15:11] Successfully tagged rust-ci:latest
[00:15:11] Built container sha256:c5ccc9e76c7f2a37bb493eccbd0323a61da65783edd467dbcf66fa04fefe3c5c
[00:15:11] Uploading finished image to s3://rust-lang-ci-sccache2/docker/92151d09012d25ae66a87636becad86ca1fc5bb763002d3eaffeb358fe628b85e1da22cbc5aafd6f4a193417c390be71037ec8b008ed49726e411e6b5066e3a3
[00:21:35] upload failed: - to s3://rust-lang-ci-sccache2/docker/92151d09012d25ae66a87636becad86ca1fc5bb763002d3eaffeb358fe628b85e1da22cbc5aafd6f4a193417c390be71037ec8b008ed49726e411e6b5066e3a3 Unable to locate credentials

[00:21:36] travis_time:end:01bb52f6:start=1546805806401197376,finish=1546807091081513920,duration=1284680316544
[CI_JOB_NAME=dist-x86_64-musl]
[00:21:36] [CI_JOB_NAME=dist-x86_64-musl]
---
[00:46:14]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:46:14]    Compiling rustc-demangle v0.1.10
[00:46:14]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:46:20]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:46:42] error: linking with `x86_64-linux-musl-gcc` failed: exit code: 1
[00:46:42]   |
[00:46:42]   = note: "x86_64-linux-musl-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-fPIC" "-Wl,--eh-frame-hdr" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-musl/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/std-17ddccb9ffa299d5.std.b0f5gycn-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/libstd-17ddccb9ffa299d5.so" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/std-17ddccb9ffa299d5.3n78dsopae6oqrp8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/std-17ddccb9ffa299d5.42d8hsc84mozgxop.rcgu.o" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/build/compiler_builtins-eb8895bdc00ff673/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/build/backtrace-sys-c197728c7f4c4e0f/out" "-L" "/usr/local/x86_64-linux-musl/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "-Wl,-../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(lio_listio.o): relocation R_X86_64_32 against `.text.wait_thread' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(confstr.o): relocation R_X86_64_32 against `.rodata.confstr.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(fpathconf.o): relocation R_X86_64_32S against `.rodata.values.1546' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(sysconf.o): relocation R_X86_64_32S against `.rodata.values.2364' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(crypt.o): relocation R_X86_64_32 against `.bss.buf.1558' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(crypt_blowfish.o): relocation R_X86_64_32S against `.rodata.BF_init_state' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(crypt_des.o): relocation R_X86_64_32S against `.rodata.key_perm_maskl' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(crypt_md5.o): relocation R_X86_64_32 against `.rodata.tab' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(crypt_sha256.o): relocation R_X86_64_32S against `.rodata.K' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(crypt_sha512.o): relocation R_X86_64_32S against `.rodata.K' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(encrypt.o): relocation R_X86_64_32 against `.bss.__encrypt_key' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6[0m          /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exit.o): relocation R_X86_64_32 against undefined hidden symbol `__fini_array_end' can not be used when making a shared object
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(floatscan.o): relocation R_X86_64_32S against `.rodata.p10s.2584' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(intscan.o): relocation R_X86_64_32S against `.rodata.table' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(procfdname.o): relocation R_X86_64_32S against `.rodata.__procfdname.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(version.o): relocation R_X86_64_32 against `.rodata.version' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__dlsym.o): relocation R__64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(daemon.o): relocation R_X86_64_32 against `.rodata.daemon.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(err.o): relocation R_X86_64_32 against `.rodata.vwarn.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getpass.o): relocation R_X86_64_32 against `.rodata.getpass.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getusershell.o): relocation R_X86_64_32 against `.rodata.setusershell.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(bind_textdomain_codeset.o): relocation R_X86_64_32 against `.rodata.bind_textdomain_codeset.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(dcngettext.o): rel/usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(newlocale.o): relocation R_X86_64_32S against symbol `__c_locale' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(pleval.o): relocation R_X86_64_32S against `.rodata.opch.1602' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(setlocale.o): relocation R_X86_64_32 against `.bss.lock.2875' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(strfmon.o): relocation R_X86_64_32 against `.rodata.vstrfmon_l.isra.0.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(textdomain.o): relocation R_X86_64_32 against `.rodata.__gettextdomain.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(uselocale.o): rusl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exp.o): relocation R_X86_64_32S against `.rodata.half' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exp10.o): relocation R_X86_64_32S against `.rodata.p10.2445' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exp10f.o): relocation R_X86_64_32S against `.rodata.p10.2445' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exp10l.o): relocation R_X86_64_32S against `.rodata.p10.2659' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exp2.o): relocation R_X86_64_32S against `.rodata.tbl' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(exp2f.o): relocation R_X86_64_32S against `.rodata.exp2ft' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(expf.o): relocation R_X86_64_32S against `.rodata.half' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(j0.o): relocation R_X86_64_32 against `.rodata.pS3' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(j0f.o): relocation R_X86_64_32 against `.rodata.pS3' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(j1.o): relocation R_X86_64_32 against `.rodata.ps3' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(j1f.o): relocation R_X86_64_32 against `.rodata.ps3' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(lgamma.o): relocation R_X86_64_32 against symbol `usl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(powf.o): relocation R_X86_64_32S against `.rodata.bp' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(powl.o): relocation R_X86_64_32S against `.rodata.A' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(tgamma.o): relocation R_X86_64_32S against `.rodata.fact' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(tgammal.o): relocation R_X86_64_32 against `.rodata.STIR' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(a64l.o): relocation R_X86_64_32 against `.rodata.digits' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(basename.o): relocation R_X86_64_32 against `.rodata.basename.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]    /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(wordexp.o): relocation R_X86_64_32 against `.rodata.wordexp.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(shm_open.o): relocation R_X86_64_32 against `.rodata.__shm_mapname.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mq_notify.o): relocation R_X86_64_32 against `.text.start' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(c16rtomb.o): relocation R_X86_64_32 against `.bss.internal_state.1610' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mbrlen.o): relocation R_X86_64_32 against `.bss.internal.1588' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mbrtoc16.o): relocation R_X86_64_32 against `.bss.internal_state.1609' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mbrtoc32.o): relocation R_X86_64_32 against `.bss.internal_state.1609' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mbrtowc.o): relocation R_X86_64_32 against `.bss.internal_state.3051' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mbsrtowcs.o): relocation R_X86_64_32S against symbol `__fsmu8' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mbtowc.o): relocation R_X86_64_32S against symbol `__fsmu8' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(ether.o): relocation R_X86_64_32 against `.bss.a.2005' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(gai_strerror.o): relocation R_X86_64_32 against `.rodata.msgs' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getifaddrs.o): relocation R_X86_64_32 against `.text.netlink_msg_to_ifaddr' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getnameinfo.o): relocation R_X86_64_32 against `.rodata.getnameinfo.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getservbyname.o): relocation R_X86_64_32 against `.bss.se.1690' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getservbyname_r.o): relocation R_X86_64_32S against `.rodata.getservbyname_r.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getservbyport.o): relocation R_X86_64_ux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getgr_a.o): relocation R_X86_64_32 against `.rodata.__getgr_a.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getgrent.o): relocation R_X86_64_32 against `.rodata.getgrent.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getgrouplist.o): relocation R_X86_64_32 against `.rodata.getgrouplist.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getpw_a.o): relocation R_X86_64_32 against `.rodata.__getpw_a.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getpwent.o): relocation R_X86_64_32 against `.rodata.getpwent.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(getspnam.o): relocation R_X86_64_32 againc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(drand48.o): relocation R_X86_64_32 against symbol `__seed48' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(lcong48.o): relocation R_X86_64_32 against symbol `__seed48' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(lrand48.o): relocation R_X86_64_32 against symbol `__seed48' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mrand48.o): relocation R_X86_64_32 against symbol `__seed48' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(random.o): relocation R_X86_64_32 against `.bss.lock' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(seed48.o): relocation R_X86_64_32 against symbol `__seed48' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(execvp.o): relocation R_X86_64_32 against `.rodata.__execvpe.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(posix_spawn.o): relocation R_X86_64_32 against `.text.child' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(posix_spawnp.o): relocation R_X86_64_32 against symbol `__execvpe' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(system.o): relocation R_X86_64_32 against `.rodata.system.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(glob.o): relocation R_X86_64_32 against `.rodata.append.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(psignal.o): relocation R_X86_64_32 against `.rodata.psignal.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(sigaction.o): relocation R_X86_64_32 against `.bss.handler_set' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(sigisemptyset.o): relocation R_X86_64_32 against `.bss.zeroset.1784' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__fdopen.o): relocation R_X86_64_32 against `.rodata.__fdopen.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__fopen_rb_ca.o): relocation R_X86_64_32S against symbol `__stdio_read' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib86_64_32S against `.text.wrap_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vfprintf.o): relocation R_X86_64_32S against `.rodata.pop_arg' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vfscanf.o): relocation R_X86_64_32S against `.rodata.store_int' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vfwprintf.o): relocation R_X86_64_32S against `.rodata.pop_arg' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vfwscanf.o): relocation R_X86_64_32S against `.rodata.vfwscanf' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vsnprintf.o): relocation R_X86_64_32S against `.text.sn_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vsscanf.o): relocation R_X86_64_32S against `.text.do_read' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vswprintf.o): relocation R_X86_64_32S against `.text.sw_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(vswscanf.o): relocation R_X86_64_32S against `.text.wstring_read' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(ecvt.o): relocation R_X86_64_32 against `.rodata.ecvt.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(fcvt.o): relocation R_X86_64_32 against `.rodata.fcvt.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(gcvt.o): relocation R_X86_64_32 against `.rodata.gcvt.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(wcstod.o): relocation R_X86_64_32 against `.rodata.do_read.str4.4' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(wcstol.o): relocation R_X86_64_32 against `.rodata.do_read.str4.4' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(strsignal.o): relocation R_X86_64_32 against `.rodata.strings' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mkdtemp.o): relocation R_X86_64_32 against `.rodata.mkdtemp.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(mkostemps.o): relocation R_X86_64_32 against `.rodata.__mkostemps.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcbd.rlib(vmlock.o): relocation R_X86_64_32 against `.bss.vmlock' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__asctime.o): relocation R_X86_64_32 against symbol `__c_locale' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__month_to_secs.o): relocation R_X86_64_32S against `.rodata.secs_through_month.1281' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__secs_to_tm.o): relocation R_X86_64_32S against `.rodata.days_in_month.1436' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(__tz.o): relocation R_X86_64_32 against `.rodata.do_tzset.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(asctime.o): relocation R_X86_64_32 against `.bss.buf.1401' can not be used when making a shared object; recompile with -fPIC
[00:46:4286_64_32S against `.rodata.__strftime_fmt_1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(strptime.o): relocation R_X86_64_32S against `.rodata.strptime' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(timegm.o): relocation R_X86_64_32S against symbol `__gmt' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(timer_create.o): relocation R_X86_64_32S against `.text.timer_handler' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(ctermid.o): relocation R_X86_64_32 against `.rodata.ctermid.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustcSOG6yX/liblibc-ca1546dd1e1833bd.rlib(faccessat.o): relocation R_X86_64_32S against `.rodata.errors' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-travis_time:end:0d70d8b7:start=1546805795068320657,finish=1546808598354316622,duration=2803285995965
travis_time:start:1e1b408c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan  6 21:03:18 UTC 2019
Sun, 06 Jan 2019 21:03:18 GMT
---
travis_time:end:1dca7b00:start=1546808599497195641,finish=1546808599509679877,duration=12484236
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:180e0892
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06568990
$ dmesg | grep -i kill
