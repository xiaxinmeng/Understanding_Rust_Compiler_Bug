
pub async fn publish(
    conn: &mut StateConnection,
    file: &FileID,
    user: &UserID,
    latest: ChangeID,
    mut subscribers: Vec<Box<dyn ChangeSubscriber>>,
) -> Result<(), ObjError> {
    let to_publish = gather_changes(conn, file, latest).await?;
    for sub in subscribers {
        sub.get_changes(file, user, latest, &to_publish).await?;
    }
    Ok(())
}
