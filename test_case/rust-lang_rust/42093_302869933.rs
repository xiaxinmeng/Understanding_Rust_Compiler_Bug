
[00:26:22] ERROR: An error was encountered with the build.
[INFO ]  Performing some trivial sanity checks
[INFO ]  Build started 20170520.061733
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
[ERROR]  (elapsed: 4:53.35)
[04:54] / make: *** [build] Error 1
[00:26:22] The command '/bin/sh -c ./build-toolchains.sh' returned a non-zero code: 1
[00:26:22] The command has failed after 5 attempts.
