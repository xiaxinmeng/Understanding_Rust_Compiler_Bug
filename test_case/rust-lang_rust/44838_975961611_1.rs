
$ pytest example.py
============================= test session starts ==============================
platform linux -- Python 3.9.7, pytest-6.2.5, py-1.10.0, pluggy-0.13.1
rootdir: /tmp
collected 1 item

example.py F                                                             [100%]

=================================== FAILURES ===================================
___________________________________ test_foo ___________________________________

    def test_foo():
>       assert X > Y + Z
E       assert 1 > (2 + 3)

example.py:7: AssertionError
=========================== short test summary info ============================
FAILED example.py::test_foo - assert 1 > (2 + 3)
============================== 1 failed in 0.01s ===============================
