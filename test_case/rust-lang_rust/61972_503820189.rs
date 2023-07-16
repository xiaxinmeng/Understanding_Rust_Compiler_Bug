rust
failures:
---- [debuginfo-gdb+lldb] debuginfo\basic-types-globals-metadata.rs stdout ----
NOTE: compiletest thinks it is using LLDB version 800
NOTE: compiletest thinks it is using LLDB without native rust support
error: Error while running LLDB
status: exit code: 1
command: "C:\\Python27\\python2.7" "C:\\projects\\rust\\src/etc/lldb_batchmode.py" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\test\\debuginfo\\basic-types-globals-metadata\\a.exe" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\test\\debuginfo\\basic-types-globals-metadata\\basic-types-globals-metadata.debugger.script"
stdout:
------------------------------------------
------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
  File "C:\projects\rust\src/etc/lldb_batchmode.py", line 17, in <module>
    import lldb
ImportError: No module named lldb
------------------------------------------
