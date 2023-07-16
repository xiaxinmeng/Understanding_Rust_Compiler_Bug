plain
   Compiling ignore v0.4.17
   Compiling toml v0.5.7
    Finished dev [unoptimized] target(s) in 42.24s
Traceback (most recent call last):
  File "../x.py", line 27, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1324, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1310, in bootstrap
    run(args, env=env, verbose=build.verbose, is_bootstrap=True)
  File "/checkout/src/bootstrap/bootstrap.py", line 178, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python3.6/subprocess.py", line 729, in __init__
    restore_signals, start_new_session)
  File "/usr/lib/python3.6/subprocess.py", line 1364, in _execute_child
    raise child_exception_type(errno_num, err_msg, err_filename)
FileNotFoundError: [Errno 2] No such file or directory: '/checkout/obj/build/bootstrap/debug/bootstrap': '/checkout/obj/build/bootstrap/debug/bootstrap'
