
error: The specified procedure could not be found. (os error 127)
   --> C:\Users\aminy\.cargo\git\checkouts\argh-4cd5f7352e552d74\cd89f02\argh\src\lib.rs:176:9
    |
176 | pub use argh_derive::FromArgs;
    |         ^^^^^^^^^^^

error: could not compile `argh` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name argh --edition=2018 C:\Users\aminy\.cargo\git\checkouts\argh-4cd5f7352e552d74\cd89f02\argh\src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 -C metadata=8f8c707a491e159a -C extra-filename=-8f8c707a491e159a --out-dir C:\Users\aminy\project\target\debug\deps -L dependency=C:\Users\aminy\project\target\debug\deps --extern argh_derive=C:\Users\aminy\project\target\debug\deps\argh_derive-6fcd7eb30e8474fa.dll --extern argh_shared=C:\Users\aminy\project\target\debug\deps\libargh_shared-03fa3a3743ff76a9.rmeta --cap-lints allow -C linker=clang` (exit code: 1)
warning: build failed, waiting for other jobs to finish...
