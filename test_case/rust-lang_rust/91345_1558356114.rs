
use tracing::debug;
client
    .protocol("https")
    .method("GET")
    .send()
    .await // returns `Result<_, _>
    .inspect(|result| debug!(?result))
    .map_err(Error::from)
