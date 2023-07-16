
C:\projects\rust\build\tmp\dist\rust.wxs(144) : warning LGHT1076 : ICE61: This product should remove only older versions of itself. The Maximum version is not less than the current product. (1.34.65535 1.34.0.0)
[TIMING] Extended { stage: 2, host: "i686-pc-windows-gnu", target: "i686-pc-windows-gnu" } -- 1101.683
Build completed successfully in 3:19:57
Command exited with code 259
set PATH=%PATH%;"C:\Program Files (x86)\Windows Kits\10\Debuggers\X64"
if exist %LOCALAPPDATA%\CrashDumps for %%f in (%LOCALAPPDATA%\CrashDumps\*) do cdb -c "k;q" -G -z "%%f"
