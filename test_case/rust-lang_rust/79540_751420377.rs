plain

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-11-18/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Traceback (most recent call last):
  File "../x.py", line 11, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1203, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1171, in bootstrap
    build.download_stage0()
  File "/checkout/src/bootstrap/bootstrap.py", line 426, in download_stage0
    rust_stamp.write(self.date + self.stage1_commit)
TypeError: must be str, not NoneType
