
running: /home/pnkfelix/Mozilla/rust.git/build/powerpc64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /home/pnkfelix/Mozilla/rust.git/src/bootstrap/Cargo.toml --verbose
Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
  File "/home/pnkfelix/Mozilla/rust.git/src/bootstrap/bootstrap.py", line 763, in main
    bootstrap()
  File "/home/pnkfelix/Mozilla/rust.git/src/bootstrap/bootstrap.py", line 743, in bootstrap
    build.build_bootstrap()
  File "/home/pnkfelix/Mozilla/rust.git/src/bootstrap/bootstrap.py", line 621, in build_bootstrap
    run(args, env=env, verbose=self.verbose)
  File "/home/pnkfelix/Mozilla/rust.git/src/bootstrap/bootstrap.py", line 143, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python2.7/subprocess.py", line 711, in __init__
    errread, errwrite)
  File "/usr/lib/python2.7/subprocess.py", line 1343, in _execute_child
    raise child_exception
OSError: [Errno 2] No such file or directory
