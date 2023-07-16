rust
async fn minimal_example(
    mut _db_connection: PgConnection,
    _sftp_connection: Sftp,
) -> Result<(), Box<dyn Error>> {
    println!("hello world");
    Ok(())
}
