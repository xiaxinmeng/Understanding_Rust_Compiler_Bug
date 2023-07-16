plain
test workspaces::ws_warn_unused ... ok

failures:

---- registry::update_multiple_packages_http stdout ----
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe fetch -Zhttp-registry`
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe update -pa -pb -Zhttp-registry`
running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe update -pb -pc -Zhttp-registry`
thread 'registry::update_multiple_packages_http' panicked at '
test failed running `D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\release\cargo.exe update -pb -pc -Zhttp-registry`
error: process exited with code 101 (expected 0)

--- stderr
--- stderr
    Updating `dummy-registry` index
error: failed to get `b` as a dependency of package `foo v0.5.0 (D:\a\rust\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\tmp\cit\t1866\foo)`
Caused by:
Caused by:
  failed to query replaced source registry `crates-io`
Caused by:
Caused by:
  download of 1/b failed
Caused by:
Caused by:
  [1] Unsupported protocol (Received HTTP/0.9 when not allowed)
', src\tools\cargo\tests\testsuite\registry.rs:1634:10

failures:
    registry::update_multiple_packages_http

