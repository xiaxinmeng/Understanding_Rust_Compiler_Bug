plain
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:23] null
[00:12:25] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:26] Sending build context to Docker daemon  483.3kB
[00:12:26] Step 1/18 : FROM ubuntu:16.04
[00:12:26]  ---> 7e87e2b3bf7a
[00:12:26] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:35] null
[00:12:36] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:38] Sending build context to Docker daemon  483.3kB
[00:12:38] Step 1/18 : FROM ubuntu:16.04
[00:12:38]  ---> 7e87e2b3bf7a
[00:12:38] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:48] null
[00:12:49] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:52] Sending build context to Docker daemon  483.3kB
[00:12:52] Step 1/18 : FROM ubuntu:16.04
[00:12:52]  ---> 7e87e2b3bf7a
[00:12:52] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:01] null
[00:13:02] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
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
[00:13:15] null
[00:13:16] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
travis_time:end:04fb3240:start=1548208980148630012,finish=1548209776664941590,duration=796516311578
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1f8c5248
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:15cb9684:start=1548209777790492619,finish=1548209777801351342,duration=10858723
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:081bbac1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:183ac0cc
travis_time:start:183ac0cc
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ba1a44c
$ dmesg | grep -i kill
