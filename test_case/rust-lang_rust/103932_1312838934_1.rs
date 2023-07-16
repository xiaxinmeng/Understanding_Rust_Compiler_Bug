
$ cargo install rustfilt --force --verbose
    Updating crates.io index
  Installing rustfilt v0.2.1
   Compiling libc v0.2.137
   Compiling memchr v2.5.0
   Compiling unicode-width v0.1.10
   Compiling regex-syntax v0.6.28
     Running `rustc --crate-name build_script_build /home/shnatsel/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.137/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debug-assertions=off --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=d85166fb06ca004c -C extra-filename=-d85166fb06ca004c --out-dir /tmp/cargo-installsUrxpw/release/build/libc-d85166fb06ca004c -L dependency=/tmp/cargo-installsUrxpw/release/deps --cap-lints allow -Csplit-debuginfo=packed`
     Running `rustc --crate-name unicode_width /home/shnatsel/.cargo/registry/src/github.com-1ecc6299db9ec823/unicode-width-0.1.10/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto --cfg 'feature="default"' -C metadata=11e76149b1936834 -C extra-filename=-11e76149b1936834 --out-dir /tmp/cargo-installsUrxpw/release/deps -L dependency=/tmp/cargo-installsUrxpw/release/deps --cap-lints allow -Csplit-debuginfo=packed`
     Running `rustc --crate-name build_script_build --edition=2018 /home/shnatsel/.cargo/registry/src/github.com-1ecc6299db9ec823/memchr-2.5.0/build.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debug-assertions=off --cfg 'feature="default"' --cfg 'feature="std"' -C metadata=88d0c53e2b65722b -C extra-filename=-88d0c53e2b65722b --out-dir /tmp/cargo-installsUrxpw/release/build/memchr-88d0c53e2b65722b -L dependency=/tmp/cargo-installsUrxpw/release/deps --cap-lints allow -Csplit-debuginfo=packed`
     Running `rustc --crate-name regex_syntax --edition=2018 /home/shnatsel/.cargo/registry/src/github.com-1ecc6299db9ec823/regex-syntax-0.6.28/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto --cfg 'feature="default"' --cfg 'feature="unicode"' --cfg 'feature="unicode-age"' --cfg 'feature="unicode-bool"' --cfg 'feature="unicode-case"' --cfg 'feature="unicode-gencat"' --cfg 'feature="unicode-perl"' --cfg 'feature="unicode-script"' --cfg 'feature="unicode-segment"' -C metadata=e35256232c1948d0 -C extra-filename=-e35256232c1948d0 --out-dir /tmp/cargo-installsUrxpw/release/deps -L dependency=/tmp/cargo-installsUrxpw/release/deps --cap-lints allow -Csplit-debuginfo=packed`
error: failed to build archive: No such file or directory

error: could not compile `unicode-width` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name unicode_width /home/shnatsel/.cargo/registry/src/github.com-1ecc6299db9ec823/unicode-width-0.1.10/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto --cfg 'feature="default"' -C metadata=11e76149b1936834 -C extra-filename=-11e76149b1936834 --out-dir /tmp/cargo-installsUrxpw/release/deps -L dependency=/tmp/cargo-installsUrxpw/release/deps --cap-lints allow -Csplit-debuginfo=packed` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
error: could not compile `regex-syntax` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name regex_syntax --edition=2018 /home/shnatsel/.cargo/registry/src/github.com-1ecc6299db9ec823/regex-syntax-0.6.28/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C linker-plugin-lto --cfg 'feature="default"' --cfg 'feature="unicode"' --cfg 'feature="unicode-age"' --cfg 'feature="unicode-bool"' --cfg 'feature="unicode-case"' --cfg 'feature="unicode-gencat"' --cfg 'feature="unicode-perl"' --cfg 'feature="unicode-script"' --cfg 'feature="unicode-segment"' -C metadata=e35256232c1948d0 -C extra-filename=-e35256232c1948d0 --out-dir /tmp/cargo-installsUrxpw/release/deps -L dependency=/tmp/cargo-installsUrxpw/release/deps --cap-lints allow -Csplit-debuginfo=packed` (exit status: 1)
error: failed to compile `rustfilt v0.2.1`, intermediate artifacts can be found at `/tmp/cargo-installsUrxpw`
