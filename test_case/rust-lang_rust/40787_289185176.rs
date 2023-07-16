
rust-lldb test                                       11:18:03 
(lldb) command source -s 0 '/tmp/rust-lldb-commands.L9YKLP'
Executing commands in '/tmp/rust-lldb-commands.L9YKLP'.
(lldb) command script import "/usr/local/Cellar/rust/1.15.1_1/lib/rustlib/etc/lldb_rust_formatters.py"
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/Applications/Xcode.app/Contents/SharedFrameworks/LLDB.framework/Resources/Python/lldb/__init__.py", line 98, in <module>
    import six
ImportError: No module named six
Traceback (most recent call last):
  File "<string>", line 1, in <module>
NameError: name 'run_one_line' is not defined
Traceback (most recent call last):
  File "<string>", line 1, in <module>
.......

