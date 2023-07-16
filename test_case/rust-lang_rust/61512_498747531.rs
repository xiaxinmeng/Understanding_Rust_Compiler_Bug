rust
thread_local! {
    static TLS: Foobar = {
        access_tls();
        Foobar {}
    };
}
fn access_tls() {
    let _ = TLS.try_with( |_| {} );
}
