plain
FAIL: bootstrap_binary (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bootstrap_binary
----------------------------------------------------------------------
Traceback (most recent call last):
  File "C:\hostedtoolcache\windows\Python\3.10.4\x64\lib\doctest.py", line 2217, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.bootstrap_binary
  File "D:\a\rust\rust\src\bootstrap\bootstrap.py", line 983, in bootstrap_binary
----------------------------------------------------------------------
----------------------------------------------------------------------
File "D:\a\rust\rust\src\bootstrap\bootstrap.py", line 988, in bootstrap.RustBuild.bootstrap_binary
Failed example:
    rb.bootstrap_binary() == os.path.join("build", "bootstrap",
    "debug", "rustbuild-binary-dispatch-shim")
    True
Got:
    False

