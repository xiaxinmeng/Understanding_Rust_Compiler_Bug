
[00:11:10] configure: run `python /checkout/obj/build/tmp/distcheck/x.py --help`
[00:11:10] configure: 
[00:11:10] Traceback (most recent call last):
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 870, in <module>
[00:11:10]     main()
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 853, in main
[00:11:10]     bootstrap(help_triggered)
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 810, in bootstrap
[00:11:10]     data = stage0_data(build.rust_root)
[00:11:10]   File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 158, in stage0_data
[00:11:10]     with open(nightlies, 'r') as nightlies:
[00:11:10] IOError: [Errno 2] No such file or directory: '/checkout/obj/build/tmp/distcheck/stage0.txt'
[00:11:10] Makefile:58: recipe for target 'check' failed
