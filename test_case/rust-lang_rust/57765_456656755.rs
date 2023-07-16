plain
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:04] null
[00:13:05] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:13:06] Sending build context to Docker daemon  483.3kB
[00:13:06] Step 1/18 : FROM ubuntu:16.04
[00:13:06]  ---> 7e87e2b3bf7a
[00:13:06] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:16] null
[00:13:17] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:13:19] Sending build context to Docker daemon  483.3kB
[00:13:19] Step 1/18 : FROM ubuntu:16.04
[00:13:19]  ---> 7e87e2b3bf7a
[00:13:19] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:28] null
[00:13:29] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:13:33] Sending build context to Docker daemon  483.3kB
[00:13:33] Step 1/18 : FROM ubuntu:16.04
[00:13:33]  ---> 7e87e2b3bf7a
[00:13:33] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:43] null
[00:13:44] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:13:48] Sending build context to Docker daemon  483.3kB
[00:13:48] Step 1/18 : FROM ubuntu:16.04
[00:13:48]  ---> 7e87e2b3bf7a
[00:13:48] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:57] null
[00:13:58] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
travis_time:end:0a7b7094:start=1548213470262473187,finish=1548214308886003510,duration=838623530323
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:006bfaa6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:068677e7:start=1548214310012051747,finish=1548214310022971122,duration=10919375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09cfa272
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0682615c
travis_time:start:0682615c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:208d3a7b
$ dmesg | grep -i kill
