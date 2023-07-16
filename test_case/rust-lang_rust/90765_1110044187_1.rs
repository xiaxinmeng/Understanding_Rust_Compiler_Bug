rs
macro_rules! helper {
    ({
        let $output:ident;
        let mut $x1:ident = $x2:ident;
        let mut $x3:ident = $e:expr;
    }) => {
        let mut $output = $e;
    };
}

let future = async {
    let value = 5;
    let reference = &value;
    yield_now().await;
    println!("{reference}");
};

expand_into!([{ let pinned; pin_mut!(future) }] helper);
// expands to: let pinned = unsafe { Pin::new_unchecked(&mut future) };
let _ = pinned.poll(cx);

let moved_future = future;
pin_mut!(moved_future);
moved_future.poll(cx); // oops!
