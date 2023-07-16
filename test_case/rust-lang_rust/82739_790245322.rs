plain
Successfully built ef22e5e5e337
Successfully tagged rust-ci:latest
Built container sha256:ef22e5e5e3378d55dc0a9bd4d40d01d9a2b2688e441fd65511de9a02582f2198
Uploading finished image to https://ci-caches.rust-lang.org/docker/ee35d0862b1e6469e5cb6dc20ac8af60a531c98fcdb6b4a6a736bd34675adebd3da7240b933e3b5e0a5b665346342a96ddae1e6a65e411850fff18b703cd20b8
upload failed: - to s3://rust-lang-ci-sccache2/docker/ee35d0862b1e6469e5cb6dc20ac8af60a531c98fcdb6b4a6a736bd34675adebd3da7240b933e3b5e0a5b665346342a96ddae1e6a65e411850fff18b703cd20b8 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-9]
---
FAIL: bin_root (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.bin_root
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/usr/lib/python3.6/doctest.py", line 2199, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.bin_root
  File "/checkout/src/bootstrap/bootstrap.py", line 709, in bin_root
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 714, in bootstrap.RustBuild.bin_root
Failed example:
    rb.bin_root() == os.path.join("build", "stage0")
Exception raised:
    Traceback (most recent call last):
      File "/usr/lib/python3.6/doctest.py", line 1330, in __run
        compileflags, 1), test.globs)
      File "<doctest bootstrap.RustBuild.bin_root[2]>", line 1, in <module>
        rb.bin_root() == os.path.join("build", "stage0")
    TypeError: bin_root() missing 1 required positional argument: 'stage0'
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 720, in bootstrap.RustBuild.bin_root
Failed example:
    rb.bin_root() == os.path.join("build", "devel", "stage0")
Exception raised:
    Traceback (most recent call last):
      File "/usr/lib/python3.6/doctest.py", line 1330, in __run
        compileflags, 1), test.globs)
      File "<doctest bootstrap.RustBuild.bin_root[4]>", line 1, in <module>
        rb.bin_root() == os.path.join("build", "devel", "stage0")
    TypeError: bin_root() missing 1 required positional argument: 'stage0'

======================================================================
FAIL: program_config (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.program_config
Doctest: bootstrap.RustBuild.program_config
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/usr/lib/python3.6/doctest.py", line 2199, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.program_config
  File "/checkout/src/bootstrap/bootstrap.py", line 800, in program_config
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 809, in bootstrap.RustBuild.program_config
Failed example:
    cargo_path.rstrip(".exe") == os.path.join(rb.bin_root(),
    "bin", "cargo")
Exception raised:
    Traceback (most recent call last):
      File "/usr/lib/python3.6/doctest.py", line 1330, in __run
        compileflags, 1), test.globs)
      File "<doctest bootstrap.RustBuild.program_config[5]>", line 1, in <module>
        cargo_path.rstrip(".exe") == os.path.join(rb.bin_root(),
    TypeError: bin_root() missing 1 required positional argument: 'stage0'

======================================================================
FAIL: rustc_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.rustc_stamp
Doctest: bootstrap.RustBuild.rustc_stamp
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/usr/lib/python3.6/doctest.py", line 2199, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.rustc_stamp
  File "/checkout/src/bootstrap/bootstrap.py", line 671, in rustc_stamp
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 676, in bootstrap.RustBuild.rustc_stamp
Failed example:
    rb.rustc_stamp() == os.path.join("build", "stage0", ".rustc-stamp")
Exception raised:
    Traceback (most recent call last):
      File "/usr/lib/python3.6/doctest.py", line 1330, in __run
        compileflags, 1), test.globs)
      File "<doctest bootstrap.RustBuild.rustc_stamp[2]>", line 1, in <module>
        rb.rustc_stamp() == os.path.join("build", "stage0", ".rustc-stamp")
    TypeError: rustc_stamp() missing 1 required positional argument: 'stage0'

----------------------------------------------------------------------
Ran 16 tests in 0.008s


FAILED (failures=3)
make: *** [check-bootstrap] Error 1
Makefile:53: recipe for target 'check-bootstrap' failed
