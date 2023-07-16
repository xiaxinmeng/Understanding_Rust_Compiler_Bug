
sudo apt-get install curl
Reading package lists... Done
Building dependency tree
Reading state information... Done
curl is already the newest version (7.52.1-5+deb9u9).
0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.

linaro@captation-techforum-rk3399-2GB:~/ffmpeg_sources/rust$ ./x.py build && ./x.py install
Updating only changed submodules
Submodules updated in 0.18 seconds
Traceback (most recent call last):
  File "./x.py", line 11, in <module>
    bootstrap.main()
  File "/home/linaro/ffmpeg_sources/rust/src/bootstrap/bootstrap.py", line 866, in main
    bootstrap(help_triggered)
  File "/home/linaro/ffmpeg_sources/rust/src/bootstrap/bootstrap.py", line 837, in bootstrap
    build.build_bootstrap()
  File "/home/linaro/ffmpeg_sources/rust/src/bootstrap/bootstrap.py", line 648, in build_bootstrap
    run(args, env=env, verbose=self.verbose)
  File "/home/linaro/ffmpeg_sources/rust/src/bootstrap/bootstrap.py", line 136, in run
    ret = subprocess.Popen(args, **kwargs)
  File "/usr/lib/python2.7/subprocess.py", line 390, in __init__
    errread, errwrite)
  File "/usr/lib/python2.7/subprocess.py", line 1024, in _execute_child
    raise child_exception
OSError: [Errno 2] No such file or directory
