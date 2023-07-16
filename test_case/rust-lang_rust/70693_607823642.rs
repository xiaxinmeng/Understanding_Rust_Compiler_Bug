rust
    base.pre_link_args_crt.get_mut(&LinkerFlavor::Gcc).unwrap().push("-static-pie".to_string());
    base.pre_link_args_crt.get_mut(&LinkerFlavor::Gcc).unwrap().push("-Wl,-pie".to_string());
    base.pre_link_objects_exe_crt.push("rcrt1.o".to_string());
