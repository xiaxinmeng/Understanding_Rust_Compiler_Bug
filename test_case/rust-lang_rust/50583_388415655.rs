
# note that I'm using gnueabihf on ubuntu so the commands are a bit different
$ cargo +nightly -v rustc --target=arm-unknown-linux-gnueabihf -- -C linker=arm-linux-gnueabihf-gcc -Z print-link-args
   Compiling test-cross v0.1.0 (file:///home/matt/test-cross)
     Running `rustc --crate-name test_cross <snip>
# Paste the rustc invocation here so we get the print-link-args output
$ rustc --crate-name test_cross  <snip>
"arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" <snip>
# paste the gcc invocation here, and add `-v -Wl,--verbose`
$ "arm-linux-gnueabihf-gcc" "-Wl,--as-needed" "-Wl,-z,noexecstack" <snip> -v -Wl,--verbose
<lots of output>
