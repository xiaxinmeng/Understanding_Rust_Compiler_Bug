
let mut x = some(thing);
do some(foo).map |foo| {
    let mut y = none;
    x <-> y;
    let x = option::unwrap(y);
    // do stuff with x
}
