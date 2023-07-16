rust
let status_update: StatusUpdateStruct = get_status_update(i).deserialize();
StatusUpdate::new(utc_from_ticks(status_update.update_date), status_update.status_code, status_update.note))
