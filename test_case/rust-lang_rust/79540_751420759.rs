plain
##########                                                                14.2%
#####################################                                     52.2%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-18/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Wrote /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/.rustc-stamp.tmp

###############################                                           43.1%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
extracting /checkout/obj/build/cache/2020-11-19/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Wrote /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/.rustfmt-stamp.tmp
---
ERROR: test_dates_are_different (__main__.ProgramOutOfDate)
Return True when the dates are different
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap_test.py", line 87, in test_dates_are_different
    self.assertTrue(self.build.program_out_of_date(self.rustc_stamp_path))
TypeError: program_out_of_date() missing 1 required positional argument: 'key'
======================================================================
ERROR: test_same_dates (__main__.ProgramOutOfDate)
Return False both dates match
----------------------------------------------------------------------
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap_test.py", line 93, in test_same_dates
    self.assertFalse(self.build.program_out_of_date(self.rustc_stamp_path))
TypeError: program_out_of_date() missing 1 required positional argument: 'key'
======================================================================
ERROR: test_stamp_path_does_not_exists (__main__.ProgramOutOfDate)
Return True when the stamp file does not exists
----------------------------------------------------------------------
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap_test.py", line 81, in test_stamp_path_does_not_exists
    self.assertTrue(self.build.program_out_of_date(self.rustc_stamp_path))
TypeError: program_out_of_date() missing 1 required positional argument: 'key'
======================================================================
FAIL: stage0_sysroot (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.stage0_sysroot
----------------------------------------------------------------------
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/usr/lib/python3.6/doctest.py", line 2199, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.stage0_sysroot
  File "/checkout/src/bootstrap/bootstrap.py", line 741, in stage0_sysroot
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 746, in bootstrap.RustBuild.stage0_sysroot
Failed example:
    rb.stage0_sysroot() == os.path.join("build", "stage1")
    True
Got:
    False
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 752, in bootstrap.RustBuild.stage0_sysroot
Failed example:
    rb.stage0_sysroot() == os.path.join("build", "devel", "stage1")
    True
Got:
    False



======================================================================
FAIL: stage1_stamp (bootstrap.RustBuild)
Doctest: bootstrap.RustBuild.stage1_stamp
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/usr/lib/python3.6/doctest.py", line 2199, in runTest
    raise self.failureException(self.format_failure(new.getvalue()))
AssertionError: Failed doctest test for bootstrap.RustBuild.stage1_stamp
  File "/checkout/src/bootstrap/bootstrap.py", line 688, in stage1_stamp
----------------------------------------------------------------------
----------------------------------------------------------------------
File "/checkout/src/bootstrap/bootstrap.py", line 693, in bootstrap.RustBuild.stage1_stamp
Failed example:
    rb.stage1_stamp() == os.path.join("build", "stage1", ".stage1-stamp")
    True
Got:
    False



----------------------------------------------------------------------
Ran 18 tests in 0.010s

FAILED (failures=2, errors=3)
make: *** [check-bootstrap] Error 1
Makefile:49: recipe for target 'check-bootstrap' failed
