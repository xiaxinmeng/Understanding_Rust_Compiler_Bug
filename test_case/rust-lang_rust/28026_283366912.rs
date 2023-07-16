rust
    // Mark all dynamic libraries and executables as compatible with the larger 4GiB a
ddress
    // space available to x86 Windows binaries on x86_64.
    base.pre_link_args.push("/LARGEADDRESSAWARE".to_string());
