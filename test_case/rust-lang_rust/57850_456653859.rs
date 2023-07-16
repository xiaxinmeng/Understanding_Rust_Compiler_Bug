plain
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:18] null
[00:12:19] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:20] Sending build context to Docker daemon  483.3kB
[00:12:20] Step 1/18 : FROM ubuntu:16.04
[00:12:20]  ---> 7e87e2b3bf7a
[00:12:20] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:29] null
[00:12:30] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:32] Sending build context to Docker daemon  483.3kB
[00:12:32] Step 1/18 : FROM ubuntu:16.04
[00:12:32]  ---> 7e87e2b3bf7a
[00:12:32] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:41] null
[00:12:42] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:45] Sending build context to Docker daemon  483.3kB
[00:12:45] Step 1/18 : FROM ubuntu:16.04
[00:12:45]  ---> 7e87e2b3bf7a
[00:12:45] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:54] null
[00:12:55] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:59] Sending build context to Docker daemon  483.3kB
[00:12:59] Step 1/18 : FROM ubuntu:16.04
[00:12:59]  ---> 7e87e2b3bf7a
[00:12:59] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:13:08] null
[00:13:09] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
travis_time:end:11cc5f7e:start=1548212492434555047,finish=1548213282270481906,duration=789835926859
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:089f7ef1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0cc10104:start=1548213283286918925,finish=1548213283295958347,duration=9039422
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d8b714b
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b702ce0
travis_time:start:2b702ce0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:310f71aa
$ dmesg | grep -i kill
