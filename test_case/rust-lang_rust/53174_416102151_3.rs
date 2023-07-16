
#![feature(async_await,futures_api,await_macro)]
use futures::Future;
use futures::stream::{ repeat, StreamExt, StreamObj };

struct Response<'a, T> {
    stream: StreamObj<'a, T>
}

fn do_stuff<'a>(mut r: Response<'a, Vec<u8>>) -> impl Future<Output=()> + 'a {
    async move {
        while let Some(item) = await!(r.stream.next()) {
            assert_eq!(item.len(), 100)
        }
    }
}

pub fn main() {
    let mut s = repeat(10).map(|v| v+1).chunks(100);
    let r = Response {
        stream: StreamObj::new(&mut s)
    };

    fahrenheit::run(do_stuff(r));
}
