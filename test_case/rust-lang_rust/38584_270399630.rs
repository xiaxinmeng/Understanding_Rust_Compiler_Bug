bash
if /I "%VSCMD_ARG_HOST_ARCH%" == "x86" (
    set __VCVARS_HOST_DIR=\HostX86
    set __VCVARS_HOST_NATIVEDIR=\x86
)
if /I "%VSCMD_ARG_HOST_ARCH%" == "x64" (
    set __VCVARS_HOST_DIR=\HostX64
    set __VCVARS_HOST_NATIVEDIR=\x64
)
if /I "%VSCMD_ARG_HOST_ARCH%" == "arm" (
    set __VCVARS_HOST_DIR=\HostARM
    set __VCVARS_HOST_NATIVEDIR=\arm
)
