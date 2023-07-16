
if config.bitcode_needed() {
            let _timer = cgcx
                .prof
                .generic_activity_with_arg("LLVM_module_codegen_make_bitcode", &*module.name);
            let thin = ThinBuffer::new(llmod); // <=== This line turns the BC module into thinLTO one
