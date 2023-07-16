sh
rm 32
rustc -C opt-level=0 -g --emit=llvm-ir main.rs -o x32.ll  --target x86_64-unknown-linux-gnux32
LLVM_HOME=/usr/lib/llvm/10/
$LLVM_HOME/bin/llvm-as x32.ll
$LLVM_HOME/bin/opt -load ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/libtest-1b777edb18de95a1.so -o x32.bc x32.bc
$LLVM_HOME/bin/llc -filetype=obj x32.bc
$LLVM_HOME/bin/clang -g -o 32 -mx32 x32.o -L ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/ -lstd-f6f0a67eb2dc7ef6
LD_LIBRARY_PATH=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/ ./32 

rm 32
rustc main.rs -C opt-level=0 -o 32 -g --emit=llvm-ir -O -C no-prepopulate-passes -C codegen-units=1 --target x86_64-unknown-linux-gnux32 
$LLVM_HOME/bin/llvm-as x32.ll
$LLVM_HOME/bin/opt -load ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/libtest-1b777edb18de95a1.so -o x32.bc x32.bc
$LLVM_HOME/bin/llc -filetype=obj x32.bc
$LLVM_HOME/bin/clang -g -o 32 -mx32 x32.o -L ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/ -lstd-f6f0a67eb2dc7ef6
LD_LIBRARY_PATH=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/ ./32 

rm 32
rustc -C opt-level=0 -g  --emit=llvm-bc main.rs -o x32.bc --target x86_64-unknown-linux-gnux32
$LLVM_HOME/bin/llc -filetype=obj x32.bc
$LLVM_HOME/bin/clang -g -o 32 -mx32 x32.o -L ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/ -lstd-f6f0a67eb2dc7ef6
LD_LIBRARY_PATH=~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnux32/lib/ ./32 
