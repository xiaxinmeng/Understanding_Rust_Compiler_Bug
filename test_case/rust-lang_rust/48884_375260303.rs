
 INFO 2018-03-22T10:46:46Z: rustc_trans::back::link: preparing CrateTypeExecutable to "/Users/berkus/Hobby/Metta/vesper/target/aarch64-vesper-metta/release/deps/vesper-b63e9d46780781be"
 INFO 2018-03-22T10:46:46Z: rustc_trans::back::link: "aarch64-unknown-linux-musl-ld" "-nodefaultlibs" "-nostdlib" "-L" "/Users/berkus/.xargo/lib/rustlib/aarch64-vesper-metta/lib" "/Users/berkus/Hobby/Metta/vesper/target/aarch64-vesper-metta/release/deps/vesper-b63e9d46780781be.vesper2.rcgu.o" "-o" "/Users/berkus/Hobby/Metta/vesper/target/aarch64-vesper-metta/release/deps/vesper-b63e9d46780781be" "--gc-sections" "-no-pie" "-z,relro,-z,now" "-O1" "-L" "/Users/berkus/Hobby/Metta/vesper/target/aarch64-vesper-metta/release/deps" "-L" "/Users/berkus/Hobby/Metta/vesper/target/release/deps" "-L" "/Users/berkus/Hobby/Metta/vesper/target/aarch64-vesper-metta/release/build/vesper-6d63ef12987ec0c8/out/src/boot" "-L" "/Users/berkus/.xargo/lib/rustlib/aarch64-vesper-metta/lib" "-Bstatic" "--whole-archive" "-l" "aarch64" "--no-whole-archive" "/Users/berkus/Hobby/Metta/vesper/target/aarch64-vesper-metta/release/deps/librlibc-ffb0a67b04c0940a.rlib" "-static" "-Tlinker/aarch64.ld" "-Bdynamic"
 INFO 2018-03-22T10:46:46Z: rustc_trans::back::link: linker stderr:
aarch64-unknown-linux-musl-ld: warning: -z ,relro,-z,now ignored.

 INFO 2018-03-22T10:46:46Z: rustc_trans::back::link: linker stdout:

    Finished release [optimized + debuginfo] target(s) in 1.69 secs
     Running `/Users/berkus/Hobby/Metta/vesper/.cargo/runscript.sh target/aarch64-vesper-metta/release/vesper`
aarch64-unknown-linux-musl-objcopy: 'target/aarch64-vesper-metta/release/vesper': No such file
