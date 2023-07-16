
---- [rustdoc] rustdoc/default-impl.rs stdout ----


executing x86_64-unknown-linux-gnu/stage2/bin/rustc /«BUILDDIR»/rustc-1.8.0~~nightly.20160209+dfsg1/src/test/auxiliary/rustdoc-default-impl.rs -L x86_64-unknown-linux-gnu/test/rustdoc/ --target=x86_64-unknown-linux-gnu --crate-type=dylib -L x86_64-unknown-linux-gnu/test/rustdoc/default-impl.stage2-x86_64-unknown-linux-gnu.rustdoc.libaux -C prefer-dynamic --out-dir x86_64-unknown-linux-gnu/test/rustdoc/default-impl.stage2-x86_64-unknown-linux-gnu.rustdoc.libaux -C link-args='-Wl,-Bsymbolic-functions -Wl,-z,relro' --cfg rtopt -C rpath -O -L x86_64-unknown-linux-gnu/rt
------stdout------------------------------

------stderr------------------------------
error: unknown lint: `l,_z,relro'`
note: requested on the command line with `-W l,_z,relro'`
error: aborting due to previous error

------------------------------------------
