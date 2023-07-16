
thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow
error: could not compile `icu_uniset`

Caused by:
  process didn't exit successfully: `rustc --crate-name icu_uniset --edition=2018 components/uniset/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="provider_serde"' --cfg 'feature="serde"' -C metadata=43bc661b47995493 -C extra-filename=-43bc661b47995493 --out-dir /tmp/tmp.C992JMFlCk/icu4x/target/debug/deps -C incremental=/tmp/tmp.C992JMFlCk/icu4x/target/debug/incremental -L dependency=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps --extern displaydoc=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps/libdisplaydoc-9150e0fe01bb7d90.so --extern icu_provider=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps/libicu_provider-e7f972a43d9578f4.rmeta --extern litemap=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps/liblitemap-cbd4369110d76085.rmeta --extern serde=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps/libserde-c4032796bbea7b74.rmeta --extern tinystr=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps/libtinystr-c47caeef66ebb1a7.rmeta --extern zerovec=/tmp/tmp.C992JMFlCk/icu4x/target/debug/deps/libzerovec-8b294284a19c2ce4.rmeta` (signal: 6, SIGABRT: process abort signal)
