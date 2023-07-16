plain
  network time: Sat, 05 Nov 2022 21:20:26 GMT
== end clock drift check ==
sccache: Starting the server...
Traceback (most recent call last):
  File "/checkout/src/bootstrap/configure.py", line 469, in <module>
    configure_section(sections[section_key], section_config)
  File "/checkout/src/bootstrap/configure.py", line 457, in configure_section
    raise RuntimeError("failed to find config line for {}".format(key))
configure: 
configure: build.build          := x86_64-unknown-linux-gnu
RuntimeError: failed to find config line for static-stdcpp
configure: target.x86_64-unknown-linux-gnu.llvm-config := /usr/lib/llvm-13/bin/ll ...
