plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-05-20/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Traceback (most recent call last):
  File "../x.py", line 27, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1132, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1106, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 498, in download_toolchain
    self._download_component_helper(filename, "clippy-preview", tarball_suffix)
  File "/checkout/src/bootstrap/bootstrap.py", line 526, in _download_component_helper
    verbose=self.verbose,
  File "/checkout/src/bootstrap/bootstrap.py", line 76, in get
    .format(url))
RuntimeError: src/stage0.json doesn't contain a checksum for dist/2022-05-20/clippy-beta-x86_64-unknown-linux-gnu.tar.xz. Pre-built artifacts might not be available for this target at this time, see https://doc.rust-lang.org/nightly/rustc/platform-support.html for more information.
