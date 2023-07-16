rust
fn unwrap_transparent<T<_>, R: ReprTransparent>(value: T<R<_>>) -> T<_> { 
   unsafe { mem::transmute(value) } 
}

fn unwrap_uninit<T<_>>(value: T<MaybeUninit<_>>) -> T<_> { 
   unwrap_transparent(value);
}
