plain
configure: 
configure: run `python /checkout/x.py --help`
configure: 
Traceback (most recent call last):
  File "../x.py", line 27, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1295, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1256, in bootstrap
    os.mkdirs(build.build_dir, exist_ok=True)
AttributeError: module 'os' has no attribute 'mkdirs'
