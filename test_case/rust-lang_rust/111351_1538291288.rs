plain
Build completed successfully in 0:03:36
##[group]Building bootstrap
    Finished dev [unoptimized] target(s) in 0.04s
##[endgroup]
Error: the automatic detection of the C compiler for target i686-pc-windows-msvc returned the same compiler as the currnet host target:
This is likely wrong. Please specify the correct compiler for that target, either with environment variables:

    CC_i686_pc_windows_msvc=path/to/target
    CXX_i686_pc_windows_msvc=path/to/target
...or in config.toml:


    [target."i686-pc-windows-msvc"]
    cc = "path/to/target"
    cxx = "path/to/target"
Build completed unsuccessfully in 0:00:00
