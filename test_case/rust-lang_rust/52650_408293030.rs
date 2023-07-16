plain
[01:46:26] [ 39%] Building MipsGenGlobalISel.inc...
[01:46:26] [ 39%] Building AArch64GenInstrInfo.inc...
[01:46:26] [ 39%] Building MipsGenInstrInfo.inc...

Broadcast message from root@travis-job-af0c6e06-6d69-43e8-bed8-f2f061e6472c
 (unknown) at 2:31 ...
The system is going down for power off NOW!
[01:46:27] 
[01:46:27] Session terminated, terminating shell...make[2]: make[2]: *** Deleting file `lib/Target/Mips/MipsGenGlobalISel.inc.tmp'
[01:46:27] *** Deleting file `lib/Target/X86/X86GenDAGISel.inc.tmp'
[01:46:27] make[2]: *** [lib/Target/Mips/MipsGenInstrInfo.inc.tmp] Terminatedmake[2]: *** [lib/Target/X86/X86GenDAGISel.inc.tmp] Terminated
[01:46:27] 
[01:46:27] make[1]: *** [lib/Target/X86/CMakeFiles/X86CommonTableGen.dir/all] Terminated
[01:46:27] make[1]: *** [lib/Target/Mips/CMakeFiles/MipsCommonTableGen.dir/all] Terminated
[01:46:27] make[2]: *** [lib/Target/AArch64/AArch64GenInstrInfo.inc.tmp] Terminated
[01:46:27] make[1]: *** [lib/Target/AArch64/CMakeFiles/AArch64CommonTableGen.dir/all] Terminated
[01:46:27]  ...terminated.
[01:46:27] make: *** [all] Terminated

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:1b968fa0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
