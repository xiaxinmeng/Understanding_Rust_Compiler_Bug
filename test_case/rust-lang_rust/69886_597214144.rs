
make[2]: warning: -jN forced in submake: disabling jobserver mode.
/cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/missing: line 81: gperf: command not found
WARNING: 'gperf' is missing on your system.
         You might have modified some files without having the proper
         tools for further handling them.  Check the 'README' file, it
         often tells you about the needed prerequisites for installing
         this package.  You may also peek at any GNU archive site, in
         case some other package contains this missing 'gperf' program.
make[4]: *** [fcobjshash.h] Error 1
make[4]: *** Waiting for unfinished jobs....
make[3]: *** [all-recursive] Error 1
make[2]: *** [all] Error 2
make[1]: *** [/checkout/obj/build/ct/webrender/target/debug/build/servo-fontconfig-sys-e3947402e30e86b7/out/libfontconfig.a] Error 2
thread 'main' panicked at 'assertion failed: Command::new("make").env("MAKEFLAGS",
                         env::var("CARGO_MAKEFLAGS").unwrap_or_default()).args(&["-R",
                                                                                 "-f",
                                                                                 "makefile.cargo"]).status().unwrap().success()', /cargo/registry/src/github.com-1ecc6299db9ec823/servo-fontconfig-sys-4.0.4/build.rs:17:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
