
     Running `/root/git/rust/rust2/build/x86_64-unknown-freebsd/stage0/bin/rustc --crate-name bootstrap bootstrap/bin/main.rs --crate-type bin --emit=dep-info,link -C debug-assertions=off -C overflow-checks=on -C metadata=6a809e7c34284e41 -C extra-filename=-6a809e7c34284e41 --out-d
ir /root/git/rust/rust2/build/bootstrap/debug/deps -C incremental=/root/git/rust/rust2/build/bootstrap/debug/incremental -L dependency=/root/git/rust/rust2/build/bootstrap/debug/deps --extern time=/root/git/rust/rust2/build/bootstrap/debug/deps/libtime-3b66b75744ce2323.rlib --exter
n serde_derive=/root/git/rust/rust2/build/bootstrap/debug/deps/libserde_derive-cb886017e511511e.so --extern build_helper=/root/git/rust/rust2/build/bootstrap/debug/deps/libbuild_helper-16ab4b07b9953b2a.rlib --extern getopts=/root/git/rust/rust2/build/bootstrap/debug/deps/libgetopts
-36a890615bb502e9.rlib --extern serde_json=/root/git/rust/rust2/build/bootstrap/debug/deps/libserde_json-c9a6ee565ee8f059.rlib --extern lazy_static=/root/git/rust/rust2/build/bootstrap/debug/deps/liblazy_static-0b94e2d6af7f839e.rlib --extern toml=/root/git/rust/rust2/build/bootstra
p/debug/deps/libtoml-76752341452dc1e9.rlib --extern cmake=/root/git/rust/rust2/build/bootstrap/debug/deps/libcmake-9e8025efc41ffe3d.rlib --extern num_cpus=/root/git/rust/rust2/build/bootstrap/debug/deps/libnum_cpus-f673db66666375f8.rlib --extern libc=/root/git/rust/rust2/build/boot
strap/debug/deps/liblibc-6335dae651da4b2d.rlib --extern filetime=/root/git/rust/rust2/build/bootstrap/debug/deps/libfiletime-d6f893a57ef1c5fb.rlib --extern serde=/root/git/rust/rust2/build/bootstrap/debug/deps/libserde-b01b65b0fba78017.rlib --extern cc=/root/git/rust/rust2/build/bo
otstrap/debug/deps/libcc-823736f6ee40bca8.rlib --extern bootstrap=/root/git/rust/rust2/build/bootstrap/debug/deps/libbootstrap-064ed40d3714f7cc.rlib -Cdebuginfo=2`
    Finished dev [unoptimized] target(s) in 64.89 secs
running: /root/git/rust/rust2/build/bootstrap/debug/bootstrap build --jobs 16 --verbose
Traceback (most recent call last):
  File "./x.py", line 20, in <module>
    bootstrap.main()
  File "/root/git/rust/rust2/src/bootstrap/bootstrap.py", line 763, in main
    bootstrap()
  File "/root/git/rust/rust2/src/bootstrap/bootstrap.py", line 754, in bootstrap
    run(args, env=env, verbose=build.verbose)
  File "/root/git/rust/rust2/src/bootstrap/bootstrap.py", line 148, in run
    raise RuntimeError(err)
RuntimeError: failed to run: /root/git/rust/rust2/build/bootstrap/debug/bootstrap build --jobs 16 --verbose
      103.00 real       172.49 user        28.16 sys
