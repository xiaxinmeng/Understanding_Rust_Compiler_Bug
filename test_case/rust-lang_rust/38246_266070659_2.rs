
warning: function is never used: `mk_naive_date`, #[warn(dead_code)] on by default
   --> tests/types_roundtrip.rs:131:1
    |
131 | >  pub fn mk_naive_date(days: u32) -> NaiveDate {
132 | |      let earliest_pg_date = NaiveDate::from_ymd(-4713, 11, 24);
133 | |      let latest_chrono_date = date::MAX;
134 | |      let num_days_representable = (latest_chrono_date - earliest_pg_date).num_days();
135 | |      earliest_pg_date + Duration::days(days as i64 % num_days_representable)
136 | >  }
    | ^ this block's help text here
