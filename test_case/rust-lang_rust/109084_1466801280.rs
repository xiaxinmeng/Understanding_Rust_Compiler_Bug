rust
 pub fn arg_expand_all(at_args: &[String]) -> Vec<String> { 
     let mut args = Vec::new(); 
     // don't care about argv[0]
     for arg in at_args.iter().skip(1) {
     ...
    }
}
