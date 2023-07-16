
use tracing::debug;
let res = client
    .protocol("https")
    .method("GET")
    .send()
    .await; // returns `Result<_, _>
debug!(?res);
res.map_err(Error::from)
