
[00:02:13] ======================================================================
[00:02:13] FAIL: program_config (bootstrap.RustBuild)
[00:02:13] Doctest: bootstrap.RustBuild.program_config
[00:02:13] ----------------------------------------------------------------------
[00:02:13] Traceback (most recent call last):
[00:02:13]   File "/usr/lib/python2.7/doctest.py", line 2226, in runTest
[00:02:13]     raise self.failureException(self.format_failure(new.getvalue()))
[00:02:13] AssertionError: Failed doctest test for bootstrap.RustBuild.program_config
[00:02:13]   File "/checkout/src/bootstrap/bootstrap.py", line 519, in program_config
[00:02:13] 
[00:02:13] ----------------------------------------------------------------------
[00:02:13] File "/checkout/src/bootstrap/bootstrap.py", line 527, in bootstrap.RustBuild.program_config
[00:02:13] Failed example:
[00:02:13]     cargo_path.rstrip(".exe") == os.path.join("/tmp/rust",
[00:02:13]     "bin", "cargo")
[00:02:13] Expected:
[00:02:13]     True
[00:02:13] Got:
[00:02:13]     False
