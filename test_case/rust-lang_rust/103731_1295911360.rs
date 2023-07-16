plain
Detected LLVM as non-available: running in CI and modified LLVM in this change
thread 'main' panicked at '
cmake does not support Visual Studio generators.

This is likely due to it being an msys/cygwin build of cmake,
rather than the required windows version, built using MinGW
or Visual Studio.
If you are building under msys2 try installing the mingw-w64-x86_64-cmake
package instead of cmake:


$ pacman -R cmake && pacman -S mingw-w64-x86_64-cmake
', sanity.rs:221:17
Build completed unsuccessfully in 0:00:00
