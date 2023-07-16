rust
format!("{:#x}", CommaSeparated(slice)) == "0x0, 0x1, 0x2"

format!("[{:#x}]", CommaSeparated(slice)) == "[0x0, 0x1, 0x2]"

format!("{:02x}", Concatenated(slice)) == "000102"
