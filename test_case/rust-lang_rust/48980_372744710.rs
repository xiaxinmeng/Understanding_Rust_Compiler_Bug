rust
let times = [
            UNIX_EPOCH,
            UNIX_EPOCH + Duration::new(13, 23),
            SystemTime::now(),
            SystemTime::now() + Duration::new(17, 15),
            UNIX_EPOCH + Duration::new(0, u32::max_value()),
            UNIX_EPOCH + Duration::new(i64::max_value() as u64, 0),
            UNIX_EPOCH + Duration::new(i64::max_value() as u64, 999_999_999),
            UNIX_EPOCH + Duration::new(i64::max_value() as u64 - 1, 1_000_000_000),
            UNIX_EPOCH + Duration::new(i64::max_value() as u64 - 4, 4_000_000_000),
            UNIX_EPOCH + Duration::new(i64::max_value() as u64 - 4, u32::max_value()),
        ];
