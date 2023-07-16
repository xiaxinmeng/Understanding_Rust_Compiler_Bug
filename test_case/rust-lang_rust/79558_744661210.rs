
echo '::run --crate-name tinyvec --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2  -C metadata=4b183dc7fcc0ca72 -C extra-filename=-4b183dc7fcc0ca72 --out-dir /tinyvec/target/debug/deps -C incremental=/tinyvec/target/debug/incremental -L dependency=/tinyvec/target/debug/deps
::stack' | mdb =rustc > mdb-tinyvec_compilation-20201214.txt 2>&1
