 bash
rustc dep/dep.rs --crate-name dep --crate-type lib -C metadata=deadbeefdeadbeef -C extra-filename=-deadbeefdeadbeef --out-dir target/deps --dep-info target/.fingerprint/dep-deadbeefdeadbeef/dep-lib-dep -L target/deps -L target/deps
