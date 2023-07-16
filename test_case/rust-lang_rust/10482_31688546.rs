
clang version 3.3 (tags/RELEASE_33/final)
Target: armv7l-unknown-linux-gnueabihf
Thread model: posix
 "/usr/local/bin/clang" -cc1 -triple armv7-unknown-linux-gnueabihf -S -disable-free -disable-llvm-verifier -main-file-name test.bc -mrelocation-model static -mdisable-fp-elim -fmath-errno -mconstructor-aliases -fuse-init-array -target-abi aapcs-linux -target-cpu cortex-a8 -mfloat-abi hard -target-linker-version 2.22.90.20120924 -v -resource-dir /usr/local/bin/../lib/clang/3.3 -fno-dwarf-directory-asm -fdebug-compilation-dir /home/picuntu/projects/rust-float/rs -ferror-limit 19 -fmessage-length 194 -mstackrealign -fno-signed-char -fobjc-runtime=gcc -fobjc-default-synthesize-properties -fdiagnostics-show-option -fcolor-diagnostics -backend-option -vectorize-loops -o /tmp/test-wzfuLc.s -x ir test.bc
clang -cc1 version 3.3 based upon LLVM 3.3 default target armv7l-unknown-linux-gnueabihf
 "/usr/bin/as" -mfloat-abi=hard -o /tmp/test-Exwdqf.o /tmp/test-wzfuLc.s
 "/usr/bin/ld" -z relro -X --hash-style=gnu --build-id --eh-frame-hdr -m armelf_linux_eabi -dynamic-linker /lib/ld-linux-armhf.so.3 -o a.out /usr/lib/gcc/arm-linux-gnueabihf/4.7/../../../arm-linux-gnueabihf/crt1.o /usr/lib/gcc/arm-linux-gnueabihf/4.7/../../../arm-linux-gnueabihf/crti.o /usr/lib/gcc/arm-linux-gnueabihf/4.7/crtbegin.o -L/usr/local/lib -L/usr/lib/gcc/arm-linux-gnueabihf/4.7 -L/usr/lib/gcc/arm-linux-gnueabihf/4.7/../../../arm-linux-gnueabihf -L/lib/arm-linux-gnueabihf -L/usr/lib/arm-linux-gnueabihf -L/usr/lib/gcc/arm-linux-gnueabihf/4.7/../../.. -L/lib -L/usr/lib /tmp/test-Exwdqf.o -lm -lstd-04ff901e-0.9-pre -lgreen-3b3a1962-0.9-pre -lrustuv-7945354c-0.9-pre -rpath=/usr/local/lib -lgcc --as-needed -lgcc_s --no-as-needed -lc -lgcc --as-needed -lgcc_s --no-as-needed /usr/lib/gcc/arm-linux-gnueabihf/4.7/crtend.o /usr/lib/gcc/arm-linux-gnueabihf/4.7/../../../arm-linux-gnueabihf/crtn.o
/usr/bin/ld: /tmp/test-Exwdqf.o(.text+0x11dc): unresolvable R_ARM_TLS_LE32 relocation against symbol `_ZN2rt9local_ptr8compiled10RT_TLS_PTR19ha35df66a83b7e2ffaP8v0.9.preE'
/usr/bin/ld: final link failed: Nonrepresentable section on output
clang: error: linker command failed with exit code 1 (use -v to see invocation)
