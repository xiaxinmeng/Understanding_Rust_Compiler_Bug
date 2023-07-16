plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
Attempting with retry: make prepare
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 961, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 944, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 901, in bootstrap
    build.check_vendored_status()
  File "/checkout/src/bootstrap/bootstrap.py", line 839, in check_vendored_status
    shutil.rmtree(cargo_dir)
  File "/usr/lib/python3.6/shutil.py", line 490, in rmtree
    onerror(os.rmdir, path, sys.exc_info())
  File "/usr/lib/python3.6/shutil.py", line 488, in rmtree
    os.rmdir(path)
PermissionError: [Errno 13] Permission denied: '/cargo'
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 2/5:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 961, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 944, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 901, in bootstrap
    build.check_vendored_status()
  File "/checkout/src/bootstrap/bootstrap.py", line 839, in check_vendored_status
    shutil.rmtree(cargo_dir)
  File "/usr/lib/python3.6/shutil.py", line 490, in rmtree
    onerror(os.rmdir, path, sys.exc_info())
  File "/usr/lib/python3.6/shutil.py", line 488, in rmtree
    os.rmdir(path)
PermissionError: [Errno 13] Permission denied: '/cargo'
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 3/5:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 961, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 944, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 901, in bootstrap
    build.check_vendored_status()
  File "/checkout/src/bootstrap/bootstrap.py", line 839, in check_vendored_status
    shutil.rmtree(cargo_dir)
  File "/usr/lib/python3.6/shutil.py", line 490, in rmtree
    onerror(os.rmdir, path, sys.exc_info())
  File "/usr/lib/python3.6/shutil.py", line 488, in rmtree
    os.rmdir(path)
PermissionError: [Errno 13] Permission denied: '/cargo'
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 4/5:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 961, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 944, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 901, in bootstrap
    build.check_vendored_status()
  File "/checkout/src/bootstrap/bootstrap.py", line 839, in check_vendored_status
    shutil.rmtree(cargo_dir)
  File "/usr/lib/python3.6/shutil.py", line 490, in rmtree
    onerror(os.rmdir, path, sys.exc_info())
  File "/usr/lib/python3.6/shutil.py", line 488, in rmtree
    os.rmdir(path)
PermissionError: [Errno 13] Permission denied: '/cargo'
Makefile:58: recipe for target 'prepare' failed
Command failed. Attempt 5/5:
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 961, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 944, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 901, in bootstrap
    build.check_vendored_status()
  File "/checkout/src/bootstrap/bootstrap.py", line 839, in check_vendored_status
    shutil.rmtree(cargo_dir)
  File "/usr/lib/python3.6/shutil.py", line 490, in rmtree
    onerror(os.rmdir, path, sys.exc_info())
  File "/usr/lib/python3.6/shutil.py", line 488, in rmtree
    os.rmdir(path)
PermissionError: [Errno 13] Permission denied: '/cargo'
Makefile:58: recipe for target 'prepare' failed
The command has failed after 5 attempts.
