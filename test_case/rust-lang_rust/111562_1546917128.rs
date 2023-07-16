plain
  network time: Sun, 14 May 2023 14:48:59 GMT
== end clock drift check ==
sccache: Starting the server...
Traceback (most recent call last):
  File "/checkout/src/bootstrap/configure.py", line 548, in <module>
    section_order, sections, targets = parse_args(sys.argv[1:])
  File "/checkout/src/bootstrap/configure.py", line 277, in parse_args
configure: processing command line
    apply_args(known_args, option_checking, config)
  File "/checkout/src/bootstrap/configure.py", line 339, in apply_args
    build_triple = build(known_args)
  File "/checkout/src/bootstrap/configure.py", line 284, in build
    return bootstrap.default_build_triple(verbose=False)
TypeError: default_build_triple() missing 1 required positional argument: 'build_dir'
configure: build.configure-args := ['--set', 'rust.validate-mir-opts=3', '--enabl ...
configure: rust.validate-mir-opts := 3
configure: rust.codegen-units-std := 1
configure: dist.compression-profile := balanced
