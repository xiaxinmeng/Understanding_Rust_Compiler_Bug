plain
[RUSTC-TIMING] smallvec test:false 0.798
error: failed to run custom build command for `proc-macro-hack v0.5.19`

Caused by:
  process didn't exit successfully: `C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\proc-macro-hack-a2f4a8c7be510b92\build-script-build` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
[RUSTC-TIMING] build_script_build test:false 0.934
error: failed to run custom build command for `proc-macro2 v1.0.53`

Caused by:
Caused by:
  process didn't exit successfully: `C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\proc-macro2-77eca587f5d44bdd\build-script-build` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
  --- stdout
  cargo:rerun-if-changed=build.rs

Caused by:
Caused by:
  process didn't exit successfully: `C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\quote-b07cc84d813763f2\build-script-build` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
  --- stdout
  cargo:rerun-if-changed=build.rs
error: failed to run custom build command for `winapi v0.3.9`
Caused by:
Caused by:
  process didn't exit successfully: `C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\winapi-c7da96fbeff9efcc\build-script-build` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
  --- stdout
  cargo:rerun-if-changed=build.rs
  cargo:rerun-if-env-changed=WINAPI_NO_BUNDLED_LIBRARIES
  cargo:rerun-if-env-changed=WINAPI_STATIC_NOBUNDLE
  cargo:rustc-cfg=feature="cfg"
  cargo:rustc-cfg=feature="windef"
  cargo:rustc-cfg=feature="reason"
  cargo:rustc-cfg=feature="excpt"
  cargo:rustc-cfg=feature="cfgmgr32"
  cargo:rustc-cfg=feature="devpropdef"
  cargo:rustc-cfg=feature="ktmtypes"
  cargo:rustc-cfg=feature="wincontypes"
  cargo:rustc-cfg=feature="wingdi"
  cargo:rustc-cfg=feature="basetsd"
  cargo:rustc-cfg=feature="guiddef"
  cargo:rustc-cfg=feature="wtypesbase"
  cargo:rustc-cfg=feature="winreg"
  cargo:rustc-cfg=feature="ntdef"
  cargo:rustc-cfg=feature="vcruntime"
  cargo:rustc-cfg=feature="vadefs"
  cargo:rustc-cfg=feature="rpcndr"
  cargo:rustc-link-lib=dylib=advapi32
  cargo:rustc-link-lib=dylib=cfgmgr32
  cargo:rustc-link-lib=dylib=gdi32
  cargo:rustc-link-lib=dylib=kernel32
  cargo:rustc-link-lib=dylib=msimg32
  cargo:rustc-link-lib=dylib=opengl32
  cargo:rustc-link-lib=dylib=user32
  cargo:rustc-link-lib=dylib=winspool
error: failed to run custom build command for `syn v1.0.102`
Caused by:
Caused by:
  process didn't exit successfully: `C:\a\rust\rust\build\x86_64-pc-windows-msvc\stage1-rustc\release\build\syn-5b24b8bfa5cdb993\build-script-build` (exit code: 0xc0000409, STATUS_STACK_BUFFER_OVERRUN)
