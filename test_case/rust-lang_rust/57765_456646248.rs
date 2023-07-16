plain
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:14:30] null
[00:14:31] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:14:32] Sending build context to Docker daemon  483.3kB
[00:14:32] Step 1/18 : FROM ubuntu:16.04
[00:14:32]  ---> 7e87e2b3bf7a
[00:14:32] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:14:42] null
[00:14:43] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:14:45] Sending build context to Docker daemon  483.3kB
[00:14:45] Step 1/18 : FROM ubuntu:16.04
[00:14:45]  ---> 7e87e2b3bf7a
[00:14:45] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:14:54] null
[00:14:55] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:14:59] Sending build context to Docker daemon  483.3kB
[00:14:59] Step 1/18 : FROM ubuntu:16.04
[00:14:59]  ---> 7e87e2b3bf7a
[00:14:59] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:15:08] null
[00:15:09] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:15:13] Sending build context to Docker daemon  483.3kB
[00:15:13] Step 1/18 : FROM ubuntu:16.04
[00:15:13]  ---> 7e87e2b3bf7a
[00:15:13] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:15:23] null
[00:15:24] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
travis_time:end:1e307134:start=1548210023240860684,finish=1548210947397906726,duration=924157046042
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:01cf2f52
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0bf839da:start=1548210948297609295,finish=1548210948306854280,duration=9244985
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:031beeee
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09f763f0
travis_time:start:09f763f0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1314a120
$ dmesg | grep -i kill
