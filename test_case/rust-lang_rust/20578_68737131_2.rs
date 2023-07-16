 rust
#![feature(unboxed_closures)]
fn foo(mut f: Box<FnMut()>) {
    f.call_mut(());          
}                            
