
1.40:

$ wasm-pack build --target web --
236K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features
240K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features --features wee_alloc
232K    pkg/squooshhqx_bg_snip_opt.wasm

1.41:

$ wasm-pack build --target web --
324K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features
328K    pkg/squooshhqx_bg_snip_opt.wasm

$ wasm-pack build --target web -- --no-default-features --features wee_alloc
324K    pkg/squooshhqx_bg_snip_opt.wasm
