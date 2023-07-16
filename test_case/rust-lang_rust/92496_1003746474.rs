plain
##########################                                                36.1%
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2021-11-30/cargo-beta-x86_64-unknown-linux-gnu.tar.xz
Traceback (most recent call last):
  File "../x.py", line 27, in <module>
    bootstrap.main()
  File "/checkout/src/bootstrap/bootstrap.py", line 1317, in main
    bootstrap(help_triggered)
  File "/checkout/src/bootstrap/bootstrap.py", line 1283, in bootstrap
    build.download_toolchain()
  File "/checkout/src/bootstrap/bootstrap.py", line 476, in download_toolchain
    shutil.copy(gcc_libs_hack + '/libgcc.a', gcc_libs_hack_dest)
  File "/usr/lib/python3.6/shutil.py", line 245, in copy
    copyfile(src, dst, follow_symlinks=follow_symlinks)
  File "/usr/lib/python3.6/shutil.py", line 120, in copyfile
    with open(src, 'rb') as fsrc:
FileNotFoundError: [Errno 2] No such file or directory: '/checkout/missing-libs-hack/libgcc.a'
