plain
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 43.35s
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 1341, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1324, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1310, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/checkout/src/bootstrap/bootstrap.py", line 178, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python3.8/subprocess.py", line 858, in __init__
    self._execute_child(args, executable, preexec_fn, close_fds,
  File "/usr/lib/python3.8/subprocess.py", line 1704, in _execute_child
    raise child_exception_type(errno_num, err_msg, err_filename)
NotADirectoryError: [Errno 20] Not a directory: '/checkout/obj/build/bootstrap/debug/rustbuild-binary-dispatch-shim/'
Command failed. Attempt 2/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.17s
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 1341, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1324, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1310, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/checkout/src/bootstrap/bootstrap.py", line 178, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python3.8/subprocess.py", line 858, in __init__
    self._execute_child(args, executable, preexec_fn, close_fds,
  File "/usr/lib/python3.8/subprocess.py", line 1704, in _execute_child
    raise child_exception_type(errno_num, err_msg, err_filename)
NotADirectoryError: [Errno 20] Not a directory: '/checkout/obj/build/bootstrap/debug/rustbuild-binary-dispatch-shim/'
Command failed. Attempt 3/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.17s
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 1341, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1324, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1310, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/checkout/src/bootstrap/bootstrap.py", line 178, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python3.8/subprocess.py", line 858, in __init__
    self._execute_child(args, executable, preexec_fn, close_fds,
  File "/usr/lib/python3.8/subprocess.py", line 1704, in _execute_child
    raise child_exception_type(errno_num, err_msg, err_filename)
NotADirectoryError: [Errno 20] Not a directory: '/checkout/obj/build/bootstrap/debug/rustbuild-binary-dispatch-shim/'
Command failed. Attempt 4/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.17s
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 1341, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1324, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1310, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/checkout/src/bootstrap/bootstrap.py", line 178, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python3.8/subprocess.py", line 858, in __init__
    self._execute_child(args, executable, preexec_fn, close_fds,
  File "/usr/lib/python3.8/subprocess.py", line 1704, in _execute_child
    raise child_exception_type(errno_num, err_msg, err_filename)
NotADirectoryError: [Errno 20] Not a directory: '/checkout/obj/build/bootstrap/debug/rustbuild-binary-dispatch-shim/'
Command failed. Attempt 5/5:
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.16s
Traceback (most recent call last):
Traceback (most recent call last):
  File "/checkout/src/bootstrap/bootstrap.py", line 1341, in <module>
    main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1324, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1310, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/checkout/src/bootstrap/bootstrap.py", line 178, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python3.8/subprocess.py", line 858, in __init__
    self._execute_child(args, executable, preexec_fn, close_fds,
  File "/usr/lib/python3.8/subprocess.py", line 1704, in _execute_child
    raise child_exception_type(errno_num, err_msg, err_filename)
NotADirectoryError: [Errno 20] Not a directory: '/checkout/obj/build/bootstrap/debug/rustbuild-binary-dispatch-shim/'
The command has failed after 5 attempts.
