rust
let o = Some(unsafe { std::mem::zeroed::<&i32>() });
if o.is_none() {
    unreachable!("wtf!?")
}
