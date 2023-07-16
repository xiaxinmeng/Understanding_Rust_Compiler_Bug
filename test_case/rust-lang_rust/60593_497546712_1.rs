rust
   for proc_macro in *proc_macros {
        match proc_macro {
            ProcMacro::Bang { client, .. } => {
                unsafe {
                    let p1: &[*const (); 3] = std::mem::transmute(client);
                    let f = &proc_macro::bridge::client::__run_expand1;
                    println!("{:#?}", p1);
                    println!("{:#?}", f as *const _);
                }
                let result = client.run(
                    &EXEC_STRATEGY,
                    rustc_server::Rustc::default(),
                    parse_string("struct S{}").expect("Cannot parse code"),
                );
