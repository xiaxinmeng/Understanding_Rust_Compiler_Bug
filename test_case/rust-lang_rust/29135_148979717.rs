
doug@Tio /c/projects/rust-all/rust-extern/target/debug
$ /c/Python27/python.exe
Python 2.7.5 (default, May 15 2013, 22:43:36) [MSC v.1500 32 bit (Intel)] on win32
Type "help", "copyright", "credits" or "license" for more information.
>>> import ctypes
>>> ex = ctypes.CDLL("extern.dll")
>>> ex.rs_str()
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
  File "c:\Python27\lib\ctypes\__init__.py", line 378, in __getattr__
    func = self.__getitem__(name)
  File "c:\Python27\lib\ctypes\__init__.py", line 383, in __getitem__
    func = self._FuncPtr((name_or_ordinal, self))
AttributeError: function 'rs_str' not found
