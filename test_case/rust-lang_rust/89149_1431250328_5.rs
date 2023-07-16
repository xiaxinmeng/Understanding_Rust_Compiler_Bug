
cd crates/boards
CARGO_INCREMENTAL=0 cargo build --bin=nucleof446re --target=thumbv7em-none-eabihf --features="dep-stm32f446" -vv
CARGO_INCREMENTAL=0 cargo build --bin=nucleof446re --target=thumbv7em-none-eabihf --features="dep-stm32f446" -vv
