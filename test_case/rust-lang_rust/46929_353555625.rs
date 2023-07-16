
    Updating git repository `https://127.0.0.1:2858/foo/bar`
error: failed to load source for a dependency on `bar`
Caused by:
  Unable to update https://127.0.0.1:2858/foo/bar
Caused by:
  failed to clone into: C:\projects\rust\build\x86_64-pc-windows-msvc\stage2-tools\x86_64-pc-windows-msvc\cit\t0\home\.cargo\git\db\bar-bb0ae0d2822494b3
Caused by:
  [35] SSL connect error (schannel: failed to receive handshake, SSL/TLS connection failed); class=Net (12)
', C:\Users\appveyor\.cargo\registry\src\github.com-1ecc6299db9ec823\hamcrest-0.1.1\src\core.rs:31:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
