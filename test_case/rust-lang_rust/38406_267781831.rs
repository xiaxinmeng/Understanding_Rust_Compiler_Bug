
asm!("blx $0" : : "r"(foo as fn()));
