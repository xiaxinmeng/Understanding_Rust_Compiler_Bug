
#![feature(async_await,futures_api,await_macro)]
use futures::stream::{ repeat, StreamExt, StreamObj };

struct Response<'a, T> {
    stream: StreamObj<'a, T>
}

async fn do_stuff<'a>(r: Response<'a, Vec<u8>>) {
    while let Some(item) = await!(r.stream.next()) {
        assert_eq!(item.len(), 100)
    };
}

pub fn main() {
    let mut s = repeat(10).map(|v| v+1).chunks(100);
    let mut r = Response {
        stream: StreamObj::new(&mut s)
    };

    fahrenheit::run(do_stuff(r));
}
