plain
 30  506M   30  154M    0     0   102M      0  0:00:04  0:00:01  0:00:03  102M
 53  506M   53  272M    0     0   109M      0  0:00:04  0:00:02  0:00:02  109M
 77  506M   77  390M    0     0   111M      0  0:00:04  0:00:03  0:00:01  111M
100  506M  100  506M    0     0   114M      0  0:00:04  0:00:04 --:--:--  114M
+ unzip -q android-ndk-r25b-linux.zip
+ rm android-ndk-r25b-linux.zip
+ mv android-ndk-r25b ndk
 ---> 6051b3f12ea3
Step 6/15 : ENV TARGETS=arm-linux-androideabi
 ---> Running in 9fe5f33d19f3
Removing intermediate container 9fe5f33d19f3
---
Step 11/15 : ENV TARGETS=$TARGETS,x86_64-linux-android
 ---> Running in 56fa46e7a41c
Removing intermediate container 56fa46e7a41c
 ---> e6edc30cc92b
Step 12/15 : ENV RUST_CONFIGURE_ARGS       --enable-extended       --enable-profiler       --arm-linux-androideabi-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --armv7-linux-androideabi-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --thumbv7neon-linux-androideabi-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --i686-linux-android-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --aarch64-linux-android-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --x86_64-linux-android-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --disable-docs
Removing intermediate container bfcf52c35922
 ---> e7b0f5f00871
Step 13/15 : ENV SCRIPT python3 ../x.py dist --host='' --target $TARGETS
 ---> Running in 723e477ca23b
