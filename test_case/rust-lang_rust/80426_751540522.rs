plain
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
----------------------------------------------------------------------
Ran 16 tests in 0.006s

FAILED (errors=3)
FAILED (errors=3)
make: *** [check-bootstrap] Error 1
Makefile:49: recipe for target 'check-bootstrap' failed
