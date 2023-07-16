
[00:02:07] ======================================================================

[00:02:07] FAIL: program_config (bootstrap.RustBuild)

[00:02:07] Doctest: bootstrap.RustBuild.program_config

[00:02:07] ----------------------------------------------------------------------

[00:02:07] Traceback (most recent call last):

[00:02:07]   File "/usr/lib/python2.7/doctest.py", line 2226, in runTest

[00:02:07]     raise self.failureException(self.format_failure(new.getvalue()))

[00:02:07] AssertionError: Failed doctest test for bootstrap.RustBuild.program_config

[00:02:07]   File "/checkout/src/bootstrap/bootstrap.py", line 519, in program_config

[00:02:07] 

[00:02:07] ----------------------------------------------------------------------

[00:02:07] File "/checkout/src/bootstrap/bootstrap.py", line 527, in bootstrap.RustBuild.program_config

[00:02:07] Failed example:

[00:02:07]     cargo_path.rstrip(".exe") == os.path.join("/tmp/rust",

[00:02:07]     "bin", "cargo")

[00:02:07] Expected:

[00:02:07]     True

[00:02:07] Got:

[00:02:07]     False
