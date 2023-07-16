
let ch0: Receiver<i32> = ...;
let ch1: Receiver<String> = ...;
match select((ch0, ch1)) {
    (x|!) => println!("Got an i32: {}", x),
    (!|y) => println!("Got a String: {}", y),
}
