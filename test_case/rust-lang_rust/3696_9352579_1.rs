
do a |c| {
    let d = c();
    d(); // 'd' cannot escape this block
}
