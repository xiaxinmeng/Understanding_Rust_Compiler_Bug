
cargo rustc --release -- --emit asm
! cat target/release/deps/*.s | rg memset
