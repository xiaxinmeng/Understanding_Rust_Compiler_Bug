plain
travis_time:end:1893031c:start=1546802484752627396,finish=1546802554286903206,duration=69534275810
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=dist-x86_64-musl
---
[00:01:10] Step 5/10 : RUN bash musl-toolchain.sh x86_64-linux-musl && rm -rf build
[00:01:10]  ---> Running in be7e1aeefb1b
[00:01:10] + TARGET=x86_64-linux-musl
[00:01:10] + ARCH=x86_64
[00:01:10] + OUTPUT=/usr/local
[00:01:10] + shift
[00:01:10] + git clone https://github.com/richfelker/musl-cross-make -b v0.9.7
[00:01:10] Cloning into 'musl-cross-make'...
[00:01:11] Note: checking out 'b85e29c00d35c8c8c196d6713505b837816ad47f'.
[00:01:11] 
[00:01:11] You are in 'detached HEAD' state. You can look around, make experimental
[00:01:11] changes and commit them, and you can discard any commits you make in this
[00:01:11] state without impacting any branches by performing another checkout.
[00:01:11] 
[00:01:11] If you want to create a new branch to retain commits you create, you may
[00:01:11] do so (now or later) by using -b with the checkout command again. Example:
[00:01:11] 
[00:01:11]   git checkout -b <new-branch-name>
[00:01:11] + cd musl-cross-make
[00:01:11] ++ nproc
[00:01:11] + hide_output make -j4 TARGET=x86_64-linux-musl
[00:01:11] + set +x
---
[00:10:11] Sun Jan 6 19:32:55 UTC 2019 - building ...
[00:10:41] Sun Jan 6 19:33:25 UTC 2019 - building ...
[00:11:11] Sun Jan 6 19:33:55 UTC 2019 - building ...
[00:11:41] Sun Jan 6 19:34:25 UTC 2019 - building ...
[00:11:55] musl-toolchain.sh: line 13:    20 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:11:55] + hide_output make install TARGET=x86_64-linux-musl OUTPUT=/usr/local
[00:12:09] /build
[00:12:09] musl-toolchain.sh: line 13:  6333 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
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
[00:15:10] Step 8/10 : ENV RUST_CONFIGURE_ARGS       --musl-root-x86_64=/usr/local/x86_64-linux-musl       --enable-extended       --disable-docs
[00:15:10]  ---> Running in bb031e788788
[00:15:11] Removing intermediate container bb031e788788
[00:15:11]  ---> 6bcb32fadd3e
[00:15:11] Step 9/10 : ENV HOSTS=x86_64-unknown-linux-musl     CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc     CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
[00:15:11] Removing intermediate container b5868d05af78
[00:15:11]  ---> 2c8d84946734
[00:15:11] Step 10/10 : ENV SCRIPT       python2.7 ../x.py test --host $HOSTS --target $HOSTS &&       python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[00:15:11]  ---> Running in 0716a7d32efa
[00:15:11]  ---> Running in 0716a7d32efa
[00:15:11] Removing intermediate container 0716a7d32efa
[00:15:11]  ---> 406229009032
[00:15:11] Successfully built 406229009032
[00:15:11] Successfully tagged rust-ci:latest
[00:15:11] Built container sha256:4062290090320fe2ce2aca375e6085058487167ad3953f77322a6c93ec7a6229
[00:15:11] Uploading finished image to s3://rust-lang-ci-sccache2/docker/92151d09012d25ae66a87636becad86ca1fc5bb763002d3eaffeb358fe628b85e1da22cbc5aafd6f4a193417c390be71037ec8b008ed49726e411e6b5066e3a3
[00:21:35] upload failed: - to s3://rust-lang-ci-sccache2/docker/92151d09012d25ae66a87636becad86ca1fc5bb763002d3eaffeb358fe628b85e1da22cbc5aafd6f4a193417c390be71037ec8b008ed49726e411e6b5066e3a3 Unable to locate credentials

