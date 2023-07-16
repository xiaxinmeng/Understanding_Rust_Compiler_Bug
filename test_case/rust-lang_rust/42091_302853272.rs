
[00:01:11] # configuration written to .config
[00:01:11] #
[00:01:11] + hide_output ct-ng build
[00:01:11] + set +x
[00:01:41] Sat May 20 05:27:24 UTC 2017 - building ...
[00:02:11] Sat May 20 05:27:54 UTC 2017 - building ...
[00:02:41] Sat May 20 05:28:24 UTC 2017 - building ...
[00:03:11] Sat May 20 05:28:54 UTC 2017 - building ...
[00:03:41] Sat May 20 05:29:24 UTC 2017 - building ...
[00:04:11] Sat May 20 05:29:54 UTC 2017 - building ...
[00:04:41] Sat May 20 05:30:24 UTC 2017 - building ...
[00:05:11] Sat May 20 05:30:54 UTC 2017 - building ...
[00:05:41] Sat May 20 05:31:24 UTC 2017 - building ...
[00:05:57] ERROR: An error was encountered with the build.
[INFO ]  Performing some trivial sanity checks
[INFO ]  Build started 20170520.052654
[INFO ]  Building environment variables
[INFO ]  =================================================================
[INFO ]  Retrieving needed toolchain components' tarballs
[ERROR]  
[ERROR]  >>
[ERROR]  >>  Build failed in step 'Retrieving needed toolchain components' tarballs'
[ERROR]  >>        called in step '(top-level)'
[ERROR]  >>
[ERROR]  >>  Error happened in: do_isl_get[scripts/build/companion_libs/121-isl.sh@741]
[ERROR]  >>        called from: do_companion_libs_get[scripts/build/companion_libs.sh@15]
[ERROR]  >>        called from: main[scripts/crosstool-NG.sh@591]
[ERROR]  >>
[ERROR]  >>  For more info on this error, look at the file: 'build.log'
[ERROR]  >>  There is a list of known issues, some with workarounds, in:
[ERROR]  >>      '/usr/local/share/doc/crosstool-ng/crosstool-ng-1.22.0/B - Known issues.txt'
[ERROR]  
[ERROR]  (elapsed: 4:46.72)
