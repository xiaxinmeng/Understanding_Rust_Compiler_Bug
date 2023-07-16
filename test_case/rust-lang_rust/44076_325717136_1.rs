
[00:02:16] test_stamp_path_does_not_exists (__main__.ProgramOutOfDate)

[00:02:16] Return True when the stamp file does not exists ... ok

[00:02:16] 

[00:02:16] ======================================================================

[00:02:16] FAIL: program_config (bootstrap.RustBuild)

[00:02:16] Doctest: bootstrap.RustBuild.program_config

[00:02:16] ----------------------------------------------------------------------

[00:02:16] Traceback (most recent call last):

[00:02:16]   File "/usr/lib/python2.7/doctest.py", line 2226, in runTest

[00:02:16]     raise self.failureException(self.format_failure(new.getvalue()))

[00:02:16] AssertionError: Failed doctest test for bootstrap.RustBuild.program_config

[00:02:16]   File "/checkout/src/bootstrap/bootstrap.py", line 519, in program_config

[00:02:16] 

[00:02:16] ----------------------------------------------------------------------

[00:02:16] File "/checkout/src/bootstrap/bootstrap.py", line 527, in bootstrap.RustBuild.program_config

[00:02:16] Failed example:

[00:02:16]     cargo_path.rstrip(".exe") == os.path.join("/tmp/rust",

[00:02:16]     "bin", "cargo")

[00:02:16] Expected:

[00:02:16]     True

[00:02:16] Got:

[00:02:16]     False
