plain
configure: 
configure: run `python /checkout/x.py --help`
configure: 
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 25, in acquire_lock
    con = sqlite3.Connection(path, timeout=0)
sqlite3.OperationalError: unable to open database file

During handling of the above exception, another exception occurred:
Traceback (most recent call last):
Traceback (most recent call last):
  File "../x.py", line 27, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1294, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1256, in bootstrap
    lock = acquire_lock(build.rust_root)
  File "/checkout/src/bootstrap/bootstrap.py", line 35, in acquire_lock
    del con
UnboundLocalError: local variable 'con' referenced before assignment
