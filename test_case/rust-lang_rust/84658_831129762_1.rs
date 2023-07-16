rust
        asm!(
            "xchg rsi, rbx",
            "enclu",
            "xchg rsi, rbx",

            in("rax") EREPORT,
            in("rsi") ti.as_ptr(),
            in("rcx") data.0.as_ptr(),
            in("rdx") report.as_mut_ptr(),
        );