[00:21:36] travis_time:end:0f2ab080:start=1546802575382331743,finish=1546803858897429345,duration=1283515097602
[CI_JOB_NAME=dist-x86_64-musl]
[00:21:36] [CI_JOB_NAME=dist-x86_64-musl]
---
[00:46:12]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:46:12]    Compiling rustc-demangle v0.1.10
[00:46:13]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:46:19]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:46:42] error: linking with `x86_64-linux-musl-gcc` failed: exit code: 1
[00:46:42]   |
[00:46:42]   = note: "x86_64-linux-musl-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-Wl,--eh-frame-hdr" "-m64" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-musl/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/std-6b83118f9487b978.std.c7gyb0y3-cgu.0.rcgu.o" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/libstd-6b83118f9487b978.so" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/std-6b83118f9487b978.3rypb9vaepbhscg.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps/std-6b83118f9487b978.52hrl9z07ffpuzux.rcgu.o" "-Wl,-zrelro" "-Wl,-znow" "-Wl,-O1" "-nodefaultlibs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/build/compiler_builtins-f991939043ebf2d7/out" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-musl/release/build/backtrace-sys-ea7d883db989c7d2/out" "-L" "/usr/local/x86_64-linux-musl/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/lib/rustlib/x86_64-unknown-linux-musl/lib" "-Wl,-Bstatic" "-Wl,--whole-arx86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(lio_listio.o): relocation R_X86_64_32 against `.text.wait_thread' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(confstr.o): relocation R_X86_64_32 against `.rodata.confstr.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(fpathconf.o): relocation R_X86_64_32S against `.rodata.values.1546' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(sysconf.o): relocation R_X86_64_32S against `.rodata.values.2364' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(crypt.o): relocation R_X86_64_32 against `.bss.buf.1558' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(crypt_blowfish.o): relocation R_X86_64_32S against `.rodata.BF_init_state' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(crypt_des.o): relocation R_X86_64_32S against `.rodata.key_perm_maskl' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(crypt_md5.o): relocation R_X86_64_32 against `.rodata.tab' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(crypt_sha256.o): relocation R_X86_64_32S against `.rodata.K' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(crypt_sha512.o): relocation R_X86_64_32S against `.rodata.K' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(encrypt.o): relocation R_X86_64_32 against `.bss.__encrypt_key' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblib    /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(exit.o): relocation R_X86_64_32 against undefined hidden symbol `__fini_array_end' can not be used when making a shared object
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(floatscan.o): relocation R_X86_64_32S against `.rodata.p10s.2584' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(intscan.o): relocation R_X86_64_32S against `.rodata.table' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(procfdname.o): relocation R_X86_64_32S against `.rodata.__procfdname.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(version.o): relocation R_X86_64_32 against `.rodata.version' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(__dlsym.o): relocation R_X86_64_32-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(daemon.o): relocation R_X86_64_32 against `.rodata.daemon.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(err.o): relocation R_X86_64_32 against `.rodata.vwarn.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getpass.o): relocation R_X86_64_32 against `.rodata.getpass.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getusershell.o): relocation R_X86_64_32 against `.rodata.setusershell.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(bind_textdomain_codeset.o): relocation R_X86_64_32 against `.rodata.bind_textdomain_codeset.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(dcngettext.o): relocation Rl/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(newlocale.o): relocation R_X86_64_32S against symbol `__c_locale' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pleval.o): relocation R_X86_64_32S against `.rodata.opch.1602' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(setlocale.o): relocation R_X86_64_32 against `.bss.lock.2875' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(strfmon.o): relocation R_X86_64_32 against `.rodata.vstrfmon_l.isra.0.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(textdomain.o): relocation R_X86_64_32 against `.rodata.__gettextdomain.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(uselocale.o): relocation' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(lgamma_r.o): relocation R_X86_64_32S against `.rodata.__lgamma_r' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(lgammaf.o): relocation R_X86_64_32 against symbol `__signgam' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(lgammaf_r.o): relocation R_X86_64_32S against `.rodata.__lgammaf_r' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(lgammal.o): relocation R_X86_64_32S against `.rodata.__lgammal_r' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pow.o): relocation R_X86_64_32S against `.rodata.bp' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(powf.o): relocation R_X86_64_32S against `.rodata.bp' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(powl.o): relocation R_X86_64_32S against `.rodata.A' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(tgamma.o): relocation R_X86_64_32S against `.rodata.fact' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(tgammal.o): relocation R_X86_64_32 against `.rodata.STIR' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(a64l.o): relocation R_X86_64_32 against `.rodata.digits' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(basename.o): relocation R_X86_64_32 against `.rodata.basename.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(dirname.o): relocation R_X86_64_32 against `.rodata.dirname.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(fmtmsg.o): relocation R_X86_64_32 against `.rodata.fmtmsg.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(get_current_dir_name.o): relocation R_X86_64_32 against `.rodata.get_current_dir_name.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getopt.o): relocation R_X86_64_32 against `.rodata.getopt.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getopt_long.o): relocation R_X86_64_32 against `.rodata.__getopt_long.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025al/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(wordexp.o): relocation R_X86_64_32 against `.rodata.wordexp.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(shm_open.o): relocation R_X86_64_32 against `.rodata.__shm_mapname.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(mq_notify.o): relocation R_X86_64_32 against `.text.start' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(c16rtomb.o): relocation R_X86_64_32 against `.bss.internal_state.1610' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(mbrlen.o): relocation R_X86_64_32 against `.bss.internal.1588' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(mbrtoc16.o): relocation R_X86_64_32 t `.bss.se.1690' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getservbyport_r.o): relocation R_X86_64_32 against `.rodata.getservbyport_r.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(h_errno.o): relocation R_X86_64_32 against undefined symbol `h_errno' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(herror.o): relocation R_X86_64_32 against `.rodata.herror.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(hstrerror.o): relocation R_X86_64_32 against `.rodata.msgs' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(if_nameindex.o): relocation R_X86_64_32 against `.text.netlink_msg_to_nameindex' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gc64_32 against `.text.cleanup' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(res_state.o): relocation R_X86_64_32 against `.bss.res.1802' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(resolvconf.o): relocation R_X86_64_32 against `.rodata.__get_resolv_conf.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(fgetgrent.o): relocation R_X86_64_32 against `.bss.mem.2215' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(fgetpwent.o): relocation R_X86_64_32 against `.bss.line.2214' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(fgetspent.o): relocation R_X86_64_32 against `.bss.line.2493' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6sp.2012' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getspnam_r.o): relocation R_X86_64_32 against `.rodata.getspnam_r.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(nscd_query.o): relocation R_X86_64_32 against `.rodata.__nscd_query.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(putgrent.o): relocation R_X86_64_32 against `.rodata.putgrent.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(putpwent.o): relocation R_X86_64_32 against `.rodata.putpwent.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(putspent.o): relocation R_X86_64_32 against `.rodata.putspent.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-bject; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(execvp.o): relocation R_X86_64_32 against `.rodata.__execvpe.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(posix_spawn.o): relocation R_X86_64_32 against `.text.child' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(posix_spawnp.o): relocation R_X86_64_32 against symbol `__execvpe' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(system.o): relocation R_X86_64_32 against `.rodata.system.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(glob.o): relocation R_X86_64_32 against `.rodata.append.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-l/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(open_wmemstream.o): relocation R_X86_64_32S against `.text.wms_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(popen.o): relocation R_X86_64_32 against `.rodata.popen.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(tempnam.o): relocation R_X86_64_32 against `.rodata.tempnam.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(tmpfile.o): relocation R_X86_64_32 against `.rodata.tmpfile.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(tmpnam.o): relocation R_X86_64_32 against `.rodata.tmpnam.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vdprintf.o): relocation R_X86_64_32S against `.text.wrap_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vfprintf.o): relocation R_X86_64_32S against `.rodata.pop_arg' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vfscanf.o): relocation R_X86_64_32S against `.rodata.store_int' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vfwprintf.o): relocation R_X86_64_32S against `.rodata.pop_arg' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vfwscanf.o): relocation R_X86_64_32S against `.rodata.vfwscanf' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vsnprintf.o): relocation R_X86_64_32S against `.text.sn_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vsscanf.o): relocation R_X86_64_32S against `.text.do_read' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vswprintf.o): relocation R_X86_64_32S against `.text.sw_write' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vswscanf.o): relocation R_X86_64_32S against `.text.wstring_read' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(ecvt.o): relocation R_X86_64_32 against `.rodata.ecvt.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(fcvt.o): relocation R_X86_64_32 against `.rodata.fcvt.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(gcvt.o): relocation R_X86_64_32 against `.rodata.gcvt.str1.1' can not be used when making a shared object;blibc-398900c3737025aa.rlib(mktemp.o): relocation R_X86_64_32 against `.rodata.mktemp.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(lock_ptc.o): relocation R_X86_64_32 against `.bss.lock' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pthread_atfork.o): relocation R_X86_64_32 against `.bss.lock' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pthread_cancel.o): relocation R_X86_64_32S against hidden symbol `__cp_begin' can not be used when making a shared object
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pthread_create.o): relocation R_X86_64_32S against symbol `__pthread_tsd_main' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pthread_key_create.o): relocation R_X86_64_32S against symbol `__pthread_tsd_main' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pthread_once.o): relocation R_X86_64_32 against `.text.undo' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(pthread_setname_np.o): relocation R_X86_64_32 against `.rodata.pthread_setname_np.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(sem_open.o): relocation R_X86_64_32 against `.bss.lock' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(sem_timedwait.o): relocation R_X86_64_32 against `.text.cleanup' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(synccall.o): relocation R_X86_64_32 against `.bss.target_tid' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(vl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(getlogin.o): relocation R_X86_64_32 against `.rodata.getlogin.str1.1' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(setxid.o): relocation R_X86_64_32 against `.text.do_setxid' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: /tmp/rustckSrKes/liblibc-398900c3737025aa.rlib(ttyname.o): relocation R_X86_64_32 against `.bss.buf.1545' can not be used when making a shared object; recompile with -fPIC
[00:46:42]           /usr/local/bin/../lib/gcc/x86_64-linux-musl/6.3.0/../../../../x86_64-linux-musl/bin/ld: final link failed: Nonrepresentable section on output
[00:46:42]           
[00:46:42] 
[00:46:42] error: aborting due to previous error
[00:46:42] 
