 rust
fn foo(args: ~[~str]) {          
    let one: @fn() -> uint = || {
        enum r { a }             
        a as uint                
    };                           
    let two: @fn() -> uint = || {
        enum r { a }             
        a as uint                
    };                           
    one(); two();                
}                                

fn main() {}
