rust
type EvalFn = Fn() -> (u8);                            
fn take_callback(f: &EvalFn) -> u8 {
    1
} 

fn test(arg: Option<&u8>) -> (u8) {
    let closure = || arg.cloned().unwrap();
    take_callback(&closure)
}  
