plain
configure: 
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/obj/build/tmp/distcheck/x.py --help`
error: vendoring required, but vendor directory does not exist.
       Run `cargo vendor --sync ./src/tools/cargo/Cargo.toml --sync ./src/tools/rust-analyzer/Cargo.toml --sync ./compiler/rustc_codegen_cranelift/Cargo.toml --sync ./src/bootstrap/Cargo.toml ` to initialize the vendor directory.
Alternatively, use the pre-vendored `rustc-src` dist component.
  File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 1100, in <module>
    main()
  File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 1084, in main
    bootstrap(args)
    bootstrap(args)
  File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 1031, in bootstrap
    build.check_vendored_status()
  File "/checkout/obj/build/tmp/distcheck/src/bootstrap/bootstrap.py", line 965, in check_vendored_status
    raise Exception("{} not found".format(vendor_dir))
Exception: /checkout/obj/build/tmp/distcheck/vendor not found
make: *** [Makefile:42: check] Error 1
