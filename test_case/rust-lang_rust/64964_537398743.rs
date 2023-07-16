rust
// compile-flags: -Z query-dep-graph --edition=2018

struct Body;

impl Body {
    async fn next(&mut self) {
        async { }.await
    }
}
