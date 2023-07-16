plain
DirectMap1G:    55574528 kB
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s


failed to execute command: "musl-g++" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-Wl,-melf_i386" "-static" "-Wa,-mrelax-relocations=no" "-print-file-name=libstdc++.a"
error: No such file or directory (os error 2)

failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target i586-unknown-linux-gnu,i686-unknown-linux-musl
Build completed unsuccessfully in 0:00:00
