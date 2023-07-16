 diff
+ .with(warp::reply::with::header("Sec-WebSocket-Protocol", "graphql-ws"))
- .map(|reply| {
-   warp::reply::with_header(reply, "Sec-WebSocket-Protocol", "graphql-ws")
- })
