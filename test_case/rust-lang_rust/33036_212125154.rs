
TypeError                                 Traceback (most recent call last)
<ipython-input-1-6a5394a49675> in <module>()
--> 1 import bootstrap

rust/src/bootstrap/bootstrap.py in <module>()
    328 
    329 # Fetch/build the bootstrap
--> 330 rb.build = rb.build_triple()
    331 rb.parse_nightly_dates()
    332 rb.download_rust_nightly()

rust/src/bootstrap/bootstrap.py in build_triple(self)
    258         elif ostype == 'Darwin':
    259             ostype = 'apple-darwin'
--> 260         elif ostype.startswith('MINGW'):
    261             # msys' `uname` does not print gcc configuration, but prints msys
    262             # configuration. so we cannot believe `uname -m`:

TypeError: startswith first arg must be bytes or a tuple of bytes, not str
