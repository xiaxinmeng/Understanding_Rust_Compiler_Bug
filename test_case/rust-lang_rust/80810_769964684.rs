 
[INFO ]  Installing pass-2 core C gcc compiler
[ERROR]    checking for suffix of executables... configure: error: in `/tmp/build/.build/s390x-ibm-linux-gnu/build/build-cc-gcc-core-pass-2/s390x-ibm-linux-gnu/libgcc':
[ERROR]    configure: error: cannot compute suffix of object files: cannot compile
[ERROR]    checking whether it is safe to define __EXTENSIONS__... make[1]: *** [configure-target-libgcc] Error 1
[ERROR]   
[ERROR]  >>  
[ERROR]  >>  Build failed in step 'Installing pass-2 core C gcc compiler'
[ERROR]  >>        called in step '(top-level)'
[ERROR]  >>  
[ERROR]  >>  Error happened in: CT_DoExecLog[scripts/functions@257]
[ERROR]  >>        called from: do_gcc_core_backend[scripts/build/cc/100-gcc.sh@537]
[ERROR]  >>        called from: do_gcc_core_pass_2[scripts/build/cc/100-gcc.sh@160]
[ERROR]  >>        called from: do_cc_core_pass_2[scripts/build/cc.sh@42]
[ERROR]  >>        called from: main[scripts/crosstool-NG.sh@646]
[ERROR]  >>  
[ERROR]  >>  For more info on this error, look at the file: 'build.log'
[ERROR]  >>  There is a list of known issues, some with workarounds, in: 
[ERROR]  >>      '/usr/local/share/doc/crosstool-ng//B - Known issues.txt'
