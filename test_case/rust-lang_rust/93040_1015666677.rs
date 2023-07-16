
~/rust $ ./x.py build && ./x.py install
Updating only changed submodules
  Submodules updated in 0.32 seconds
extracting /data/data/com.termux/files/home/rust/build/cache/2021-11-30/rust-std-beta-armv7-linux-androideabi.tar.xz
Traceback (most recent call last):
  File "/data/data/com.termux/files/home/rust/./x.py", line 27, in <module>
    bootstrap.main()
  File "/data/data/com.termux/files/home/rust/src/bootstrap/bootstrap.py", line 1313, in main
    bootstrap(help_triggered)
  File "/data/data/com.termux/files/home/rust/src/bootstrap/bootstrap.py", line 1279, in bootstrap
    build.download_toolchain()
  File "/data/data/com.termux/files/home/rust/src/bootstrap/bootstrap.py", line 453, in download_toolchain
    self._download_component_helper(filename, "rustc", tarball_suffix, stage0)
  File "/data/data/com.termux/files/home/rust/src/bootstrap/bootstrap.py", line 591, in _download_component_helper
    get(
  File "/data/data/com.termux/files/home/rust/src/bootstrap/bootstrap.py", line 73, in get
    raise RuntimeError("src/stage0.json doesn't contain a checksum for {}".format(url))
RuntimeError: src/stage0.json doesn't contain a checksum for dist/2021-11-30/rustc-beta-armv7-linux-androideabi.tar.xz
