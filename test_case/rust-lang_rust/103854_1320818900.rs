
error: future cannot be sent between threads safely
   --> src/lib.rs:159:44
    |
159 |     let server = Server::bind(&addr).serve(make_service);
    |                                            ^^^^^^^^^^^^ future created by async block is not `Send`
    |
    = help: within `hyper::proto::h2::server::H2Stream<impl Future<Output = Result<hyper::Response<Body>, Infallible>>, Body>`, the trait `Send` is not implemented for `impl Future<Output = Routing<<R as Router>::Response>>`
note: future is not `Send` as it awaits another future which is not `Send`
   --> src/lib.rs:142:42
    |
142 |                       let response = match routes
    |  __________________________________________^
143 | |                         .route(&mut State::default(), &request)
    | |_______________________________________________________________^ await occurs here on type `impl Future<Output = Routing<<R as Router>::Response>>`, which is not `Send`
note: required by a bound in `hyper::server::Builder::<I, E>::serve`
   --> /home/charles/.cargo/registry/src/github.com-1ecc6299db9ec823/hyper-0.14.23/src/server/server.rs:548:12
    |
548 |         E: ConnStreamExec<<S::Service as HttpService<Body>>::Future, B>,
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `hyper::server::Builder::<I, E>::serve`
help: consider further restricting the associated type
    |
132 |     H: IntoResponse, impl Future<Output = Routing<<R as Router>::Response>>: Send
    |                    ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
