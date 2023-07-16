
fn func() ->impl FnOnce()->u8{
    || 123u8
}

fn main() {
    println!("{}", func()())
}
