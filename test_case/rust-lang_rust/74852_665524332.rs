
thread 'lto::dev_profile' panicked at '
Expected: execs
    but: differences:
  6 - |[RUNNING] `rustc --crate-name foo [..]--crate-type lib --emit=dep-info,metadata,link -Clinker-plugin-lto [..]|
    + |     Running `rustc --crate-name foo --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --emit=dep-info,link -Cembed-bitcode=no -C debuginfo=2 --test -C metadata=110e52c96c1b8618 -C extra-filename=-110e52c96c1b8618 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps/libbar-d2876efba7515ca8.rlib`|

  7 - |[RUNNING] `rustc --crate-name foo [..]--emit=dep-info,link -Cembed-bitcode=no [..]--test[..]|
    + |     Running `rustc --crate-name foo --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata,link -Clinker-plugin-lto -C debuginfo=2 -C metadata=aae1986730f9b7d9 -C extra-filename=-aae1986730f9b7d9 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps --extern bar=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t1067/foo/target/debug/deps/libbar-d2876efba7515ca8.rmeta`|
