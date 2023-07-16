plain
configure: 
configure: build.configure-args := ['--enable-full-tools', '--enable-profiler', ' ...
configure: rust.codegen-backends := ['llvm']
Traceback (most recent call last):
  File "/checkout/src/bootstrap/configure.py", line 548, in <module>
    section_order, sections, targets = parse_args(sys.argv[1:])
  File "/checkout/src/bootstrap/configure.py", line 277, in parse_args
    apply_args(known_args, option_checking, config)
  File "/checkout/src/bootstrap/configure.py", line 370, in apply_args
    set('rust.codegen-backends', ['llvm'], config)
  File "/checkout/src/bootstrap/configure.py", line 303, in set
    value = value.split(',')
AttributeError: 'list' object has no attribute 'split'
