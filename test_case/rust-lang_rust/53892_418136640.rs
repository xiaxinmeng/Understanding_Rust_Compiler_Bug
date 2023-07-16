
fn what_date_time_is_in_zurich() -> NaiveDateTime {
  let now = SystemTime::now();
  let now_in_zurich = now - Duration::from_seconds(2 * 3600);
  now_in_zurich.to_utc()
}
