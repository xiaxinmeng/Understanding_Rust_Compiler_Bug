
thread '[debuginfo-lldb] debuginfo/self-in-generic-default-method.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2617:8

---- [debuginfo-lldb] debuginfo/shadowed-argument.rs stdout ----
	NOTE: compiletest thinks it is using LLDB version 900

error: Error while running LLDB
status: exit code: 1
command: "/usr/bin/python" "/Users/k0pernicus/Code/rust/src/etc/lldb_batchmode.py" "/Users/k0pernicus/Code/rust/build/x86_64-apple-darwin/test/debuginfo/shadowed-argument.stage1-x86_64-apple-darwin" "/Users/k0pernicus/Code/rust/build/x86_64-apple-darwin/test/debuginfo/shadowed-argument.debugger.script"
stdout:
------------------------------------------
LLDB batch-mode script
----------------------
Debugger commands script is '/Users/k0pernicus/Code/rust/build/x86_64-apple-darwin/test/debuginfo/shadowed-argument.debugger.script'.
Target executable is '/Users/k0pernicus/Code/rust/build/x86_64-apple-darwin/test/debuginfo/shadowed-argument.stage1-x86_64-apple-darwin'.
Current working directory is '/Users/k0pernicus/Code/rust'
Creating a target for '/Users/k0pernicus/Code/rust/build/x86_64-apple-darwin/test/debuginfo/shadowed-argument.stage1-x86_64-apple-darwin'
settings set auto-confirm true

------------------------------------------
stderr:
------------------------------------------
Traceback (most recent call last):
  File "/Users/k0pernicus/Code/rust/src/etc/lldb_batchmode.py", line 211, in <module>
    execute_command(command_interpreter, command)
  File "/Users/k0pernicus/Code/rust/src/etc/lldb_batchmode.py", line 84, in execute_command
    print(normalize_whitespace(res.GetOutput()), end='\n')
  File "/Users/k0pernicus/Code/rust/src/etc/lldb_batchmode.py", line 48, in normalize_whitespace
    return re.sub("\s+", " ", s)
  File "/System/Library/Frameworks/Python.framework/Versions/2.7/lib/python2.7/re.py", line 155, in sub
    return _compile(pattern, flags).sub(repl, string, count)
TypeError: expected string or buffer
