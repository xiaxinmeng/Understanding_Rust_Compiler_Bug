console
root@e8db2cec4f95:/# python3 -c "import os; os.environ['a'] = 'abc\0xyz'"
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/usr/lib/python3.8/os.py", line 681, in __setitem__
    self.putenv(key, value)
ValueError: embedded null byte
root@e8db2cec4f95:/# python3 -c "import os; os.environ['a\0b'] = 'e'"
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/usr/lib/python3.8/os.py", line 681, in __setitem__
    self.putenv(key, value)
ValueError: embedded null byte
root@e8db2cec4f95:/# python3 -c "import os; os.environ['a=b'] = 'e'"
Traceback (most recent call last):
  File "<string>", line 1, in <module>
  File "/usr/lib/python3.8/os.py", line 681, in __setitem__
    self.putenv(key, value)
ValueError: illegal environment variable name
root@e8db2cec4f95:/# python3 -c "import os; print(os.getenv('a=b'))"
None
root@e8db2cec4f95:/# python3 -c "import os; print(os.getenv('a\0b'))"
None
root@e8db2cec4f95:/# python3 -c "import os; print(os.getenv('a=b'))"
None
root@e8db2cec4f95:/# python3 -c "import os; print(os.environ.get('a\0b'))"
None
root@e8db2cec4f95:/# python3 -c "import os; print(os.environ.get('a=b'))"
None

root@e8db2cec4f95:/# ruby -e 'puts ENV["x\0y"]'
Traceback (most recent call last):
	1: from -e:1:in `<main>'
-e:1:in `[]': bad environment variable name: contains null byte (ArgumentError)
root@e8db2cec4f95:/# ruby -e 'puts ENV["x=y"]'

root@e8db2cec4f95:/# ruby -e 'ENV["x\0y"] = "a"'
Traceback (most recent call last):
	1: from -e:1:in `<main>'
-e:1:in `[]=': bad environment variable name: contains null byte (ArgumentError)
root@e8db2cec4f95:/# ruby -e 'ENV["x=y"] = "a"'
Traceback (most recent call last):
	1: from -e:1:in `<main>'
-e:1:in `[]=': Invalid argument - setenv(x=y) (Errno::EINVAL)
root@e8db2cec4f95:/# ruby -e 'ENV["xy"] = "a\0z"'
Traceback (most recent call last):
	1: from -e:1:in `<main>'
-e:1:in `[]=': bad environment variable value: contains null byte (ArgumentError)
