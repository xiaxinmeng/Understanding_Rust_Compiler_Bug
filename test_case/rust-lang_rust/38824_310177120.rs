
llc -O0 -debug-only isel test.ll 2> isel.log  
llc -O0 -print-before-all -print-after-all -print-machineinstrs test.ll 2> passes.log
