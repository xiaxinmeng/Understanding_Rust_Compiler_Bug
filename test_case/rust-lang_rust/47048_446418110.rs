shell
::  ## * workaround for rust-lang/rust#47048 / rust-lang/rust#53454  ## !maint: remove when resolved
::  # ** ref: <https://github.com/rust-lang/rust/issues/47048>, <https://github.com/rust-lang/rust/issues/53454>
::  # ** egs: <https://github.com/pkgw/tectonic/commit/29686db533d8732d7d97fc94270ed33b77f29295>, <https://github.com/rukai/PF_Sandbox/blob/e842613cf9ff102dfb3fbd87381319e6e6dfe3ae/appveyor.yml>
( setlocal
rem :: update gcc libraries
for /f %%G in ('where gcc') do (
    set "gcc_arch=i686"
    set "ERRORLEVEL="
    "%%G" --version | findstr /C:"i686" >NUL 2>&1
    if ERRORLEVEL 1 ( set "gcc_arch=x86_64" )
    call set "d=%%gcc_arch%%-pc-windows-gnu"
    cd "%USERPROFILE%\.rustup\toolchains"
    for /f %%H in ('dir /b/s/a:d %%d%% ^| findstr pc-windows-msvc') do (
        call copy "%%G\..\..\%%gcc_arch%%-w64-mingw32\lib\*.o" "%%H\lib" >NUL
    )
    call echo gcc libraries copied ^(%%gcc_arch%%-type; workaround for rust-lang/rust#47048 + rust-lang/rust#53454^)
)
endlocal )
