
λ build\x86_64-pc-windows-msvc\stage1\bin\rustc -C target-cpu=help
Available CPUs for this target:
    native         - Select the CPU of the current host (currently haswell).
    amdfam10       - Select the amdfam10 processor.
    athlon         - Select the athlon processor.
<snip>
λ build\x86_64-pc-windows-msvc\stage1\bin\rustc -C target-cpu=help --target arm-unknown-linux-gnueabi
Available CPUs for this target:
    arm1020e      - Select the arm1020e processor.
    arm1020t      - Select the arm1020t processor.
<snip>
