plain
FAIL: get_string (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.get_string
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/usr/lib/python3.8/doctest.py", line 2204, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.get_string
  File "/checkout/src/bootstrap/bootstrap.py", line 954, in get_string
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 961, in bootstrap.RustBuild.get_string
Failed example:
    RustBuild.get_string("  "devel"  ")
Exception raised:
    Traceback (most recent call last):
      File "/usr/lib/python3.8/doctest.py", line 1336, in __run
        exec(compile(example.source, filename, "single",
      File "<doctest bootstrap.RustBuild.get_string[2]>", line 1
        RustBuild.get_string("  "devel"  ")
    SyntaxError: invalid syntax


----------------------------------------------------------------------
