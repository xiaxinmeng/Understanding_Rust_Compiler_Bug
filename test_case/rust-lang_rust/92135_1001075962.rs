
cat results/cgann-aad4f1039fd5d6bf961ed08eebc6eb69b577f1be-475b00aa4037461b506539a06d15ca6091b461a7-keccak-Check-Full
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-aad4f1039fd5d6bf961ed08eebc6eb69b577f1be-keccak-Check-Full; results/cgfilt-475b00aa4037461b506539a06d15ca6091b461a7-keccak-Check-Full
Command:          /home/angel/.rustup/toolchains/aad4f1039fd5d6bf961ed08eebc6eb69b577f1be/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpeGwhH8/target/debug/deps -C linker=clang++-11 -L dependency=/tmp/.tmpeGwhH8/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /home/angel/.rustup/toolchains/475b00aa4037461b506539a06d15ca6091b461a7/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmp1P7Vd0/target/debug/deps -C linker=clang++-11 -L dependency=/tmp/.tmp1P7Vd0/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgdiff-aad4f1039fd5d6bf961ed08eebc6eb69b577f1be-475b00aa4037461b506539a06d15ca6091b461a7-keccak-Check-Full
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Thresholds:       0.1
Include dirs:
User annotated:
Auto-annotation:  off

--------------------------------------------------------------------------------
Ir
--------------------------------------------------------------------------------
960,870,160  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
963,910,535  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
 -1,120,020  ???:<rustc_middle::mir::Statement as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
