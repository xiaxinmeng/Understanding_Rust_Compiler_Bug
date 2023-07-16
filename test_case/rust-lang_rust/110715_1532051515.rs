rust
if false {
    let _a = $expr;
} else {
    unsafe { 
        write_via_move(p, $expr);
    }
}
