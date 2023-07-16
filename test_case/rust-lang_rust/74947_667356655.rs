
commands run after each wasm-pack command:
wasm-snip target/wasm32-unknown-unknown/release/squooshhqx.wasm --snip-rust-panicking-code -o pkg/squooshhqx_bg_snip.wasm
wasm-opt -O3 --dce  -o pkg/squooshhqx_bg_snip_opt.wasm 

----

1.40.0:

$ wasm-pack build --target web --
n/a     pkg/squooshhqx_bg.wasm
668K target/wasm32-unknown-unknown/release/squooshhqx.wasm
304K    pkg/squooshhqx_bg_snip.wasm
236K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features
n/a     pkg/squooshhqx_bg.wasm
700K    target/wasm32-unknown-unknown/release/squooshhqx.wasm
308K    pkg/squooshhqx_bg_snip.wasm
240K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features --features wee_alloc
n/a     pkg/squooshhqx_bg.wasm
668K    target/wasm32-unknown-unknown/release/squooshhqx.wasm
304K    pkg/squooshhqx_bg_snip.wasm
232K    pkg/squooshhqx_bg_snip_opt.wasm

1.41.0:

$ wasm-pack build --target web --
384K    pkg/squooshhqx_bg.wasm
760K    target/wasm32-unknown-unknown/release/squooshhqx.wasm
392K    pkg/squooshhqx_bg_snip.wasm
324K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features
388K    pkg/squooshhqx_bg.wasm
792K    target/wasm32-unknown-unknown/release/squooshhqx.wasm
392K    pkg/squooshhqx_bg_snip.wasm
328K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features --features wee_alloc
384K    pkg/squooshhqx_bg.wasm
760K    target/wasm32-unknown-unknown/release/squooshhqx.wasm
392K    pkg/squooshhqx_bg_snip.wasm
324K    pkg/squooshhqx_bg_snip_opt.wasm
