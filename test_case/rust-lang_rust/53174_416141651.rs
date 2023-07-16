rust
async fn async_main() {
    let mut s = repeat(10).map(|v| v+1).chunks(100);
    let r = Response {
        stream: StreamObj::new(&mut s)
    };

    await!(do_stuff(r));
}

pub fn main() {
    fahrenheit::run(async_main());
}
