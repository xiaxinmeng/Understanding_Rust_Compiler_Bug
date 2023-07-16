 Rust
asm!( assembler template      // literal string
       : output operands      // optional, format: "constraint1"(expr1), "constraint2"(expr2), etc
       : input operands     // optional, format: "constraint1"(expr1), "constraint2"(expr2), etc
       : clobbers            // optional, format: "%eax", "%ebx", "memory", etc
       : options             // optional, comma separated strings
       );
