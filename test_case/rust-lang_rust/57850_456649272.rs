plain
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:07] null
[00:12:08] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:10] Sending build context to Docker daemon  483.3kB
[00:12:10] Step 1/18 : FROM ubuntu:16.04
[00:12:10]  ---> 7e87e2b3bf7a
[00:12:10] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:19] null
[00:12:20] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:22] Sending build context to Docker daemon  483.3kB
[00:12:22] Step 1/18 : FROM ubuntu:16.04
[00:12:22]  ---> 7e87e2b3bf7a
[00:12:22] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:31] null
[00:12:32] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:35] Sending build context to Docker daemon  483.3kB
[00:12:35] Step 1/18 : FROM ubuntu:16.04
[00:12:35]  ---> 7e87e2b3bf7a
[00:12:35] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:45] null
[00:12:46] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
[00:12:50] Sending build context to Docker daemon  483.3kB
[00:12:50] Step 1/18 : FROM ubuntu:16.04
[00:12:50]  ---> 7e87e2b3bf7a
[00:12:50] Step 2/18 : COPY scripts/android-base-apt-get.sh /scripts/
---
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=========                              ] 25% Fetch remote repository...        
[=======================================] 100% Fetch remote repository...       
Error: Package path is not valid. Valid system image paths are:
[00:12:59] null
[00:13:00] The command '/bin/sh -c . /scripts/android-sdk.sh &&     download_and_create_avd 4333796 armeabi-v7a 18' returned a non-zero code: 1
travis_time:end:05a6f34e:start=1548211081944419566,finish=1548211862530559801,duration=780586140235
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:1c67b61b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0e67226f:start=1548211863423073310,finish=1548211863432032933,duration=8959623
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f81457
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ffa1838
travis_time:start:0ffa1838
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f3d9c7a
$ dmesg | grep -i kill
