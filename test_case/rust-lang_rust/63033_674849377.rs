
#[allow(dead_code)]
struct SomeStruct<'a> {
    s: &'a str,
}

async fn something_asynchronous(_a: &'static str, _b: &str, _c: &SomeStruct<'_>) {}
