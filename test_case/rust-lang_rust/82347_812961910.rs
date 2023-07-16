
$ ./x.py check --target x86_64-pc-windows-msvc
Updating only changed submodules
Submodules updated in 0.02 seconds
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
thread 'main' panicked at '
cmake does not support Visual Studio generators.

This is likely due to it being an msys/cygwin build of cmake,
rather than the required windows version, built using MinGW
or Visual Studio.
