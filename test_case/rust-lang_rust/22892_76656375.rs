 rust
asm!("out %al, %dx" :: "{ax}" (byte), "{dx}" (port) :: "volatile");
