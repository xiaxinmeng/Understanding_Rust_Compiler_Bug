
$ ./configure 
configure: processing command line
configure: 
configure: build.configure-args := []
0 build
1 configure-args
configure: profile              := user
0 profile
Traceback (most recent call last):
  File "./src/bootstrap/configure.py", line 449, in <module>
    raise RuntimeError("config key {} not in sections".format(section_key))
RuntimeError: config key profile not in sections
