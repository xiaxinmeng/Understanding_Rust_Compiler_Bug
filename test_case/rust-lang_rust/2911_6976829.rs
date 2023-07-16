
let x = thing;
do some(foo).map |foo, move x| {
    // do stuff with x, including possibly giving up ownership.
}
