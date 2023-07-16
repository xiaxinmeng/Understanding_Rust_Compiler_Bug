
-> % cat > hello.rs
fn main() {
    info!("info");
    debug!("debug");
    println!("Hello World!");
}

-> % rustc --target=arm-linux-androideabi --android-cross-path=$ANDROID_STANDALONE hello.rs --cfg debug

-> % android create avd --force -n test -t android-18 --abi armeabi-v7a
Android 4.3 is a basic Android platform.
Do you wish to create a custom hardware profile [no]
Created AVD 'test' based on Android 4.3, ARM (armeabi-v7a) processor,
with the following hardware config:
hw.lcd.density=240
vm.heapSize=48
hw.ramSize=512

-> % emulator -avd test -no-skin -no-audio -no-window &
[1] 6621

-> % adb remount
remount succeeded

-> % adb push $ANDROID_STANDALONE/arm-linux-androideabi/lib/libgnustl_shared.so /data/tmp/
1015 KB/s (1211344 bytes in 1.165s)

-> % adb push hello /data/tmp/hello
136 KB/s (8000 bytes in 0.057s)

-> % adb push /usr/local/lib/rustc/arm-linux-androideabi/lib/libstd-6c65cf4b443341b1-0.9-pre.so /data/tmp/
905 KB/s (6223580 bytes in 6.715s)

-> % adb push /usr/local/lib/rustc/arm-linux-androideabi/lib/librustrt.so /data/tmp/
1001 KB/s (3947716 bytes in 3.849s)

-> % adb shell
root@generic:/ # cd /data/tmp
root@generic:/data/tmp # ls
hello
libgnustl_shared.so
librustrt.so
libstd-6c65cf4b443341b1-0.9-pre.so
root@generic:/data/tmp # LD_LIBRARY_PATH=. ./hello
Hello World!
root@generic:/data/tmp # LD_LIBRARY_PATH=. RUST_LOG=::help ./hello

Crate log map:

 hello
 std::io
 std::condition
 std::ptr
 std::os
1|root@generic:/data/tmp # LD_LIBRARY_PATH=. RUST_LOG=hello ./hello
"info"
"debug"
Hello World!
root@generic:/data/tmp # LD_LIBRARY_PATH=. RUST_LOG=hello=info ./hello
"info"
Hello World!
root@generic:/data/tmp # LD_LIBRARY_PATH=. RUST_LOG=hello=debug ./hello
"info"
"debug"
Hello World!
