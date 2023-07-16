plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/18/61/4e0f977cfe063188d73622a91cab8b8b409b662f422303fc687f362d941f/awscli-1.15.18-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 37.9MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 1.8MB/s eta 0:00:01
---
[00:00:11] travis_time:end:2ab1c090:start=1525985727743557782,finish=1525985739345480126,duration=11601922344
travis_fold:start:build_docker
travis_time:start:0f38db4c
Attempting to download s3://rust-lang-ci-sccache2/docker/de94e11fb622ebd51292930689046e4dcd653833568b97f8dbcebaae8181ce5ef0a7935c29e835bdd3f7ec37b1c23cf77eb4686cd315c7beefdafb75370cd2ba
[00:00:11] Attempting with retry: curl -f -L -C - -o /tmp/rustci_docker_cache https://s3-us-west-1.amazonaws.com/rust-lang-ci-sccache2/docker/de94e11fb622ebd51292930689046e4dcd653833568b97f8dbcebaae8181ce5ef0a7935c29e835bdd3f7ec37b1c23cf77eb4686cd315c7beefdafb75370cd2ba
[00:00:11]                                  Dload  Upload   Total   Spent    Left  Speed
[00:00:18] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0  234M    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:05:13] * wrapping_int_impl                lib      unstable     None    
[00:05:13] * wrapping_iter_arith              lib      stable       1.14.0  
[00:05:13] * wrapping_neg                     lib      stable       1.10.0  
[00:05:13] * wrapping_ref                     lib      stable       1.14.0  
[00:05:13] tidy error: /checkout/src/liballoc/tests/str.rs:482: platform-specific cfg: cfg(not(target_os = "emscripten"))
[00:05:14] some tidy checks failed
[00:05:14] 
[00:05:14] 
[00:05:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:05:14] 
[00:05:14] 
[00:05:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:05:14] Build completed unsuccessfully in 0:02:05
[00:05:14] Build completed unsuccessfully in 0:02:05
[00:05:14] Makefile:79: recipe for target 'tidy' failed
[00:05:14] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:09488b0c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
