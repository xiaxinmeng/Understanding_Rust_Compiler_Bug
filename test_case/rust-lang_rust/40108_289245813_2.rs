
➜  rust git:(master) ./x.py build && sudo ./x.py dist --install
Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
  File "/home/user/Sources/rust/src/bootstrap/bootstrap.py", line 602, in main
    bootstrap()
  File "/home/user/Sources/rust/src/bootstrap/bootstrap.py", line 577, in bootstrap
    shutil.rmtree('.cargo')
  File "/usr/lib/python2.7/shutil.py", line 252, in rmtree
    onerror(os.remove, fullname, sys.exc_info())
  File "/usr/lib/python2.7/shutil.py", line 250, in rmtree
    os.remove(fullname)
OSError: [Errno 13] Permission denied: '.cargo/config'
