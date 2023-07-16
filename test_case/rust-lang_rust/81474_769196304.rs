rust
#![allow(unused)]

trait Empty {
    
}

fn launder<'a>(t:*mut (dyn Empty + 'a)) -> *mut (dyn Empty + 'a) {
    t
}

fn test(e: &mut dyn Empty) {
    launder(e as *mut dyn Empty);
}
