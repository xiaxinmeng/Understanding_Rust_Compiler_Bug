
use futures::future::FutureExt;

fn main() {
    let _ = async {
        async {
            async {}.await; // await inside a `.catch_unwind()` currently fails
        }
        .catch_unwind().await
    };
}
