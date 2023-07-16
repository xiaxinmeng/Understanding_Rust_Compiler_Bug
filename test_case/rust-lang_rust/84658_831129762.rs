rust
        asm!(
            "enclu",

            in("rax") EREPORT,
            in("rbx") ti.as_ptr(),
            in("rcx") data.0.as_ptr(),
            in("rdx") report.as_mut_ptr(),
        );
