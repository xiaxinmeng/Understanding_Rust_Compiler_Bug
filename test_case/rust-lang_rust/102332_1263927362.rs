plain
 ---> 8357f533af4e
Step 11/15 : ENV TARGETS=$TARGETS,x86_64-linux-android
 ---> Using cache
 ---> e6edc30cc92b
Step 12/15 : ENV RUST_CONFIGURE_ARGS       --enable-extended       --enable-profiler       --arm-linux-androideabi-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --armv7-linux-androideabi-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --thumbv7neon-linux-androideabi-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --i686-linux-android-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --aarch64-linux-android-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --x86_64-linux-android-ndk=/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/       --disable-docs
 ---> e7b0f5f00871
Step 13/15 : ENV SCRIPT python3 ../x.py dist --host='' --target $TARGETS
 ---> Using cache
 ---> f898ebb56b07
---
  IMAGE: dist-android
  AWS_ACCESS_KEY_ID: 
  AWS_SECRET_ACCESS_KEY: 
##[endgroup]
cp: cannot stat 'obj/build/metrics.json': No such file or directory
##[error]Process completed with exit code 1.
