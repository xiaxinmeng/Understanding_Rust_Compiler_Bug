
#![feature(async_await,futures_api,await_macro)]
use futures::stream::{ repeat, StreamExt, StreamObj };

struct Response<'a, T> {
    stream: StreamObj<'a, T>
}

async fn do_stuff() {
    let mut s = repeat(10).map(|v| v+1).chunks(100);
    let mut response_a_tron = Response {
        stream: StreamObj::new(&mut s)
    };
    while let Some(item) = await!(response_a_tron.stream.next()) {
        assert_eq!(item.len(), 100)
    };
}

pub fn main() {
    fahrenheit::run(do_stuff());
}
