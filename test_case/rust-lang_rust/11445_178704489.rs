
fn foo(x: Vec<u8>) -> String {unsafe{std::mem::transmute(x)}}
