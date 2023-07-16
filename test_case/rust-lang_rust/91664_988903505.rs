plain
  network time: Wed, 08 Dec 2021 14:57:33 GMT
== end clock drift check ==
sccache: Starting the server...
Traceback (most recent call last):
  File "/checkout/src/bootstrap/configure.py", line 12, in <module>
    import bootstrap
  File "/checkout/src/bootstrap/bootstrap.py", line 15, in <module>
    import sqlite3
  File "/rustroot/lib/python3.9/sqlite3/__init__.py", line 23, in <module>
    from sqlite3.dbapi2 import *
  File "/rustroot/lib/python3.9/sqlite3/dbapi2.py", line 27, in <module>
    from _sqlite3 import *
ModuleNotFoundError: No module named '_sqlite3'
