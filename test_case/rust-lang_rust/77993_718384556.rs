Rust
use std::error::Error;

use sqlx::PgConnection;
use ssh2::Sftp;

async fn minimal_example(
    mut _db_connection: PgConnection,
    _sftp_connection: Sftp,
) -> Result<(), Box<dyn Error>> {
    todo!()
}
