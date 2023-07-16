plain
[00:22:40] [ 52%] Building PPCGenCallingConv.inc...
[00:22:40] [ 52%] Building PPCGenDAGISel.inc...
[00:22:41] [ 52%] Building MipsGenMCCodeEmitter.inc...

Broadcast message from root@travis-job-d9de6239-e4b5-4669-b407-ef0813043ba8
 (unknown) at 6:56 ...
The system is going down for power off NOW!
[00:22:41] 
[00:22:41] Session terminated, terminating shell...make[3]: *** Deleting file 'lib/Target/Mips/MipsGenMCCodeEmitter.inc.tmp'
[00:22:41] make[3]: *** Deleting file 'lib/Target/X86/X86GenDAGISel.inc.tmp'
[00:22:41] make[3]: *** Deleting file 'lib/Target/AArch64/AArch64GenSubtargetInfo.inc.tmp'
[00:22:41] make[3]: *** Deleting file 'lib/Target/PowerPC/PPCGenDAGISel.inc.tmp'
[00:22:41] make[3]: *** [lib/Target/PowerPC/PPCGenDAGISel.inc.tmp] Terminated
[00:22:41] make[3]: *** [lib/Target/AArch64/AArch64GenSubtargetInfo.inc.tmp] Terminated
[00:22:41] make[2]: *** [lib/Target/PowerPC/CMakeFiles/PowerPCCommonTableGen.dir/all] Terminated
[00:22:41] make[1]: *** [all] Terminated
[00:22:41] lib/Target/AArch64/CMakeFiles/AArch64CommonTableGen.dir/build.make:928: recipe for target 'lib/Target/AArch64/AArch64GenSubtargetInfo.inc.tmp' failed
[00:22:41] lib/Target/PowerPC/CMakeFiles/PowerPCCommonTableGen.dir/build.make:341: recipe for target 'lib/Target/PowerPC/PPCGenDAGISel.inc.tmp' failed
[00:22:41] CMakeFiles/Makefile2:9727: recipe for target 'lib/Target/PowerPC/CMakeFiles/PowerPCCommonTableGen.dir/all' failed
[00:22:41] Makefile:149: recipe for target 'all' failed
[00:22:41] lib/Target/X86/CMakeFiles/X86CommonTableGen.dir/build.make:485: recipe for target 'lib/Target/X86/X86GenDAGISel.inc.tmp' failed
[00:22:41] lib/Target/Mips/CMakeFiles/MipsCommonTableGen.dir/build.make:714: recipe for target 'lib/Target/Mips/MipsGenMCCodeEmitter.inc.tmp' failed
[00:22:41] make: *** wait: No child processes.  Stop.
[00:22:41] make: *** Waiting for unfinished jobs....
[00:22:41] make: *** wait: No child processes.  Stop.
[00:22:41] make[3]: *** [lib/Target/X86/X86GenDAGISel.inc.tmp] Terminated
[00:22:41] make[3]: *** [lib/Target/Mips/MipsGenMCCodeEmitter.inc.tmp] Terminated
[00:22:41]  ...terminated.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 143.
travis_time:start:000beb6f
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
