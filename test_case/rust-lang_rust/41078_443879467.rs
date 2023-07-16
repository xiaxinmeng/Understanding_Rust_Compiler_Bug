
struct My(i32);

fn call_with_ref<F>(some_closure: F) -> i32
where
    F: for<'a> Fn(&'a My) -> &'a i32,
{
    let value = My(0);
    *some_closure(&value)
}

fn main() {
    
    
    let _f = |arg: &My| &arg.0;
    //This doesn't work
    call_with_ref(_f);
    
    //This is ok.
    //call_with_ref(|arg: &My| &arg.0);
}
