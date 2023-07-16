plain
######################################################################    98.4%
######################################################################## 100.0%
[00:01:08] extracting /checkout/obj/build/cache/2018-04-04/rustc-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:08] Traceback (most recent call last):
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 838, in <module>
[00:01:08]     main()
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 821, in main
[00:01:08]     bootstrap(help_triggered)
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 798, in bootstrap
[00:01:08]     build.download_stage0()
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 365, in download_stage0
[00:01:08]     lib_path = os.path.join(self.bin_root(), "lib", "rustlib", self.host, "lib")
[00:01:08]   File "/usr/lib/python2.7/posixpath.py", line 68, in join
[00:01:08]     if b.startswith('/'):
[00:01:08] AttributeError: 'NoneType' object has no attribute 'startswith'
[00:01:08] Makefile:81: recipe for target 'prepare' failed
[00:01:08] make: *** [prepare] Error 1
[00:01:08] Traceback (most recent call last):
[00:01:08] Traceback (most recent call last):
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 838, in <module>
[00:01:08]     main()
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 821, in main
[00:01:08]     bootstrap(help_triggered)
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 798, in bootstrap
[00:01:08]     build.download_stage0()
[00:01:08]   File "/checkout/src/bootstrap/bootstrap.py", line 365, in download_stage0
[00:01:08]     lib_path = os.path.join(self.bin_root(), "lib", "rustlib", self.host, "lib")
[00:01:08]   File "/usr/lib/python2.7/posixpath.py", line 68, in join
[00:01:08]     if b.startswith('/'):
[00:01:08] AttributeError: 'NoneType' object has no attribute 'startswith'
[00:01:09] make: *** [prepare] Error 1
[00:01:09] Makefile:81: recipe for target 'prepare' failed
[00:01:09] Traceback (most recent call last):
[00:01:09] Traceback (most recent call last):
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 838, in <module>
[00:01:09]     main()
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 821, in main
[00:01:09]     bootstrap(help_triggered)
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 798, in bootstrap
[00:01:09]     build.download_stage0()
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 365, in download_stage0
[00:01:09]     lib_path = os.path.join(self.bin_root(), "lib", "rustlib", self.host, "lib")
[00:01:09]   File "/usr/lib/python2.7/posixpath.py", line 68, in join
[00:01:09]     if b.startswith('/'):
[00:01:09] AttributeError: 'NoneType' object has no attribute 'startswith'
[00:01:09] make: *** [prepare] Error 1
[00:01:09] Makefile:81: recipe for target 'prepare' failed
[00:01:09] Traceback (most recent call last):
[00:01:09] Traceback (most recent call last):
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 838, in <module>
[00:01:09]     main()
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 821, in main
[00:01:09]     bootstrap(help_triggered)
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 798, in bootstrap
[00:01:09]     build.download_stage0()
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 365, in download_stage0
[00:01:09]     lib_path = os.path.join(self.bin_root(), "lib", "rustlib", self.host, "lib")
[00:01:09]   File "/usr/lib/python2.7/posixpath.py", line 68, in join
[00:01:09]     if b.startswith('/'):
[00:01:09] AttributeError: 'NoneType' object has no attribute 'startswith'
[00:01:09] make: *** [prepare] Error 1
[00:01:09] Makefile:81: recipe for target 'prepare' failed
[00:01:09] Traceback (most recent call last):
[00:01:09] Traceback (most recent call last):
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 838, in <module>
[00:01:09]     main()
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 821, in main
[00:01:09]     bootstrap(help_triggered)
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 798, in bootstrap
[00:01:09]     build.download_stage0()
[00:01:09]   File "/checkout/src/bootstrap/bootstrap.py", line 365, in download_stage0
[00:01:09]     lib_path = os.path.join(self.bin_root(), "lib", "rustlib", self.host, "lib")
[00:01:09]   File "/usr/lib/python2.7/posixpath.py", line 68, in join
[00:01:09]     if b.startswith('/'):
[00:01:09] AttributeError: 'NoneType' object has no attribute 'startswith'
[00:01:09] make: *** [prepare] Error 1
[00:01:09] Makefile:81: recipe for target 'prepare' failed
[00:01:09] The command has failed after 5 attempts.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:01173f00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
