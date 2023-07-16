rust
pub async fn get_statuses() -> Vec<StatusUpdate> {
    // get_status_updates is also an `async fn`, but calling it works just like any other call:
    let count = get_status_updates();

    let mut tasks = vec![];
    for i in 0..count {
        // Isn't "just a construction" anymore
        task.push({
            let status_update: StatusUpdateStruct = get_status_update(i).deserialize();
            StatusUpdate::new(utc_from_ticks(status_update.update_date), status_update.status_code, status_update.note))
        });
    }
    tasks 
}
