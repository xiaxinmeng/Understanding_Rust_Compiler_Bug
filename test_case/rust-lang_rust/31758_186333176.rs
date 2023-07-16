
cargo build --release
objdump -h target/release/liblink_section.a | grep .custom_section
